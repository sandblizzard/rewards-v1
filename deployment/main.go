package main

import (
	"encoding/base64"
	"errors"
	"strings"

	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/acm"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecr"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecs"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/lb"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/secretsmanager"

	"github.com/pulumi/pulumi-docker/sdk/v3/go/docker"

	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func authenticate(ctx *pulumi.Context, repo *ecr.Repository) docker.ImageRegistryOutput {
	registryInfo := repo.RegistryId.ApplyT(func(id string) (docker.ImageRegistry, error) {
		creds, err := ecr.GetCredentials(ctx, &ecr.GetCredentialsArgs{RegistryId: id})
		if err != nil {
			return docker.ImageRegistry{}, err
		}
		decoded, err := base64.StdEncoding.DecodeString(creds.AuthorizationToken)
		if err != nil {
			return docker.ImageRegistry{}, err
		}
		parts := strings.Split(string(decoded), ":")
		if len(parts) != 2 {
			return docker.ImageRegistry{}, errors.New("Invalid credentials")
		}
		return docker.ImageRegistry{
			Server:   creds.ProxyEndpoint,
			Username: parts[0],
			Password: parts[1],
		}, nil
	})

	return registryInfo.(docker.ImageRegistryOutput)
}

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {

		cfg := config.New(ctx, "")
		githubKey := cfg.RequireSecret("githubKey")
		underdogKey := cfg.RequireSecret("underdogKey")
		key := cfg.RequireSecret("key")

		// setup aws secret manager
		githubKeySecret, err := secretsmanager.NewSecret(ctx, "githubKey", nil)
		if err != nil {
			return err
		}

		// github Secret
		_, err = secretsmanager.NewSecretVersion(ctx, "githubKey-ver", &secretsmanager.SecretVersionArgs{
			SecretId:     githubKeySecret.ID(),
			SecretString: githubKey,
		})
		if err != nil {
			return err
		}

		// underdog Secret
		underdogKeySecret, err := secretsmanager.NewSecret(ctx, "underdogKey", nil)
		if err != nil {
			return err
		}
		_, err = secretsmanager.NewSecretVersion(ctx, "underdogKey-ver", &secretsmanager.SecretVersionArgs{
			SecretId:     underdogKeySecret.ID(),
			SecretString: underdogKey,
		})
		if err != nil {
			return err
		}

		// key secret
		keySecret, err := secretsmanager.NewSecret(ctx, "key", nil)
		if err != nil {
			return err
		}
		_, err = secretsmanager.NewSecretVersion(ctx, "key-ver", &secretsmanager.SecretVersionArgs{
			SecretId:     keySecret.ID(),
			SecretString: key,
		})
		if err != nil {
			return err
		}

		// setup aws ecs
		t := true
		vpc, err := ec2.LookupVpc(ctx, &ec2.LookupVpcArgs{Default: &t})
		if err != nil {
			return err
		}

		// get the subnets of the vpc
		subnet, err := ec2.GetSubnetIds(ctx, &ec2.GetSubnetIdsArgs{VpcId: vpc.Id})
		if err != nil {
			return err
		}

		// security group to allow traffic into the networks
		// allow for unlimited egresss
		// limited ingress to 80 on http and 443 on https
		securityGroup, err := ec2.NewSecurityGroup(ctx, "sandblizzard-sg", &ec2.SecurityGroupArgs{
			VpcId: pulumi.String(vpc.Id),
			Egress: ec2.SecurityGroupEgressArray{
				ec2.SecurityGroupEgressArgs{
					Protocol:   pulumi.String("-1"),
					FromPort:   pulumi.Int(0),
					ToPort:     pulumi.Int(0),
					CidrBlocks: pulumi.StringArray{pulumi.String("0.0.0.0/0")},
				},
			},
			Ingress: ec2.SecurityGroupIngressArray{
				ec2.SecurityGroupIngressArgs{
					Protocol:   pulumi.String("tcp"),
					FromPort:   pulumi.Int(80),
					ToPort:     pulumi.Int(80),
					CidrBlocks: pulumi.StringArray{pulumi.String("0.0.0.0/0")},
				},
				ec2.SecurityGroupIngressArgs{
					Protocol:   pulumi.String("tcp"),
					FromPort:   pulumi.Int(443),
					ToPort:     pulumi.Int(443),
					CidrBlocks: pulumi.StringArray{pulumi.String("0.0.0.0/0")},
				},
			},
		})
		if err != nil {
			return err
		}

		// set up load balancer that receives data on port 80
		// must be validated by the security group
		loadBalancer, err := lb.NewLoadBalancer(ctx, "sandblizzard-lb", &lb.LoadBalancerArgs{
			Subnets:        pulumi.ToStringArray(subnet.Ids),
			SecurityGroups: pulumi.StringArray{securityGroup.ID().ToStringOutput()},
		})
		if err != nil {
			return err
		}

		ctx.Export("loadBalancer", loadBalancer.DnsName)

		// target group hit by listener on http
		targetGroupHttp, err := lb.NewTargetGroup(ctx, "sandblizzard-tg-http", &lb.TargetGroupArgs{
			Port:       pulumi.Int(80),
			Protocol:   pulumi.String("HTTP"),
			TargetType: pulumi.String("ip"),
			VpcId:      pulumi.String(vpc.Id),
		})
		if err != nil {
			return err
		}

		// target group hit by listener on https
		targetGroupHttps, err := lb.NewTargetGroup(ctx, "sandblizzard-tg-https", &lb.TargetGroupArgs{
			Port:       pulumi.Int(443),
			Protocol:   pulumi.String("HTTPS"),
			TargetType: pulumi.String("ip"),
			VpcId:      pulumi.String(vpc.Id),
		})
		if err != nil {
			return err
		}

		// add DNS resolution for certificate
		sandblizzardCert, err := acm.NewCertificate(ctx, "sandblizzard-dapp-cert", &acm.CertificateArgs{
			DomainName:       pulumi.String("sandblizzard.app"),
			ValidationMethod: pulumi.String("DNS"),
		})
		if err != nil {
			return err
		}
		ctx.Export("sandblizzardCert", sandblizzardCert.Status)

		// create a new listener on port 80 and redirect to https
		_, err = lb.NewListener(ctx, "http-listener", &lb.ListenerArgs{
			LoadBalancerArn: loadBalancer.Arn,
			Port:            pulumi.Int(80),
			DefaultActions: lb.ListenerDefaultActionArray{
				lb.ListenerDefaultActionArgs{
					Type: pulumi.String("redirect"),
					Redirect: lb.ListenerDefaultActionRedirectArgs{
						Protocol:   pulumi.String("HTTPS"),
						Port:       pulumi.String("443"),
						StatusCode: pulumi.String("HTTP_301"),
					},
				},
			},
		})
		if err != nil {
			return err
		}

		// create a new listener on port 443 and redirect to https
		httpsListener, err := lb.NewListener(ctx, "https-listener", &lb.ListenerArgs{
			LoadBalancerArn: loadBalancer.Arn,
			Port:            pulumi.Int(443),
			Protocol:        pulumi.String("HTTPS"),
			SslPolicy:       pulumi.String("ELBSecurityPolicy-2016-08"),
			CertificateArn:  sandblizzardCert.Arn,
			DefaultActions: lb.ListenerDefaultActionArray{
				lb.ListenerDefaultActionArgs{
					Type:           pulumi.String("forward"),
					TargetGroupArn: targetGroupHttps.Arn,
				},
			},
		})
		if err != nil {
			return err
		}

		// create task execution role for
		taskExecRole, err := iam.NewRole(ctx, "task-exec-role", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(`{
				"Version": "2008-10-17",
				"Statement": [{
					"Sid": "",
					"Effect": "Allow",
					"Principal": {
						"Service": "ecs-tasks.amazonaws.com"
					},
					"Action": "sts:AssumeRole"
				}]
				}`),
		})
		if err != nil {
			return err
		}

		// create new policy for role
		_, err = iam.NewRolePolicyAttachment(ctx, "task-exec-policy", &iam.RolePolicyAttachmentArgs{
			Role:      taskExecRole.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"),
		})
		if err != nil {
			return err
		}

		// create a new container cluster
		cluster, err := ecs.NewCluster(ctx, "sure-cluster", nil)
		if err != nil {
			return err
		}
		ctx.Export("sandblizzard-cluster", cluster.ID())

		// Build and push relayer and dapp
		relayerRepo, err := ecr.NewRepository(ctx, "sb-relayer-ecr", nil)
		if err != nil {
			return err
		}

		dappRepo, err := ecr.NewRepository(ctx, "sb-dapp-ecr", nil)
		if err != nil {
			return err
		}

		// authenticate with relayer repo
		relayerRegistryInfo := authenticate(ctx, relayerRepo)
		relayerImage, _ := docker.NewImage(ctx, "sb-relayer", &docker.ImageArgs{
			Build:     &docker.DockerBuildArgs{Context: pulumi.String("./.."), Dockerfile: pulumi.String("./../dockerfile.relayer")},
			ImageName: relayerRepo.RepositoryUrl,
			Registry:  relayerRegistryInfo,
		}, pulumi.DependsOn([]pulumi.Resource{relayerRepo}))

		// Export the base and specific version image name.
		ctx.Export("relayerBaseImageName", relayerImage.BaseImageName)
		ctx.Export("relayerFullImageName", relayerImage.ImageName)

		dappRegistryInfo := authenticate(ctx, dappRepo)
		dappImage, _ := docker.NewImage(ctx, "sb-dapp", &docker.ImageArgs{
			Build:     &docker.DockerBuildArgs{Context: pulumi.String("./.."), Dockerfile: pulumi.String("./../dockerfile.dapp")},
			ImageName: dappRepo.RepositoryUrl,
			Registry:  dappRegistryInfo,
		}, pulumi.DependsOn([]pulumi.Resource{dappRepo, relayerImage}))

		// Export the base and specific version image name.
		ctx.Export("dappBaseImageName", dappImage.BaseImageName)
		ctx.Export("dappFullImageName", dappImage.ImageName)

		// create container definition
		containerDefinition := pulumi.Sprintf(`[
			{
				"name":"sandlizzard-relayer-cd",
				"image": %q,
				"secrets": [
					{
						"Name": "GITHUB_KEY",
						"ValueFrom": %q
					},
					{
						"Name": "UNDERDOG_KEY",
						"ValueFrom": %q
					},
					{
						"Name": "KEY",
						"ValueFrom": %q
					},
					{
						"Name": "GITHUB_ID",
						"ValueFrom": "282074"
					},
					{
						"Name": "SANDBLIZZARD_COLLECTION_ADDRESS",
						"ValueFrom": "2AHfNu6sWRMPWKKQJTffWMWjkYL8AnYY852Fd7ZrkrFw"
					},
					{
						"Name": "GITHUB_APP_LOGIN",
						"ValueFrom": "sandblizzard-app[bot]"
					},
					{
						"Name": "CLUSTER",
						"ValueFrom": "devnet"
					},
					{
						"Name": "RUST_LOG",
						"ValueFrom": "info"
					}
				]
			},
			{
				"name":"sandlizzard-dapp-cd",
				"image": %q,
				"portMappings":  [{
					"containerPort": 80,
					"hostPort": 80,
					"protocol": "tcp"
				}]
			}
		]`, relayerImage.ImageName, githubKeySecret.Arn, underdogKeySecret.Arn, keySecret.Arn, dappImage.ImageName)

		ctx.Export("containerDefinition", containerDefinition)
		//load the docker containers
		// load the docker container
		taskDefinition, err := ecs.NewTaskDefinition(ctx, "sandblizzard-td", &ecs.TaskDefinitionArgs{
			Family:                  pulumi.String("fargate-task-definition"),
			Cpu:                     pulumi.String("256"),
			Memory:                  pulumi.String("512"),
			NetworkMode:             pulumi.String("awsvpc"),
			RequiresCompatibilities: pulumi.StringArray{pulumi.String("FARGATE")},
			ExecutionRoleArn:        taskExecRole.Arn,
			ContainerDefinitions:    containerDefinition,
		})
		if err != nil {
			return err
		}

		// setup the service
		_, err = ecs.NewService(ctx, "sandblizzard-svc", &ecs.ServiceArgs{
			Cluster:        cluster.ID(),
			DesiredCount:   pulumi.Int(5),
			LaunchType:     pulumi.String("FARGATE"),
			TaskDefinition: taskDefinition.Arn,
			NetworkConfiguration: ecs.ServiceNetworkConfigurationArgs{
				AssignPublicIp: pulumi.Bool(true),
				Subnets:        pulumi.ToStringArray(subnet.Ids),
				SecurityGroups: pulumi.StringArray{securityGroup.ID().ToStringOutput()},
			},
			LoadBalancers: ecs.ServiceLoadBalancerArray{
				ecs.ServiceLoadBalancerArgs{
					TargetGroupArn: targetGroupHttp.Arn,
					ContainerName:  pulumi.String("sandlizzard-dapp-cd"),
					ContainerPort:  pulumi.Int(80),
				},
				ecs.ServiceLoadBalancerArgs{
					ContainerName: pulumi.String("sandlizzard-relayer-cd"),
					ContainerPort: pulumi.Int(81),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{cluster, httpsListener}))
		if err != nil {
			return err
		}
		ctx.Export("url", loadBalancer.DnsName)

		return nil
	})
}

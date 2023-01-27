package main

import (
	"encoding/base64"
	"errors"
	"fmt"
	"strings"

	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/eks"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/iam"

	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecr"
	"github.com/pulumi/pulumi-docker/sdk/v3/go/docker"

	"github.com/pulumi/pulumi-kubernetes/sdk/v3/go/kubernetes"
	appsv1 "github.com/pulumi/pulumi-kubernetes/sdk/v3/go/kubernetes/apps/v1"
	corev1 "github.com/pulumi/pulumi-kubernetes/sdk/v3/go/kubernetes/core/v1"
	metav1 "github.com/pulumi/pulumi-kubernetes/sdk/v3/go/kubernetes/meta/v1"

	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func authenticate(ctx *pulumi.Context, repo *ecr.Repository) (docker.ImageRegistryOutput, error) {
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
	}).(docker.ImageRegistryOutput)
	return registryInfo, nil
}

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {

		appLabels := pulumi.StringMap{
			"app": pulumi.String("relayer"),
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

		// create an eks role
		eksRole, err := iam.NewRole(ctx, "eks-iam-eksRole", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(`{
				"Version": "2008-10-17",
				"Statement": [{
					"Sid": "",
					"Effect": "Allow",
					"Principal": {
						"Service": "eks.amazonaws.com"
					},
					"Action": "sts:AssumeRole"
				}]
			}`),
		})

		if err != nil {
			return err
		}

		eksPolicies := []string{
			"arn:aws:iam::aws:policy/AmazonEKSServicePolicy",
			"arn:aws:iam::aws:policy/AmazonEKSClusterPolicy",
		}
		// attach policies to eks role
		for i, eksPolicy := range eksPolicies {
			_, err := iam.NewRolePolicyAttachment(ctx, fmt.Sprintf("rpa-%", i), &iam.RolePolicyAttachmentArgs{
				PolicyArn: pulumi.String(eksPolicy),
				Role:      eksRole.Name,
			})
			if err != nil {
				return err
			}
		}

		// Create teh EC2 NodeGroup
		nodeGroupRole, err := iam.NewRole(ctx, "nodegroup-iam-role", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(`{
				"Version": "2012-10-17",
				"Statement": [{
					"Sid": "",
					"Effect": "Allow",
					"Principal": {
						"Service": "ec2.amazonaws.com"
					},
					"Action": "sts:AssumeRole"
				}]
			}`),
		})
		if err != nil {
			return err
		}

		nodeGroupPolicies := []string{
			"arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy",
			"arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy",
			"arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly",
		}

		for i, nodeGroupPolicy := range nodeGroupPolicies {
			_, err := iam.NewRolePolicyAttachment(ctx, fmt.Sprintf("ngpa-%d", i), &iam.RolePolicyAttachmentArgs{
				Role:      nodeGroupRole.Name,
				PolicyArn: pulumi.String(nodeGroupPolicy),
			})
			if err != nil {
				return err
			}
		}

		// create security group to be used to connect to cluster
		clusterSg, err := ec2.NewSecurityGroup(ctx, "cluster-sg", &ec2.SecurityGroupArgs{
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
			},
		})
		if err != nil {
			return err
		}

		// Create eks cluster
		eksCluster, err := eks.NewCluster(ctx, "eks-cluster", &eks.ClusterArgs{
			RoleArn: pulumi.StringInput(eksRole.Arn),
			VpcConfig: &eks.ClusterVpcConfigArgs{
				PublicAccessCidrs: pulumi.StringArray{
					pulumi.String("0.0.0.0/0"),
				},
				SecurityGroupIds: pulumi.StringArray{
					clusterSg.ID().ToStringOutput(),
				},
				SubnetIds: toPulumiStringArray(subnet.Ids),
			},
		})
		if err != nil {
			return err
		}

		// new node
		nodeGroup, err := eks.NewNodeGroup(ctx, "node-group-2", &eks.NodeGroupArgs{
			ClusterName:   eksCluster.Name,
			NodeGroupName: pulumi.String("eks-nodegroup-2"),
			NodeRoleArn:   pulumi.StringInput(nodeGroupRole.Arn),
			SubnetIds:     toPulumiStringArray(subnet.Ids),
			ScalingConfig: &eks.NodeGroupScalingConfigArgs{
				DesiredSize: pulumi.Int(2),
				MaxSize:     pulumi.Int(2),
				MinSize:     pulumi.Int(1),
			},
		})
		if err != nil {
			return err
		}

		// Create kubeconfig
		kubeconfig := generateKubeconfig(eksCluster.Endpoint,
			eksCluster.CertificateAuthority.Data().Elem(), eksCluster.Name)
		ctx.Export("kubeconfig", generateKubeconfig(eksCluster.Endpoint,
			eksCluster.CertificateAuthority.Data().Elem(), eksCluster.Name))

		// create new k8s provider
		k8sProvider, err := kubernetes.NewProvider(ctx, "k8sprovider", &kubernetes.ProviderArgs{
			Kubeconfig: kubeconfig,
		}, pulumi.DependsOn([]pulumi.Resource{nodeGroup}))
		if err != nil {
			return err
		}

		namespace, err := corev1.NewNamespace(ctx, "app-ns", &corev1.NamespaceArgs{
			Metadata: &metav1.ObjectMetaArgs{
				Name: pulumi.String("sandblizzard"),
			},
		}, pulumi.Provider(k8sProvider))
		if err != nil {
			return err
		}

		repo, err := ecr.NewRepository(ctx, "sandblizzard-relayer", nil)
		if err != nil {
			return err
		}

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
		}).(docker.ImageRegistryOutput)

		image, err := docker.NewImage(ctx, "relayer", &docker.ImageArgs{
			Build:     &docker.DockerBuildArgs{Context: pulumi.String("./.."), Dockerfile: pulumi.String("./../dockerfile.relayer")},
			ImageName: repo.RepositoryUrl,
			Registry:  registryInfo,
		}, pulumi.DependsOn([]pulumi.Resource{repo}))
		if err != nil {
			return err
		}
		// Export the base and specific version image name.
		ctx.Export("baseImageName", image.BaseImageName)
		ctx.Export("fullImageName", image.ImageName)

		// Deploy relayer to the new cluster
		deployment, err := appsv1.NewDeployment(ctx, "relayer-deployment", &appsv1.DeploymentArgs{
			Metadata: &metav1.ObjectMetaArgs{
				Namespace: namespace.Metadata.Elem().Name(),
			},
			Spec: appsv1.DeploymentSpecArgs{
				Selector: &metav1.LabelSelectorArgs{
					MatchLabels: appLabels,
				},
				Replicas: pulumi.Int(1),
				Template: &corev1.PodTemplateSpecArgs{
					Metadata: &metav1.ObjectMetaArgs{
						Labels: appLabels,
					},
					Spec: &corev1.PodSpecArgs{
						Containers: corev1.ContainerArray{
							corev1.ContainerArgs{
								Name:  pulumi.String("relayer"),
								Image: repo.RepositoryUrl,
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}

		service, err := corev1.NewService(ctx, "app-service", &corev1.ServiceArgs{
			Metadata: &metav1.ObjectMetaArgs{
				Namespace: namespace.Metadata.Elem().Name(),
				Labels:    appLabels,
			},
			Spec: &corev1.ServiceSpecArgs{
				Ports: corev1.ServicePortArray{
					corev1.ServicePortArgs{
						Port:       pulumi.Int(80),
						TargetPort: pulumi.Int(8080),
					},
				},
				Selector: appLabels,
				Type:     pulumi.String("LoadBalancer"),
			},
		}, pulumi.Provider(k8sProvider))
		if err != nil {
			return err
		}

		ctx.Export("url", service.Status.ApplyT(func(status *corev1.ServiceStatus) *string {
			ingress := status.LoadBalancer.Ingress[0]
			if ingress.Hostname != nil {
				return ingress.Hostname
			}
			return ingress.Ip
		}))

		ctx.Export("name", deployment.Metadata.Elem().Name())
		return nil
	})
}

// Create the KubeConfig Structure as per https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html
func generateKubeconfig(clusterEndpoint pulumi.StringOutput, certData pulumi.StringOutput, clusterName pulumi.StringOutput) pulumi.StringOutput {
	return pulumi.Sprintf(`{
        "apiVersion": "v1",
        "clusters": [{
            "cluster": {
                "server": "%s",
                "certificate-authority-data": "%s"
            },
            "name": "kubernetes",
        }],
        "contexts": [{
            "context": {
                "cluster": "kubernetes",
                "user": "aws",
            },
            "name": "aws",
        }],
        "current-context": "aws",
        "kind": "Config",
        "users": [{
            "name": "aws",
            "user": {
                "exec": {
                    "apiVersion": "client.authentication.k8s.io/v1beta1",
                    "command": "aws-iam-authenticator",
                    "args": [
                        "token",
                        "-i",
                        "%s",
                    ],
                },
            },
        }],
    }`, clusterEndpoint, certData, clusterName)
}

func toPulumiStringArray(a []string) pulumi.StringArrayInput {
	var res []pulumi.StringInput
	for _, s := range a {
		res = append(res, pulumi.String(s))
	}
	return pulumi.StringArray(res)
}

package main

import (
	"encoding/base64"
	"errors"
	"strings"

	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecr"
	"github.com/pulumi/pulumi-docker/sdk/v3/go/docker"

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
			"app": pulumi.String("nginx"),
		}

		repo, err := ecr.NewRepository(ctx, "relayer", nil)
		if err != nil {
			return err
		}

		auth, err := authenticate(ctx, repo)
		if err != nil {
			return err
		}

		image, err := docker.NewImage(ctx, "relayer", &docker.ImageArgs{
			Build:     &docker.DockerBuildArgs{Context: pulumi.String("./.."), Dockerfile: pulumi.String("./../dockerfile.relayer")},
			ImageName: pulumi.String("relayer"),
			Registry:  auth,
		})
		// Export the base and specific version image name.
		ctx.Export("baseImageName", image.BaseImageName)
		ctx.Export("fullImageName", image.ImageName)

		// deploy relayer

		_, err = appsv1.NewDeployment(ctx, "app-dep", &appsv1.DeploymentArgs{
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
								Name:  pulumi.String("replayer"),
								Image: repo.RepositoryUrl,
							},
						},
					},
				},
			},
		})

		return nil
	})
}

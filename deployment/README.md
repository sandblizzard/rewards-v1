## Deployment of Sandblizzard

### Kubeconfig

```
> pulumi stack output kubeconfig > kubeconfig.yml
```

### Get nodes

```
> KUBECONFIG=kubeconfig.yml kubectl get nodes
```

```
> KUBECONFIG=kubeconfig.yml kubectl get deployment $(pulumi stack output deploymentName) --namespace=$(pulumi stack output namespaceName)
```

Authenticate against aws ecr

```
aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 686024338500.dkr.ecr.us-east-2.amazonaws.com
```

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

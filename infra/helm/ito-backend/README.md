# ito-backend Helm Chart

Deploy the Ito Backend API server to Kubernetes.

## Prerequisites

- Kubernetes 1.24+
- Helm 3.x
- PV provisioner (if persistence is enabled)

## Installation

```bash
helm install ito-backend ./infra/helm/ito-backend/ \
  --set auth.adminToken="$(openssl rand -base64 32)" \
  --set auth.tokenSeed="$(openssl rand -base64 32)"
```

Or with a values file:

```bash
helm install ito-backend ./infra/helm/ito-backend/ -f my-values.yaml
```

## Using the GHCR Image

The image is published to `ghcr.io/withakay/ito-backend` on every release tag:

```bash
docker pull ghcr.io/withakay/ito-backend:latest
docker pull ghcr.io/withakay/ito-backend:0.1.16
```

## Configuration

| Parameter | Description | Default |
|---|---|---|
| `replicaCount` | Number of replicas | `1` |
| `image.repository` | Container image | `ghcr.io/withakay/ito-backend` |
| `image.tag` | Image tag (defaults to appVersion) | `""` |
| `auth.adminToken` | Admin token for API auth | `""` |
| `auth.tokenSeed` | HMAC seed for token generation | `""` |
| `auth.existingSecret` | Use an existing Secret (skips creation) | `""` |
| `persistence.enabled` | Enable PVC for data | `true` |
| `persistence.size` | PVC size | `1Gi` |
| `persistence.storageClass` | Storage class | `""` (cluster default) |
| `service.type` | Kubernetes Service type | `ClusterIP` |
| `service.port` | Service port | `9010` |
| `ingress.enabled` | Enable Ingress | `false` |
| `ingress.host` | Ingress hostname | `""` |
| `ingress.className` | Ingress class name | `""` |
| `resources.requests.cpu` | CPU request | `100m` |
| `resources.requests.memory` | Memory request | `128Mi` |
| `resources.limits.cpu` | CPU limit | `500m` |
| `resources.limits.memory` | Memory limit | `512Mi` |

## Authentication

### Inline Tokens

Set tokens directly in values:

```yaml
auth:
  adminToken: "your-admin-token"
  tokenSeed: "your-token-seed"
```

### Existing Secret

Reference a pre-created Kubernetes Secret:

```yaml
auth:
  existingSecret: "my-ito-secret"
```

The Secret must contain keys `admin-token` and `token-seed`:

```bash
kubectl create secret generic my-ito-secret \
  --from-literal=admin-token="your-admin-token" \
  --from-literal=token-seed="your-token-seed"
```

## Persistence

By default, a 1Gi PVC is created for SQLite data at `/data`. Disable for ephemeral setups:

```yaml
persistence:
  enabled: false
```

## Ingress

Enable with a hostname:

```yaml
ingress:
  enabled: true
  className: nginx
  host: ito.example.com
  tls:
    - secretName: ito-tls
      hosts:
        - ito.example.com
```

## Tailscale Integration

The Ito backend can be exposed exclusively on your Tailscale network using the
[Tailscale Kubernetes operator](https://tailscale.com/kb/1236/kubernetes-operator).
This is handled entirely at the infrastructure level — no sidecar or custom code
is needed in this chart.

### Option 1: Tailscale Ingress

Use the Tailscale ingress class to expose the service on your tailnet:

```yaml
ingress:
  enabled: true
  className: tailscale
  host: ito-backend  # becomes ito-backend.<tailnet>.ts.net
```

### Option 2: Tailscale Service Annotation

Annotate the Service to get a tailnet IP directly:

```yaml
service:
  type: ClusterIP
  # Add via extraAnnotations or post-install:
  # tailscale.com/expose: "true"
  # tailscale.com/hostname: "ito-backend"
```

Then apply manually:

```bash
kubectl annotate svc ito-backend \
  tailscale.com/expose=true \
  tailscale.com/hostname=ito-backend
```

### Option 3: Tailscale Proxy Sidecar (Manual)

For clusters without the Tailscale operator, you can manually add a Tailscale
sidecar using `extraContainers` or a separate Deployment. See the
[Tailscale container documentation](https://tailscale.com/kb/1185/kubernetes)
for details.

## Uninstalling

```bash
helm uninstall ito-backend
# PVC is retained by default; delete manually if no longer needed:
kubectl delete pvc ito-backend-data
```

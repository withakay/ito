<!-- ITO:START -->
## Why

There is no containerized deployment option for the Ito backend API server. Users who want to run it on Kubernetes, a homelab, or any container-based infrastructure must build the binary themselves and figure out the deployment configuration. A Docker image published to GHCR with an accompanying Helm chart removes this friction and makes the backend a first-class deployable service.

## What Changes

- Add a multi-stage Dockerfile (`infra/docker/Dockerfile`) that builds the `ito` binary from source and packages it in a distroless image, exposing port 9010 with `--bind 0.0.0.0` as the default.
- Add a Helm chart (`infra/helm/ito-backend/`) with support for:
  - Kubernetes Secret injection of `ITO_BACKEND_ADMIN_TOKEN` and `ITO_BACKEND_TOKEN_SEED` as env vars.
  - PersistentVolumeClaim for the SQLite data directory (`/data`).
  - Configurable replicas, resource limits, service type, and ingress.
  - Health check probes targeting `/api/v1/health`.
- Add a GitHub Actions workflow to build and push the image to `ghcr.io/withakay/ito-backend` on release tags.
- Document Tailscale integration as an upstream pattern (Tailscale Kubernetes operator / ingress controller) — no custom sidecar code.

## Capabilities

### New Capabilities

- `container-image`: Dockerfile, multi-stage build, distroless base, GHCR publish workflow.
- `helm-chart`: Helm chart for Kubernetes deployment with secrets, PVC, health probes, and ingress.

### Modified Capabilities

(none — this is additive infrastructure; no existing spec requirements change)

## Impact

- **New files**: `infra/docker/Dockerfile`, `infra/helm/ito-backend/` (Chart.yaml, values.yaml, templates/*), `.github/workflows/docker-publish.yml`.
- **Documentation**: Updates to `docs/backend-client-mode.md` and `README.md` with container deployment instructions and Tailscale guidance.
- **Dependencies**: No new Rust dependencies. Build-time dependency on Docker and Helm for packaging.
- **APIs**: No API changes. The existing `ito serve-api` command is the entrypoint.
<!-- ITO:END -->

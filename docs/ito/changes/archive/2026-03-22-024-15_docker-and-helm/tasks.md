<!-- ITO:START -->
# Tasks for: 024-15_docker-and-helm

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-15
ito tasks next 024-15
ito tasks start 024-15 1.1
ito tasks complete 024-15 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Create multi-stage Dockerfile

- **Files**: `infra/docker/Dockerfile`
- **Dependencies**: None
- **Action**: Write a multi-stage Dockerfile: builder stage uses `rust:1-bookworm` to compile `ito` binary, final stage uses `gcr.io/distroless/cc-debian12`. Entrypoint runs `ito serve-api --bind 0.0.0.0 --data-dir /data`. Expose port 9010.
- **Verify**: `docker build -f infra/docker/Dockerfile -t ito-backend . && docker inspect ito-backend --format '{{.Config.ExposedPorts}}'`
- **Done When**: Image builds, binary runs inside the container, port 9010 is exposed, image is under 50 MB.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 1.2: Create Helm Chart.yaml and values.yaml

- **Files**: `infra/helm/ito-backend/Chart.yaml`, `infra/helm/ito-backend/values.yaml`
- **Dependencies**: None
- **Action**: Create the Helm chart metadata and default values including: image repository/tag, auth.adminToken, auth.tokenSeed, auth.existingSecret, persistence (enabled, size, storageClass), service (type, port), ingress (enabled, host, annotations), resources (requests/limits), replicaCount.
- **Verify**: `helm lint infra/helm/ito-backend/`
- **Done When**: `helm lint` passes with no errors.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Create Deployment template

- **Files**: `infra/helm/ito-backend/templates/deployment.yaml`
- **Dependencies**: None
- **Action**: Create a Deployment template that references the container image, injects auth secrets as env vars (from created or existing Secret), mounts PVC or emptyDir at `/data`, configures liveness/readiness probes on `/api/v1/health:9010`, and sets resource requests/limits.
- **Verify**: `helm template ito-backend infra/helm/ito-backend/ | kubectl apply --dry-run=client -f -`
- **Done When**: Template renders valid Kubernetes YAML; dry-run passes.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 2.2: Create Secret template

- **Files**: `infra/helm/ito-backend/templates/secret.yaml`
- **Dependencies**: None
- **Action**: Create a Secret template that encodes `auth.adminToken` and `auth.tokenSeed` from values.yaml. Skip creation when `auth.existingSecret` is set.
- **Verify**: `helm template ito-backend infra/helm/ito-backend/ --set auth.adminToken=test --set auth.tokenSeed=seed | grep -A5 'kind: Secret'`
- **Done When**: Secret renders with base64 encoded values; is absent when `existingSecret` is set.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 2.3: Create Service template

- **Files**: `infra/helm/ito-backend/templates/service.yaml`
- **Dependencies**: None
- **Action**: Create a Service template with configurable type (default ClusterIP) on port 9010.
- **Verify**: `helm template ito-backend infra/helm/ito-backend/ | grep 'kind: Service' -A10`
- **Done When**: Service renders with correct port and selector.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 2.4: Create PVC template

- **Files**: `infra/helm/ito-backend/templates/pvc.yaml`
- **Dependencies**: None
- **Action**: Create a PVC template with configurable size (default 1Gi), access mode (ReadWriteOnce), and storage class. Conditional on `persistence.enabled`.
- **Verify**: `helm template ito-backend infra/helm/ito-backend/ | grep 'kind: PersistentVolumeClaim' -A10`
- **Done When**: PVC renders when enabled, absent when disabled.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 2.5: Create Ingress template

- **Files**: `infra/helm/ito-backend/templates/ingress.yaml`
- **Dependencies**: None
- **Action**: Create an optional Ingress template, disabled by default, with configurable host, annotations, and TLS.
- **Verify**: `helm template ito-backend infra/helm/ito-backend/ --set ingress.enabled=true --set ingress.host=ito.example.com | grep 'kind: Ingress' -A15`
- **Done When**: Ingress absent by default; renders correctly when enabled.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Create GHCR publish workflow

- **Files**: `.github/workflows/docker-publish.yml`
- **Dependencies**: None
- **Action**: Create a GitHub Actions workflow that triggers on `v*` tags, builds the Docker image, tags it with the version and `latest`, and pushes to `ghcr.io/withakay/ito-backend`.
- **Verify**: Review workflow YAML syntax; `actionlint .github/workflows/docker-publish.yml` if available.
- **Done When**: Workflow file is valid and targets the correct registry/image name.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 3.2: Create chart README with Tailscale guidance

- **Files**: `infra/helm/ito-backend/README.md`, `infra/helm/ito-backend/templates/NOTES.txt`
- **Dependencies**: None
- **Action**: Write chart README covering: installation, values reference, auth configuration, persistence, ingress setup, and a Tailscale section explaining how to use the Tailscale Kubernetes operator or ingress class for tailnet-only access. Add NOTES.txt with post-install guidance.
- **Verify**: Read the rendered notes: `helm template ito-backend infra/helm/ito-backend/ | tail -20`
- **Done When**: README has all sections including Tailscale guidance; NOTES.txt renders after install.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

### Task 3.3: Update project documentation

- **Files**: `docs/backend-client-mode.md`, `README.md`
- **Dependencies**: None
- **Action**: Add Docker/Kubernetes deployment sections to existing docs, referencing the image name `ghcr.io/withakay/ito-backend` and the Helm chart.
- **Verify**: Verify links and formatting are consistent with existing doc style.
- **Done When**: Both docs reference container and Helm deployment options.
- **Updated At**: 2026-03-07
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->

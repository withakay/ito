<!-- ITO:START -->
## ADDED Requirements

### Requirement: Helm chart provides a deployable ito-backend

A Helm chart at `infra/helm/ito-backend/` SHALL deploy the `ito-backend` container image to Kubernetes with sensible defaults.

#### Scenario: Helm install creates a running deployment

- **WHEN** a user runs `helm install ito-backend infra/helm/ito-backend/`
- **THEN** Kubernetes creates a Deployment, Service, and PersistentVolumeClaim for the ito-backend

### Requirement: Auth secrets are injected from a Kubernetes Secret

The chart SHALL create a Kubernetes Secret containing `ITO_BACKEND_ADMIN_TOKEN` and `ITO_BACKEND_TOKEN_SEED`, injected as environment variables into the container.

#### Scenario: Tokens provided in values.yaml

- **WHEN** `auth.adminToken` and `auth.tokenSeed` are set in values.yaml
- **THEN** the chart creates a Secret and mounts the values as env vars in the Deployment

#### Scenario: External secret reference

- **WHEN** `auth.existingSecret` is set in values.yaml
- **THEN** the chart uses the named Secret instead of creating one, and `auth.adminToken`/`auth.tokenSeed` are ignored

### Requirement: Persistent storage via PVC

The chart SHALL create a PersistentVolumeClaim mounted at `/data` in the container for SQLite state persistence.

#### Scenario: Default PVC is created

- **WHEN** the chart is installed with default values
- **THEN** a 1Gi PVC with `ReadWriteOnce` access mode is created and mounted at `/data`

#### Scenario: Storage class and size are configurable

- **WHEN** `persistence.storageClass` and `persistence.size` are set in values.yaml
- **THEN** the PVC uses the specified storage class and size

#### Scenario: PVC can be disabled

- **WHEN** `persistence.enabled` is set to `false`
- **THEN** no PVC is created and the container uses an emptyDir volume

### Requirement: Health check probes target the health endpoint

The Deployment SHALL configure liveness and readiness probes against `/api/v1/health` on port 9010.

#### Scenario: Probes are configured

- **WHEN** the Deployment is created
- **THEN** livenessProbe and readinessProbe both use HTTP GET on `/api/v1/health` port 9010

### Requirement: Service exposes port 9010

The chart SHALL create a Service of configurable type (default `ClusterIP`) that routes traffic to the container on port 9010.

#### Scenario: Default ClusterIP service

- **WHEN** the chart is installed with default values
- **THEN** a ClusterIP Service is created on port 9010

#### Scenario: Service type is configurable

- **WHEN** `service.type` is set to `LoadBalancer` in values.yaml
- **THEN** the Service type is LoadBalancer

### Requirement: Optional Ingress resource

The chart SHALL support an optional Ingress resource, disabled by default.

#### Scenario: Ingress disabled by default

- **WHEN** the chart is installed with default values
- **THEN** no Ingress resource is created

#### Scenario: Ingress enabled with host

- **WHEN** `ingress.enabled` is `true` and `ingress.host` is set
- **THEN** an Ingress resource is created routing traffic to the Service

### Requirement: Resource limits are configurable

The Deployment SHALL support configurable CPU and memory requests/limits via values.yaml.

#### Scenario: Default resource values

- **WHEN** the chart is installed with default values
- **THEN** the container has resource requests of 100m CPU / 128Mi memory and limits of 500m CPU / 512Mi memory

#### Scenario: Custom resources

- **WHEN** `resources.requests.cpu` is set to `250m` in values.yaml
- **THEN** the container's CPU request is 250m

### Requirement: Tailscale integration is documented

The chart documentation SHALL describe how to use the Tailscale Kubernetes operator or Tailscale ingress controller as an upstream pattern for private network access, without including custom Tailscale sidecar code.

#### Scenario: Tailscale section in chart README

- **WHEN** a user reads the chart's README or NOTES.txt
- **THEN** there is a section explaining how to use Tailscale operator annotations or ingress class for tailnet-only access
<!-- ITO:END -->

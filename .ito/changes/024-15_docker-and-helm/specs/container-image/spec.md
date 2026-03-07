<!-- ITO:START -->
## ADDED Requirements

### Requirement: Multi-stage Dockerfile produces minimal image

The build system SHALL provide a multi-stage Dockerfile at `infra/docker/Dockerfile` that compiles the `ito` binary from source using a Rust builder stage and copies only the final binary into a `gcr.io/distroless/cc-debian12` base image.

#### Scenario: Build produces a working image

- **WHEN** a user runs `docker build -f infra/docker/Dockerfile -t ito-backend .` from the repo root
- **THEN** the resulting image contains the `ito` binary at `/usr/local/bin/ito` and no shell or package manager

#### Scenario: Image size is minimal

- **WHEN** the image is built
- **THEN** the final image size SHALL be under 50 MB (excluding build cache layers)

### Requirement: Container binds to all interfaces by default

The container entrypoint SHALL run `ito serve-api --bind 0.0.0.0` so the server is reachable from outside the container without additional configuration.

#### Scenario: Default entrypoint listens on 0.0.0.0

- **WHEN** the container starts with no arguments
- **THEN** the `ito serve-api` process binds to `0.0.0.0:9010`

#### Scenario: Port and bind are overridable

- **WHEN** the container starts with `--port 8080 --bind 127.0.0.1`
- **THEN** the process binds to `127.0.0.1:8080` instead of the defaults

### Requirement: Container exposes port 9010

The Dockerfile SHALL declare `EXPOSE 9010` to document the default listening port.

#### Scenario: Port metadata is present

- **WHEN** a user inspects the image metadata
- **THEN** port 9010/tcp is listed as an exposed port

### Requirement: Auth tokens are injectable via environment variables

The container SHALL support `ITO_BACKEND_ADMIN_TOKEN` and `ITO_BACKEND_TOKEN_SEED` environment variables for auth configuration, consistent with the existing `serve-api` env var precedence.

#### Scenario: Env var auth works without config file

- **WHEN** the container starts with `ITO_BACKEND_ADMIN_TOKEN=secret` and `ITO_BACKEND_TOKEN_SEED=seed` set
- **THEN** the server authenticates requests using those values

#### Scenario: Mounted config file is also supported

- **WHEN** a config.json is mounted at `/etc/ito/config.json` and `ITO_GLOBAL_CONFIG` is set to that path
- **THEN** the server reads auth values from the mounted file

### Requirement: Data directory defaults to /data

The container entrypoint SHALL pass `--data-dir /data` so that SQLite state is written to a well-known mountable path.

#### Scenario: Default data directory is /data

- **WHEN** the container starts with no `--data-dir` override
- **THEN** the server writes state to `/data`

### Requirement: GHCR publish workflow

A GitHub Actions workflow SHALL build and push the image to `ghcr.io/withakay/ito-backend` on release tags matching `v*`.

#### Scenario: Release tag triggers image publish

- **WHEN** a tag matching `v*` is pushed to the repository
- **THEN** the workflow builds the image, tags it with the version and `latest`, and pushes to GHCR

#### Scenario: Non-release pushes do not publish

- **WHEN** a commit is pushed to `main` without a release tag
- **THEN** the workflow does not push an image to GHCR
<!-- ITO:END -->

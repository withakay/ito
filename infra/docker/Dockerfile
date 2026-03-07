# Multi-stage build for the Ito backend API server.
#
# Build:
#   docker build -f infra/docker/Dockerfile -t ito-backend .
#
# Run:
#   docker run -p 9010:9010 \
#     -e ITO_BACKEND_ADMIN_TOKEN=<token> \
#     -e ITO_BACKEND_TOKEN_SEED=<seed> \
#     -v ito-data:/data \
#     ito-backend

# ---------------------------------------------------------------------------
# Stage 1: Build the ito binary
# ---------------------------------------------------------------------------
FROM rust:1-bookworm AS builder

WORKDIR /src

# Copy manifests first to cache dependency builds
COPY Cargo.toml Cargo.lock ./
COPY ito-rs/ ito-rs/

# Build the release binary
RUN cargo build --release --package ito-cli \
    && strip /src/target/release/ito

# Create the data directory so it exists in the final image
RUN mkdir -p /data && chown 65534:65534 /data

# ---------------------------------------------------------------------------
# Stage 2: Minimal runtime image
# ---------------------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12

LABEL org.opencontainers.image.source="https://github.com/withakay/ito"
LABEL org.opencontainers.image.description="Ito Backend API Server"
LABEL org.opencontainers.image.licenses="MIT"

COPY --from=builder /src/target/release/ito /usr/local/bin/ito
COPY --from=builder --chown=65534:65534 /data /data

USER nonroot:nonroot

EXPOSE 9010

# Default: bind to all interfaces so the container is reachable,
# store data in /data for easy volume mounting.
ENTRYPOINT ["ito", "serve-api", "--bind", "0.0.0.0", "--data-dir", "/data"]

# Change: Add Cloudflare Deployment Support for Backend Artifacts (R2)

## Why

The Ito backend currently supports filesystem and SQLite storage backends, which are suitable for local development and single-server deployments. However, for production deployments on serverless platforms like Cloudflare Workers, we need artifact storage that works well in distributed, edge-deployed environments.

Cloudflare provides **R2** (blob storage) for artifact storage.

Adding support for Cloudflare R2 will enable artifact storage to work reliably with Cloudflare Workers deployments.

## What Changes

- Add Cloudflare R2 blob storage support as an artifact store implementation
- Implement repository adapter that conforms to existing artifact storage abstraction
- Add configuration options to select Cloudflare R2 artifact storage
- Add deployment configuration for Cloudflare Workers
- Ensure artifact semantics remain compatible with existing API behavior

## Impact

- Affected specs:
  - `backend-artifact-store` - Add R2 blob storage implementation
  - `distribution` - Add Cloudflare Workers deployment configuration

- Affected code:
  - `ito-rs/crates/ito-backend/src/repositories/` - New R2 repository implementation
  - `ito-rs/crates/ito-backend/src/config/` - Configuration for R2 backend
  - Backend configuration schema to include Cloudflare R2 options
  - Deployment scripts/configurations for Cloudflare Workers

- Benefits:
  - Enables serverless edge deployment path for backend artifact storage
  - Provides low-latency global access to artifact data
  - Leverages Cloudflare's distributed infrastructure
  - Maintains compatibility with existing storage abstractions

- Risks:
  - R2 pricing model differs from local storage
  - May require additional Cloudflare-specific authentication handling

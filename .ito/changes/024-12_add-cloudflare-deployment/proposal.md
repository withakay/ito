# Change: Add Cloudflare Deployment Support for Backend

## Why

The Ito backend currently supports filesystem and SQLite storage backends, which are suitable for local development and single-server deployments. However, for production deployments on serverless platforms like Cloudflare Workers, we need storage options that work well in distributed, edge-deployed environments.

Cloudflare provides two primary storage solutions that would enable the backend to run at the edge:
- **D1** (distributed SQLite database) for structured data storage
- **R2** (blob storage) for artifact storage

Adding support for these Cloudflare storage backends will enable the Ito backend to be deployed as a Cloudflare Worker, providing low-latency global access to project state and artifact data.

## What Changes

- Add Cloudflare D1 database support as a project store implementation
- Add Cloudflare R2 blob storage support as an artifact store implementation
- Implement repository adapters that conform to existing storage abstractions
- Add configuration options to select Cloudflare storage backends
- Add deployment configuration for Cloudflare Workers
- Ensure both storage backends maintain compatibility with existing API semantics

## Impact

- Affected specs:
  - `backend-project-store` - Add D1 database implementation
  - `backend-artifact-store` - Add R2 blob storage implementation
  - `backend-auth` - May need updates for Cloudflare-specific auth patterns
  - `distribution` - Add Cloudflare Workers deployment configuration

- Affected code:
  - `ito-rs/crates/ito-backend/src/repositories/` - New D1 and R2 repository implementations
  - `ito-rs/crates/ito-backend/src/config/` - Configuration for Cloudflare backends
  - Backend configuration schema to include Cloudflare options
  - Deployment scripts/configurations for Cloudflare Workers

- Benefits:
  - Enables serverless edge deployment of Ito backend
  - Provides low-latency global access to project state
  - Leverages Cloudflare's distributed infrastructure
  - Maintains compatibility with existing storage abstractions

- Risks:
  - D1 has some limitations compared to full SQLite (need to verify compatibility)
  - R2 pricing model differs from local storage
  - May require additional Cloudflare-specific authentication handling

## Why

Developers configuring Ito do not get consistent editor autocomplete and validation because there is no repo-tracked schema artifact that stays in sync with the Rust config model. We should provide a generated schema file and stable remote schema reference so contributors get reliable code completion without extra setup.

## What Changes

- Generate a canonical JSON schema artifact for Ito configuration from the Rust config types.
- Add build integration so schema generation is part of normal build/check workflows.
- Commit the generated schema file in the repository so editors/tools can resolve it from a versioned source of truth.
- Ensure project config files reference a release-tagged GitHub Raw schema URL for completion/validation.
- Add verification that prevents stale schema output from drifting from source config types.

## Capabilities

### New Capabilities

- `config-schema-artifact`: Build-generated, committed schema artifact behavior and drift checks.

### Modified Capabilities

- `config-schema`: Tighten schema location/reference behavior to use a release-tagged schema URL backed by the committed repository artifact.

## Impact

- Affected code:
  - `ito-rs/crates/ito-config` (schema source types)
  - `ito-rs/crates/ito-cli` (`ito config schema` behavior/reuse)
  - `Makefile` and/or build scripts (schema generation + verification)
  - `schemas/` (generated artifact committed to repo)
  - config templates/files that should carry release-tagged `$schema` URL references
- Developer experience: better autocomplete and inline validation in JSON editors.
- CI/build behavior: may fail when schema artifact is out of date until regenerated and committed.

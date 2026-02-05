# Change: NPM binary packages

## Why

Some users prefer to install tools via npm (especially in JavaScript-heavy repos and CI). Providing an optional npm distribution channel for Ito can reduce friction while keeping the Rust-native install paths as the default.

## What Changes

- Define an npm packaging approach for distributing prebuilt `ito` binaries via the npm registry.
- Add a publish pipeline (likely as part of the release process) to produce platform-specific npm packages.
- Provide documentation for `npm install -g ...` as an optional install method.

## Capabilities

### New Capabilities

- `npm-binary-distribution`: Optional npm-based installation that provides the native `ito` binary for supported OS/arch targets.

### Modified Capabilities

<!-- None (must remain true that Ito does not *require* Node/npm) -->

## Impact

- Release automation may be extended to publish to npm.
- New package metadata and CI credentials/secrets handling will be required.
- Documentation must clearly position npm install as optional.

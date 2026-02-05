## Why

Ito currently uses ESLint (+ `typescript-eslint`) for linting, which adds dependency weight and configuration complexity for relatively straightforward rules. Migrating to Biome keeps the workflow fast and consistent while preserving the project’s critical import guardrails (notably restricting `@inquirer/*` static imports that can hang in non-interactive hooks).

## What Changes

- Replace ESLint with Biome for linting (`bun run lint` runs Biome).
- Add a Biome configuration file and enable `style/noRestrictedImports` to continue blocking `@inquirer/*` imports with a clear message.
- Remove ESLint configuration (`eslint.config.js`) and drop ESLint-related dev dependencies.
- Update CI and docs to reference Biome where relevant (without changing the public CLI behavior).

## Capabilities

### New Capabilities

- `biome-linting`: Run JS/TS linting via Biome and keep existing guardrails (restricted imports).
- `biome-formatting`: Provide a consistent formatting command using Biome.
- `eslint-removal`: Remove ESLint tooling cleanly while keeping `bun run lint`/CI behavior stable.

### Modified Capabilities

<!-- None. This change affects developer tooling and does not alter user-facing Ito behavior. -->

## Impact

- Affected files: `package.json`, `Makefile`, `.github/workflows/ci.yml`, `README.md`, `eslint.config.js` (removed), new `biome.json`.
- Dependencies: remove `eslint` + `typescript-eslint`; add `@biomejs/biome`.
- Developer experience: `bun run lint` and CI lint steps remain, but are implemented via Biome.

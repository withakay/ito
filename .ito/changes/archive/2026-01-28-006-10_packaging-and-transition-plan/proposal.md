## Why

Users currently install Ito via npm as `@withakay/ito`. Replacing that distribution with a Rust binary risks UX drift (install paths, shell completion, update semantics). We need a transition plan that lets users adopt the Rust CLI without breaking existing workflows and that preserves behavior identically.

## What Changes

- Define a packaging strategy for distributing the Rust `ito` binary.
- Define a transition plan for npm users (coexistence or replacement) without behavior drift.
- Define CI release artifacts and verification steps.

## Capabilities

### New Capabilities

- `rust-packaging-transition`: documented packaging and release plan for Rust Ito.

### Modified Capabilities

<!-- None. Documentation and release plan only. -->

## Impact

**Affected areas:**

- Documentation and CI config (future)

**Behavioral impact:**

- None immediately

**Risks:**

- Breaking installs or completions; mitigated by staged rollout and compatibility wrappers.

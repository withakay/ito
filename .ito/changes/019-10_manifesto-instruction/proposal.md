<!-- ITO:START -->
## Why

Ito can already compile deterministic agent instructions when an agent can run the CLI, but prompt-only or sandboxed environments lose that guidance and drift into improvisation. This branch already contains a first-pass manifesto template, so the missing work is to make that fallback official, strict, config-bound, and safe to hand to non-executable agent environments.

## What Changes

- Add `ito agent instruction manifesto` as a first-class instruction artifact for both project-wide and change-scoped use.
- Support `light` and `full` manifesto variants plus profile-restricted operating modes such as `planning`, `proposal-only`, `review-only`, `apply`, `archive`, and `full`.
- Render a state-aware manifesto that includes redacted config and state capsules, source-of-truth ordering, worktree and coordination rules, and operation playbooks.
- In full mode, compose relevant existing Ito instruction artifacts so current templates remain the source of truth while manifesto-level rules still take precedence.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `agent-instructions`: extend the instruction system with a manifesto artifact, variant and profile rendering, change-scoped state capsules, and embedded-instruction precedence rules.

## Impact

- Affected code: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/src/cli/agent.rs`, `ito-rs/crates/ito-templates/src/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/manifesto.md.j2`, related CLI and template tests.
- Affected systems: CLI instruction routing, template rendering, config and state resolution, worktree and coordination guidance, agent-facing help output, and JSON instruction responses.
<!-- ITO:END -->

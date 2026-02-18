<!-- ITO:START -->
## Why

`ito init` and `ito update` currently have inconsistent overwrite/merge behavior. In practice, init without `--force` can fail in surprising ways (refusing to touch existing files), while update can clobber files users expected to be preserved. This undermines confidence in the tooling and makes harness hook installation unreliable.

## What Changes

- Define a clear file ownership/merge policy for init/update across marker-managed files, tool adapter assets, and explicitly user-owned project files.
- Make `ito update` reliably refresh Ito-managed assets (commands/prompts/skills/plugins/hooks) while preserving user-owned configuration.
- Add regression tests that reproduce the current inconsistent/blocking behavior and lock in the new semantics.

## Capabilities

### New Capabilities

- `installer-merge-policy`: Deterministic, test-covered init/update semantics for template installation.

### Modified Capabilities

- `cli-init`: Clarify and enforce init behavior when files already exist.
- `cli-update`: Clarify and enforce update behavior to avoid clobbering user-owned files.
- `rust-installers`: Implement the merge/overwrite policy consistently in core installers.

## Impact

- Changes in installer logic (`ito-core`) and CLI glue (`ito-cli`) to correctly apply the selected policy.
- Expanded test coverage for init/update behaviors across `.opencode/`, `.claude/`, `.github/`, `.codex/`, and `.ito/`.
<!-- ITO:END -->

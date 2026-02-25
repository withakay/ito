## Why

Projects customize prompt/template files, but still need to adopt newer Ito guidance and scaffolding over time. Today, upgrades are not explicit enough for this workflow, and users can end up either missing template updates or risking overwrite of their local customizations.

## What Changes

- Add an explicit prompt/template upgrade workflow through `ito init --upgrade` (compatible with existing update behavior).
- Define marker-scoped upgrade behavior so only content inside Ito-managed comment markers is refreshed.
- Preserve all user-authored content outside managed markers during upgrade operations.
- Define fail-safe behavior for legacy files that no longer contain expected markers.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `ito-init`: extend `ito init` semantics with explicit `--upgrade` behavior for template refresh.
- `rust-installers`: tighten installer merge rules for marker-managed prompt/template upgrades.

## Impact

- Affected code: `ito-rs/crates/ito-cli` argument parsing/command flow for init upgrade mode and `ito-rs/crates/ito-core` installer merge logic.
- Affected templates: project/home prompt/template assets that use Ito markers for managed sections.
- Affected tests: CLI init behavior tests and installer merge-policy tests covering marker-managed prompt/template files.

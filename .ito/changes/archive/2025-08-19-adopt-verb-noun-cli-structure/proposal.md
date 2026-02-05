# Change: Adopt Verb–Noun CLI Structure (Deprecate Noun-Based Commands)

## Why

Most widely used CLIs (git, docker, kubectl) start with an action (verb) followed by the object (noun). This matches how users think: “do X to Y”. Using verbs as top-level commands improves clarity, discoverability, and extensibility.

## What Changes

- Promote top-level verb commands as primary entry points: `list`, `show`, `validate`, `diff`, `archive`.
- Deprecate noun-based top-level commands: `ito spec ...` and `ito change ...`.
- Introduce consistent noun scoping via flags where applicable (e.g., `--changes`, `--specs`) and keep smart defaults.
- Clarify disambiguation for `show` and `validate` when names collide.

### Mappings (From → To)

- **List**

  - From: `ito change list`
  - To: `ito list --changes` (default), or `ito list --specs`

- **Show**

  - From: `ito spec show <spec-id>` / `ito change show <change-id>`
  - To: `ito show <item-id>` with auto-detect, use `--type spec|change` if ambiguous

- **Validate**

  - From: `ito spec validate <spec-id>` / `ito change validate <change-id>`
  - To: `ito validate <item-id> --type spec|change`, or bulk: `ito validate --specs` / `--changes` / `--all`

### Backward Compatibility

- Keep `ito spec` and `ito change` available with deprecation warnings for one release cycle.
- Update help text to point users to the verb–noun alternatives.

## Impact

- **Affected specs**:
  - `cli-list`: Add support for `--specs` and explicit `--changes` (default remains changes)
  - `ito-conventions`: Add explicit requirement establishing verb–noun CLI design and deprecation guidance
- **Affected code**:
  - `src/cli/index.ts`: Un-deprecate top-level `list`; mark `change list` as deprecated; ensure help text and warnings align
  - `src/core/list.ts`: Support listing specs via `--specs` and default to changes; shared output shape
  - Optional follow-ups: tighten `show`/`validate` help and ambiguity handling

## Explicit Changes

**CLI Design**

- From: Mixed model with nouns (`spec`, `change`) and some top-level verbs; `ito list` currently deprecated
- To: Verbs as primary: `ito list|show|validate|diff|archive`; nouns scoped via flags or item ids; noun commands deprecated
- Reason: Align with common CLIs; improve UX; simpler mental model
- Impact: Non-breaking with deprecation period; users migrate incrementally

**Listing Behavior**

- From: `ito change list` (primary), `ito list` (deprecated)
- To: `ito list` as primary, defaulting to `--changes`; add `--specs` to list specs
- Reason: Consistent verb–noun style; better discoverability
- Impact: New option; preserves existing behavior via default

## Rollout and Deprecation Policy

- Show deprecation warnings on noun-based commands for one release.
- Document new usage in `ito/README.md` and CLI help.
- After one release, consider removing noun-based commands, or keep as thin aliases without warnings.

## Open Questions

- Should `show` also accept `--changes`/`--specs` for discovery without an id? (Out of scope here; current auto-detect and `--type` remain.)

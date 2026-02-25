# Tasks for: 019-05_embed-openspec-schemas

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 019-05_embed-openspec-schemas
ito tasks next 019-05_embed-openspec-schemas
ito tasks start 019-05_embed-openspec-schemas 1.1
ito tasks complete 019-05_embed-openspec-schemas 1.1
ito tasks shelve 019-05_embed-openspec-schemas 1.1
ito tasks unshelve 019-05_embed-openspec-schemas 1.1
ito tasks show 019-05_embed-openspec-schemas
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Vendor OpenSpec schemas into embedded assets

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/**`
- **Dependencies**: None
- **Action**:
  - Vendor the upstream `openspec-schemas` directories for `minimalist` and `event-driven` into embedded schema assets.
  - Record upstream attribution metadata in-schema (for example an `UPSTREAM.md` with repo URL + pinned commit hash) to make updates traceable.
- **Verify**: `make check`
- **Done When**: Built-in schema assets include `minimalist` and `event-driven` and the source/pin is recorded.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.2: Add third-party attribution and license compliance artifacts

- **Files**: `THIRD_PARTY_NOTICES.md` (or equivalent), `docs/**` (if needed)
- **Dependencies**: None
- **Action**:
  - Add explicit in-tree attribution for `https://github.com/intent-driven-dev/openspec-schemas`.
  - Include the upstream license text or required references per the upstream license.
  - Clearly list which schemas were vendored.
- **Verify**: `make check`
- **Done When**: Attribution is present, unambiguous, and license requirements are satisfied.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.3: Ship Ito validation.yaml for embedded OpenSpec schemas

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/minimalist/validation.yaml`, `ito-rs/crates/ito-templates/assets/schemas/event-driven/validation.yaml`
- **Dependencies**: Task 1.1
- **Action**:
  - Add Ito-authored `validation.yaml` next to each embedded OpenSpec schema's `schema.yaml`.
  - Configure validation for presence-only checks plus an explicit manual semantic validation note (INFO).
  - Add tests so `ito validate` does not report Ito delta-spec failures for these schemas.
- **Verify**: `make check`
- **Done When**: Each embedded schema has `validation.yaml` and validation output is non-misleading.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.4: Ensure schema export and listing include the new schemas

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `ito-rs/crates/ito-templates/**`
- **Dependencies**: Task 1.1
- **Action**:
  - Add/update tests verifying `ito templates schemas export` includes `minimalist` and `event-driven`.
  - Add/update tests verifying schema listing/selection UX includes the new schema names.
- **Verify**: `make check`
- **Done When**: Export and listing tests cover the new schemas.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Document embedded OpenSpec schemas and validation behavior

- **Files**: `docs/**`
- **Dependencies**: None
- **Action**:
  - Document which OpenSpec schemas are embedded, how to export them, and how schema override precedence works.
  - Document that initial validation is presence-only + manual semantic validation note.
- **Verify**: `make check`
- **Done When**: Docs exist and match the behavior required by the proposal/specs.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

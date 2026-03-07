# Tasks for: 024-11_export-changes-zip-archive

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-11_export-changes-zip-archive
ito tasks next 024-11_export-changes-zip-archive
ito tasks start 024-11_export-changes-zip-archive 1.1
ito tasks complete 024-11_export-changes-zip-archive 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement backend export orchestration in core

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-domain/src/`
- **Dependencies**: None
- **Action**: Add core export flow that collects active and archived changes from backend-backed repositories and prepares deterministic export items.
- **Verify**: `cargo test -p ito-core export`
- **Done When**: Core exposes a tested export API returning deterministic active/archived artifact sets.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

### Task 1.2: Add canonical zip and manifest generation

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 1.1
- **Action**: Implement zip writer with canonical layout, manifest versioning, and per-file checksums.
- **Verify**: `cargo test -p ito-core export_manifest`
- **Done When**: Export archives contain `changes/active/`, `changes/archived/`, and `manifest.json` with integrity metadata.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Wire `ito backend export` CLI command

- **Files**: `ito-rs/crates/ito-cli/src/app/`, `ito-rs/crates/ito-cli/src/runtime.rs`
- **Dependencies**: None
- **Action**: Add CLI command surface for backend export, backend-mode gating, optional output-path argument, and summary output.
- **Verify**: `cargo test -p ito-cli backend_export`
- **Done When**: CLI writes archive at requested/default path and reports exported counts and integrity summary.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

### Task 2.2: Add deterministic packaging and mode-gating tests

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 2.1
- **Action**: Add tests for deterministic ordering, mixed active/archived exports, and rejection when backend mode is disabled.
- **Verify**: `cargo test -p ito-cli backend_export && cargo test -p ito-core export`
- **Done When**: Test coverage proves canonical layout, stable ordering, and correct backend-only behavior.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Document canonical backend export workflow

- **Files**: `docs/`, `.ito/`
- **Dependencies**: None
- **Action**: Add user-facing documentation for `ito backend export`, archive structure, and when to run export before migrations/cleanup.
- **Verify**: `make check`
- **Done When**: Docs describe command usage, output format, and validation expectations for operators.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

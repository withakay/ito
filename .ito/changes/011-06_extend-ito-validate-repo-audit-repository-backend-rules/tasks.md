<!-- ITO:START -->
# Tasks for: 011-06_extend-ito-validate-repo-audit-repository-backend-rules

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`.
- **Hard precondition**: change `011-05_add-ito-validate-repo-coordination-rules` MUST be merged to `main` before any task in this change is started — the rule engine that this change extends is introduced there.
- **Testing policy**: per `ito-rs/AGENTS.md` — TDD by default, hard floor 80% coverage, target 90%, prefer real implementations over mocks.

```bash
ito tasks status 011-06_extend-ito-validate-repo-audit-repository-backend-rules
ito tasks next 011-06_extend-ito-validate-repo-audit-repository-backend-rules
ito tasks start 011-06_extend-ito-validate-repo-audit-repository-backend-rules 1.1
ito tasks complete 011-06_extend-ito-validate-repo-audit-repository-backend-rules 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement audit rules

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/audit_rules.rs`, `ito-rs/crates/ito-core/src/validate_repo/registry.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: None
- **Action**: Implement `audit/mirror-branch-set` and `audit/mirror-branch-distinct-from-coordination`. Register both in `RuleRegistry::built_in()`. Each rule reads `audit.mirror.*` from `ItoConfig`; the second also reads `changes.coordination_branch.*` and only activates when both gates are satisfied.
- **Verify**: `cargo test -p ito-core --lib validate_repo::audit_rules`.
- **Done When**: Each rule has positive and negative fixtures plus a "skipped" fixture covering the disabled-mirror branch.
- **Requirements**: validate-repo-audit-rules:mirror-branch-set, validate-repo-audit-rules:mirror-branch-distinct-from-coordination
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 1.2: Implement repository rules

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/repository_rules.rs`, `ito-rs/crates/ito-core/src/validate_repo/registry.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: None
- **Action**: Implement `repository/sqlite-db-path-set` (resolves `repository.sqlite.db_path` against the project root, checks parent directory existence) and `repository/sqlite-db-not-committed` (uses `git check-ignore` and `git ls-files --error-unmatch` via `ProcessRunner` to classify the path). Register both rules.
- **Verify**: `cargo test -p ito-core --lib validate_repo::repository_rules` (uses `tempfile::TempDir` plus a small git harness from `ito-test-support`).
- **Done When**: All four scenarios in `validate-repo-repository-rules:*` are covered by fixture-based tests including the filesystem-mode skip path.
- **Requirements**: validate-repo-repository-rules:sqlite-db-path-set, validate-repo-repository-rules:sqlite-db-not-committed
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 1.3: Add per-layer config access for backend/token-not-committed

- **Files**: `ito-rs/crates/ito-config/src/config/mod.rs`, `ito-rs/crates/ito-config/src/lib.rs`
- **Dependencies**: None
- **Action**: Confirm that `load_cascading_project_config(...)` returns its constituent layers (e.g. as `.layers: Vec<ResolvedConfigLayer>` with origin metadata). If only the merged view is exposed today, extend the type minimally so callers can ask "did this value come from a tracked-by-git file?". Add unit tests for the new accessor with synthetic layer fixtures.
- **Verify**: `cargo test -p ito-config`.
- **Done When**: The accessor is documented (`#![warn(missing_docs)]`), and tests cover at least three layers (committed `.ito/config.json`, gitignored `.ito/config.local.json`, env-var override).
- **Requirements**: validate-repo-backend-rules:token-not-committed
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 1.4: Implement backend rules

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/backend_rules.rs`, `ito-rs/crates/ito-core/src/validate_repo/registry.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: Task 1.3
- **Note**: Task 1.3 is in the same wave; the engine accessor must land before the rule that consumes it. If 1.3 is delayed, this task can stub the accessor and rebase before merge.
- **Action**: Implement `backend/token-not-committed` (uses the per-layer accessor and consults the `ITO_BACKEND_TOKEN` env var), `backend/url-scheme-valid` (uses `url::Url::parse` and constrains scheme to `http`/`https`), and `backend/project-org-repo-set`. Register all three. `backend/token-not-committed` SHALL emit `LEVEL_ERROR` regardless of the engine's `--strict` flag.
- **Verify**: `cargo test -p ito-core --lib validate_repo::backend_rules`.
- **Done When**: All scenarios from `validate-repo-backend-rules:*` are covered by fixture tests, including the env-var-passes case and the strict-flag-does-not-weaken-severity case.
- **Requirements**: validate-repo-backend-rules:token-not-committed, validate-repo-backend-rules:url-scheme-valid, validate-repo-backend-rules:project-org-repo-set
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Registry integration test for the full rule set

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: None
- **Action**: Add an integration test that seeds `ItoConfig` permutations (audit on/off, sqlite/filesystem, backend on/off, with and without coordination worktree) and asserts the active-rule set produced by `list_active_rules` matches the expected matrix.
- **Verify**: `cargo test -p ito-core --lib validate_repo::tests::registry_integration`.
- **Done When**: The matrix covers all gate combinations introduced by this change.
- **Requirements**: validate-repo-audit-rules:mirror-branch-set, validate-repo-repository-rules:sqlite-db-path-set, validate-repo-backend-rules:token-not-committed
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 2.2: CLI snapshot tests for rule output

- **Files**: `ito-rs/crates/ito-cli/tests/validate_repo_extended.rs` (or equivalent existing harness)
- **Dependencies**: None
- **Action**: Add CLI snapshot tests covering human and JSON output for each new rule, plus an `ito validate repo --list-rules` snapshot showing the expanded registry.
- **Verify**: `cargo test -p ito-cli`.
- **Done When**: Snapshots are checked in and stable across reruns; `ito validate repo --list-rules` includes the seven new rule ids in deterministic order.
- **Requirements**: validate-repo-audit-rules:mirror-branch-distinct-from-coordination, validate-repo-repository-rules:sqlite-db-not-committed, validate-repo-backend-rules:url-scheme-valid, validate-repo-backend-rules:project-org-repo-set
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Documentation + final quality gate

- **Files**: `.ito/architecture.md`, `ito-rs/crates/ito-core/AGENTS.md`, `Makefile`
- **Dependencies**: None
- **Action**: Cross-reference the new rule modules from `.ito/architecture.md` and `ito-core`'s AGENTS.md "Key Modules" table. Run `make check`, `make test-coverage`, `make arch-guardrails`, `make cargo-deny`, `make check-max-lines` from the change worktree and address any findings.
- **Verify**: All gates exit 0; `make docs` passes.
- **Done When**: The change is ready for `ito archive 011-06_extend-ito-validate-repo-audit-repository-backend-rules`.
- **Requirements**: validate-repo-audit-rules:mirror-branch-set, validate-repo-repository-rules:sqlite-db-path-set, validate-repo-backend-rules:token-not-committed
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave.
- Wave 1 may proceed in parallel internally; Wave 2 cannot start until Wave 1 is complete (engine API stability matters for snapshot tests).
- Per `ito-rs/AGENTS.md`, run `rust-quality-checker`, `rust-code-reviewer`, and `codex-review` subagents on non-trivial diffs before commit.
<!-- ITO:END -->

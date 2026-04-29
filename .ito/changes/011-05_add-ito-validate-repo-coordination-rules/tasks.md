<!-- ITO:START -->
# Tasks for: 011-05_add-ito-validate-repo-coordination-rules

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`.
- **Testing policy**: per `ito-rs/AGENTS.md` — TDD by default, hard floor 80% coverage, target 90%, prefer real implementations over mocks. New tests live alongside the code they cover.
- **Verification**: `make check` is the local gate; CI mirrors the pre-push prek run.

```bash
ito tasks status 011-05_add-ito-validate-repo-coordination-rules
ito tasks next 011-05_add-ito-validate-repo-coordination-rules
ito tasks start 011-05_add-ito-validate-repo-coordination-rules 1.1
ito tasks complete 011-05_add-ito-validate-repo-coordination-rules 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Scaffold ito-core validate_repo module skeleton

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/mod.rs`, `ito-rs/crates/ito-core/src/validate_repo/rule.rs`, `ito-rs/crates/ito-core/src/lib.rs`
- **Dependencies**: None
- **Action**: Create the module tree with `Rule` trait, `RuleId`, `RuleSeverity`, `RuleContext`, `RuleRegistry`, `StagedFiles`, and a `run_repo_validation` stub returning an empty `ValidationReport`. Wire the module into `ito-core::lib.rs`. No rules yet.
- **Verify**: `make check` passes; `cargo test -p ito-core --lib validate_repo` runs (suite empty, exit 0).
- **Done When**: Module compiles, public types are documented (`#![warn(missing_docs)]`), and the `Rule` trait + registry compile against an empty rule list.
- **Requirements**: validate-repo-engine:gate-filtering, validate-repo-engine:reuse-envelope, validate-repo-engine:gate-metadata
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 1.2: Implement list_active_rules introspection

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/registry.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: Task 1.1
- **Action**: Implement `list_active_rules(config) -> Vec<ActiveRule>` returning `{rule_id, severity, active, gate}` for every registered rule. Add unit tests covering an empty registry and a registry with one always-active stub rule.
- **Verify**: `cargo test -p ito-core --lib validate_repo::registry`.
- **Done When**: The function compiles, returns deterministic ordering by `RuleId`, and is fully covered by tests.
- **Requirements**: validate-repo-engine:list-active-rules
- **Updated At**: 2026-04-29
- **Status**: [x] complete

### Task 1.3: Implement StagedFiles snapshot reader

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/staged.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: Task 1.1
- **Action**: Implement `StagedFiles::from_git(project_root) -> CoreResult<StagedFiles>` that runs `git diff --cached --name-only -z` via `ito-core::process::ProcessRunner`, parses null-byte-delimited paths, and stores them in a sorted `BTreeSet<PathBuf>`. Provide a constructor accepting a fixed list for tests. Surface git failures with What/Why/Fix messages per `ito-rs/AGENTS.md`.
- **Verify**: `cargo test -p ito-core --lib validate_repo::staged` (use `ito-test-support` git harness).
- **Done When**: Reader handles empty index, paths with newlines (zero-byte separator), and missing-git error, each with a regression test.
- **Requirements**: validate-repo-engine:staged-context
- **Updated At**: 2026-04-29
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Refactor coordination to expose canonical gitignore entries

- **Files**: `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-core/src/coordination_tests.rs`
- **Dependencies**: None
- **Action**: Extract the canonical `.ito/<dir>` list used by `update_gitignore_for_symlinks` into a pure function `gitignore_entries() -> Vec<&'static str>`. `update_gitignore_for_symlinks` calls it and the new rule reuses it. No behavioural change to existing call sites.
- **Verify**: `cargo test -p ito-core --lib coordination`.
- **Done When**: `update_gitignore_for_symlinks` and the new rule both consume the same canonical list and there is no duplicated literal.
- **Requirements**: validate-repo-coordination-rules:gitignore-entries
- **Updated At**: 2026-04-29
- **Status**: [>] in-progress

### Task 2.2: Implement coordination rules

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/coordination_rules.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: Task 2.1
- **Action**: Implement `coordination/symlinks-wired` (wraps `check_coordination_health` + `format_health_message`), `coordination/gitignore-entries` (uses `gitignore_entries()`), `coordination/staged-symlinked-paths` (uses `StagedFiles`), and `coordination/branch-name-set`. Register each in `RuleRegistry::built_in()`.
- **Verify**: `cargo test -p ito-core --lib validate_repo::coordination_rules`.
- **Done When**: Each rule has at least one passing-fixture and one failing-fixture test, and the embedded-storage skip path is covered.
- **Requirements**: validate-repo-coordination-rules:symlinks-wired, validate-repo-coordination-rules:gitignore-entries, validate-repo-coordination-rules:staged-symlinked-paths, validate-repo-coordination-rules:branch-name-set
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 2.3: Implement worktrees rules

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/worktrees_rules.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: None
- **Action**: Implement `worktrees/no-write-on-control` (reusing branch / worktree detection from `worktree_validate`) and `worktrees/layout-consistent`. Register each in `RuleRegistry::built_in()`.
- **Verify**: `cargo test -p ito-core --lib validate_repo::worktrees_rules`.
- **Done When**: Tests cover the active/inactive matrix for `worktrees.enabled`, all `WorktreeStrategy` variants, and the empty/non-empty staged context.
- **Requirements**: validate-repo-worktrees-rules:no-write-on-control, validate-repo-worktrees-rules:layout-consistent
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 2.4: Implement detect_pre_commit_system

- **Files**: `ito-rs/crates/ito-core/src/validate_repo/pre_commit_detect.rs`, `ito-rs/crates/ito-core/src/validate_repo/tests.rs`
- **Dependencies**: None
- **Action**: Implement the detection algorithm documented in the design doc and the `pre-commit-hook-detection` spec. Pure function over filesystem reads; no shell-out beyond an optional `which prek` resolution. Expose `PreCommitSystem` enum publicly.
- **Verify**: `cargo test -p ito-core --lib validate_repo::pre_commit_detect` covers all five variants via fixture project layouts under `tempfile::TempDir`.
- **Done When**: Each `PreCommitSystem` variant has at least one positive and one negative fixture; the detector is byte-equivalent before/after the call.
- **Requirements**: pre-commit-hook-detection:detector-api, pre-commit-hook-detection:prek-vs-precommit, pre-commit-hook-detection:husky-and-lefthook, pre-commit-hook-detection:none-default, pre-commit-hook-detection:read-only
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add Repo variant to ValidateCommand

- **Files**: `ito-rs/crates/ito-cli/src/cli/validate.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**: Extend the existing `ValidateCommand` enum with a `Repo(RepoValidateArgs)` variant. Define `RepoValidateArgs` with `--staged`, `--strict`, `--json`, `--rule <id>` (repeatable), `--no-rule <id>` (repeatable, mutually exclusive with `--rule`), `--list-rules`, `--explain <id>` flags. Update help text.
- **Verify**: `cargo build -p ito-cli`; `cargo test -p ito-cli cli_tests`.
- **Done When**: `ito validate --help` lists the new subcommand and `ito validate repo --help` documents every flag.
- **Requirements**: validate-repo-cli-surface:repo-subcommand, validate-repo-cli-surface:hook-flags
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 3.2: Implement validate_repo CLI adapter

- **Files**: `ito-rs/crates/ito-cli/src/app/validate_repo.rs`, `ito-rs/crates/ito-cli/src/app/mod.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`
- **Dependencies**: Task 3.1
- **Action**: Wire `Repo(RepoValidateArgs)` to a thin handler that loads `ItoConfig`, calls the engine, and renders human or JSON output reusing helpers from `app::validate`. Implement exit-code mapping (0 / 1 / 2) and ensure `--list-rules` and `--explain` short-circuit before running rule checks.
- **Verify**: `cargo test -p ito-cli` (snapshot tests for human and JSON output).
- **Done When**: All scenarios from `validate-repo-cli-surface` are covered by integration tests; `ito validate repo --json` matches the `ValidationReport` JSON envelope used elsewhere.
- **Requirements**: validate-repo-cli-surface:repo-subcommand, validate-repo-cli-surface:hook-flags, validate-repo-cli-surface:exit-codes
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Emit ito init repo-validation advisory

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs` (or new `app/init_advisory.rs`), `ito-rs/crates/ito-cli/src/app/run.rs`
- **Dependencies**: None
- **Action**: After `ito init` and `ito init --upgrade` complete their primary work, call `validate_repo::list_active_rules(config)`. If at least one rule activates AND no pre-commit hook for `ito validate repo` is detected (probe `.pre-commit-config.yaml`, `.husky/pre-commit`, etc.), print the advisory naming the detected `PreCommitSystem` and pointing at the `ito-update-repo` skill. The advisory MUST NOT mutate `.pre-commit-config.yaml` or any other file — that is left to the `ito-update-repo` skill so downstream projects can opt in by running the skill explicitly.
- **Verify**: `cargo test -p ito-cli` (new tests covering active-rule, quiet, and "init does not write hook" paths).
- **Done When**: All `ito-init:*` scenarios pass; the advisory is silent when no rules activate and a hook is already installed; `ito init` is verified by test to leave `.pre-commit-config.yaml` byte-identical.
- **Requirements**: ito-init:repo-validation-advisory, ito-init:advisory-detected-system, ito-init:advisory-references-update-repo, pre-commit-hooks:opt-in-downstream
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 4.2: Replace pre-commit no-op stub

- **Files**: `ito-rs/tools/hooks/pre-commit`
- **Dependencies**: None
- **Action**: Replace the no-op stub with a script that runs `ito validate repo --staged --strict` and exits with the command's exit code. Preserve POSIX-only syntax so the script runs on Linux/macOS; document the Windows alternative in `ito-rs/AGENTS.md`.
- **Verify**: Run the script manually with a staged dirty file; confirm non-zero exit aborts the commit. `make check` still passes.
- **Done When**: The script invokes the new command; the previous "intentionally no-op" comment is replaced by a short rationale plus a link to the `pre-commit-hooks` capability spec.
- **Requirements**: pre-commit-hooks:replace-no-op-stub
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 4.3: Wire pre-commit hook into pre-commit-config

- **Files**: `.pre-commit-config.yaml`, `ito-rs/AGENTS.md`
- **Dependencies**: Task 4.2
- **Action**: Add a `local` repo with an `id: ito-validate-repo` hook running `ito validate repo --staged --strict` at `pre-commit` stage. Update `ito-rs/AGENTS.md` "Git Hooks (prek)" section so the convention change is documented.
- **Verify**: `prek run --all-files --hook-stage pre-commit ito-validate-repo` exits 0 on a clean checkout.
- **Done When**: The hook runs as part of `prek install -t pre-commit` and CI's `make check` mirror.
- **Requirements**: pre-commit-hooks:repo-pre-commit-stage, pre-commit-hooks:replace-no-op-stub
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Extend canonical ito-update-repo skill

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-update-repo/SKILL.md`
- **Dependencies**: None
- **Action**: Add a "Pre-commit hook setup" step after the orphan-cleanup step. Document the detection table verbatim from the design doc, the per-system edits, the dry-run / approval requirement, and the verification step (`ito validate repo --staged --strict`). Update the front-matter `description` to mention pre-commit setup.
- **Verify**: `cargo test -p ito-templates` (asset bundling tests still pass); `cargo test -p ito-cli init_tests` (post-install asset diff tests).
- **Done When**: The skill prose covers all `ito-update-repo-skill:*` scenarios; the file size stays under the 1000-line soft cap.
- **Requirements**: ito-update-repo-skill:pre-commit-step, ito-update-repo-skill:dry-run-default, ito-update-repo-skill:verify-after-install
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 5.2: Sync ito-update-repo harness command shells

- **Files**: `ito-rs/crates/ito-templates/assets/commands/ito-update-repo.md`, plus harness equivalents installed by Ito templates
- **Dependencies**: Task 5.1
- **Action**: Update each command shell's description and Notes block so it mentions pre-commit hook setup as part of the skill's scope. Keep each harness functionally equivalent per `ito-templates/AGENTS.md`.
- **Verify**: `cargo test -p ito-templates`; spot-check `ito init --force --tools all` against a temp project and confirm the rendered files match.
- **Done When**: All harness command shells mention pre-commit setup; no harness shell drifts from the canonical version.
- **Requirements**: ito-update-repo-skill:harness-discoverability
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 5.3: Update project-setup instruction

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/project-setup.md.j2`
- **Dependencies**: Task 5.1
- **Action**: Add a step (or extend an existing one) directing the project-setup agent to invoke `ito-update-repo` to wire the pre-commit hook once the rest of setup is done. Keep the instruction terse and cite the detection table in the skill rather than duplicating it.
- **Verify**: `cargo test -p ito-templates` (instruction-rendering tests still pass).
- **Done When**: The rendered instruction includes the reference and downstream consumers see it via `ito agent instruction project-setup`.
- **Requirements**: ito-init:advisory-references-update-repo, ito-update-repo-skill:pre-commit-step
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave 6

- **Depends On**: Wave 4, Wave 5

### Task 6.1: Self-test pre-commit hook on this repo

- **Files**: `.ito/changes/011-05_add-ito-validate-repo-coordination-rules/demos/`
- **Dependencies**: None
- **Action**: From a change worktree, run the `ito-update-repo` skill end-to-end (via the harness), accept the proposed pre-commit edit, then run `prek run --all-files --hook-stage pre-commit ito-validate-repo` and `ito validate repo --staged --strict` and capture the output as a Showboat demo under `.ito/changes/011-05_…/demos/`.
- **Verify**: Demo doc shows a clean exit 0 from both commands; the wired symlinks make `coordination/symlinks-wired` pass on this repo.
- **Done When**: The demo doc is committed and `make check` is clean from the change worktree.
- **Requirements**: validate-repo-cli-surface:exit-codes, ito-update-repo-skill:verify-after-install, pre-commit-hooks:repo-pre-commit-stage
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 6.2: Documentation pass

- **Files**: `.ito/architecture.md`, `ito-rs/AGENTS.md`, `ito-rs/crates/ito-core/AGENTS.md`, `ito-rs/crates/ito-cli/AGENTS.md`
- **Dependencies**: None
- **Action**: Add a one-paragraph "Repository validation rules" section to `.ito/architecture.md` linking to the `validate_repo` module. Cross-reference the new module from `ito-core`'s and `ito-cli`'s AGENTS.md "Key Modules" tables.
- **Verify**: `make docs` passes; `documentation-police` subagent reports no missing docs.
- **Done When**: New module is discoverable from the top-level architecture doc and both adapter / core AGENTS.md tables.
- **Requirements**: ito-init:repo-validation-advisory
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

### Task 6.3: Final quality gate

- **Files**: `Makefile`
- **Dependencies**: Task 6.1, Task 6.2
- **Action**: Run `make check`, `make test-coverage`, `make arch-guardrails`, `make cargo-deny`, and `make check-max-lines` from the change worktree. Address any findings.
- **Verify**: All gates exit 0.
- **Done When**: The change is ready for `ito archive 011-05_add-ito-validate-repo-coordination-rules`.
- **Requirements**: validate-repo-cli-surface:exit-codes
- **Updated At**: 2026-04-29
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave.
- Mark a task `in-progress` only when actively working on it; the audit log records every transition.
- Per `ito-rs/AGENTS.md`, run `rust-quality-checker`, `rust-code-reviewer`, and `codex-review` subagents on non-trivial diffs before commit.
<!-- ITO:END -->

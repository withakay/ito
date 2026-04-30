<!-- ITO:START -->
# Tasks for: 001-37_fix-worktree-symlink-recovery

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-37_fix-worktree-symlink-recovery
ito tasks next 001-37_fix-worktree-symlink-recovery
ito tasks start 001-37_fix-worktree-symlink-recovery 1.1
ito tasks complete 001-37_fix-worktree-symlink-recovery 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Wire coordination symlinks during `ito worktree ensure`

- **Files**: `ito-rs/crates/ito-core/src/worktree_ensure.rs`, `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-core/tests/worktree_ensure_e2e.rs`
- **Dependencies**: None
- **Action**: Update worktree creation so `ito worktree ensure` wires `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` before returning success in coordination-worktree mode.
- **Verify**: `cargo test -p ito-core --test worktree_ensure_e2e`
- **Done When**: A new ensured worktree is immediately Ito-ready without requiring a follow-up `ito init --update`.
- **Requirements**: `worktree-lifecycle:ensure-wires-coordination-links`, `coordination-worktree:repair-current-worktree-links`
- **Updated At**: 2026-04-30
- **Status**: [x] complete

### Task 1.2: Formalize repair behavior for existing unwired worktrees

- **Files**: `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-cli/src/app/init.rs`, `ito-rs/crates/ito-cli/tests/init_more.rs`
- **Dependencies**: Task 1.1
- **Action**: Make the worktree-local repair path explicit so `ito init --update` on an existing worktree rewires missing or stale coordination symlinks in worktree mode and is covered by tests.
- **Verify**: `cargo test -p ito-cli --test init_more`
- **Done When**: The repair path used in the reproduction is contractually supported and regression-tested.
- **Requirements**: `coordination-worktree:repair-current-worktree-links`, `cli-init:repair-coordination-links-in-existing-worktree`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Improve create-change recovery and error messaging

- **Files**: `ito-rs/crates/ito-core/src/create/mod.rs`, `ito-rs/crates/ito-core/tests/`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Detect missing coordination wiring during `ito create change` and either repair it automatically or fail with a specific recovery message that names the missing path and recommended next step.
- **Verify**: `cargo test -p ito-core create && cargo test -p ito-cli cli_smoke`
- **Done When**: Missing wiring no longer produces the opaque module-not-found or generic `os error 2` path without context.
- **Requirements**: `change-creation:missing-coordination-wiring-recovery`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

### Task 2.2: Reproduce the reported session end-to-end

- **Files**: `ito-rs/crates/ito-core/tests/worktree_ensure_e2e.rs`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 2.1
- **Action**: Add regression coverage for the exact sequence captured in `issues.md`, including a fresh worktree, missing links, repair, and change creation.
- **Verify**: `cargo test -p ito-core --test worktree_ensure_e2e && cargo test -p ito-cli --test init_more`
- **Done When**: The observed symlink-recovery failure sequence is encoded as a stable regression test.
- **Requirements**: `worktree-lifecycle:ensure-wires-coordination-links`, `cli-init:repair-coordination-links-in-existing-worktree`, `change-creation:missing-coordination-wiring-recovery`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update worktree guidance and recovery instructions

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`, `ito-rs/crates/ito-templates/assets/instructions/agent/worktree-init.md.j2`, `docs/agent-workflow.md`
- **Dependencies**: None
- **Action**: Align the instructions with the fixed behavior: `ito worktree ensure` should be the default one-step path, with `ito init --update` documented as the repair fallback for pre-existing or manually created worktrees.
- **Verify**: `make docs`
- **Done When**: Agent and human guidance match the implemented behavior and recovery path.
- **Requirements**: `worktree-lifecycle:ensure-wires-coordination-links`, `cli-init:repair-coordination-links-in-existing-worktree`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

### Task 3.2: Final validation and quality gate

- **Files**: `.ito/changes/001-37_fix-worktree-symlink-recovery/`, affected Rust and template files
- **Dependencies**: Task 3.1
- **Action**: Run strict Ito validation and the targeted Rust quality checks for worktree wiring, init recovery, and create-change behavior.
- **Verify**: `ito validate 001-37_fix-worktree-symlink-recovery --strict && cargo test -p ito-core --test worktree_ensure_e2e && cargo test -p ito-cli --test init_more`
- **Done When**: The change validates strictly and the targeted worktree regressions pass.
- **Requirements**:
- **Updated At**: 2026-04-30
- **Status**: [ ] pending
<!-- ITO:END -->

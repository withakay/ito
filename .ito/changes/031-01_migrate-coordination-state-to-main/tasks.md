<!-- ITO:START -->
# Tasks for: 031-01_migrate-coordination-state-to-main

## Execution Notes
- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 031-01_migrate-coordination-state-to-main
ito tasks next 031-01_migrate-coordination-state-to-main
ito tasks start 031-01_migrate-coordination-state-to-main 1.1
ito tasks complete 031-01_migrate-coordination-state-to-main 1.1
```

______________________________________________________________________
## Wave 1: Detection and intent contracts

- **Depends On**: None

### Task 1.1: Implement legacy coordination evidence detection
- **Files**: `ito-rs/crates/ito-core/src/legacy_coordination.rs`; `ito-rs/crates/ito-core/src/legacy_coordination_tests.rs`; `ito-rs/crates/ito-core/src/lib.rs`
- **Dependencies**: None
- **Action**: Add the unconditionally compiled report/classification API. Inspect resolved coordination configuration, the five managed state paths with `symlink_metadata`, exact/wrong/broken link targets, and the managed gitignore marker. Report main-compatible, legacy, or ambiguous state with structured evidence and no side effects.
- **Verify**: `cargo test -p ito-core legacy_coordination`
- **Done When**: The test matrix covers configuration-only, correct links, broken links, wrong links, mixed real/link state, residual markers, and fully materialized real directories; detector calls leave fixture state unchanged.
- **Updated At**: 2026-07-13
- **Requirements**: coordination-main-migration:legacy-state-detection
- **Status**: [x] complete

### Task 1.2: Define exhaustive CLI command intent policy
- **Files**: `ito-rs/crates/ito-cli/src/app/legacy_coordination.rs`; `ito-rs/crates/ito-cli/src/app/legacy_coordination_tests.rs`; `ito-rs/crates/ito-cli/src/app/mod.rs`; `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Classify every parsed top-level operation as read-only, mutating, or recovery. Default unrecognized/future operations to mutating and explicitly keep instruction rendering and diagnostic reads available.
- **Verify**: `cargo test -p ito-cli command_intent`
- **Done When**: Tests enumerate every command variant, prove mutating is the fail-closed default, and document the recovery exceptions.
- **Updated At**: 2026-07-13
- **Requirements**: coordination-main-migration:read-write-safety-policy
- **Status**: [x] complete

______________________________________________________________________
## Wave 2: Runtime guard and migration instruction
- **Depends On**: Wave 1

### Task 2.1: Enforce warning and blocking before dispatch side effects
- **Files**: `ito-rs/crates/ito-cli/src/app/run.rs`; `ito-rs/crates/ito-cli/src/app/legacy_coordination.rs`; `ito-rs/crates/ito-cli/src/diagnostics.rs`; `ito-rs/crates/ito-cli/tests/legacy_coordination_guard.rs`
- **Dependencies**: None
- **Action**: Invoke the detector after configuration/path resolution but before sync, repository construction, command handling, filesystem mutation, worktree creation, or network calls. Emit one warning for allowed reads and return a typed no-mutation error for writes, both naming the exact remediation instruction.
- **Verify**: `cargo test -p ito-cli --test legacy_coordination_guard`
- **Done When**: Integration fixtures prove read commands succeed with one warning, mutating commands fail with unchanged filesystem/Git/task state, and ambiguous legacy state fails closed.
- **Updated At**: 2026-07-13
- **Requirements**: coordination-main-migration:read-write-safety-policy, agent-instructions:legacy-coordination-remediation
- **Status**: [ ] pending

### Task 2.2: Add the always-available migrate-to-main instruction
- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`; `ito-rs/crates/ito-cli/src/cli/agent.rs`; `ito-rs/crates/ito-templates/assets/instructions/agent/migrate-to-main.md.j2`; `ito-rs/crates/ito-templates/src/instructions.rs`; `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**: Add dispatch, help, JSON identity, context construction, and template rendering. Encode ordered snapshot, inventory/hash, conflict stop, real-directory materialization, config update, validation, review/PR, and source-preservation steps without importing feature-gated coordination runtime modules.
- **Verify**: `cargo test -p ito-templates migrate_to_main && cargo test -p ito-cli migrate_to_main`
- **Done When**: Default/no-coordination builds render contextual instructions; tests assert stop conditions, source preservation, embedded/disabled config, hash parity, validation, and reviewed integration guidance.
- **Updated At**: 2026-07-13
- **Requirements**: coordination-main-migration:preserve-and-prove-state, coordination-worktree-migration:reverse-agent-driven-migration, coordination-worktree-migration:reverse-migration-rollback-evidence, agent-instructions:migrate-to-main-availability
- **Status**: [ ] pending

______________________________________________________________________
## Wave 3: Distribution and end-to-end proof
- **Depends On**: Wave 2

### Task 3.1: Distribute remediation and update current guidance
- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`; `ito-rs/crates/ito-core/tests/distribution.rs`; `ito-rs/crates/ito-templates/assets/project/.ito/AGENTS.md`; `docs/src/content/docs/reference/agent-instructions.md`; `.ito/wiki/topics/runtime-and-storage.md`
- **Dependencies**: None
- **Action**: Ensure all supported harness command/prompt surfaces can invoke the instruction without a separate migration skill. Update current guidance and wiki synthesis to describe detection, read/write policy, and the non-destructive agent-driven bridge while leaving archived history intact.
- **Verify**: `cargo test -p ito-core distribution && cargo test -p ito-templates instruction`
- **Done When**: Every supported harness exposes one route to the instruction, current docs no longer recommend deepening legacy coordination state, and generated surfaces remain refreshable.
- **Updated At**: 2026-07-13
- **Requirements**: agent-instructions:migrate-to-main-availability, agent-instructions:legacy-coordination-remediation
- **Status**: [ ] pending

### Task 3.2: Prove a reversible fixture migration
- **Files**: `ito-rs/crates/ito-cli/tests/migrate_to_main_instruction.rs`; `ito-rs/crates/ito-test-support/src/fixtures.rs`; `demos/031-01-migrate-to-main.md`
- **Dependencies**: Task 3.1
- **Action**: Add a legacy fixture with coordination links and controlled content, render/follow the instruction in a reproducible Showboat demo, and record inventories/hashes before and after materialization. Validate the migrated fixture and verify the external source commit and content are unchanged.
- **Verify**: `cargo test -p ito-cli --test migrate_to_main_instruction && ito validate 031-01_migrate-coordination-state-to-main --strict`
- **Done When**: Automated tests and captured demo evidence prove clean migration, conflict-stop behavior, exact parity, main-compatible post-state, and untouched rollback source.
- **Updated At**: 2026-07-13
- **Requirements**: coordination-main-migration:legacy-state-detection, coordination-main-migration:preserve-and-prove-state, coordination-worktree-migration:reverse-migration-rollback-evidence
- **Status**: [ ] pending

______________________________________________________________________
## Wave Guidelines
- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Keep exactly one task in progress at a time for this change
- Do not begin `031-02` or later implementation until this migration bridge has passed its change-level review gate
<!-- ITO:END -->

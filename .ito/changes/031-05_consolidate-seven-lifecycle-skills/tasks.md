<!-- ITO:START -->
# Tasks for: 031-05_consolidate-seven-lifecycle-skills

## Execution Notes
- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 031-05_consolidate-seven-lifecycle-skills
ito tasks next 031-05_consolidate-seven-lifecycle-skills
ito tasks start 031-05_consolidate-seven-lifecycle-skills 1.1
ito tasks complete 031-05_consolidate-seven-lifecycle-skills 1.1
```

______________________________________________________________________
## Wave 1: Exact inventory and content contracts

- **Depends On**: None

### Task 1.1: Define exact seven-skill manifest tests
- **Files**: `ito-rs/crates/ito-templates/src/manifest_tests.rs`; `ito-rs/crates/ito-templates/src/lib_tests.rs`; `ito-rs/crates/ito-core/tests/distribution.rs`; `ito-rs/crates/ito-cli/tests/init_agent_activation.rs`
- **Dependencies**: None
- **Action**: Add failing exact-set assertions for shared assets, every harness manifest, and installed `SKILL.md` entrypoints. Assert the seven ordered names and prove `ito-loop` remains present.
- **Verify**: `cargo test -p ito-templates skill_inventory -- --nocapture && cargo test -p ito-core skill_inventory -- --nocapture && cargo test -p ito-cli --test init_agent_activation -- --nocapture`
- **Done When**: Tests fail on any missing, duplicate, or additional Ito-managed skill and distinguish native agent files from skills.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.2: Map every retired helper to a retained lifecycle phase
- **Files**: `ito-rs/crates/ito-templates/assets/skills/`; `ito-rs/crates/ito-templates/assets/commands/`; `ito-rs/crates/ito-templates/src/legacy.rs`; `docs/src/content/docs/reference/skills.md`
- **Dependencies**: None
- **Action**: Inventory all current skill and command assets, record a replacement phase, intentional CLI-only path, native-agent carveout, or deliberate removal for each retired name, and add content tests proving no unique safety/gate guidance is lost before deletion. Cover planning→`ito-proposal`, memory→`ito-research`/`ito-archive`, orchestration→`ito-loop`, archive/sync→`ito-archive`, update/remediation→root `ito` or direct CLI, and tmux→removed.
- **Verify**: `cargo test -p ito-templates lifecycle_skill_content`
- **Done When**: Every retired managed entry has exactly one replacement owner and the replacement map covers intake, planning, tasks, worktrees, verification, finish, memory, wiki, orchestration, update, cleanup, commit, and path/list helpers.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.3: Lock the current-spec conflict matrix
- **Files**: delta specs under `.ito/changes/031-05_consolidate-seven-lifecycle-skills/specs/`; current specs under `.ito/specs/`; proposal capability list
- **Dependencies**: None
- **Action**: Verify identity-matched deltas for every current requirement that promises a retired planning, memory, orchestration, update, tmux, archive, sync, agent-as-skill, or wildcard-routing surface. Ensure native agents remain separate and the exact seven names are unchanged.
- **Verify**: `ito validate 031-05_consolidate-seven-lifecycle-skills --strict`
- **Done When**: Strict validation resolves every affected current requirement by its exact heading, and the proposal capability list matches all delta-spec directories.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

______________________________________________________________________
## Wave 2: Consolidate shared skills and manifests

- **Depends On**: Wave 1

### Task 2.1: Implement the canonical lifecycle inventory
- **Files**: `ito-rs/crates/ito-templates/src/manifest.rs`; `ito-rs/crates/ito-templates/src/lib.rs`; `ito-rs/crates/ito-core/src/distribution.rs`; associated sibling test modules
- **Dependencies**: None
- **Action**: Define the canonical seven names once, select retained shared assets from it, make every harness adapter consume that selection, and fail clearly when a retained asset is absent or duplicated.
- **Verify**: `cargo test -p ito-templates manifest && cargo test -p ito-core distribution`
- **Done When**: All logical harness manifests expose the identical exact set and no harness-specific code can append an Ito skill outside the canonical inventory.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 2.2: Rewrite retained skills as lifecycle phase entrypoints
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito/`; `ito-proposal/`; `ito-research/`; `ito-apply/`; `ito-review/`; `ito-archive/`; `ito-loop/`
- **Dependencies**: None
- **Action**: Fold the approved helper mapping into the seven managed sections and phase-specific resources. Simplify `ito` to fixed lifecycle routing plus direct CLI fallback; route planning through `ito-proposal`, memory through `ito-research`/`ito-archive`, orchestration through `ito-loop`, and spec promotion through `ito-archive`; reference authoritative instruction artifacts instead of duplicating long policy.
- **Verify**: `cargo test -p ito-templates lifecycle_skill_content && cargo test -p ito-templates template_markdown`
- **Done When**: Each former helper concern is discoverable from its owner, router tests cover retained/retired/direct-CLI cases, and the retained skills contain no wildcard skill discovery/cache behavior.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 2.3: Delete retired shared skills and helper command wrappers
- **Files**: retired directories under `ito-rs/crates/ito-templates/assets/skills/`; retired files under `assets/commands/`; template embed/snapshot tests
- **Dependencies**: Task 2.2
- **Action**: Remove all non-seven shared skill entrypoints and command/prompt wrappers that expose retired helper activation names, including planning, memory, orchestration setup/workflow, update-repo, archive-change, sync-specs, and tmux assets/scripts. Keep direct CLI commands and instruction templates required by the retained phases.
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-core distribution`
- **Done When**: The embedded shared asset tree has exactly seven `SKILL.md` entrypoints, retained commands resolve, no retired wrapper is emitted, and no tmux helper script remains in the managed bundle.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

______________________________________________________________________
## Wave 3: Agent separation, upgrade cleanup, and proof

- **Depends On**: Wave 2

### Task 3.1: Separate native delegated agents from skills
- **Files**: `ito-rs/crates/ito-templates/src/agents.rs`; `ito-rs/crates/ito-templates/src/agents_tests.rs`; `ito-rs/crates/ito-templates/assets/agents/`; `ito-rs/crates/ito-cli/tests/init_agent_activation.rs`
- **Dependencies**: None
- **Action**: Retain role definitions only at harness-native agent destinations, remove Codex/other role `SKILL.md` installation paths, and use instruction-backed or ordinary harness delegation where no native role format exists.
- **Verify**: `cargo test -p ito-templates agents && cargo test -p ito-cli --test init_agent_activation`
- **Done When**: Native role tests pass independently and installed skill directories still contain exactly seven Ito-managed entries for every harness.
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 3.2: Prune obsolete managed surfaces without losing user content
- **Files**: `ito-rs/crates/ito-templates/src/legacy.rs`; `ito-rs/crates/ito-templates/src/legacy_tests.rs`; `ito-rs/crates/ito-core/src/installers/`; repository-validation rule/remediation sources; `ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs`; update smoke/marker/naming tests
- **Dependencies**: None
- **Action**: Add the complete retired path manifest and run cleanup before retained assets are written. Remove managed-only files/directories and broken symlinks; preserve/report Markdown with user content outside managed markers; expose version-stamp and naming diagnostics through direct update/validation code; replace every `ito-update-repo` remediation reference; prove idempotence.
- **Verify**: `cargo test -p ito-templates legacy && cargo test -p ito-cli --test init_obsolete_cleanup && cargo test -p ito-cli --test update_marker_scoped && cargo test -p ito-core validate_repo`
- **Done When**: A full current-surface fixture upgrades to the canonical managed seven, preserved user extensions are reported, unrelated skills remain, direct diagnostics name no retired helper, and a second update is byte-identical.
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 3.3: Update specs/docs and run cross-harness completion audit
- **Files**: current Ito specs for routing, planning, agents, templates, memory, orchestration, init/update/hooks, asset naming/versioning, coordination remediation, tmux, archive, and spec sync; `.ito/wiki/topics/distribution-and-agents.md`; `docs/src/content/docs/`; `CHANGELOG.md`; harness fixture snapshots
- **Dependencies**: Task 3.1, Task 3.2
- **Action**: Promote every identity-matched delta into current specs, remove retired-name guidance, document the phase/direct-CLI/removal replacement map and breaking change, regenerate managed fixtures, and audit fresh plus upgraded installations across all harnesses.
- **Verify**: `make check && cargo test --workspace --all-features --exclude ito-web && ito validate 031-05_consolidate-seven-lifecycle-skills --strict`
- **Done When**: Specs and docs define one seven-skill contract, every fresh harness install has the exact inventory, upgraded fixtures are safe/idempotent, and default Ralph/loop tests pass.
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________
## Wave Guidelines
- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Keep exactly one task in progress at a time for this change
- User/project extensions are preserved but are not part of the Ito-managed default inventory
<!-- ITO:END -->

<!-- ITO:START -->
# Tasks for: 019-11_rename-orchestrator-assets

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Scope**: Rename only specialist roles `planner`, `researcher`, `reviewer`, and `worker` from `ito-orchestrator-*` to `ito-*`.
- **Exclusions**: Do not rename top-level `ito-orchestrator` or `ito-orchestrator-workflow`.

```bash
ito tasks status 019-11_rename-orchestrator-assets
ito tasks next 019-11_rename-orchestrator-assets
ito tasks start 019-11_rename-orchestrator-assets 1.1
ito tasks complete 019-11_rename-orchestrator-assets 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Locate scoped specialist role rename surfaces

- **Files**: `ito-rs/crates/ito-templates`, `ito-rs/crates/ito-cli/tests`, generated managed assets under repo root
- **Dependencies**: None
- **Action**: Inventory every source template, generated asset, preset reference, manifest entry, and test expectation that still uses `ito-orchestrator-{planner,researcher,reviewer,worker}`.
- **Verify**: `rg "ito-orchestrator-(planner|researcher|reviewer|worker)" ito-rs/crates/ito-templates ito-rs/crates/ito-cli/tests .agents .opencode .claude .github .pi`
- **Done When**: The scoped rename surface is identified without including top-level `ito-orchestrator` or `ito-orchestrator-workflow`.
- **Requirements**: `template-assets:orchestration-asset-names`
- **Updated At**: 2026-04-28
- **Status**: [x] complete

### Task 1.2: Rename source templates, manifests, and managed assets

- **Files**: `ito-rs/crates/ito-templates/assets`, `ito-rs/crates/ito-templates/src/lib.rs`, `.agents`, `.opencode`, `.claude`, `.github`, `.pi`
- **Dependencies**: Task 1.1
- **Action**: Rename the scoped specialist role files/directories to concise `ito-*` names and update embedded metadata, manifest lists, and preset references while preserving excluded orchestrator assets.
- **Verify**: `rg "ito-orchestrator-(planner|researcher|reviewer|worker)" ito-rs/crates/ito-templates .agents .opencode .claude .github .pi`
- **Done When**: No scoped source or managed asset emits the old specialist names, and excluded orchestrator assets remain unchanged.
- **Requirements**: `template-assets:orchestration-asset-names`
- **Updated At**: 2026-04-28
- **Status**: [x] complete

### Task 1.3: Update tests and expectations for concise specialist names

- **Files**: `ito-rs/crates/ito-templates/src/instructions_tests.rs`, `ito-rs/crates/ito-cli/tests/init_more.rs`, related expectations/tests
- **Dependencies**: Task 1.2
- **Action**: Adjust tests and expectation fixtures so installation and manifest assertions match the renamed specialist assets and references.
- **Verify**: `cargo test -p ito-cli init_installs_orchestration_agents_and_skills && cargo test -p ito-templates instructions_tests`
- **Done When**: Focused template/install tests pass against the renamed specialist assets.
- **Requirements**: `template-assets:orchestration-asset-names`
- **Updated At**: 2026-04-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Run formatting, checks, reviews, and demo capture

- **Files**: repository-wide; `.ito/changes/019-11_rename-orchestrator-assets/demos`
- **Dependencies**: None
- **Action**: Run formatting and verification, create Showboat demo evidence for the logical batch, and complete the required code-quality-squad review remediation.
- **Verify**: `cargo fmt --check && make check`
- **Done When**: Verification passes or pre-existing unrelated failures are documented, Showboat demo documents exist, and review feedback is addressed.
- **Requirements**: `template-assets:orchestration-asset-names`
- **Updated At**: 2026-04-28
- **Status**: [ ] pending
<!-- ITO:END -->

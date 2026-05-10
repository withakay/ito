<!-- ITO:START -->
# Tasks for: 019-13_fix-opencode-agent-mode

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-13_fix-opencode-agent-mode
ito tasks next 019-13_fix-opencode-agent-mode
ito tasks start 019-13_fix-opencode-agent-mode 1.1
ito tasks complete 019-13_fix-opencode-agent-mode 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Normalize stale OpenCode agent frontmatter on update

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: None
- **Action**: Strip stale `mode: subagent` and `subagent:` frontmatter from `.opencode/agents/*.md` files while preserving the existing model refresh path.
- **Verify**: `cargo test -p ito-core installers::tests::update_model_in_yaml_strips_stale_opencode_subagent_fields_when_requested -- --nocapture`
- **Done When**: OpenCode agent frontmatter is normalized on refresh without changing the existing body-preservation semantics.
- **Requirements**: `rust-installers:opencode-agent-frontmatter-normalization`
- **Updated At**: 2026-04-30
- **Status**: [x] complete

### Task 1.2: Add focused OpenCode install/update regressions

- **Files**: `ito-rs/crates/ito-cli/tests/init_more.rs`
- **Dependencies**: Task 1.1
- **Action**: Assert that fresh OpenCode agent installs and update flows do not leave `mode: subagent` on `ito-general` and `ito-orchestrator`.
- **Verify**: `cargo test -p ito-cli --test init_more init_with_tools_opencode_installs_orchestrator_agent_template -- --nocapture && cargo test -p ito-cli --test init_more init_update_refreshes_existing_opencode_orchestrator_agent_template -- --nocapture`
- **Done When**: The focused OpenCode tests fail if the stale subagent metadata survives install or update.
- **Requirements**: `rust-installers:opencode-agent-frontmatter-normalization`
- **Updated At**: 2026-04-30
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Validate the backfilled change package

- **Files**: `.ito/changes/019-13_fix-opencode-agent-mode/`
- **Dependencies**: None
- **Action**: Validate the backfilled proposal/spec/tasks package after the code fix is in place.
- **Verify**: `ito validate 019-13_fix-opencode-agent-mode --strict`
- **Done When**: The change package validates strictly and reflects the implemented OpenCode installer fix.
- **Requirements**: `rust-installers:opencode-agent-frontmatter-normalization`
- **Updated At**: 2026-04-30
- **Status**: [x] complete
<!-- ITO:END -->

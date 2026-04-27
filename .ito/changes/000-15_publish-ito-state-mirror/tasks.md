<!-- ITO:START -->
# Tasks for: 000-15_publish-ito-state-mirror

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 000-15_publish-ito-state-mirror
ito tasks next 000-15_publish-ito-state-mirror
ito tasks start 000-15_publish-ito-state-mirror 1.1
ito tasks complete 000-15_publish-ito-state-mirror 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add published mirror config surface

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, schema/config tests, `.ito/specs/ito-config-crate/spec.md`
- **Dependencies**: None
- **Action**: Add config/schema support for a published Ito mirror path with default `docs/ito` and project override behavior.
- **Verify**: `cargo test -p ito-config`
- **Done When**: The config model resolves the default mirror path and accepts a custom override.
- **Requirements**: ito-config-crate:published-mirror-path
- **Updated At**: 2026-04-27
- **Status**: [ ] pending

### Task 1.2: Implement mirror rendering contract

- **Files**: `ito-rs/crates/ito-core/src/**`, publication renderer/tests, mirror docs fixtures
- **Dependencies**: Task 1.1
- **Action**: Implement generation of a read-only published mirror that includes active changes, archived changes, and canonical specs.
- **Verify**: `cargo test -p ito-core published` or the equivalent focused test target
- **Done When**: Generated mirror content is emitted to the resolved path and includes the required Ito state without relying on symlinked `.ito` access.
- **Requirements**: published-ito-mirror:plain-checkout-visibility, published-ito-mirror:default-and-configurable-path
- **Updated At**: 2026-04-27
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add publication workflow and drift handling

- **Files**: `ito-rs/crates/ito-core/src/**`, possibly `ito-rs/crates/ito-cli/src/**`, audit/drift tests
- **Dependencies**: None
- **Action**: Introduce the workflow that publishes the generated mirror onto `main` and define behavior when users edit published output directly.
- **Verify**: `cargo test -p ito-core`, focused publication/drift tests, and any CLI tests for the chosen workflow surface
- **Done When**: Mirror publication updates committed main-facing content and direct mirror edits are treated as generated-output drift.
- **Requirements**: published-ito-mirror:generated-read-only-output, published-ito-mirror:main-publication-workflow
- **Updated At**: 2026-04-27
- **Status**: [ ] pending

### Task 2.2: Document and expose the mirror for plain consumers

- **Files**: project templates/docs/help text as needed, `docs/ito` examples or generated stubs, user guidance
- **Dependencies**: Task 2.1
- **Action**: Document the published mirror path, source-of-truth rules, and how plain GitHub readers or non-Ito agents should use the mirror.
- **Verify**: `ito validate 000-15_publish-ito-state-mirror --strict` and any docs/template checks that cover the new guidance
- **Done When**: The proposal implementation has clear docs explaining that coordination state is writable truth and the published mirror is committed read-only output.
- **Requirements**: published-ito-mirror:plain-checkout-visibility, published-ito-mirror:generated-read-only-output
- **Updated At**: 2026-04-27
- **Status**: [ ] pending
<!-- ITO:END -->

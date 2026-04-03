<!-- ITO:START -->
# Tasks for: 019-08_proposal-intake-and-schema-routing

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-08_proposal-intake-and-schema-routing
ito tasks next 019-08_proposal-intake-and-schema-routing
ito tasks start 019-08_proposal-intake-and-schema-routing 1.1
ito tasks complete 019-08_proposal-intake-and-schema-routing 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define intake and routing workflow assets

- **Files**: `ito-rs/crates/ito-templates/assets/skills/`, `ito-rs/crates/ito-templates/assets/commands/`, related installed harness outputs
- **Dependencies**: None
- **Action**: Add the new proposal-intake capability assets and create `ito-fix` / `ito-feature` command-skill entrypoints with clear role boundaries against `ito-proposal` and `ito-brainstorming`.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Embedded assets define the new intake lane and the intent-biased entrypoints consistently across installed harness outputs.
- **Requirements**: proposal-intake:clarify-change-before-scaffold, proposal-intake:produce-handoff-outcome, change-request-routing:intent-biased-entrypoints
- **Updated At**: 2026-04-01
- **Status**: [x] complete

### Task 1.2: Encode schema recommendation rules in guidance

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-proposal/SKILL.md`, new intake or routing skill assets, instruction templates related to schema choice
- **Dependencies**: Task 1.1
- **Action**: Update workflow guidance so schema choice is recommended by change shape, including bounded fixes, regression work, and supporting platform or infrastructure changes.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed guidance recommends `spec-driven`, `minimalist`, or `tdd` intentionally instead of only listing schemas or defaulting to `spec-driven`.
- **Requirements**: change-request-routing:bias-with-override, schema-selection-guidance:recommend-by-change-shape, schema-selection-guidance:cover-supporting-platform-work
- **Updated At**: 2026-04-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Integrate brownfield-aware intake behavior

- **Files**: new intake skill assets, related proposal guidance assets, any supporting instruction text
- **Dependencies**: None
- **Action**: Ensure the intake lane uses repo and spec evidence for brownfield confirmation questions and hands off a concise summary to downstream proposal creation.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The workflow clearly distinguishes repo-discoverable facts from user decisions and defines a non-duplicative handoff into proposal creation.
- **Requirements**: proposal-intake:ground-brownfield-questions, proposal-intake:produce-handoff-outcome
- **Updated At**: 2026-04-01
- **Status**: [x] complete

### Task 2.2: Add template and instruction tests for routing behavior

- **Files**: `ito-rs/crates/ito-templates/tests/`, `ito-rs/crates/ito-templates/src/instructions_tests.rs`, related asset tests
- **Dependencies**: Task 2.1
- **Action**: Add tests that cover installed command assets, skill availability, and the key schema recommendation and routing behaviors for fix, feature, and neutral proposal lanes.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Template and instruction tests fail if the new routing assets or decision guidance regress.
- **Requirements**: change-request-routing:intent-biased-entrypoints, change-request-routing:bias-with-override, schema-selection-guidance:recommend-by-change-shape
- **Updated At**: 2026-04-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update installed project guidance and docs references

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`, adapter/bootstrap guidance, related docs references
- **Dependencies**: None
- **Action**: Update the installed guidance so agents learn when to use `ito-fix`, `ito-feature`, `ito-proposal`, and `ito-brainstorming`.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Generated project guidance presents the new front-door workflow clearly and without conflicting instructions.
- **Requirements**: change-request-routing:intent-biased-entrypoints, schema-selection-guidance:cover-supporting-platform-work
- **Updated At**: 2026-04-01
- **Status**: [x] complete

### Task 3.2: Validate the change strictly

- **Files**: `.ito/changes/019-08_proposal-intake-and-schema-routing/`
- **Dependencies**: Task 3.1
- **Action**: Run strict Ito validation and resolve any artifact or traceability issues introduced by the new proposal package.
- **Verify**: `ito validate 019-08_proposal-intake-and-schema-routing --strict`
- **Done When**: The proposal package validates cleanly with no strict-mode errors.
- **Requirements**: proposal-intake:clarify-change-before-scaffold, change-request-routing:intent-biased-entrypoints, schema-selection-guidance:recommend-by-change-shape
- **Updated At**: 2026-04-01
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->

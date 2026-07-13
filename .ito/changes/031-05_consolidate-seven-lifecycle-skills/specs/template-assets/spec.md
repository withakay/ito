<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Orchestration Asset Names
Ito template assets SHALL install retained orchestration role definitions only in harness-native agent surfaces, using concise `ito-*` names. The templates bundle MUST NOT install `ito-planner`, `ito-researcher`, `ito-reviewer`, `ito-worker`, `ito-orchestrator`, or `ito-orchestrator-workflow` as skill directories.

#### Scenario: Native specialist agents use concise names
- **GIVEN** a harness provides a native delegated-agent surface
- **WHEN** Ito emits retained planner, researcher, reviewer, worker, or test-runner roles
- **THEN** those definitions use concise `ito-*` agent names in the native surface
- **AND** no corresponding `SKILL.md` directory is emitted

#### Scenario: Orchestration remains lifecycle-accessible
- **WHEN** a user requests iterative or multi-change orchestration
- **THEN** `ito-loop` is the installed lifecycle skill entrypoint
- **AND** `ito agent instruction orchestrate` remains the authoritative detailed policy
<!-- ITO:END -->

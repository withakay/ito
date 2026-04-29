<!-- ITO:START -->
## ADDED Requirements

### Requirement: Engine loads ItoConfig and filters rules by active gates

The system SHALL provide a `ito-core::validate_repo` engine that loads `ItoConfig` once per invocation, exposes a `Rule` trait with an `is_active(&RuleContext)` predicate, and runs only the rules whose gates are satisfied by the resolved configuration.

- **Requirement ID**: validate-repo-engine:gate-filtering

#### Scenario: Inactive rule is skipped without I/O

- **GIVEN** a built-in rule whose `is_active` predicate inspects an `ItoConfig` field
- **AND** that field's value disables the gate
- **WHEN** the engine runs against the configuration
- **THEN** the rule SHALL be reported as skipped
- **AND** its `check` function SHALL NOT be invoked

#### Scenario: Active rule is executed and contributes issues

- **GIVEN** a built-in rule whose `is_active` predicate evaluates to `true`
- **WHEN** the engine runs against the configuration
- **THEN** the rule's `check` function SHALL be invoked exactly once
- **AND** any returned issues SHALL appear in the report with the rule's `rule_id` and severity

### Requirement: Engine reuses ValidationIssue and ValidationReport envelope

The system SHALL emit issues using the existing `ValidationIssue` shape from `ito-core::validate` so that JSON consumers can parse `ito validate repo` output with the same code that parses `ito validate <item>`.

- **Requirement ID**: validate-repo-engine:reuse-envelope

#### Scenario: Engine emits ValidationIssue with rule_id

- **WHEN** the engine emits an issue from a rule
- **THEN** the resulting `ValidationIssue.rule_id` SHALL equal the rule's `RuleId`
- **AND** the `ValidationIssue.level` SHALL match one of the existing `LEVEL_ERROR`, `LEVEL_WARNING`, or `LEVEL_INFO` constants

#### Scenario: Engine emits ValidationReport with summary

- **WHEN** the engine completes a run
- **THEN** it SHALL return a `ValidationReport` whose `valid` flag is `false` whenever at least one issue has level `ERROR`
- **AND** whose `valid` flag is `false` whenever at least one issue has level `WARNING` and the run was strict

### Requirement: Engine attaches config gate metadata to every issue

When a rule emits an issue, the engine SHALL annotate that issue with structured metadata describing the configuration gate that activated the rule, so that humans and agents can answer "why does this rule apply to my repo?".

- **Requirement ID**: validate-repo-engine:gate-metadata

#### Scenario: Issue metadata names the activating config key

- **GIVEN** a rule active because `changes.coordination_branch.storage = "worktree"`
- **WHEN** the rule emits an issue
- **THEN** the issue's `metadata` SHALL include the config key path and resolved value that activated the rule

### Requirement: Engine accepts optional staged-files snapshot

The engine SHALL accept an optional `StagedFiles` snapshot in its `RuleContext`. Rules whose `id` is namespaced under `*-staged-*` or whose documentation declares them staged-only SHALL only become active when a staged-files snapshot is supplied.

- **Requirement ID**: validate-repo-engine:staged-context

#### Scenario: Staged-only rule is inactive without snapshot

- **GIVEN** the engine is run without a `StagedFiles` snapshot
- **WHEN** the registry is filtered
- **THEN** any rule documented as staged-only SHALL be skipped

#### Scenario: Staged-only rule is active when snapshot is provided

- **GIVEN** the engine is run with a `StagedFiles` snapshot
- **AND** the rule's other gate predicates are satisfied
- **WHEN** the registry is filtered
- **THEN** the rule SHALL be active

### Requirement: Engine exposes list_active_rules introspection

The system SHALL expose a `list_active_rules(config)` function that returns each built-in rule together with its severity, whether it is active for the supplied `ItoConfig`, and the resolved value of its primary config gate, without performing any check work.

- **Requirement ID**: validate-repo-engine:list-active-rules

#### Scenario: Introspection lists active and skipped rules

- **GIVEN** a configuration that activates a subset of rules
- **WHEN** `list_active_rules` is called
- **THEN** the result SHALL include every built-in rule
- **AND** each entry SHALL declare its `active` flag, `rule_id`, `severity`, and gate value
<!-- ITO:END -->

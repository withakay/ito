# Spec: peer-review-instruction

## Purpose

Defines the agent instruction for peer-reviewing change proposals before implementation begins. This instruction provides structured guidance for an agent to evaluate proposal quality, spec completeness, design soundness, and task coverage, producing a clear verdict.

## ADDED Requirements

### Requirement: Review instruction command

The system SHALL expose a `review` instruction type via `ito agent instruction review --change <id>` that produces structured peer-review guidance for a change proposal.

#### Scenario: Generate review instruction for a complete change

- **WHEN** an agent runs `ito agent instruction review --change 001-18_agent-instruction-peer-review` and the change has proposal.md, specs/, design.md, and tasks.md
- **THEN** the instruction output SHALL contain a review protocol with sections for each artifact present

#### Scenario: Generate review instruction for a partial change

- **WHEN** an agent runs `ito agent instruction review --change <id>` and the change only has proposal.md
- **THEN** the instruction output SHALL indicate which artifacts are present and which are missing, and limit review guidance to the artifacts that exist

#### Scenario: Review instruction for non-existent change

- **WHEN** an agent runs `ito agent instruction review --change nonexistent`
- **THEN** the system SHALL return an error indicating the change does not exist

### Requirement: Review context gathering

The system SHALL gather and present the following context in the review instruction: change name, schema name, change directory path, list of artifacts present with their file paths, structural validation results from `ito validate`, and the list of existing main specs affected by the change's spec deltas.

#### Scenario: Context includes validation results

- **WHEN** the review instruction is generated for a change that has validation warnings
- **THEN** the review context SHALL include the count and details of validation issues, distinguishing errors from warnings

#### Scenario: Context identifies affected specs

- **WHEN** the change has MODIFIED spec deltas referencing existing capabilities
- **THEN** the review context SHALL list the affected main spec paths so the reviewer can compare against them

### Requirement: Proposal review checklist

The review template SHALL include a proposal review checklist covering: clarity and justification of the "Why" section, appropriateness of scope, correct categorization of new vs modified capabilities, explicit identification of breaking changes, and accurate impact assessment.

#### Scenario: Proposal checklist rendered

- **WHEN** the review instruction is generated for a change with proposal.md
- **THEN** the output SHALL contain a "Proposal Review" section with at least 5 actionable checklist items

### Requirement: Spec review checklist

The review template SHALL include a spec review checklist covering: well-formed requirements using SHALL/MUST normative language, at least one scenario per requirement, testable and specific scenarios with WHEN/THEN format, full content in MODIFIED requirements, edge cases and error scenarios, and consistency with existing specs.

#### Scenario: Spec checklist rendered

- **WHEN** the review instruction is generated for a change with specs/
- **THEN** the output SHALL contain a "Spec Review" section with at least 6 actionable checklist items

#### Scenario: Spec checklist skipped when no specs

- **WHEN** the review instruction is generated for a change without specs/
- **THEN** the output SHALL NOT contain a "Spec Review" section

### Requirement: Design review checklist

The review template SHALL include a design review checklist covering: key decisions justified with rationale, alternatives considered, risks identified with mitigations, migration plan adequacy, and consistency with existing architecture.

#### Scenario: Design checklist rendered

- **WHEN** the review instruction is generated for a change with design.md
- **THEN** the output SHALL contain a "Design Review" section with at least 4 actionable checklist items

#### Scenario: Design checklist skipped when no design

- **WHEN** the review instruction is generated for a change without design.md
- **THEN** the output SHALL NOT contain a "Design Review" section

### Requirement: Task review checklist

The review template SHALL include a task review checklist covering: appropriate scoping and ordering, valid dependencies, verifiability, coverage of all spec requirements, and alignment with design decisions.

#### Scenario: Task checklist rendered

- **WHEN** the review instruction is generated for a change with tasks.md
- **THEN** the output SHALL contain a "Task Review" section with at least 4 actionable checklist items

### Requirement: Review output format

The review template SHALL instruct the reviewing agent to produce a structured review report with: a summary, findings organized by severity (blocking / suggestion / note), and a clear verdict of `approve`, `request-changes`, or `needs-discussion`.

#### Scenario: Output format specified

- **WHEN** the review instruction is generated
- **THEN** the instruction SHALL include an "Output Format" section specifying the expected structure of the review report including verdict options

### Requirement: Cross-cutting review concerns

The review template SHALL prompt the reviewer to consider: conflicts with other active changes, impact on the broader system, testing strategy adequacy, and whether the change could be decomposed further.

#### Scenario: Cross-cutting section rendered

- **WHEN** the review instruction is generated
- **THEN** the output SHALL contain a "Cross-Cutting Concerns" section with prompts about conflicts, system impact, testing strategy, and decomposition

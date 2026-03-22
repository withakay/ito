## ADDED Requirements

### Requirement: Enhanced tasks can declare covered requirement references

In enhanced encoding, the tasks tracking format SHALL allow a task block to include a metadata line of the form `- **Requirements**: <id>[, <id> ...]` to declare which requirement references the task covers.

#### Scenario: Enhanced task exposes covered requirements

- **GIVEN** an enhanced task block contains `- **Requirements**: delta-specs:normative-language, cli-validate:strict-coverage`
- **WHEN** the tasks tracking file is parsed
- **THEN** Ito preserves both requirement references as structured metadata on that task

#### Scenario: Empty or duplicate requirement references are invalid

- **GIVEN** an enhanced task block declares an empty, whitespace-only, or duplicate requirement reference in its `Requirements` metadata
- **WHEN** the tasks tracking file is parsed and validated for traceability
- **THEN** Ito reports the metadata as invalid instead of silently normalizing it

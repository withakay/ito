<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph supports multiple task sources

Ralph SHALL support task execution from Ito change context, markdown task files, YAML task files, and GitHub issue queues.

- **Requirement ID**: ralph-task-sources:multiple-task-sources

#### Scenario: Markdown task source drives Ralph

- **WHEN** the user points Ralph at a markdown task file
- **THEN** Ralph SHALL load pending tasks from that file and execute against them

#### Scenario: YAML task source drives Ralph

- **WHEN** the user points Ralph at a YAML task file
- **THEN** Ralph SHALL load pending tasks and any declared parallel grouping metadata from that file

#### Scenario: GitHub issue source drives Ralph

- **WHEN** the user configures a GitHub repository task source
- **THEN** Ralph SHALL load open issues as executable work items

### Requirement: Ralph can sync external task state

When Ralph operates against external task sources, it SHALL update the source of truth as work completes.

- **Requirement ID**: ralph-task-sources:sync-external-state

#### Scenario: GitHub-backed run closes or syncs issues

- **WHEN** a GitHub-backed Ralph run completes a task successfully
- **THEN** Ralph SHALL update the corresponding issue state or body as configured
<!-- ITO:END -->

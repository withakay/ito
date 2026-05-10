<!-- ITO:START -->
## ADDED Requirements

### Requirement: max-parallel flag and named aliases

The system SHALL accept a `--max-parallel` flag on the orchestrator invocation accepting either a positive integer or one of the following named aliases: `serial`, `sync`, `synchronous` (resolve to 1); `parallel`, `fan-out`, `swarm`, `distributed` (resolve to the configured cap). The default mode SHALL be `auto` with a cap of 4.

- **Requirement ID**: orchestrate-parallelism:max-parallel-flag

#### Scenario: Numeric value sets concurrency limit

- **WHEN** `--max-parallel 2` is passed
- **THEN** at most 2 change pipelines execute concurrently

#### Scenario: serial alias enforces sequential execution

- **WHEN** `--max-parallel serial` is passed
- **THEN** changes are processed one at a time in dependency order

#### Scenario: parallel alias enables full fan-out up to cap

- **WHEN** `--max-parallel parallel` is passed
- **THEN** the orchestrator dispatches up to the configured cap of concurrent workers (default 4)

#### Scenario: auto mode applies default cap

- **WHEN** no `--max-parallel` flag is provided
- **THEN** the orchestrator uses `auto` mode with a cap of 4 concurrent change pipelines

### Requirement: Dependency graph enforcement

The system SHALL respect `depends_on` declarations in `.ito/changes/<id>/.ito.yaml` when building the execution plan. A change SHALL NOT be dispatched until all changes it depends on have passed all gates.

- **Requirement ID**: orchestrate-parallelism:dependency-graph

#### Scenario: Dependent change waits for dependency

- **WHEN** change B declares `depends_on: [change-a]`
- **THEN** the orchestrator does not dispatch change B until change A's pipeline has passed all gates

#### Scenario: Independent changes run concurrently

- **WHEN** two changes share no dependency relationship
- **THEN** the orchestrator dispatches them concurrently up to the `max-parallel` limit

#### Scenario: Circular dependency is rejected

- **WHEN** the dependency graph contains a cycle
- **THEN** the orchestrator emits a clear error identifying the cycle and exits without beginning the run
<!-- ITO:END -->

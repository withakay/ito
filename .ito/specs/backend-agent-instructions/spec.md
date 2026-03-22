## ADDED Requirements

### Requirement: Backend-aware agent instructions are CLI-first for active work

When remote/API-backed persistence mode is active, backend-aware agent instructions SHALL direct agents to use repository-backed CLI workflows for active-work mutations instead of editing markdown artifacts directly.

#### Scenario: Active-work markdown is absent in remote mode

- **GIVEN** remote/API-backed persistence mode is active
- **AND** active-work markdown is not materialized locally
- **WHEN** an agent receives backend-aware instructions
- **THEN** the instructions describe that absence as expected
- **AND** direct the agent to the appropriate CLI/repository-backed workflow instead of local markdown editing

#### Scenario: Git projections are described as scan/backup surfaces

- **GIVEN** promoted specs or archived changes exist as Git projections
- **WHEN** an agent receives backend-aware instructions
- **THEN** the instructions explain when those projections are useful for scanning
- **AND** distinguish them from mutation paths that must go through CLI/repository-backed interfaces

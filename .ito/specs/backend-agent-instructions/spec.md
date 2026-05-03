## MODIFIED Requirements

### Requirement: Backend-aware agent instructions are CLI-first for active work

When remote/API-backed persistence mode is active, backend-aware agent instructions SHALL direct agents to use repository-backed CLI workflows for active-work mutations instead of editing markdown artifacts directly.

When those instructions describe updating active-work change/spec artifacts, they SHALL point agents at Ito artifact mutation commands instead of manual markdown edits.

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

#### Scenario: Backend-aware instructions name the artifact mutation commands

- **GIVEN** remote/API-backed persistence mode is active
- **WHEN** an agent needs to update a proposal, tasks artifact, change design, or spec artifact
- **THEN** the backend-aware instructions direct the agent to `ito patch` and/or `ito write`
- **AND** they do not recommend direct markdown editing for those active-work artifacts

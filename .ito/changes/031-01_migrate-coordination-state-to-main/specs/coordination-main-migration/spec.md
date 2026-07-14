<!-- ITO:START -->
## ADDED Requirements

### Requirement: Legacy coordination state is detected centrally
Ito SHALL classify a repository as using legacy coordination storage when resolved worktree coordination configuration, managed `.ito/{changes,specs,modules,workflows,audit}` symlinks, or the managed coordination `.gitignore` marker proves that coordinated state remains active or partially active. The detector MUST report the evidence it found and MUST distinguish absent, embedded, legacy, and ambiguous states.
- **Requirement ID**: coordination-main-migration:legacy-state-detection
- **Tags**: behavior, stateful

#### Rules / Invariants
- A broken symlink MUST still be inspected through symlink metadata.
- A mixture of legacy authority links with materialized authoritative directories, or of runtime links with non-empty runtime directories, MUST NOT be treated as safely migrated. The explicit experimental layout with real tracked `changes`/`specs` and coordinated `modules`/`workflows`/`audit` is not ambiguous when its targets and marker are consistent.
- Detection MUST NOT mutate repository or coordination state.

#### Scenario: Configured worktree storage is detected
- **GIVEN** resolved configuration enables coordination worktree storage
- **WHEN** Ito evaluates the repository storage state
- **THEN** the detector reports legacy coordination state and the configuration evidence

#### Scenario: Partial legacy wiring is detected
- **GIVEN** configuration says embedded storage
- **BUT** at least one managed coordination symlink or legacy gitignore marker remains
- **WHEN** Ito evaluates the repository storage state
- **THEN** the detector reports legacy or ambiguous state rather than declaring migration complete

#### Scenario: Real tracked directories are accepted
- **GIVEN** coordination storage is disabled or embedded
- **AND** every managed Ito state path is a real directory with no legacy marker
- **WHEN** Ito evaluates the repository storage state
- **THEN** the detector reports main-compatible embedded state

### Requirement: Legacy state applies a read-write safety policy
Ito SHALL apply one shared command-intent policy when legacy coordination state is detected. Read-only operations SHALL remain available with an actionable warning, while mutating operations MUST fail before changing files, task state, Git state, or remote state.
- **Requirement ID**: coordination-main-migration:read-write-safety-policy
- **Tags**: behavior, stateful

#### Scenario: Read operation warns and continues
- **GIVEN** legacy coordination state is detected
- **WHEN** a user runs a read-only command such as list, show, status, validate, or agent instruction rendering
- **THEN** Ito prints a warning naming `ito agent instruction migrate-to-main`
- **AND** completes the read without modifying state

#### Scenario: Mutation is blocked before execution
- **GIVEN** legacy coordination state is detected
- **WHEN** a user invokes a command that creates, replaces, patches, archives, starts, completes, or otherwise mutates Ito state
- **THEN** Ito returns a typed blocking diagnostic before the mutation
- **AND** identifies `ito agent instruction migrate-to-main` as the remediation

### Requirement: Migration prompt preserves and proves state
The `migrate-to-main` agent instruction SHALL define a reversible migration that snapshots and hashes source and destination state, materializes real tracked Ito directories, disables coordination storage, validates exact content parity, and prepares reviewable main integration without deleting or rewriting the legacy external store.
- **Requirement ID**: coordination-main-migration:preserve-and-prove-state
- **Tags**: behavior, stateful

#### Rules / Invariants
- The source coordination worktree and branch MUST remain untouched after the migration preparation.
- Existing non-empty destination directories MUST be compared and reconciled explicitly; they MUST NOT be overwritten silently.
- The prompt MUST stop before integration when hashes, file inventories, or validation do not agree.
- Integration MUST be proposed through the configured review workflow.

#### State Transitions
| From | Event | To | Notes |
| --- | --- | --- | --- |
| legacy | inspect | legacy | Read-only evidence collection |
| legacy | prepare migration | prepared | Real directories and config changes exist on a migration branch |
| prepared | validate parity | reviewable | Hashes, inventory, and Ito validation agree |
| reviewable | merge reviewed proposal | main-authoritative | Legacy external state remains available for rollback |

#### Scenario: Clean migration preparation
- **GIVEN** a healthy legacy coordination worktree and no conflicting destination content
- **WHEN** an agent follows the emitted migration instruction
- **THEN** it records source inventory and hashes
- **AND** replaces managed symlinks with real directories containing byte-equivalent state
- **AND** sets coordination storage to embedded and disabled
- **AND** prepares a reviewed main integration change

#### Scenario: Conflicting destination content stops migration
- **GIVEN** a real destination directory contains content not present in the coordination source
- **WHEN** an agent compares migration state
- **THEN** the instruction requires the agent to stop and report the conflict
- **AND** neither source nor destination content is deleted
<!-- ITO:END -->

<!-- ITO:START -->

## Requirements

### Requirement: Default Ito skill inventory contains exactly seven lifecycle skills
Ito SHALL install exactly these Ito-managed skills by default: `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`. Every supported harness MUST derive its installed skill set from one canonical inventory.

#### Rules / Invariants
- Harness-native agent definitions are not skills and MUST NOT be installed under a skill discovery directory.
- Project/user skills not owned by Ito are outside the default inventory and MUST be preserved.
- Ralph/iteration remains available through `ito-loop` in the standard installation.

#### Scenario: Fresh install has exact inventory
- **WHEN** `ito init` installs managed skills for any supported harness
- **THEN** the Ito-managed skill names are exactly `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`
- **AND** no other `ito-*` skill directory is emitted by default

#### Scenario: Every harness uses the same inventory
- **WHEN** manifests are generated for OpenCode, Claude, Codex, Pi, and GitHub Copilot
- **THEN** each manifest resolves the same seven Ito-managed skill names
- **AND** harness path conventions do not change the logical inventory

### Requirement: Helper guidance is owned by lifecycle phases
Guidance formerly exposed through standalone helper skills SHALL be folded into the retained lifecycle skill that owns the relevant phase or into an authoritative CLI-emitted instruction referenced by that skill.

#### Scenario: Proposal helpers are consolidated
- **WHEN** an agent needs intake, feature/fix framing, brainstorming, planning, or task scaffolding
- **THEN** `ito-proposal` supplies or routes to that guidance
- **AND** no standalone helper skill is required

#### Scenario: Implementation and verification helpers are consolidated
- **WHEN** an agent needs worktree setup, task tracking, sub-agent implementation, commits, tests, completion verification, or finish guidance
- **THEN** `ito-apply` or `ito-review` owns the phase and references authoritative CLI instructions where needed

#### Scenario: Knowledge and orchestration helpers are consolidated
- **WHEN** an agent needs research/wiki lookup, memory operations, archive follow-through, or iterative/orchestrated execution
- **THEN** `ito-research`, `ito-archive`, or `ito-loop` owns the phase
- **AND** no separate Ito-managed skill expands the default inventory

### Requirement: Retired managed skills are migration-safe
Ito update/upgrade SHALL remove obsolete Ito-owned skill and command assets only when managed ownership is proven. User-authored content and non-Ito skills MUST be preserved and reported when they prevent safe pruning.

#### Scenario: Unmodified obsolete managed skill is pruned
- **GIVEN** an installed retired skill contains only Ito-managed content
- **WHEN** `ito update` or `ito init --upgrade` runs
- **THEN** the obsolete managed skill and empty directories are removed

#### Scenario: User content prevents destructive pruning
- **GIVEN** a retired Ito skill contains content outside its managed block
- **WHEN** managed cleanup runs
- **THEN** Ito preserves the user content
- **AND** reports the obsolete path and manual migration guidance
<!-- ITO:END -->

<!-- ITO:START -->
## ADDED Requirements

### Requirement: ByteRover hub skills are discoverable by every supported harness

The repository SHALL host the four ByteRover hub skills (`byterover-review`,
`byterover-plan`, `byterover-explore`, `byterover-audit`) in locations that
every supported agent harness can discover. Concretely, `brv hub install`
writes skills into a target agent's connector directory (e.g. `.claude/skills/`
for Claude Code), so the repository SHALL additionally mirror the installed
skills into the shared `.agents/skills/byterover/` directory used by all
non-Claude agent harnesses.

- **Requirement ID**: `agent-memory-byterover:hub-skills-location`

#### Scenario: Required hub skills present under `.agents/skills/byterover/`

- **WHEN** a contributor inspects `.agents/skills/byterover/` after onboarding
- **THEN** the directory contains at least four sub-directories: `byterover-review`, `byterover-plan`, `byterover-explore`, `byterover-audit`
- **AND** each sub-directory contains a `SKILL.md` file

#### Scenario: Required hub skills present under `.claude/skills/`

- **WHEN** a contributor inspects `.claude/skills/` after onboarding
- **THEN** the directory contains sub-directories `byterover-review`, `byterover-plan`, `byterover-explore`, and `byterover-audit`
- **AND** each sub-directory contains a `SKILL.md` file

#### Scenario: Fresh install plus mirror reproduces both locations

- **WHEN** a contributor runs `brv hub install byterover-review --agent "Claude Code"` (and the equivalent for `byterover-plan`, `byterover-explore`, `byterover-audit`) from the project root, then mirrors each installed skill directory from `.claude/skills/<skill-id>/` to `.agents/skills/byterover/<skill-id>/`
- **THEN** the four hub skills exist under both `.claude/skills/` and `.agents/skills/byterover/`
- **AND** `SKILL.md` content under each location matches byte-for-byte immediately after the mirror step

### Requirement: Claude Code connector is the only connector this change installs

The repository SHALL install the ByteRover agent-skill connector for Claude
Code because Claude Code does not consume `.agents/skills/`. No other agent
connectors SHALL be installed by this change.

- **Requirement ID**: `agent-memory-byterover:claude-connector-only`

#### Scenario: Claude Code connector installed

- **WHEN** a contributor runs `brv connectors install "Claude Code" --type skill` from the project root
- **THEN** the connector files are created under `.claude/` (per Claude Code connector conventions)
- **AND** `brv connectors` lists `Claude Code` with connector type `Agent Skill`

#### Scenario: Other connectors not installed

- **WHEN** a contributor inspects `brv connectors` after this change is applied
- **THEN** OpenCode, Github Copilot, Cursor, Gemini CLI, and other non-Claude agent connectors are not listed (unless installed separately and outside this change)

### Requirement: Local-only operation — no cloud sync

The ByteRover integration SHALL NOT require cloud login, push, pull, or
`brv vc …` commands in any workflow, script, skill, or instruction template
produced by this change. Repo-authored instructions MAY mention those commands
only to say they are intentionally out of scope for this local-only bootstrap.

- **Requirement ID**: `agent-memory-byterover:local-only`

#### Scenario: Repo-authored instructions keep cloud sync out of scope

- **WHEN** a contributor follows the proposal, tasks, and README authored for this change
- **THEN** no required step asks the contributor to run `brv login`, `brv push`, `brv pull`, or `brv vc …`
- **AND** any mention of those commands states they are intentionally out of scope for this change

#### Scenario: `brv status` reports Not connected on a clean machine

- **WHEN** a contributor completes the integration on a fresh machine without running `brv login`
- **THEN** `brv status` reports `Account: Not connected`
- **AND** curation and local search still succeed

### Requirement: Bootstrap curation pass covers the canonical Ito knowledge

As part of applying this change, the contributor SHALL curate the following
paths (relative to the project root) into the local ByteRover context tree.
Directory paths SHALL use `brv curate --folder <path>`. Standalone files SHALL
use file-mode `brv curate` with `-f <path>`. These paths represent Ito's
canonical, agent-facing knowledge today.

- **Requirement ID**: `agent-memory-byterover:bootstrap-curation`

#### Scenario: Required paths curated

- **WHEN** the contributor inspects `brv curate view --status completed` after running the bootstrap pass
- **THEN** the completed-curation history lists folder-pack entries for each of: `.ito/specs/`, `docs/`, `.ito/changes/archive/`, `.ito/modules/`
- **AND** file-mode curate entries exist for `AGENTS.md`, `.ito/AGENTS.md`, and `.ito/architecture.md`

#### Scenario: Broken symlinks do not abort the pass

- **WHEN** a curated path (e.g. `.ito/modules/`) is a symlink that resolves successfully
- **THEN** `brv curate --folder <path>` follows the symlink and completes
- **AND** a symlink that resolves to a missing target is skipped with a warning rather than crashing the whole pass

### Requirement: README documents how to reproduce the integration

A `.agents/skills/byterover/README.md` file SHALL exist that lists the
installed ByteRover hub skills, describes what they are for, and documents
the exact commands required to reproduce the integration on a new machine.

- **Requirement ID**: `agent-memory-byterover:readme-reproducibility`

#### Scenario: README lists installed skills and commands

- **WHEN** a contributor opens `.agents/skills/byterover/README.md`
- **THEN** the file lists each installed skill (`byterover-review`, `byterover-plan`, `byterover-explore`, `byterover-audit`) with a one-line purpose
- **AND** the file lists the exact `brv hub install …` and `brv connectors install …` commands used
- **AND** the file lists the bootstrap `brv curate --folder …` commands used for directories and the file-mode `brv curate … -f …` commands used for the top-level docs
- **AND** the file states that cloud sync (`brv login`, `brv push`, `brv pull`, `brv vc …`) is intentionally not used
<!-- ITO:END -->

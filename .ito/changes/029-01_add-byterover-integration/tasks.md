<!-- ITO:START -->
# Tasks for: 029-01_add-byterover-integration

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 029-01_add-byterover-integration
ito tasks next 029-01_add-byterover-integration
ito tasks start 029-01_add-byterover-integration 1.1
ito tasks complete 029-01_add-byterover-integration 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Install the Claude Code connector

- **Files**: `.claude/` (connector-generated files)
- **Dependencies**: None
- **Action**: Run `brv connectors install "Claude Code" --type skill` from the project root. Review the files the connector writes under `.claude/`, stage and commit them.
- **Verify**: `brv connectors` lists `Claude Code` with type `Agent Skill`. The `.claude/` tree contains the new connector files.
- **Done When**: Claude Code connector files are committed; `brv connectors` shows `Claude Code`.
- **Requirements**: `agent-memory-byterover:claude-connector-only`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.2: Install the ByteRover hub skills into `.agents/skills/byterover/`

- **Files**: `.agents/skills/byterover/byterover-{review,plan,explore,audit}/**`
- **Dependencies**: None
- **Action**: Run `brv hub install byterover-review`, `brv hub install byterover-plan`, `brv hub install byterover-explore`, and `brv hub install byterover-audit` from the project root so the installed skill folders land under `.agents/skills/byterover/`.
- **Verify**: `.agents/skills/byterover/byterover-<id>/SKILL.md` exists for each of the four skills.
- **Done When**: All four skills present under `.agents/skills/byterover/`; each has a `SKILL.md`.
- **Requirements**: `agent-memory-byterover:hub-skills-location`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.3: Author `.agents/skills/byterover/README.md`

- **Files**: `.agents/skills/byterover/README.md`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: Write a short README that: (a) lists the installed ByteRover hub skills with a one-line purpose each, (b) records the exact `brv hub install` and `brv connectors install` commands used, (c) records the exact `brv curate --folder …` and file-mode `brv curate … -f …` commands used for the bootstrap pass (Wave 2), (d) states that cloud sync (`brv login`, `brv push`, `brv pull`, `brv vc …`) is intentionally not used.
- **Verify**: The README lists all installed skills and includes an explicit note that `brv login`, `brv push`, `brv pull`, and `brv vc …` are out of scope for this change.
- **Done When**: README committed and the reproducibility / local-only notes are present.
- **Requirements**: `agent-memory-byterover:readme-reproducibility`, `agent-memory-byterover:local-only`
- **Updated At**: 2026-04-24
- **Status**: [>] in-progress

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Curate folder packs into the local ByteRover context tree

- **Files**: _(no repo files changed; local ByteRover cache only)_
- **Dependencies**: None
- **Action**: From the project root, run folder-pack curations:
  - `brv curate --folder .ito/specs/`
  - `brv curate --folder docs/`
  - `brv curate --folder .ito/changes/archive/`
  - `brv curate --folder .ito/modules/`
- **Verify**: `brv curate view --status completed` lists entries for each folder. `brv search <topic>` on a known spec name returns results.
- **Done When**: All four folder-pack curations completed successfully.
- **Requirements**: `agent-memory-byterover:bootstrap-curation`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: Curate the agent-facing root docs

- **Files**: _(no repo files changed; local ByteRover cache only)_
- **Dependencies**: None
- **Action**: Run file-mode `brv curate` for `AGENTS.md`, `.ito/AGENTS.md`, and `.ito/architecture.md` with a short descriptive prompt (at most 5 files per invocation, e.g. `brv curate "Bootstrap authoritative root docs for Ito" -f AGENTS.md -f .ito/AGENTS.md -f .ito/architecture.md`).
- **Verify**: `brv curate view --status completed` lists the three files.
- **Done When**: All three files ingested into the context tree.
- **Requirements**: `agent-memory-byterover:bootstrap-curation`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.3: Validate the local-only workflow instructions

- **Files**: `proposal.md`, `tasks.md`, `README.md`, `.agents/skills/byterover/README.md`
- **Dependencies**: Task 2.1, Task 2.2
- **Action**: Review the repo-authored instructions for this change (`proposal.md`, `tasks.md`, the change README, and `.agents/skills/byterover/README.md`) and confirm they do not require `brv login`, `brv push`, `brv pull`, or `brv vc …` as part of the bootstrap workflow. Mentions are allowed only when marking those commands as intentionally out of scope.
- **Verify**: The reviewed instructions contain no required cloud-sync or `brv vc …` step, and `brv status` on a fresh-clone reviewer machine still reports `Account: Not connected` after running Waves 1-2.
- **Done When**: Instruction review is clean and the local-only check passes.
- **Requirements**: `agent-memory-byterover:local-only`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Run `ito validate --strict`

- **Files**: _(no changes)_
- **Dependencies**: None
- **Action**: Run `ito validate 029-01_add-byterover-integration --strict` and address any validation errors.
- **Verify**: Command exits 0.
- **Done When**: Strict validation passes.
- **Requirements**: `agent-memory-byterover:bootstrap-curation`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: Update change README with implementation notes

- **Files**: `.ito/changes/029-01_add-byterover-integration/README.md`
- **Dependencies**: Task 3.1
- **Action**: Append a short section that summarizes what was installed, how a reviewer can reproduce the bootstrap pass, and where to find the detailed reproducibility notes (for example, `.agents/skills/byterover/README.md`).
- **Verify**: README mentions the installed skills, summarizes the bootstrap pass, and points to the reproducibility doc.
- **Done When**: README updated and committed with implementation notes.
- **Requirements**: `agent-memory-byterover:readme-reproducibility`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->

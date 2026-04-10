# Tasks: 027-01_add-ito-wiki

## Wave 1: Wiki Scaffold and Boundaries
- **Depends On**: none

### Task 1.1: Add `.ito/wiki/` scaffold to project templates
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Add the initial wiki root to the default project template, including `index.md`, `log.md`, `overview.md`, `_meta/config.yaml`, `_meta/schema.md`, and `_meta/status.md`. Keep the scaffold Obsidian-friendly and clearly Ito-scoped.
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/index.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/log.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/overview.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/config.yaml`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/schema.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/status.md`
- **Verify**: `cargo test -p ito-templates`
- **Done When**: New Ito projects receive a valid wiki scaffold and existing files are preserved on upgrade.

### Task 1.2: Document wiki source boundaries and page model
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Write the wiki schema and configuration so the source boundary is `.ito`-first: changes, specs, research, modules, project guidance, and architecture. Allow explicit references to files outside `.ito/` without treating the rest of the repo as default wiki input.
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/schema.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/config.yaml`
- **Dependencies**: Task 1.1
- **Verify**: Manual review of generated scaffold and template snapshots
- **Done When**: The schema clearly forbids turning the wiki into a general project wiki and defines durable page types.

## Wave 2: Wiki Maintenance and Search Skills
- **Depends On**: Wave 1

### Task 2.1: Add Ito wiki maintenance skill
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Add a shared skill that tells the harness how to set up, refresh, ingest, query-file-back, and lint the `.ito/wiki/` knowledge layer while respecting the configured write boundary.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-wiki/SKILL.md`
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed skill teaches agents to maintain the wiki incrementally and to update index/log/status after meaningful changes.

### Task 2.2: Add wiki search skill
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Add a shared skill focused on searching and answering from the wiki first, using `index.md` as the entry point and optionally filing durable query results back into the wiki.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-wiki-search/SKILL.md`
- **Dependencies**: Task 2.1
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed skill gives a predictable query workflow for the Ito wiki and distinguishes chat answers from filed wiki artifacts.

## Wave 3: Workflow Integration
- **Depends On**: Wave 2

### Task 3.1: Integrate wiki reminders into proposal and research instructions
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Update the relevant proposal and research instruction/guidance assets so agents are prompted to consult `.ito/wiki/index.md` early when it exists and to file durable outputs back into the wiki when appropriate.
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/ito-proposal/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-research/SKILL.md`
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-cli`
- **Done When**: Proposal and research workflows mention the wiki at the right time without making it a hard blocker.

### Task 3.2: Integrate archive-triggered wiki refresh guidance
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Update archive-facing instruction and skill assets so that after a successful archive or spec sync, the agent is prompted to refresh the wiki from the archived change and any affected specs.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-archive/SKILL.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`, `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-cli`
- **Done When**: Archive workflows consistently treat wiki refresh as a normal post-archive maintenance step.

## Wave 4: Validation and Initial Rollout
- **Depends On**: Wave 3

### Task 4.1: Add template and instruction coverage tests
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: Add or update tests that verify the wiki scaffold is installed, the wiki skills are embedded, and instruction output includes the intended wiki guidance touchpoints.
- **Files**: `ito-rs/crates/ito-templates/tests/`, `ito-rs/crates/ito-cli/tests/`
- **Verify**: `make check && make test`
- **Done When**: The wiki scaffold and guidance are covered by automated tests.

### Task 4.2: Seed or refresh the repo's Ito wiki
- **Status**: [ ] pending
- **Updated At**: 2026-04-08
- **Description**: After the scaffold and skills exist, create the first repo-local `.ito/wiki/` content from current specs, modules, research, and high-signal archived changes so future proposal work can immediately use it.
- **Files**: `.ito/wiki/**`
- **Verify**: Manual review of `.ito/wiki/index.md`, `.ito/wiki/log.md`, and representative pages
- **Done When**: The repo contains an initial wiki that reflects current Ito knowledge and can be queried in future sessions.

# Tasks: 027-01_add-ito-wiki

## Wave 1: Wiki Scaffold, Ownership, and Boundaries
- **Depends On**: none

### Task 1.1: Add `.ito/wiki/` scaffold to project templates
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Add the initial wiki root to the default project template, including `index.md`, `log.md`, `overview.md`, `_meta/config.yaml`, `_meta/schema.md`, and `_meta/status.md`. Keep the scaffold Obsidian-friendly, plain-markdown-first, and clearly Ito-scoped.
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/index.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/log.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/overview.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/config.yaml`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/schema.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/status.md`
- **Verify**: `cargo test -p ito-templates`
- **Done When**: New Ito projects receive a valid wiki scaffold with stable entry points.

### Task 1.2: Define ownership and upgrade preservation semantics
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Ensure `.ito/wiki/**` scaffold installation preserves existing LLM-authored/user-owned content on `ito init --upgrade`, `ito update`, and non-force refreshes. Document any marker-managed sections explicitly.
- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/schema.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/config.yaml`, `ito-rs/crates/ito-templates/tests/`
- **Dependencies**: Task 1.1
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-core`
- **Done When**: Automated tests prove existing wiki content is not overwritten blindly and missing scaffold files can be installed safely.

### Task 1.3: Document wiki source boundaries, page model, and authority metadata
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Write the wiki schema/config so the source boundary is `.ito`-first and every durable page can declare page type, authority, source references, freshness, known gaps, and cross-links.
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/schema.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/config.yaml`, `ito-rs/crates/ito-templates/assets/default/project/.ito/wiki/_meta/status.md`
- **Dependencies**: Task 1.1
- **Verify**: Manual review plus `cargo test -p ito-templates`
- **Done When**: The schema clearly forbids turning the wiki into a general project wiki and defines durable page types, case-by-case authority, source refs, freshness, and graph links.

## Wave 2: Wiki Maintenance, Search, and Lint Skills
- **Depends On**: Wave 1

### Task 2.1: Add Ito wiki maintenance and lint skill
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Add a shared skill that tells the harness how to set up, refresh, ingest into, repair, and lint the `.ito/wiki/` knowledge layer while respecting the configured write boundary and warn-and-update behavior.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-wiki/SKILL.md`
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed skill teaches agents to maintain the wiki incrementally, update topic pages first, record source/freshness metadata, lint health issues, and update index/log/status after meaningful changes.

### Task 2.2: Add wiki search skill
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Add a shared skill focused on searching and answering from the wiki first, using `index.md` as the entry point and falling back to raw Ito artifacts when wiki coverage is missing, stale, or contradictory.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-wiki-search/SKILL.md`
- **Dependencies**: Task 2.1
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The installed skill gives a predictable query workflow for cited wiki answers, distinguishes chat answers from durable wiki artifacts, and calls out stale or missing coverage.

### Task 2.3: Verify skill distribution across harnesses
- **Status**: [x] complete
- **Updated At**: 2026-04-26
- **Description**: Ensure the new wiki skills are embedded and distributed through all supported harness install paths without requiring harness-specific duplicate implementations.
- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`, `ito-rs/crates/ito-templates/src/lib.rs`, `ito-rs/crates/ito-templates/tests/`, `ito-rs/crates/ito-core/tests/distribution.rs`
- **Dependencies**: Task 2.1, Task 2.2
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-core --test distribution`
- **Done When**: Tests confirm the wiki skills are available in generated harness assets.

## Wave 3: Workflow Integration
- **Depends On**: Wave 2

### Task 3.1: Integrate warn-and-update wiki guidance into proposal instructions
- **Status**: [x] complete
- **Updated At**: 2026-04-27
- **Description**: Update proposal-facing instruction/guidance assets so agents consult `.ito/wiki/index.md` early when it exists, warn on stale or contradictory coverage, fall back to raw Ito sources, and update the wiki when proposal work creates durable synthesis.
- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2`, `ito-rs/crates/ito-templates/assets/skills/ito-proposal/SKILL.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-cli`
- **Done When**: Proposal workflows mention the wiki at the right time without making stale/absent wiki coverage a hard blocker.

### Task 3.2: Integrate wiki guidance into research instructions
- **Status**: [x] complete
- **Updated At**: 2026-04-27
- **Description**: Update research-facing instructions so durable findings can be filed back into topic pages or query artifacts while the original research output remains in `.ito/research/` or change review directories.
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-research/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-research/*.md`
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Research workflows distinguish source research artifacts from wiki synthesis and specify when to update the wiki.

### Task 3.3: Integrate archive-triggered topic-page refresh guidance
- **Status**: [x] complete
- **Updated At**: 2026-04-27
- **Description**: Update archive-facing instruction and skill assets so that after successful archive/spec sync, agents refresh relevant topic pages with links to archived changes, specs, modules, research, architecture notes, and documentation.
- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/archive.md.j2`, `ito-rs/crates/ito-templates/assets/skills/ito-archive/SKILL.md`, `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`, `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
- **Verify**: `cargo test -p ito-templates && cargo test -p ito-cli`
- **Done When**: Archive workflows consistently treat wiki refresh as recommended post-archive follow-through and prefer topic-page synthesis over one page per archived change.

## Wave 4: Validation and Initial Rollout
- **Depends On**: Wave 3

### Task 4.1: Add template, preservation, and instruction coverage tests
- **Status**: [ ] pending
- **Updated At**: 2026-04-24
- **Description**: Add or update tests that verify the wiki scaffold is installed, existing wiki files are preserved on upgrade, wiki skills are embedded, and instruction output includes intended wiki guidance touchpoints.
- **Files**: `ito-rs/crates/ito-templates/tests/`, `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/distribution.rs`, `ito-rs/crates/ito-core/tests/`
- **Verify**: `make check && make test`
- **Done When**: The scaffold, preservation behavior, skill distribution, and guidance output are covered by automated tests.

### Task 4.2: Seed this repo's initial Ito wiki
- **Status**: [ ] pending
- **Updated At**: 2026-04-24
- **Description**: After scaffold and skills exist, create the first repo-local `.ito/wiki/` content from current specs, modules, research, high-signal archived changes, and architecture guidance. Prefer topic pages with links to specs, modules, changes, research, and relevant documentation.
- **Files**: `.ito/wiki/**`
- **Verify**: Manual review of `.ito/wiki/index.md`, `.ito/wiki/log.md`, `.ito/wiki/_meta/status.md`, and representative topic pages
- **Done When**: The repo contains an initial wiki that supports cited search, graph-style cross-reference discovery, and future proposal/research/archive sessions.

### Task 4.3: Run final validation and review
- **Status**: [ ] pending
- **Updated At**: 2026-04-24
- **Description**: Validate the change package and run the project quality gate after implementation.
- **Files**: `.ito/changes/027-01_add-ito-wiki/**`, `ito-rs/crates/ito-templates/**`, `ito-rs/crates/ito-core/**`, `ito-rs/crates/ito-cli/**`
- **Verify**: `ito validate 027-01_add-ito-wiki --strict && make check && make test`
- **Done When**: Ito validation passes and implementation checks pass or any residual risk is explicitly documented.

# Ito Text Sources Audit (ito-rs only)

This report inventories where Ito's agent-facing (or agent-consumed) text lives, and where repository-installed text lives, **restricted to `ito-rs/*`**.

Out of scope (by request): any directories outside `ito-rs/`.

Excluded (by request): error messages and generic CLI help output.

## 1) Text Ito Installs Into Repositories

The canonical source-of-truth for installed text (templates, skills, commands, adapters) is the **embedded assets** owned by `ito-rs/crates/ito-templates/`.

### 1.1 Embedded Project/Home Templates (repo files)

Source (embedded):

- `ito-rs/crates/ito-templates/assets/default/project/`
- `ito-rs/crates/ito-templates/assets/default/home/` (currently may be empty/unused)

What these include (examples visible in-tree):

- Project root docs: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
- Tool-specific root doc(s): `ito-rs/crates/ito-templates/assets/default/project/CLAUDE.md`
- Ito project docs and state:
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/project.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/config.json`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/user-guidance.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/planning/STATE.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/planning/ROADMAP.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/planning/PROJECT.md`
  - `ito-rs/crates/ito-templates/assets/default/project/.ito/commands/*.md`

Installation behavior (where it is wired up):

- Asset embedding and path rewriting:
  - `ito-rs/crates/ito-templates/src/lib.rs`
    - Embeds `assets/default/project`, `assets/default/home`
    - Rewrites literal `.ito/` paths when the configured ito directory name differs
    - Supports marker-managed blocks (`<!-- ITO:START -->` / `<!-- ITO:END -->`)
- Installer that writes/updates files:
  - `ito-rs/crates/ito-core/src/installers/mod.rs`
    - Copies embedded project files into the target repo root
    - For marker-managed files, updates only the managed block
    - For init, refuses to overwrite non-marker files unless `--force`
    - Adds a `.gitignore` entry for `{ito_dir}/session.json` on init

### 1.2 Skills (installed to tool skill directories)

Source (embedded):

- `ito-rs/crates/ito-templates/assets/skills/`

What these are:

- Markdown skills (typically `SKILL.md`, plus supporting markdown/scripts in subfolders)
- Includes both general development skills and Ito workflow skills

Installation behavior (where it is wired up):

- Embed accessors:
  - `ito-rs/crates/ito-templates/src/lib.rs` (`skills_files()`, `get_skill_file()`)
- Manifest + install targets per tool:
  - `ito-rs/crates/ito-core/src/distribution.rs`
    - Installs to:
      - OpenCode: `.opencode/skills/<name>/...` (with `ito-` prefix unless already `ito...`)
      - Claude Code: `.claude/skills/<name>/...` (same prefix rule)
      - Codex: `.codex/skills/<name>/...` (same prefix rule)
      - GitHub Copilot: `.github/skills/<name>/...` (same prefix rule)

### 1.3 Commands/Prompts (installed to tool command/prompt directories)

Source (embedded):

- `ito-rs/crates/ito-templates/assets/commands/`

Observed command markdown sources:

- `ito-rs/crates/ito-templates/assets/commands/ito.md`
- `ito-rs/crates/ito-templates/assets/commands/ito-apply.md`
- `ito-rs/crates/ito-templates/assets/commands/ito-proposal.md`
- `ito-rs/crates/ito-templates/assets/commands/ito-review.md`
- `ito-rs/crates/ito-templates/assets/commands/ito-archive.md`
- `ito-rs/crates/ito-templates/assets/commands/ito-research.md`

Installation behavior (where it is wired up):

- Embed accessors:
  - `ito-rs/crates/ito-templates/src/lib.rs` (`commands_files()`, `get_command_file()`)
- Manifest + install targets per tool:
  - `ito-rs/crates/ito-core/src/distribution.rs`
    - OpenCode: `.opencode/commands/<same filename>`
    - Claude Code: `.claude/commands/<same filename>`
    - Codex: `.codex/prompts/<same filename>`
    - GitHub Copilot: `.github/prompts/<same name>.prompt.md` (converts `.md` -> `.prompt.md`)

### 1.4 Adapters / Bootstrap Files

Source (embedded):

- `ito-rs/crates/ito-templates/assets/adapters/`

Observed adapter files:

- `ito-rs/crates/ito-templates/assets/adapters/opencode/ito-skills.js`
- `ito-rs/crates/ito-templates/assets/adapters/claude/session-start.sh`
- `ito-rs/crates/ito-templates/assets/adapters/codex/ito-skills-bootstrap.md`

Installation behavior (where it is wired up):

- `ito-rs/crates/ito-core/src/distribution.rs`
  - OpenCode: installs plugin to `.opencode/plugins/ito-skills.js`
  - Claude Code: installs bootstrap to `.claude/session-start.sh`
  - Codex: installs bootstrap to `.codex/instructions/ito-skills-bootstrap.md`

## 2) Text Produced for Agents ("ito agent instruction …")

This section covers text emitted by commands that are intended for an agent to read.

### 2.1 "Bootstrap" instruction text

Source:

- `ito-rs/crates/ito-cli/src/app/instructions.rs`

Notes:

- `ito agent instruction bootstrap --tool <opencode|claude|codex>` returns a large, hardcoded markdown instruction block.
- This is agent-facing text generated from Rust string literals (not from embedded markdown files).

### 2.2 Artifact instructions (proposal/specs/tasks/apply/review/archive/etc)

Primary output assembly:

- `ito-rs/crates/ito-cli/src/app/instructions.rs`
  - Prints structured, agent-consumable output (includes tags like `<artifact>`, `<context>`, `<template>`, etc. for non-apply artifacts)
  - For `apply`, prints a markdown-ish sectioned output (Context Files, Task Tracking, Testing Policy, Tasks, Instruction)
  - Injects "User Guidance" content when present

Where the underlying artifact text comes from (within `ito-rs/` code paths):

- Workflow resolution and apply instruction generation:
  - `ito-rs/crates/ito-core/src/workflow/mod.rs`
    - Loads workflow schema definitions (YAML) and artifact templates (markdown)
    - Returns:
      - `InstructionsResponse` (description, instruction, template, dependency list, output path)
      - `ApplyInstructionsResponse` (context files, task list, instruction strings)
- Instruction templates (minijinja, embedded):
  - `ito-rs/crates/ito-templates/src/instructions.rs`
  - `ito-rs/crates/ito-templates/assets/instructions/` (e.g. `*.md.j2`)
  - `ito-rs/crates/ito-templates/assets/instructions/README.md`

User guidance injection:

- Reads `.ito/user-guidance.md` and emits the portion after the ITO managed block:
  - `ito-rs/crates/ito-core/src/workflow/mod.rs` (`load_user_guidance`)
  - Consumed by: `ito-rs/crates/ito-cli/src/app/instructions.rs`

Testing policy text (agent-facing):

- Derived from project config and printed as part of agent instructions:
  - `ito-rs/crates/ito-cli/src/app/instructions.rs` (`load_testing_policy`, `print_testing_policy_xml`, `print_apply_instructions_text`)

## 3) Schemas (data formats that drive text)

Even when not stored as markdown, these schemas define the structure of the text-producing workflows.

### 3.1 Workflow definition/execution "schemas" (Rust serde models)

Source:

- `ito-rs/crates/ito-schemas/src/workflow.rs` (workflow definition model)
- `ito-rs/crates/ito-schemas/src/workflow_state.rs` (execution state model)
- `ito-rs/crates/ito-schemas/src/workflow_plan.rs` (plan model)
- `ito-rs/crates/ito-schemas/src/lib.rs`

Notes:

- These are the canonical structs for YAML/JSON formats consumed/produced by Ito.
- They are not markdown, but they are the "schema" layer governing what text and artifacts exist.

## 4) "Installed output" copies inside `ito-rs/`

Within `ito-rs/` itself, there are checked-in, tool-specific directories containing skills/commands/prompts that mirror what `ito init` installs.

Examples (not exhaustive):

- `ito-rs/.opencode/skills/`
- `ito-rs/.claude/skills/`
- `ito-rs/.github/skills/`
- `ito-rs/.codex/skills/`
- `ito-rs/.claude/commands/`
- `ito-rs/.github/prompts/`

These are useful for development/review, but the source-of-truth for installation content is the embedded assets under:

- `ito-rs/crates/ito-templates/assets/`

## 5) Other markdown docs under `ito-rs/`

These are agent/dev-facing documentation within the `ito-rs` repo itself:

- `ito-rs/AGENTS.md`
- `ito-rs/CLAUDE.md`
- `ito-rs/scripts/README.md`
- `ito-rs/crates/ito-templates/AGENTS.md` (maintainer notes for template sources)

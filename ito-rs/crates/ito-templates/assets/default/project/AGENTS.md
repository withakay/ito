<!-- ITO:START -->

# Ito Instructions

These instructions are for AI assistants working in this project.

Always open `@/.ito/AGENTS.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/.ito/AGENTS.md` to learn:

- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Project setup: run `/ito-project-setup` (or `ito agent instruction project-setup`) until `.ito/project.md` is marked `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Note: Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.
Add project-specific guidance in `.ito/user-prompts/guidance.md` (shared), `.ito/user-prompts/<artifact>.md` (artifact-specific), and/or below this managed block.

Keep this managed block so `ito init --upgrade` can refresh the managed instructions non-destructively.
To refresh only the Ito-managed content in this file, run: `ito init --upgrade`

## Path Helpers

Use `ito path ...` to get absolute paths at runtime (do not hardcode absolute paths into committed files):

- `ito path project-root`
- `ito path worktree-root`
- `ito path ito-root`
- `ito path worktrees-root`
- `ito path worktree --main|--branch <name>|--change <id>`

## Worktree Workflow

{% if enabled %}
**Strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`

Worktree rules:

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no Ito change ID exists yet, create a temporary proposal worktree first, run change creation there, then move into the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

{% if strategy == "checkout_subdir" %}
This project uses in-repo worktrees under a dedicated subdirectory:

```bash
.{{ layout_dir_name }}/<full-change-id>/
```

To create a worktree for a change:

```bash
mkdir -p ".{{ layout_dir_name }}"
git worktree add ".{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "checkout_siblings" %}
This project uses sibling-directory worktrees:

```bash
../<project-name>-{{ layout_dir_name }}/<full-change-id>/
```

To create a worktree for a change:

```bash
mkdir -p "../<project-name>-{{ layout_dir_name }}"
git worktree add "../<project-name>-{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "bare_control_siblings" %}
This project uses a bare/control repo layout with worktrees as siblings:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- {{ default_branch }}/               # {{ default_branch }} branch worktree
`-- {{ layout_dir_name }}/              # change worktrees
    `-- <full-change-id>/
```

To create a worktree for a change:

```bash
mkdir -p "../{{ layout_dir_name }}"
git worktree add "../{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```

Always branch new change worktrees from `{{ default_branch }}`. Do not create them from the bare/control repo placeholder `HEAD`.
{% else %}
This project uses a custom worktree strategy. Use the configured values above.
{% endif %}

Do NOT ask the user where to create worktrees. Use the configured locations above.

After the change branch is merged, ask Ito for cleanup instructions:

```bash
ito agent instruction finish --change "<full-change-id>"
```
{% else %}
Worktrees are not configured for this project.

- Do NOT create git worktrees by default.
- Work in the current checkout unless the user explicitly requests a worktree workflow.
{% endif %}

<!-- ITO:END -->

<!-- ITO:INTERNAL:START -->
## Project Guidance

### Subagent Collaboration

Subagents are first-class tools. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.

Diversity is good: for non-trivial changes, get at least two independent review passes (for example: a Rust-focused reviewer plus a general diff reviewer).

Commonly useful subagents:

- `explore` - fast codebase navigation/search
- `ito-test-runner` - runs project tests/checks with curated output
- `rust-quality-checker` - Rust style/idioms/conventions checks
- `rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)
- `rust-test-engineer` - test strategy and coverage design
- `codex-review` - diff review for correctness and edge cases
- `documentation-police` - docs coverage/quality
- `code-simplifier` - refactor for clarity and maintainability
- `code-quality-squad` - parallel quality workflows
- `perplexity-researcher` / `perplexity-researcher-pro` - web research with citations
- `multi-agent` - explore multiple approaches and synthesize
<!-- ITO:INTERNAL:END -->

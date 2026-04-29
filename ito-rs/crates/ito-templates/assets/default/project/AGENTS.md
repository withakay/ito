<!-- ITO:START -->

# Ito Instructions

Use `@/.ito/AGENTS.md` as the source of truth when work involves planning/proposals, new capabilities, breaking or architectural changes, major performance/security work, or any ambiguous request that needs Ito workflow guidance.

Project setup: run `/ito-project-setup` (or `ito agent instruction project-setup`) until `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are Ito-managed and may be overwritten. Put project-specific guidance in `.ito/user-prompts/guidance.md`, `.ito/user-prompts/<artifact>.md`, or below this block.

Keep this block so `ito init --upgrade` can refresh managed content safely. To refresh only this managed section, run `ito init --upgrade`.

## Path Helpers

Use `ito path ...` for runtime absolute paths; do not hardcode machine-specific paths into committed files:

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

Rules:

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- The main worktree is the only worktree that may check out `{{ default_branch }}`; `{{ default_branch }}` must only ever be checked out in the main worktree.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no change ID exists yet, create a temporary proposal worktree, create the change there, then switch to the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

{% if strategy == "checkout_subdir" %}
In-repo worktrees live under:

```bash
.{{ layout_dir_name }}/<full-change-id>/
```

Create one with:

```bash
mkdir -p ".{{ layout_dir_name }}"
git worktree add ".{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "checkout_siblings" %}
Sibling-directory worktrees live under:

```bash
../<project-name>-{{ layout_dir_name }}/<full-change-id>/
```

Create one with:

```bash
mkdir -p "../<project-name>-{{ layout_dir_name }}"
git worktree add "../<project-name>-{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "bare_control_siblings" %}
Bare/control layout:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- {{ default_branch }}/               # {{ default_branch }} branch worktree
`-- {{ layout_dir_name }}/              # change worktrees
    `-- <full-change-id>/
```

Create one with:

```bash
mkdir -p "../{{ layout_dir_name }}"
git worktree add "../{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```

Always branch new change worktrees from `{{ default_branch }}`. Do not create them from the bare/control repo placeholder `HEAD`.
{% else %}
This project uses a custom worktree strategy. Use the configured values above.
{% endif %}

Do NOT ask the user where to create worktrees; use the configured locations above.

After merge, ask Ito for cleanup instructions:

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

Prefer specialist subagents for independent work, often in parallel. For non-trivial changes, get at least two independent review passes.

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

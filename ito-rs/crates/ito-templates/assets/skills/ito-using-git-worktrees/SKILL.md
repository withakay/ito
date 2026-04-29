---
name: ito-using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

<!-- ITO:START -->

# Using Git Worktrees

Use isolated worktrees for change work so the main/control checkout stays clean.

{% if enabled %}
**Configured strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`

## Rules

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- The main worktree is the only worktree that may check out `{{ default_branch }}`; `{{ default_branch }}` must only ever be checked out in the main worktree.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no change ID exists yet, create a temporary proposal worktree, create the change there, then switch to the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

## Layout

{% if strategy == "checkout_subdir" %}
Worktrees live under:

```bash
.{{ layout_dir_name }}/<full-change-id>/
```

Create one with:

```bash
mkdir -p ".{{ layout_dir_name }}"
git worktree add ".{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "checkout_siblings" %}
Worktrees live under a sibling directory:

```bash
../<project-name>-{{ layout_dir_name }}/<full-change-id>/
```

Create one with:

```bash
mkdir -p "../<project-name>-{{ layout_dir_name }}"
git worktree add "../<project-name>-{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "bare_control_siblings" %}
Worktrees live under the bare/control layout:

```bash
../
|-- {{ default_branch }}/
`-- {{ layout_dir_name }}/<full-change-id>/
```

Create one with:

```bash
mkdir -p "../{{ layout_dir_name }}"
git worktree add "../{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```

Always branch from `{{ default_branch }}`. Never use the bare/control repo placeholder `HEAD` as the checkout source.
{% else %}
Use the configured strategy and directory values above.
{% endif %}

Do NOT ask the user where to create worktrees.

## Path Helpers

For absolute paths, use:

- `ito path project-root`
- `ito path worktree-root`
- `ito path worktrees-root`
- `ito path worktree --main|--branch <name>|--change <id>`

## Safety Checks

- Ensure the parent directory exists.
- Run a clean baseline build/test in the new worktree so new failures are attributable.
- If the baseline fails, stop or call it out explicitly before proceeding.

## Cleanup

After merge, ask Ito for cleanup instructions:

```bash
ito agent instruction finish --change "<full-change-id>"
```

If a worktree is locked, assume that was intentional; do NOT unlock/remove it unless the user explicitly asks.

{% else %}
Worktrees are not configured for this project.

- Do NOT create git worktrees by default.
- Work in the current checkout.
- Only use worktrees when the user explicitly requests that workflow.
{% endif %}

## Integration

Called by any workflow that needs an isolated workspace.

<!-- ITO:END -->

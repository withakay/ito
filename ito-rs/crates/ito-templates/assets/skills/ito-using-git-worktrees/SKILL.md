---
name: ito-using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

<!-- ITO:START -->

# Using Git Worktrees

## Overview

Git worktrees create isolated workspaces that share the same repository, allowing work on multiple branches simultaneously.

{% if enabled %}
**Configured strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`

## Change Worktree Rules

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no Ito change ID exists yet, create a temporary proposal worktree first, run change creation there, then move into the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

## Worktree Location

{% if strategy == "checkout_subdir" %}
Worktrees live under:

```bash
.{{ layout_dir_name }}/<full-change-id>/
```

Create a worktree:

```bash
mkdir -p ".{{ layout_dir_name }}"
git worktree add ".{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```
{% elif strategy == "checkout_siblings" %}
Worktrees live under a sibling directory:

```bash
../<project-name>-{{ layout_dir_name }}/<full-change-id>/
```

Create a worktree:

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

Create a worktree:

```bash
mkdir -p "../{{ layout_dir_name }}"
git worktree add "../{{ layout_dir_name }}/<full-change-id>" -b <full-change-id> {{ default_branch }}
```

Always branch new change worktrees from `{{ default_branch }}`. Never use the bare/control repo placeholder `HEAD` as the checkout source.
{% else %}
Use the configured strategy and directory values above.
{% endif %}

Do NOT ask the user where to create worktrees.

## Path Helpers

If you need absolute paths (for logs, scripts, or agent instructions), use:

- `ito path project-root`
- `ito path worktree-root`
- `ito path worktrees-root`
- `ito path worktree --main|--branch <name>|--change <id>`

## Safety Checks

- Ensure the parent directory for the worktree exists (create it if needed).
- Run a clean baseline build/test in the new worktree so new failures are attributable.
- Do not proceed if baseline tests fail without explicitly calling that out.

## Cleanup

After the branch is merged, ask Ito for cleanup instructions:

```bash
ito agent instruction finish --change "<full-change-id>"
```

If a worktree is locked, assume it was locked on purpose; do NOT unlock/remove it unless the user explicitly asks.

{% else %}
Worktrees are not configured for this project.

- Do NOT create git worktrees by default.
- Work in the current checkout.
- Only use worktrees when the user explicitly requests that workflow.
{% endif %}

## Integration

**Called by:**
- Any workflow that needs isolated workspace

<!-- ITO:END -->

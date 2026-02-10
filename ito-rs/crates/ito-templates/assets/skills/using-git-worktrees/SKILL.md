---
name: using-git-worktrees
description: Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification
---

# Using Git Worktrees

## Overview

Git worktrees create isolated workspaces that share the same repository, allowing work on multiple branches simultaneously.

{% if enabled %}
**Configured strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`

## Worktree Location

{% if strategy == "checkout_subdir" %}
Worktrees live under:

```bash
<project-root>/{{ layout_dir_name }}/<change-name>/
```

Create a worktree:

```bash
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% elif strategy == "checkout_siblings" %}
Worktrees live under a sibling directory:

```bash
<project-root>/
../<project-name>-{{ layout_dir_name }}/<change-name>/
```

Create a worktree:

```bash
mkdir -p "../<project-name>-{{ layout_dir_name }}"
git worktree add "../<project-name>-{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% elif strategy == "bare_control_siblings" %}
Worktrees live under the bare/control layout:

```bash
<project>/
|-- {{ default_branch }}/
`-- {{ layout_dir_name }}/<change-name>/
```

Create a worktree:

```bash
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% else %}
Use the configured strategy and directory values above.
{% endif %}

Do NOT ask the user where to create worktrees.

## Safety Checks

- Ensure the parent directory for the worktree exists (create it if needed).
- Run a clean baseline build/test in the new worktree so new failures are attributable.
- Do not proceed if baseline tests fail without explicitly calling that out.

## Cleanup

After the branch is merged:

```bash
git worktree remove "<worktree-path>" 2>/dev/null || true
git branch -d "<branch-name>" 2>/dev/null || true
git worktree prune
```

{% else %}
Worktrees are not configured for this project.

- Do NOT create git worktrees by default.
- Work in the current checkout.
- Only use worktrees when the user explicitly requests that workflow.
{% endif %}

## Integration

**Called by:**
- Any workflow that needs isolated workspace

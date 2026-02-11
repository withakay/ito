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

Note: Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.
Add project-specific guidance in `.ito/user-guidance.md` (injected into agent instruction outputs) and/or below this managed block.

Keep this managed block so 'ito update' can refresh the instructions.

## Worktree Workflow

{% if enabled %}
**Strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`

{% if strategy == "checkout_subdir" %}
This project uses in-repo worktrees under a dedicated subdirectory:

```bash
<project-root>/{{ layout_dir_name }}/<change-name>/
```

To create a worktree for a change:

```bash
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% elif strategy == "checkout_siblings" %}
This project uses sibling-directory worktrees:

```bash
<project-root>/
../<project-name>-{{ layout_dir_name }}/<change-name>/
```

To create a worktree for a change:

```bash
mkdir -p "../<project-name>-{{ layout_dir_name }}"
git worktree add "../<project-name>-{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% elif strategy == "bare_control_siblings" %}
This project uses a bare/control repo layout with worktrees as siblings:

```bash
<project>/                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- {{ default_branch }}/               # {{ default_branch }} branch worktree
`-- {{ layout_dir_name }}/              # change worktrees
    `-- <change-name>/
```

To create a worktree for a change:

```bash
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/<change-name>" -b <change-name>
```
{% else %}
This project uses a custom worktree strategy. Use the configured values above.
{% endif %}

Do NOT ask the user where to create worktrees. Use the configured locations above.

After the change branch is merged, clean up:

```bash
git worktree remove <change-name> 2>/dev/null || true
git branch -d <change-name> 2>/dev/null || true
git worktree prune
```
{% else %}
Worktrees are not configured for this project.

- Do NOT create git worktrees by default.
- Work in the current checkout unless the user explicitly requests a worktree workflow.
{% endif %}

<!-- ITO:END -->

## Project Guidance

(Add any project-specific assistant guidance here. Prefer `.ito/user-guidance.md` for instructions you want applied consistently to Ito change workflows.)

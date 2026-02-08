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
{% if enabled %}

## Worktree Workflow

**Strategy:** `{{ strategy }}`
**Directory name:** `{{ layout_dir_name }}`
**Default branch:** `{{ default_branch }}`
**Integration mode:** `{{ integration_mode }}`
{% if strategy == "checkout_subdir" %}

Worktrees live in a hidden subdirectory inside the checkout:

```
<project>/                          # {{ default_branch }} branch checkout
├── .git/
├── src/
└── .{{ layout_dir_name }}/         # gitignored worktree directory
    └── <change-name>/              # one worktree per change
```

To create a worktree for a change:

```bash
# Ensure .{{ layout_dir_name }}/ is gitignored
grep -qxF '.{{ layout_dir_name }}/' .gitignore 2>/dev/null || echo '.{{ layout_dir_name }}/' >> .gitignore

# Create the worktree
git worktree add ".{{ layout_dir_name }}/<change-name>" -b <change-name>
```

Do NOT ask the user where to create worktrees. Use `.{{ layout_dir_name }}/` inside the project root.
{% elif strategy == "checkout_siblings" %}

Worktrees live in a sibling directory next to the project checkout:

```
~/Code/
├── <project>/                              # {{ default_branch }} branch checkout
│   ├── .git/
│   └── src/
└── <project>-{{ layout_dir_name }}/        # sibling worktree directory
    └── <change-name>/                      # one worktree per change
```

To create a worktree for a change:

```bash
PROJECT_NAME=$(basename "$(pwd)")
WORKTREE_BASE="../${PROJECT_NAME}-{{ layout_dir_name }}"
mkdir -p "$WORKTREE_BASE"

git worktree add "${WORKTREE_BASE}/<change-name>" -b <change-name>
```

Do NOT ask the user where to create worktrees. Use `../<project>-{{ layout_dir_name }}/`.
{% elif strategy == "bare_control_siblings" %}

This project uses a bare/control repo layout with worktrees as siblings:

```
<project>/                              # bare/control repo
├── .bare/                              # git object store
├── .git                                # gitdir pointer
├── {{ default_branch }}/               # main branch worktree
└── {{ layout_dir_name }}/              # Ito-managed change worktrees
    └── <change-name>/                  # one worktree per change
```

To create a worktree for a change:

```bash
mkdir -p "{{ layout_dir_name }}"
git worktree add "{{ layout_dir_name }}/<change-name>" -b <change-name>
```

Do NOT ask the user where to create worktrees. Use `{{ layout_dir_name }}/` inside the bare repo root.
{% endif %}
{% if integration_mode == "commit_pr" %}

**Integration:** Commit changes in the worktree, push the branch, and create a pull request.
{% elif integration_mode == "merge_parent" %}

**Integration:** Commit changes in the worktree, then merge the branch into `{{ default_branch }}`.
{% endif %}

After the change branch is merged, clean up:

```bash
git worktree remove <change-name> 2>/dev/null || true
git branch -d <change-name> 2>/dev/null || true
git worktree prune
```
{% else %}

## Worktree Workflow

Worktrees are **not configured** for this project. Do NOT create git worktrees unless the user explicitly requests it. Work in the current checkout.

To enable worktrees, run: `ito init` and follow the worktree wizard, or set configuration directly:

```bash
ito config set worktrees.enabled true
ito config set worktrees.strategy checkout_subdir
ito update
```
{% endif %}

<!-- ITO:END -->

## Project Guidance

(Add any project-specific assistant guidance here. Prefer `.ito/user-guidance.md` for instructions you want applied consistently to Ito change workflows.)

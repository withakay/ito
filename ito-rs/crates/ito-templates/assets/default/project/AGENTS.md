<!-- ITO:START -->

# Ito Instructions

Use `@/.ito/AGENTS.md` as the source of truth when work involves planning/proposals, new capabilities, breaking or architectural changes, major performance/security work, or any ambiguous request that needs Ito workflow guidance.

Project setup: run `ito agent instruction project-setup` and follow the emitted prompt until `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are Ito-managed and may be overwritten. Put project-specific guidance in `.ito/user-prompts/guidance.md`, `.ito/user-prompts/<artifact>.md`, or below this block.

Keep this block so `ito init --upgrade` can refresh managed content safely. To refresh only this managed section, run `ito init --upgrade`.

When present, use `.ito/wiki/index.md` for Ito-scoped synthesis, freshness warnings, and archive follow-through.

## Legacy Coordination Recovery

If Ito reports legacy or ambiguous coordination-worktree storage, inspection commands remain available but stateful commands are blocked. Do not repair links or copy state by hand. Run `/ito-migrate-to-main` or `ito agent instruction migrate-to-main`, then follow the emitted inventory, conflict-stop, validation, and reviewed-integration procedure before starting implementation.

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
- Before implementation writes, create or reuse the dedicated change worktree through `ito worktree ensure`. If no change ID exists yet, author the proposal in an already-writable proposal checkout and keep it proposal-only until review and integration.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

Worktrunk path configuration for Ito-managed worktrees:

```toml
worktree-path = "<ito-worktrees-root>/{% raw %}{{ branch | sanitize }}{% endraw %}"
```

{% if strategy == "checkout_subdir" %}
In-repo worktrees live under:

```bash
.{{ layout_dir_name }}/<full-change-id>/
```

{% elif strategy == "checkout_siblings" %}
Sibling-directory worktrees live under:

```bash
../<project-name>-{{ layout_dir_name }}/<full-change-id>/
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

Ito creates new change worktrees from the captured authority OID. Do not substitute the bare/control repo placeholder `HEAD`.
{% else %}
This project uses a custom worktree strategy. Use the configured values above.
{% endif %}

Create or reuse the implementation worktree only through the guarded lifecycle:

```bash
CHANGE_DIR=$(ito worktree ensure --change "<full-change-id>") || exit 1
cd "$CHANGE_DIR"
ito change preflight "<full-change-id>" --for execute
```

`ito worktree ensure` proves the reviewed proposal exists on authoritative main, creates from the captured authority OID, and rejects stale or unrelated existing worktrees.

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

<!-- Project-specific agent guidance below this line is preserved by Ito. -->

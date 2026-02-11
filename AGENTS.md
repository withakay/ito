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
Add project-specific guidance in `.ito/user-prompts/guidance.md` (shared), `.ito/user-prompts/<artifact>.md` (artifact-specific), and/or below this managed block.

Keep this managed block so 'ito update' can refresh the instructions.

## Worktree Workflow


**Strategy:** `bare_control_siblings`
**Directory name:** `ito-worktrees`
**Default branch:** `main`
**Integration mode:** `commit_pr`


This project uses a bare/control repo layout with worktrees as siblings:

```bash
<project>/                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- main/               # main branch worktree
`-- ito-worktrees/              # change worktrees
    `-- <change-name>/
```

To create a worktree for a change:

```bash
mkdir -p "ito-worktrees"
git worktree add "ito-worktrees/<change-name>" -b <change-name>
```


Do NOT ask the user where to create worktrees. Use the configured locations above.

After the change branch is merged, clean up:

```bash
git worktree remove <change-name> 2>/dev/null || true
git branch -d <change-name> 2>/dev/null || true
git worktree prune
```


<!-- ITO:END -->

## Worktree Usage

This repo uses a **bare-repo-with-worktrees** layout. All work happens inside worktrees — there is no checked-out tree at the bare repo root.

```
ito/                          # bare/control repo root
├── .bare/                    # git object store
├── .git                      # gitdir pointer → .bare
├── main/                     # locked worktree for the main branch
└── ito-worktrees/            # feature/change worktrees
    └── <branch-name>/
```

### Rules

1. **The `main` worktree is locked and must not be removed.** It is protected with `git worktree lock`. Do not unlock, remove, or prune it. If the lock is missing, restore it:
   ```bash
   git worktree lock main --reason "Primary worktree - do not remove"
   ```
2. **Always create feature worktrees under `ito-worktrees/`:**
   ```bash
   git worktree add ito-worktrees/<branch-name> -b <branch-name>
   ```
3. **Clean up feature worktrees after merge** (never clean up `main`):
   ```bash
   git worktree remove ito-worktrees/<branch-name> 2>/dev/null || true
   git branch -d <branch-name> 2>/dev/null || true
   git worktree prune
   ```
4. **All git commands must be run from inside a worktree** (e.g., `main/` or `ito-worktrees/<branch>/`), not from the bare repo root, unless you are managing worktrees themselves.

## Architecture

See [`.ito/architecture.md`](.ito/architecture.md) for the full architectural guidelines, including the layered (onion) architecture, crate structure, dependency rules, domain purity constraints, design patterns, and quality enforcement.

## Supported Implementation

`ito-rs/` is the supported Ito implementation and should be favored for all new work.

## Prompt Templates

Ito project/home templates are owned by the Rust embedded assets:

- `ito-rs/crates/ito-templates/assets/default/project/`
- `ito-rs/crates/ito-templates/assets/default/home/`

## Rust `ito init` Embedded Markdown

`ito init` (Rust CLI) installs files from embedded assets, not from this repo's checked-in `.opencode/` directory.

- **Shared skills**: `ito-rs/crates/ito-templates/assets/skills/` - installed to all harnesses
- **Shared adapters**: `ito-rs/crates/ito-templates/assets/adapters/` - harness-specific bootstrap files
- **Project templates**: `ito-rs/crates/ito-templates/assets/default/project/` (includes `.ito/`, harness commands/prompts)
- **Home templates**: `ito-rs/crates/ito-templates/assets/default/home/` (e.g., `.codex/...`)
- Assets are embedded via `include_dir!` in `ito-rs/crates/ito-templates/src/lib.rs` and written by `ito-rs/crates/ito-core/src/installers/mod.rs`

If you want agents to learn new workflows (e.g., task tracking), update the embedded skill markdown in those assets.

**See `ito-rs/crates/ito-templates/AGENTS.md` for detailed guidance on maintaining templates and keeping harness files in sync.**

## Rust Development

See [`ito-rs/AGENTS.md`](ito-rs/AGENTS.md) for all Rust-specific guidance: development commands, coding conventions, testing policy, quality gates, dependency rules, and git hooks.

## Using Bacon for Development, testing and debugging

Bacon provides continuous background checks for Rust code and can export error locations for agent workflows.

- Start in watch mode (default check job): `make bacon`
- Start headless with exported locations for tooling/agents: `make bacon-export`
- Export file location: `ito-rs/.bacon-locations`

For direct usage:

```bash
cd ito-rs
bacon --headless --export-locations
```

Agent workflow with exports:

1. Read `ito-rs/.bacon-locations` for `file:line:column` entries.
2. Fix issues at those locations.
3. Save changes and let bacon auto-recheck.
4. Repeat until the locations file is empty.
## Pull Request Titles

When creating a PR for a specific Ito change, include the change ID in the PR title to simplify reconciliation.

- Format: `<type>(<change-id>): <short summary>`
- Example: `feat(001-23): My cool change`
- If no change ID applies, use normal conventional commit style without a change-id scope.

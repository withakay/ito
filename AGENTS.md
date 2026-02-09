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

**Strategy:** `bare_control_siblings`
**Directory name:** `ito-worktrees`
**Default branch:** `main`
**Integration mode:** `commit_pr`


This project uses a bare/control repo layout with worktrees as siblings:

```
<project>/                              # bare/control repo
├── .bare/                              # git object store
├── .git                                # gitdir pointer
├── main/               # main branch worktree
└── ito-worktrees/              # Ito-managed change worktrees
    └── <change-name>/                  # one worktree per change
```

To create a worktree for a change:

```bash
mkdir -p "ito-worktrees"
git worktree add "ito-worktrees/<change-name>" -b <change-name>
```

Do NOT ask the user where to create worktrees. Use `ito-worktrees/` inside the bare repo root.



**Integration:** Commit changes in the worktree, push the branch, and create a pull request.


After the change branch is merged, clean up:

```bash
git worktree remove <change-name> 2>/dev/null || true
git branch -d <change-name> 2>/dev/null || true
git worktree prune
```


<!-- ITO:END -->

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

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


**Strategy:** `bare_control_siblings`
**Directory name:** `ito-worktrees`
**Default branch:** `main`
**Integration mode:** `commit_pr`


This project uses a bare/control repo layout with worktrees as siblings:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- main/               # main branch worktree
`-- ito-worktrees/              # change worktrees
    `-- <change-name>/
```

To create a worktree for a change:

```bash
mkdir -p "../ito-worktrees"
git worktree add "../ito-worktrees/<change-name>" -b <change-name>
```


Do NOT ask the user where to create worktrees. Use the configured locations above.

After the change branch is merged, clean up:

```bash
git worktree remove <worktree-path> 2>/dev/null || true
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

### Fixing Corrupt Index in Worktrees

Worktrees in the bare-repo layout occasionally develop corrupt index entries, where a blob hash (e.g., `a2b091c0...`) no longer resolves. Symptoms:

- `git diff` or `git commit` fails with `fatal: unable to read <hash>`
- `error: invalid object ... for '<file>'` during commit tree-building
- Pre-commit hooks abort with `Command 'git diff' exited with an error`

**Fix** -- delete the worktree's index and let git rebuild it from HEAD:

```bash
# 1. Identify the worktree index file
WTGITDIR="$(git rev-parse --git-dir)"          # e.g. /path/to/.bare/worktrees/<branch>
rm "$WTGITDIR/index"

# 2. Rebuild index from HEAD (unstages everything)
git reset HEAD

# 3. Re-stage your changes
git add <files...>

# 4. Commit (use --no-verify if the hook's internal stash is also corrupt)
git commit -m "..."
```

**Why this happens**: `prek` (the pre-commit runner) stashes/unstashes unstaged changes around hook execution. If a previous hook run was interrupted or the worktree was created while another process held a stale reference, the index can end up pointing to a blob that was never packed into the shared object store. `git update-index --cacheinfo` and `git read-tree HEAD` often fail to fix it because the index file itself is internally inconsistent; deleting it is the reliable fix.

### Testing Changes in Worktrees

When developing in a worktree, **always use the binary built in that worktree's target directory**, not the root workspace binary:

```bash
# ✅ Correct: Use the worktree's binary
cd ito-worktrees/<branch-name>
cargo build -p ito-cli
./target/debug/ito <command>

# ❌ Wrong: Using root workspace binary (old code)
cd /path/to/root
./target/debug/ito <command>
```

**Why this matters**: The root workspace's `target/` directory contains binaries from the `main` branch. When you build in a worktree, cargo creates a separate `target/` directory in that worktree with the new code. Using the wrong binary means testing old code, not your changes.

**Common mistake**: Running tests or manual commands with the root binary and concluding that changes don't work, when they actually do work in the worktree binary.

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

<!-- This section is maintained by the coding agent via lore (https://github.com/BYK/opencode-lore) -->
## Long-term Knowledge

### Architecture

<!-- lore:019cad70-3bb8-7d8d-9148-eb9c826a3a11 -->
- **CLI layer stays thin; core owns behavior**: Project convention is layered: \`ito-cli\` should parse flags, call \`ito-core\`, and format output only. If a command handler starts doing orchestration/repository business logic, treat that as a refactor target into \`ito-core\` use-cases.

### Gotcha

<!-- lore:019cad70-3bb9-7343-8fd3-1d870a4fe4f6 -->
- **Worktree conflicts on PR checkout**: \`gh pr checkout\` can fail when the PR branch is already attached to another worktree. In this repo, switch to the existing sibling worktree under \`ito-worktrees/\<branch>\` instead of forcing another checkout.
<!-- lore:019cad70-3bb9-7343-8fd3-1d887e310eba -->
- **Git wrapper rejects -C**: This environment routes git through an \`rtk git\` wrapper that may reject \`git -C ...\`. Run git commands from the target directory via the tool \`workdir\` (or enter that worktree) rather than relying on \`-C\`.
<!-- lore:019cad70-3bb9-7343-8fd3-1d8a011aa03e -->
- **Post-commit hooks can leave new unstaged edits**: After committing, immediately run \`git status\` before pushing. In this repo, hooks/formatting can rewrite files (for example import ordering), leaving the branch dirty and requiring a follow-up commit rather than assuming the push includes everything.

### Pattern

<!-- lore:019cad70-3bb8-7d8d-9148-eb9dde2d5ff8 -->
- **Listing-style commands require JSON mode**: For commands that list/show data, expose \`--json\` and implement structured output in the handler. Follow existing clap/app patterns (\`json\` bool arg + output branching) to stay consistent with repo conventions and bot review expectations.
<!-- lore:019cad70-3bb9-7343-8fd3-1d894dfb8799 -->
- **PR triage must parse review-body nitpicks**: When handling PR feedback, collect all channels: review threads, review bodies, and issue comments. CodeRabbit nitpicks can exist only in review-body sections, so run nitpick extraction and triage those with inline comments; otherwise actionable items are missed.
<!-- End lore-managed section -->

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

Project setup: run `/ito-project-setup` (or `ito agent instruction project-setup`) until `.ito/project.md` is marked `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

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

<!-- lore:019cad59-2b4d-7362-aebd-ec31b5a76c21 -->
- **Bare repo worktree layout for feature development**: This repo uses a bare-repo-with-worktrees layout. The \`main\` worktree is locked at \`main/\`. Feature worktrees go under \`ito-worktrees/\<branch-name>\`. When a branch is already checked out in a worktree, you cannot check it out elsewhere — work from the existing worktree path instead. All git commands must run from inside a worktree, not the bare repo root. The current working directory \`/Users/jack/Code/withakay/ito/dev\` appears to be the bare/control repo root.

### Decision

<!-- lore:019cad72-298b-70bf-8907-4f802d612520 -->
- **Feature branches must be created from main, not dev**: Branch from \`main\` only; do not base feature work on intermediate integration branches. Earlier \`dev\`-based flow caused PRs to include many phantom commits after squash merges (same content, different SHAs). If this ever recurs, recover by rebasing only unique work onto \`origin/main\` using first-parent ancestry (\`git log --first-parent\` + \`git rebase --onto\`) instead of rebasing full history.

### Gotcha

<!-- lore:019cbad7-45c8-747f-8c96-470e6099c0a7 -->
- **Edit tool may silently fail to persist file changes**: The Edit tool can report success while not actually writing changes to disk. This was observed with \`AGENTS.md\` — multiple Edit calls returned success but \`grep\` and \`git diff\` confirmed the file was unchanged. When edits are critical, verify with \`grep\` or \`sha256sum\` after editing, and fall back to \`sed\` via Bash if the Edit tool's changes don't persist. This may be related to file size or the bare-repo worktree layout.
<!-- lore:019cbad7-45c7-7818-8b30-facf89a98eda -->
- **Lore writes asterisk lists that fail MD004 lint**: The lore system (opencode-lore) writes \`AGENTS.md\` list items using \`\*\` (asterisk) markers, but the project's markdownlint config enforces MD004 (dash-style lists). This causes pre-push hook failures on any commit that includes lore-managed sections. Fix with \`sed -i '' 's/^\\\* /- /' AGENTS.md\` after lore updates. The Edit tool may silently fail to persist changes to this file — verify with \`grep '^\ \* ' AGENTS.md\` and use \`sed\` via Bash if edits don't stick.
<!-- lore:019caf3f-be0c-7797-bac1-e8a8a7b3af71 -->
- **Stale worktree index.lock blocks git add**: In this bare-repo worktree layout, \`git add\` can fail with an existing \`.bare/worktrees/\<name>/index.lock\` after interrupted git operations. Verify no active git process, then remove the stale lock file and retry staging. Treat this as a local lock cleanup, not a repository reset.
<!-- lore:019caf3c-0454-79ff-a5fb-9cd9683056b9 -->
- **Archive may overwrite specs with delta blocks**: Bulk \`ito archive\` runs can overwrite canonical spec files with delta-only blocks (\`## ADDED/MODIFIED Requirements\`), dropping \`# Spec\`, \`## Purpose\`, and \`## Requirements\`. After archiving, immediately validate and inspect touched specs; if canonical structure was replaced, restore the full spec shape from prior history, then reapply intended requirement changes. This prevents cascading \`ito validate --specs\` failures.
<!-- lore:019cad59-2b4d-7362-aebd-ec32c59b4025 -->
- **rtk gh wrapper ignores --jq and other gh flags**: \`rtk\` wrappers can break normal CLI expectations. \`rtk gh\` may ignore raw JSON filtering flags, and \`rtk git\` may reject options like \`-C\` or \`-c\`; when you need those behaviors, use native \`gh\`/\`git\` binaries directly. In automation, prefer setting the tool/workdir context instead of relying on \`git -C\`.

### Pattern

<!-- lore:019cad88-b1ce-71b9-9304-642d211a9a1f -->
- **Rust style prefers explicit for-loops**: In \`ito-rs/\*\*/\*.rs\`, prefer explicit \`for\` loops over iterator chains like \`.iter().filter().map().collect()\`. Even when bot feedback suggests iterator-style refactors, keep loop-based implementations to match project conventions and avoid churn in review.
<!-- lore:019caf4e-2d50-755f-8fcc-cb5ea06e216c -->
- **Agentic workflows are markdown-first**: In this repo, GitHub Agentic Workflows are markdown-first: edit \`.github/workflows/\*.md\` and validate with \`gh aw validate \<workflow>\`. Only compile when needed; \`gh aw compile \<workflow>\` generates the lock artifacts (\`.github/workflows/\<name>.lock.yml\`) and updates \`.github/aw/actions-lock.json\`, which should be committed together. For workflow changes, run both \`gh aw\` validation and \`ito validate \<change-id> --strict\` before marking tasks complete.

### Preference

<!-- lore:019caf3f-be0c-7797-bac1-e8a70ff56b27 -->
- **Include AGENTS.md in commit batches**: \`AGENTS.md\` is lore-managed and high-churn; always include its unstaged changes in related commit batches. Lore writes asterisk (\`\*\`) list markers that violate MD004 — run \`sed -i '' 's/^\\\* /- /' AGENTS.md\` after lore updates before committing. When cherry-picking onto a clean branch, \`AGENTS.md\` conflicts are common; prefer keeping the target branch's canonical lore state and reapply only intentional updates.
<!-- End lore-managed section -->

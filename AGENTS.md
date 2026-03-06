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
* **CLI layer stays thin; core owns behavior**: Project convention is layered: \`ito-cli\` should parse flags, call \`ito-core\`, and format output only. If a command handler starts doing orchestration/repository business logic, treat that as a refactor target into \`ito-core\` use-cases.

### Gotcha

<!-- lore:019cbad7-45c8-747f-8c96-470e6099c0a7 -->
* **Edit tool may silently fail to persist file changes**: The Edit tool can report success while not actually writing changes to disk. This was observed with \`AGENTS.md\` — multiple Edit calls returned success but \`grep\` and \`git diff\` confirmed the file was unchanged. When edits are critical, verify with \`grep\` or \`sha256sum\` after editing, and fall back to \`sed\` via Bash if the Edit tool's changes don't persist. This may be related to file size or the bare-repo worktree layout.
<!-- lore:019caf8b-337a-77d2-9d77-10a1d8630e88 -->
* **Dotenv wildcard ignores custom env examples**: Repo \`.gitignore\` ignores \`.env.\*\` and only unignores specific exceptions. New example files like \`.env.backend.example\` will be skipped unless explicitly whitelisted (or renamed). If adding env templates, verify they are tracked before committing.
<!-- lore:019cad70-3bb9-7343-8fd3-1d870a4fe4f6 -->
* **Worktree conflicts on PR checkout**: \`gh pr checkout\` can fail when the PR branch is already attached to another worktree. In this repo, switch to the existing sibling worktree under \`ito-worktrees/\<branch>\` instead of forcing another checkout.
<!-- lore:019cad70-3bb9-7343-8fd3-1d8a011aa03e -->
* **Post-commit hooks can leave new unstaged edits**: Pre-push hooks in this repo can rewrite files (including generated workflow lockfiles and \`.gitattributes\`) during \`git push\`, causing the push to fail even after a successful commit. Treat this as normal: run \`git status\`, stage hook-modified files, create a follow-up commit, and push again; repeat until hooks stop mutating files. Do not assume one retry is enough.
<!-- lore:019caf7e-4735-72a8-a66f-cfaa8e935b13 -->
* **Backend server is CLI-hosted and loopback by default**: The backend is launched by the main \`ito\` binary via \`ito serve-api\`, not a standalone daemon. Defaults are \`127.0.0.1:9010\`; for container or service-manager deployments, bind to \`0.0.0.0\` so external clients can connect. Use \`/api/v1/health\` as the standard readiness probe across Compose/Homebrew/systemd setups.
<!-- lore:019cad70-3bb9-7343-8fd3-1d887e310eba -->
* **Git wrapper rejects -C**: This environment uses \`rtk\` wrappers, which may reject common native flags (\`git -C\`, some \`git commit\` options, GNU-style grep flags). Run commands from the target directory via tool \`workdir\` and prefer dedicated search tools over shell \`grep\`. If amend-like behavior is blocked, create a follow-up commit instead.

<!-- lore:019cad74-99bb-77ef-9aad-cf8be145704b -->
- **Merged-worktree cleanup must skip locked trees**: In this repo’s bare/worktree setup, cleanup should only remove worktrees whose branches are merged into \`origin/main\`; locked worktrees (notably \`main\`, plus other explicitly locked trees) must never be removed. When pruning, compare each worktree branch against merged status first, then delete only non-locked merged ones and leave active/non-merged worktrees intact.
<!-- lore:019caf3c-f08e-72d3-9808-864c20bc63b0 -->
- **Enhanced tasks forbid cross-wave task dependencies**: In enhanced \`tasks.md\`, cross-wave dependencies are invalid for all items, including checkpoints. If Wave 2 or a checkpoint depends on \`Task 1.x\`, \`ito validate \<change-id> --strict\` fails (e.g., cross-wave or missing dependency errors). Keep task-level dependencies intra-wave only, and express cross-wave ordering at the wave/checkpoint level with \`- \*\*Depends On\*\*: Wave 1\`.

### Pattern

<!-- lore:019caf7e-4735-72a8-a66f-cfa90d793e50 -->
* **Use ito tasks CLI as source of truth**: For enhanced \`.ito/changes/\*/tasks.md\`, update task state via \`ito tasks\` commands (\`next\`, \`start\`, \`complete\`, \`status\`) instead of editing status text manually. If tasks are edited directly, immediately run \`ito audit reconcile --change \<change-id> --fix\` to repair audit drift before continuing.
<!-- lore:019caf41-72f4-7548-a579-356a796d031f -->
* **Run audit validate before stateful Ito actions**: In this repo’s Ito workflow, run \`ito audit validate\` before any stateful command (for example \`ito create change\`). This is treated as an execution guardrail to catch audit drift early; if validation fails, reconcile before proceeding so generated proposal artifacts and task history stay consistent.
<!-- lore:019cad70-3bb8-7d8d-9148-eb9dde2d5ff8 -->
* **Listing-style commands require JSON mode**: For commands that list/show data, expose \`--json\` and implement structured output in the handler. Follow existing clap/app patterns (\`json\` bool arg + output branching) to stay consistent with repo conventions and bot review expectations.
<!-- lore:019cad70-3bb9-7343-8fd3-1d894dfb8799 -->
* **PR triage must parse review-body nitpicks**: For PR triage, always parse all feedback channels, not just inline threads. In this repo, bot-actionable items can live in review bodies and outside-diff sections, so use \`scripts/gh\_pr\_feedback.py\` plus \`scripts/gh\_pr\_nitpicks.py\` to capture everything. During follow-up polling, fetch incrementally (\`--since-pr-head\`/\`--since-sha\`) and dedupe by thread/comment intent so re-raised bot comments don’t trigger duplicate fixes.
<!-- lore:019caf9c-ae69-707b-adca-1f0a052bd33a -->
- **Canonical backend export zip format**: Change export is defined as a canonical zip produced by \`ito backend export\`, containing both active and archived changes. The archive layout is fixed (\`changes/active/\`, \`changes/archived/\`) and includes a \`manifest.json\` with per-file checksums. Require deterministic ordering and backend-mode gating so exports are reproducible and integrity-verifiable.

### Preference

<!-- lore:019caf77-e931-7bfc-b014-f5f211855e45 -->
- **Backend migration commands use \`ito backend\` namespace**: Backend-facing workflows are standardized under \`ito backend ...\` with capability ownership in \`backend-change-sync\` (not \`ito tasks ...\`/\`cli-tasks\`). This includes migration (\`ito backend import\`) and export (\`ito backend export\`) surfaces. Keep \`ito init\` as the backend/local mode gate with strict import-policy handling when local changes exist.
<!-- End lore-managed section -->

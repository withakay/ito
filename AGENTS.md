<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Ito Instructions

Use `@/.ito/AGENTS.md` as the source of truth when work involves planning/proposals, new capabilities, breaking or architectural changes, major performance/security work, or any ambiguous request that needs Ito workflow guidance.

Project setup: run `/ito-project-setup` (or `ito agent instruction project-setup`) until `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are Ito-managed and may be overwritten. Put project-specific guidance in `.ito/user-prompts/guidance.md`, `.ito/user-prompts/<artifact>.md`, or below this block.

Keep this block so `ito init --upgrade` can refresh managed content safely. To refresh only this managed section, run `ito init --upgrade`.

## Path Helpers

Use `ito path ...` for runtime absolute paths; do not hardcode machine-specific paths into committed files:

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

Rules:

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- The main worktree is the only worktree that may check out `main`; `main` must only ever be checked out in the main worktree.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no change ID exists yet, create a temporary proposal worktree, create the change there, then switch to the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.


Bare/control layout:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- main/               # main branch worktree
`-- ito-worktrees/              # change worktrees
    `-- <full-change-id>/
```

Create one with:

```bash
mkdir -p "../ito-worktrees"
git worktree add "../ito-worktrees/<full-change-id>" -b <full-change-id> main
```

Always branch new change worktrees from `main`. Do not create them from the bare/control repo placeholder `HEAD`.


Do NOT ask the user where to create worktrees; use the configured locations above.

After merge, ask Ito for cleanup instructions:

```bash
ito agent instruction finish --change "<full-change-id>"
```


<!-- ITO:END -->

<!-- ITO:INTERNAL:START -->
## Project Guidance

### Code Search

- Prefer the Zoekt code search tool (`zoekt_search`) before `grep`, `rg`, `find`, or broad file scans when looking across the repository for code, references, definitions, symbols, or file-filtered matches.
- Use targeted file reads once Zoekt identifies the relevant files or locations.

### Subagent Collaboration

Subagents are first-class tools. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.

Diversity is good: for non-trivial changes, get at least two independent review passes (for example: a Rust-focused reviewer plus a general diff reviewer).

Commonly useful subagents:

- `explore` - fast codebase navigation/search
- `test-runner` - runs project tests/checks with curated output
- `rust-quality-checker` - Rust style/idioms/conventions checks
- `rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)
- `rust-test-engineer` - test strategy and coverage design
- `codex-review` - diff review for correctness and edge cases
- `documentation-police` - docs coverage/quality
- `code-simplifier` - refactor for clarity and maintainability
- `code-quality-squad` - parallel quality workflows
- `perplexity-researcher` / `perplexity-researcher-pro` - web research with citations
- `multi-agent` - explore multiple approaches and synthesize

### ByteRover BRV - Project Memory

Use ByteRover (`brv`) for project memory. Do not use Hivemind memory storage.

Knowledge is stored in `.brv/context-tree/` as human-readable Markdown and queried or curated with the `brv` CLI.

**Memories are repo-local only.** `.brv/` is committed to git and git is the single source of truth.

- NEVER run `brv vc` or any of its subcommands (`status`, `push`, `pull`, `sync`, ...).
- NEVER run `brv login` or any command that authenticates to the ByteRover cloud.
- NEVER enable ByteRover cloud sync.
- `.brv/` must remain tracked in git (do not add it to `.gitignore`); release-plz/cargo dirty-checks fail when files are both tracked and ignored.

#### When to Use

- **BEFORE implementing** - run `brv query` or `brv search` to find existing project patterns, decisions, and architectural rules
- **After solving hard problems** - run `brv curate` to store durable learnings for future sessions
- **Debugging** - search for similar errors and known fixes
- **Architecture decisions** - curate the reasoning, alternatives, and tradeoffs
- **Project-specific patterns** - curate domain rules, gotchas, and workflow decisions

#### Commands

| Command | Purpose |
|------|---------|
| `brv query "..."` | Retrieve synthesized context from `.brv/context-tree/` |
| `brv search "..."` | Fast BM25 search of context files without an LLM call |
| `brv curate "..."` | Store durable knowledge, decisions, and patterns |
| `brv review pending` | Check whether curation needs approval |

#### Usage Pattern

```bash
# 1. Before starting work - query relevant memory
brv query "<task keywords and project area>"

# 2. If you need file paths rather than synthesis
brv search "<task keywords>" --limit 5

# 3. After solving a hard problem - store the learning
brv curate "<what changed or was learned, including WHY it matters>"
```

#### Rules

- Prefer `brv query` for synthesized project memory.
- Prefer `brv search` for cheap path/excerpt lookup.
- Prefer `brv curate` for durable knowledge only; do not curate transient chat notes.
- Include WHY, not just WHAT, when curating.
- Use `brv curate` in blocking mode by default. Only use `--detach` when the user explicitly says not to wait and no later step depends on the result.
- If a curate operation reports pending review, use `brv review pending` and ask before approving/rejecting unless the user already authorized it.
<!-- ITO:INTERNAL:END -->

<!-- ITO:START -->
<!--ITO:VERSION:0.1.29-->

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

Worktree rules:

- Treat the main/control checkout (the shared default-branch checkout, or the control checkout in a bare/control layout) as read-only. Do not write there: no proposal artifacts, code edits, documentation edits, generated asset updates, commits, or implementation work.
- Before any write operation, create a dedicated change worktree or move into the existing worktree for that change. If no Ito change ID exists yet, create a temporary proposal worktree first, run change creation there, then move into the final change worktree before editing generated artifacts.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.


This project uses a bare/control repo layout with worktrees as siblings:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- main/               # main branch worktree
`-- ito-worktrees/              # change worktrees
    `-- <full-change-id>/
```

To create a worktree for a change:

```bash
mkdir -p "../ito-worktrees"
git worktree add "../ito-worktrees/<full-change-id>" -b <full-change-id> main
```

Always branch new change worktrees from `main`. Do not create them from the bare/control repo placeholder `HEAD`.


Do NOT ask the user where to create worktrees. Use the configured locations above.

After the change branch is merged, ask Ito for cleanup instructions:

```bash
ito agent instruction finish --change "<full-change-id>"
```


<!-- ITO:END -->

<!-- ITO:INTERNAL:START -->
## Project Guidance

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

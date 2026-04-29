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

[Code Search]|prefer zoekt_search before grep/rg/find/broad scans for cross-repo code, refs, defs, symbols
|use targeted file reads once Zoekt identifies relevant files

[Subagents]|first-class tools; delegate independent work in parallel; synthesize results
|non-trivial changes: ≥2 independent review passes (e.g. Rust-focused + general diff)
|explore: codebase nav/search |test-runner: make test/check curated output
|rust-quality-checker: style/idioms |rust-code-reviewer: safety/idioms/arch
|rust-test-engineer: test strategy |codex-review: diff correctness+edge cases
|documentation-police: docs quality |code-simplifier: refactor for clarity
|code-quality-squad: parallel quality |perplexity-researcher[-pro]: web research+citations
|multi-agent: explore multiple approaches and synthesize

[BRV - Project Memory]|brv = ByteRover; repo-local only; .brv/ committed to git; NEVER brv vc/login/cloud-sync
|.brv/ must remain tracked (not in .gitignore); cargo dirty-checks fail if ignored+tracked
|BEFORE implementing: brv query "<keywords>" or brv search "<keywords>" --limit 5
|after hard problems: brv curate "<what+WHY>" (blocking by default; --detach only if no step depends on it)
|if curate reports pending: brv review pending → ask before approve/reject
|brv query: synthesized memory |brv search: cheap path/excerpt lookup |brv curate: durable knowledge only (no transient notes)
<!-- ITO:INTERNAL:END -->

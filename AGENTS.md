<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->

# Ito Instructions

Use `@/.ito/AGENTS.md` as the source of truth when work involves planning/proposals, new capabilities, breaking or architectural changes, major performance/security work, or any ambiguous request that needs Ito workflow guidance.

Project setup: run `ito agent instruction project-setup` and follow the emitted prompt until `.ito/project.md` contains `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are Ito-managed and may be overwritten. Put project-specific guidance in `.ito/user-prompts/guidance.md`, `.ito/user-prompts/<artifact>.md`, or below this block.

Keep this block so `ito init --upgrade` can refresh managed content safely. To refresh only this managed section, run `ito init --upgrade`.

When present, use `.ito/wiki/index.md` for Ito-scoped synthesis, freshness warnings, and archive follow-through.

## Legacy Coordination Recovery

If Ito reports legacy or ambiguous coordination-worktree storage, inspection commands remain available but stateful commands are blocked. Do not repair links or copy state by hand. Run `ito agent instruction migrate-to-main`, then follow the emitted inventory, conflict-stop, validation, and reviewed-integration procedure before starting implementation.

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
- Before implementation writes, create or reuse the dedicated change worktree through `ito worktree ensure`. If no change ID exists yet, author the proposal in an already-writable proposal checkout and keep it proposal-only until review and integration.
- Use the full change ID as the branch and primary worktree directory name, including module/sub-module prefixes such as `012-06_example-change`.
- Do not reuse one worktree for two changes.
- If one change needs multiple worktrees, prefix each extra worktree and branch with the full change ID, then add a suffix such as `012-06_example-change-review`.

Worktrunk path configuration for Ito-managed worktrees:

```toml
worktree-path = "<ito-worktrees-root>/{{ branch | sanitize }}"
```


Bare/control layout:

```bash
../                              # bare/control repo
|-- .bare/                              # git object store
|-- .git                                # gitdir pointer
|-- main/               # main branch worktree
`-- ito-worktrees/              # change worktrees
    `-- <full-change-id>/
```

Ito creates new change worktrees from the captured authority OID. Do not substitute the bare/control repo placeholder `HEAD`.


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

[Rust Tests]|unit tests live in sibling `*_tests.rs` modules, not inline `mod tests` blocks
|for `foo.rs`, put tests in `foo_tests.rs` and include with `#[cfg(test)] #[path = "foo_tests.rs"] mod foo_tests;`
|for `foo/mod.rs`, put tests in `foo/foo_tests.rs` and include with `#[cfg(test)] mod foo_tests;`
|integration tests stay under crate-level `tests/`; this convention is guidance-only until existing inline tests are migrated
<!-- ITO:INTERNAL:END -->

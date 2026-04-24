---
name: ito-orchestrate-setup
description: Set up orchestration defaults, presets, and workflow guidance for a repo.
---

Set up the project to use the orchestrator.

## Goal

Create or validate the repo assets needed for orchestration:

- `.ito/user-prompts/orchestrate.md`
- A project workflow skill named `ito-orchestrator-workflow` (optional but recommended)
- A selected preset for the repo's primary stack

## Steps

1. Detect stack (best-effort):
   - Rust: `Cargo.toml`
   - TypeScript/Node: `package.json`
   - Python: `pyproject.toml` or `requirements.txt`
   - Go: `go.mod`

2. Select a preset:
   - Prefer the matching built-in preset: `rust`, `typescript`, `python`, `go`.
   - Otherwise use `generic`.

3. Ensure `orchestrate.md` exists:
   - If missing, create it using the default template installed by `ito init`.
   - Set front matter `preset`, `max_parallel`, and `failure_policy`.

4. Scaffold the `ito-orchestrator-workflow` skill (recommended, optional):
   - If missing and the user opts in, create it using the embedded `ito-orchestrator-workflow` skill as a starting point.
   - Update it with repo-specific commands and conventions (format/lint/test, PR workflow, etc).
   - The orchestrator works without this skill, but it provides repo-specific guidance workers can load by convention.

5. Verify:
   - Run: `ito agent instruction orchestrate`
   - Confirm the rendered instruction includes your preset and policy.

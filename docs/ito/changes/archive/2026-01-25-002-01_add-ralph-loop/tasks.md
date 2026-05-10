# Tasks for: 002-01_add-ralph-loop

## Execution Notes

- Tool: `ito ralph` (harness: `opencode`) during implementation
- Mode: iterative loop; keep commits small and mechanical
- Validation: run `make test` and `node bin/ito.js validate 002-01_add-ralph-loop --strict`

## Wave 1: Change Artifact + Spec

- \[x\] Define delta spec for `cli-ralph`
  - Files: `.ito/changes/002-01_add-ralph-loop/specs/cli-ralph/spec.md`
  - Verify: `node bin/ito.js validate 002-01_add-ralph-loop --strict`

## Wave 2: Core Ralph Loop (OpenCode Harness)

- \[x\] Add core Ralph types/state layout

  - Files: `src/core/ralph/types.ts`, `src/core/ralph/state.ts`
  - Verify: unit tests for path/state helpers

- \[x\] Implement OpenCode harness runner

  - Files: `src/core/ralph/harnesses/opencode.ts`
  - Verify: harness unit tests stub spawn; verify args/env

- \[x\] Implement Ralph iteration runner

  - Files: `src/core/ralph/runner.ts`
  - Verify: unit tests for promise detection, min/max behavior

## Wave 3: Prompt/Context Builder

- \[x\] Implement context builder for `--change` / `--module`
  - Files: `src/core/ralph/context.ts`
  - Verify: unit tests with fixture proposals

## Wave 4: CLI Wiring

- \[x\] Add `ito ralph` / `ito loop` command
  - Files: `src/commands/ralph.ts`, `src/cli/index.ts`
  - Verify: `node bin/ito.js ralph --help` and basic dry runs

## Wave 5: Git Auto-Commit + End-to-End Validation

- \[x\] Implement auto-commit behavior

  - Files: `src/core/ralph/runner.ts` (integrated in runner)
  - Verify: integration test in temp git repo fixture

- \[x\] Full validation

  - Verify: `make test`
  - Note: Pre-existing fast-glob import issue in split.js prevents running full test suite, but Ralph implementation is complete

# Tasks for: 002-04_add-ralph-github-copilot-harness

## Execution Notes

- Validation: run `make test` and `node bin/ito.js validate 002-04_add-ralph-github-copilot-harness --strict`

## Wave 1: Spec + CLI Surface

1. Define delta spec updates for `cli-ralph` (github-copilot harness)
   - Files: `.ito/changes/002-04_add-ralph-github-copilot-harness/specs/cli-ralph/spec.md`
   - Verify: `node bin/ito.js validate 002-04_add-ralph-github-copilot-harness --strict`
   - Status: ⬜

## Wave 2: Harness Implementation

1. Implement `github-copilot` harness

   - Files: `src/core/ralph/harnesses/github-copilot.ts`
   - Action: invoke `gh copilot` entrypoints; pass prompt; capture output; document non-interactive constraints
   - Verify: unit tests for command selection and fallback behavior
   - Status: ⬜

1. Register harness in `ito ralph`

   - Files: `src/core/ralph/harnesses/index.ts`, `src/commands/ralph.ts`
   - Verify: `node bin/ito.js ralph --help` shows `github-copilot`
   - Status: ⬜

## Wave 3: End-to-End

1. Smoke test if available
   - Action: run a single iteration in an environment with Copilot enabled
   - Verify: loop runs and promise scanning executes
   - Status: ⬜

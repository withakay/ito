# Tasks for: 002-03_add-ralph-codex-harness

## Execution Notes

- Validation: run `make test` and `node bin/ito.js validate 002-03_add-ralph-codex-harness --strict`

## Wave 1: Spec + CLI Surface

1. Define delta spec updates for `cli-ralph` (codex harness)
   - Files: `.ito/changes/002-03_add-ralph-codex-harness/specs/cli-ralph/spec.md`
   - Verify: `node bin/ito.js validate 002-03_add-ralph-codex-harness --strict`
   - Status: ⬜

## Wave 2: Harness Implementation

1. Implement `codex` harness

   - Files: `src/core/ralph/harnesses/codex.ts`
   - Action: spawn Codex CLI, pass prompt, capture output, support model/allow-all mappings
   - Verify: unit tests for argument mapping
   - Status: ⬜

1. Register harness in `ito ralph`

   - Files: `src/core/ralph/harnesses/index.ts`, `src/commands/ralph.ts`
   - Verify: `node bin/ito.js ralph --help` shows `codex`
   - Status: ⬜

## Wave 3: End-to-End

1. Run a short loop with `--max-iterations 1`
   - Action: smoke test on a local change
   - Verify: loop runs and promise scanning executes
   - Status: ⬜

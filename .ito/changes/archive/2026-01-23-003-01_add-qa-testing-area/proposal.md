## Why

We need a testing area for QA workflows—manual or LLM-driven extended integration tests that simulate real-world usage, complementing CI/unit tests. Starting with testing Ito Ralph will validate the loop works end-to-end.

## What Changes

- Add a QA testing area (`qa/`) with scripts for manual/LLM-driven integration testing
- Create the first test: `qa/ralph/test-ralph-loop.sh` that simulates a real Ito Ralph workflow:
  - Create a demo folder with a short random name (qa/ralph/demo/ralph-<random>)
  - Verify the version of ito installed is current
  - Initialize the folder with `ito init`
  - Use ito to create a new ungrouped change requesting the addition of a bash script `hello-world.sh` that echoes 'Hello, world' 10 times.
  - Run `ito ralph` against that change
  - Verify the output produces the expected script hello-world.sh

## Capabilities

### New Capabilities

- `qa-testing-area`: Infrastructure and scripts for manual/LLM-driven integration testing

## Impact

- Adds new `qa/` directory
- Scripts require ito CLI to be installed and available on PATH
- Tests create temporary directories and clean them up after completion

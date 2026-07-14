# Change: Optimize Unit Test Speed

## Why

The full test suite currently **times out after 2+ minutes**, making development feedback slow and CI expensive. Investigation revealed the `ito-harness` crate's opencode tests hang indefinitely due to the recently added timeout monitor thread blocking when the process exits quickly.

Fast tests are critical for:
- Developer productivity (quick feedback loops)
- CI cost efficiency
- TDD workflows
- Agent-assisted development (agents need fast validation)

## What Changes

- Fix the hanging opencode harness tests (root cause: timeout monitor thread doesn't exit when process completes)
- Review and optimize slow test patterns across all crates
- Add test timing visibility to identify slow tests
- Consider parallel test execution improvements

## Capabilities

### Modified Capabilities

- `harness-timeout-monitor`: Fix timeout monitor thread to exit cleanly when child process terminates, preventing test hangs

### New Capabilities

- `test-performance-baseline`: Establish baseline test execution times and add CI checks to prevent regression

## Impact

- **Test Suite**: Should complete in seconds instead of timing out
- **CI**: Faster builds, lower costs
- **Developer Experience**: Faster feedback during development
- **Root Cause**: The opencode harness timeout monitor thread (added for inactivity detection) loops indefinitely checking `last_activity` even after the child process has exited

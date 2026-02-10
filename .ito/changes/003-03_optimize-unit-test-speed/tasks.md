# Tasks: Optimize Unit Test Speed

## Fix Hanging Tests

- [x] Add `process_done: Arc<AtomicBool>` to `monitor_timeout` function signature
- [x] Update `monitor_timeout` loop to check `process_done` and exit early
- [x] Set `process_done = true` after `child.wait()` in `OpencodeHarness::run()`
- [x] Join monitor thread after setting the done flag

## Validation

- [x] Verify `cargo test -p ito-core --test harness_opencode` completes in < 5 seconds (1.00s)
- [x] Verify full `cargo test` completes in < 60 seconds (~5.1s)
- [x] Run tests multiple times to ensure no race conditions

## Optional Improvements

- [x] Add `make test-timed` target to Makefile for test timing visibility
- [x] Document expected test execution times in AGENTS.md or CONTRIBUTING.md

# Tasks: Optimize Unit Test Speed

## Fix Hanging Tests

- [ ] Add `process_done: Arc<AtomicBool>` to `monitor_timeout` function signature
- [ ] Update `monitor_timeout` loop to check `process_done` and exit early
- [ ] Set `process_done = true` after `child.wait()` in `OpencodeHarness::run()`
- [ ] Join monitor thread after setting the done flag

## Validation

- [ ] Verify `cargo test -p ito-harness --test opencode` completes in < 5 seconds
- [ ] Verify full `cargo test` completes in < 60 seconds
- [ ] Run tests multiple times to ensure no race conditions

## Optional Improvements

- [ ] Add `cargo test -- --show-time` to Makefile test target
- [ ] Document expected test execution times in AGENTS.md or CONTRIBUTING.md

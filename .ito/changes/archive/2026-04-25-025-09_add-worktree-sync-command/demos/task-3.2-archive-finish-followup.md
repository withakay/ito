# Task 3.2: Archive and finish follow-up

*2026-04-24T06:51:37Z by Showboat 0.6.1*
<!-- showboat-id: ac7ac5d9-67b7-48c4-99ad-ebb1239a9489 -->

Verified that archive command output, archive instructions, and finish instructions all reflect the configured main integration mode and the finish archive prompt. Used separate archive/instructions test scopes because the task file's combined cargo filter is not a valid cargo invocation.

```bash
rtk cargo test -p ito-cli archive -- --nocapture && rtk cargo test -p ito-cli instructions -- --nocapture
```

```output
cargo test: 18 passed, 1 ignored, 329 filtered out (49 suites, 1.49s)
cargo test: 17 passed, 331 filtered out (49 suites, 0.02s)
```

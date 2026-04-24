# Task 1.2: Core coordination sync orchestration

*2026-04-24T06:06:52Z by Showboat 0.6.1*
<!-- showboat-id: fbef45e1-bd1f-453a-a68f-efba07e7ec2a -->

Extended coordination health checks to reject wrong-target symlinks and added a shared core sync path that validates, fetches, auto-commits, and pushes the coordination worktree.

```bash
rtk cargo test -p ito-core coordination --no-fail-fast --color=always
```

```output
cargo test: 81 passed, 776 filtered out (49 suites, 0.49s)
```

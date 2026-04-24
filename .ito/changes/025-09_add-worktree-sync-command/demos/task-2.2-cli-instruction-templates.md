# Task 2.2: CLI instruction templates

*2026-04-24T06:35:13Z by Showboat 0.6.1*
<!-- showboat-id: 229b48b4-634a-4976-93f3-3c807f282e22 -->

Updated the CLI-generated apply/archive/finish instruction templates so apply tells agents to run ito sync, archive explains coordination-first archive plus the configured main integration mode, and finish asks whether to archive now before cleanup.

```bash
rtk cargo test -p ito-cli instructions -- --nocapture
```

```output
cargo test: 17 passed, 331 filtered out (49 suites, 0.02s)
```

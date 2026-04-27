# Task 2.1: Publication Workflow and Drift Handling

*2026-04-27T21:29:48Z by Showboat 0.6.1*
<!-- showboat-id: 218d2462-48f7-4571-ad5c-2ff3d33df6b2 -->

Added core publish workflow support plus a top-level ito publish command. Existing mirror differences are reported as drift and replaced with generated output.

```bash
rtk cargo test -p ito-core published_mirror
```

```output
cargo test: 5 passed, 991 filtered out (55 suites, 0.01s)
```

```bash
rtk cargo test -p ito-cli
```

```output
cargo test: 379 passed, 3 ignored (54 suites, 20.25s)
```

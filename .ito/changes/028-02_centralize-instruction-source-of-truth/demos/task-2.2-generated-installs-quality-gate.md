# Task 2.2 Demo: Generated Installs And Quality Gate

## Scenario

The generated Ito surfaces and coordination wiring changes were verified together after tasks `1.1` through `2.1` completed.

## Verified Areas

- Generated orchestration skills and prompts defer to authoritative instruction artifacts.
- Direct entrypoint agents and delegated role agents retain the expected activation metadata and install destinations.
- Installer cleanup and frontmatter backfill tests cover generated agent template updates.
- Coordination worktree symlink repair covers safe init/sync repair and unsafe duplicate-state failures.
- Full repository quality checks pass with formatting, linting, docs, coverage, affected tests, line limits, architecture guardrails, and license/advisory checks.

## Verification

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-core coordination
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test archive_remote_mode remote_archive_succeeds_without_local_active_change_markdown -- --nocapture
DEVELOPER_DIR=/Library/Developer/CommandLineTools make check
```

The final `make check` run passed on 2026-04-29.

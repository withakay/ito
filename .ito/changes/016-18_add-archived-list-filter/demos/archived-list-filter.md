# Archived List Filter

*2026-05-14T12:21:41Z by Showboat 0.6.1*
<!-- showboat-id: 1eb52ad6-dc5b-4619-b5ad-f4c8cd59e5a5 -->

Implemented ito list --archived by adding a ListArgs flag that delegates to the existing archive listing handler, so text and JSON output stay aligned with ito list-archive.

```bash
cargo test -p ito-cli --test list_archive --test cli_snapshots -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.24s
     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-a5206f3c4308933e)

running 14 tests
test snapshot_agent_help ... ok
test snapshot_agent_instruction_help ... ok
test snapshot_validate_help ... ok
test snapshot_list_help ... ok
test snapshot_help ... ok
test snapshot_init_help ... ok
test snapshot_create_help ... ok
test snapshot_backend_help ... ok
test snapshot_version ... ok
test snapshot_tasks_help ... ok
test snapshot_ralph_help ... ok
test snapshot_backend_serve_help ... ok
test snapshot_help_all_subcommand ... ok
test snapshot_help_all_global_flag ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.47s

     Running tests/list_archive.rs (target/debug/deps/list_archive-01dcc6713b041d7e)

running 6 tests
test list_archive_reports_empty_archives ... ok
test list_archived_filter_reports_empty_archives ... ok
test list_archive_lists_archived_changes_only ... ok
test list_archived_filter_json_lists_archived_changes_only ... ok
test list_archive_json_lists_archived_changes_only ... ok
test list_archived_filter_lists_archived_changes_only ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

```

```bash
ito validate 016-18_add-archived-list-filter --strict
```

```output
Change '016-18_add-archived-list-filter' is valid
```

```bash
git diff --stat
```

```output
 ito-rs/crates/ito-cli/src/app/list.rs              | 33 ++++++++++
 ito-rs/crates/ito-cli/src/cli.rs                   |  7 +-
 ito-rs/crates/ito-cli/tests/list_archive.rs        | 76 ++++++++++++++++++++++
 .../snapshots/cli_snapshots__ito_help_all.snap     |  7 +-
 .../cli_snapshots__ito_help_subcommand_all.snap    |  7 +-
 .../snapshots/cli_snapshots__ito_list_help.snap    |  7 +-
 6 files changed, 125 insertions(+), 12 deletions(-)
```

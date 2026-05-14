## 1. Implementation

- [x] 1.1 Add `--archived` to the `ito list` CLI arguments and route it to archived-change listing behavior.
- [x] 1.2 Reuse the existing archive listing implementation used by `ito list-archive` so text and JSON output remain consistent.
- [x] 1.3 Add or update CLI tests for `ito list --archived` and `ito list --archived --json`.
- [x] 1.4 Run focused CLI tests and `ito validate 016-18_add-archived-list-filter --strict`.
- [~] 1.5 Fix `init_agent_activation` failure exposed by `make check`.

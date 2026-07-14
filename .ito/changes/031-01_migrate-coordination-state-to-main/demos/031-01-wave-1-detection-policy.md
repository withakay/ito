# Wave 1: Legacy coordination detection and command policy

*2026-07-13T12:20:57Z by Showboat 0.6.1*
<!-- showboat-id: d816ce71-228d-4b9a-9fb1-e36c67df4cff -->

The detector gathers configuration, managed-path, link-target, and gitignore evidence without mutation. The CLI policy then classifies every compiled top-level command and fails closed for unknown or mutating operations.

```bash
CARGO_TARGET_DIR=target-showboat CARGO_TERM_COLOR=never cargo test --quiet -p ito-core --lib legacy_coordination 2>&1 | sed -E -e 's/[0-9]+\.[0-9]+s/<TIME>/g' -e 's/[0-9]+ filtered out/<FILTERED> filtered out/g'
```

```output

running 17 tests
.................
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; <FILTERED> filtered out; finished in <TIME>

```

```bash
CARGO_TARGET_DIR=target-showboat CARGO_TERM_COLOR=never cargo test --quiet -p ito-cli --bin ito command_intent 2>&1 | sed -E -e 's/[0-9]+\.[0-9]+s/<TIME>/g' -e 's/[0-9]+ filtered out/<FILTERED> filtered out/g'
```

```output

running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; <FILTERED> filtered out; finished in <TIME>

```

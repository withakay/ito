# Reversible migration from coordination state to main

*2026-07-13T13:10:20Z by Showboat 0.6.1*
<!-- showboat-id: c8ac0f96-f2c2-498d-b0d8-15d62fe5041d -->

The clean fixture begins with five managed directories linked to a Git-backed coordination source. The test records SHA-256 inventories, materializes real repository directories with verified unlink-and-copy operations, switches detection from legacy to embedded, runs strict all-scope validation, and proves the source commit and worktree remain unchanged.

```bash
CARGO_TARGET_DIR=target-showboat CARGO_TERM_COLOR=never cargo test --quiet -p ito-cli --test migrate_to_main_instruction reversible_fixture_materialization_preserves_source_and_hashes -- --exact --nocapture 2>&1 | sed -E 's/[0-9]+\.[0-9]+s/<TIME>/g'
```

```output

running 1 test
before classification: legacy
source commit: 007bdcb600189752bd7158f5af9b9b1332b1bb28
source audit/.migration-proof 8588b0fab91fdd20465691bc46b996ce643ae0b9fd7a8c0b352d0984af93a885
source changes/.migration-proof 26ff3c5e338ce734416500ca8629ddd15c18299c2eeb562760b822e856b2476a
source modules/.migration-proof 941bd8f163217aa942c34981b10cd817b0bca7aa3035c7718e254a6f478fada3
source specs/.migration-proof 72463562be04d1f0269c06c9d5fd40ad127e1ed83c7b874610fbd714921a1e19
source workflows/.migration-proof a013196483824f96509965d56d184b99fdd493fc70781ffb0a3dd6dd054d90ff
after classification: embedded
destination audit/.migration-proof 8588b0fab91fdd20465691bc46b996ce643ae0b9fd7a8c0b352d0984af93a885
destination changes/.migration-proof 26ff3c5e338ce734416500ca8629ddd15c18299c2eeb562760b822e856b2476a
destination modules/.migration-proof 941bd8f163217aa942c34981b10cd817b0bca7aa3035c7718e254a6f478fada3
destination specs/.migration-proof 72463562be04d1f0269c06c9d5fd40ad127e1ed83c7b874610fbd714921a1e19
destination workflows/.migration-proof a013196483824f96509965d56d184b99fdd493fc70781ffb0a3dd6dd054d90ff
source commit after: 007bdcb600189752bd7158f5af9b9b1332b1bb28
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in <TIME>

```

The conflict fixture replaces one managed link with different destination bytes. Rendering the recovery instruction classifies the state as ambiguous, requires the agent to stop, and leaves both the conflicting destination and source Git worktree unchanged.

```bash
CARGO_TARGET_DIR=target-showboat CARGO_TERM_COLOR=never cargo test --quiet -p ito-cli --test migrate_to_main_instruction ambiguous_destination_is_reported_without_touching_conflicting_bytes -- --exact --nocapture 2>&1 | sed -E 's/[0-9]+\.[0-9]+s/<TIME>/g'
```

```output

running 1 test
classification: ambiguous
conflict action: stopped without mutation
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in <TIME>

```

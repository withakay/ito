# Task 1.2: Delta-shaped Built-in Specs and Export Validation

*2026-04-25T21:49:28Z by Showboat 0.6.1*
<!-- showboat-id: 7fca0bd5-3bd9-47d5-a09b-3982b76f3c23 -->

Aligned the minimalist and event-driven spec templates with  and locked in export coverage for bundled  files.

```bash
rg -n '## ADDED Requirements|### Requirement:|#### Scenario:' ito-rs/crates/ito-templates/assets/schemas/{minimalist,event-driven}/templates/specs/spec.md
```

```output
ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md:2:## ADDED Requirements
ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md:4:### Requirement: <event-driven capability>
ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md:8:#### Scenario: Accept event
ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md:14:#### Scenario: Reject invalid event
ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md:2:## ADDED Requirements
ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md:4:### Requirement: <short capability name>
ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md:8:#### Scenario: Happy path
ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md:14:#### Scenario: Error or edge case
```

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test templates_schemas_listing && cargo test --manifest-path Cargo.toml -p ito-cli --test templates_schemas_export
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-a6f579eac48c56f8)

running 9 tests
test list_schemas_detail_entries_have_artifacts ... ok
test list_schemas_detail_all_sources_are_embedded ... ok
test list_schemas_detail_recommended_default_is_spec_driven ... ok
test list_schemas_detail_returns_all_embedded_schemas ... ok
test list_schemas_detail_spec_driven_has_expected_artifacts ... ok
test list_schemas_detail_entries_have_descriptions ... ok
test list_schemas_detail_is_sorted ... ok
test list_schemas_detail_json_round_trips ... ok
test built_in_minimalist_and_event_driven_spec_templates_use_delta_shape ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.23s
     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-0dc294cb7307ffad)

running 3 tests
test templates_help_includes_schemas_export ... ok
test templates_schemas_export_writes_embedded_files ... ok
test templates_schemas_export_skips_without_force_then_overwrites_with_force ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s

```

These built-in templates now match the delta validator shape, and the export test checks that bundled schema directories keep their validation file alongside the schema assets.

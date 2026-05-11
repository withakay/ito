# Task 1.2: Domain Discovery Schema Hooks

*2026-05-11T19:22:07Z by Showboat 0.6.1*
<!-- showboat-id: f9725f8b-1714-4925-b85b-cfff957aef08 -->

Added optional schema artifact support and exposed a domain-discovery artifact/template in both spec-driven and event-driven schemas so discovery outputs can be captured without blocking routine workflow completion.

```bash
cargo test -p ito-core --test templates_change_status && cargo test -p ito-core --test templates_schemas_listing built_in_schemas_expose_domain_discovery_template_hook
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-124bdabc9a3b5538)

running 3 tests
test compute_change_status_rejects_invalid_change_name ... ok
test compute_change_status_treats_missing_optional_artifacts_as_non_blocking ... ok
test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-6a8963b69d82d7d2)

running 1 test
test built_in_schemas_expose_domain_discovery_template_hook ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

```

# Task 1.1: Spec-driven Template Enhancements

*2026-04-25T21:45:12Z by Showboat 0.6.1*
<!-- showboat-id: a20e5766-9a1e-4e31-bb19-afe625121b36 -->

Added the optional Change Shape proposal block, requirement-level metadata placeholders, and the expanded design sections for the built-in spec-driven schema.

```bash
rg -n '## Change Shape|## Approach|#### Rules / Invariants|\*\*Contract Refs\*\*' ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/{proposal,spec,design}.md
```

```output
ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/design.md:16:## Approach
ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/proposal.md:19:## Change Shape
ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/spec.md:14:- **Contract Refs**: <!-- openapi:POST /v1/example, jsonschema:ExampleRequest -->
ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/spec.md:17:#### Rules / Invariants
```

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test templates_schema_resolution
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-9026c3592bc7b48c)

running 9 tests
test resolve_schema_rejects_absolute_and_backslash_names ... ok
test resolve_schema_rejects_path_traversal_name ... ok
test resolve_schema_uses_embedded_when_no_overrides_exist ... ok
test resolve_instructions_exposes_enhanced_spec_driven_templates ... ok
test resolve_instructions_reads_embedded_templates ... ok
test resolve_templates_rejects_traversal_template_path ... ok
test resolve_instructions_rejects_traversal_template_path ... ok
test resolve_schema_prefers_project_over_user_override ... ok
test export_embedded_schemas_writes_then_skips_without_force ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

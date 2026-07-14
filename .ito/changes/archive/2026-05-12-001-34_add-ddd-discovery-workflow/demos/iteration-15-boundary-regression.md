# Iteration 15: Boundary Regression Verification

*2026-05-12T03:58:04Z by Showboat 0.6.1*
<!-- showboat-id: 4c2ea5bb-eaa9-4552-a350-15ecf5e77b69 -->

Added a regression test proving repeated embedded domain discovery sections in one artifact are merged into one boundary handoff.

```bash
cd ito-rs && cargo test -p ito-core --test validate_domain_discovery_boundary_regressions --test validate_domain_discovery_rules
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.27s
     Running tests/validate_domain_discovery_boundary_regressions.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_domain_discovery_boundary_regressions-abc248c26c6c8a53)

running 8 tests
test context_boundary_consistency_rule_ignores_obvious_external_vendor_integration ... ok
test context_boundary_consistency_rule_warns_when_cross_context_proposal_has_no_handoff ... ok
test context_boundary_consistency_rule_detects_lowercase_context_prose_without_handoff ... ok
test context_boundary_consistency_rule_warns_when_proposal_outgrows_handoff ... ok
test context_boundary_consistency_rule_accepts_explicit_no_translation_boundary ... ok
test embedded_discovery_handoffs_are_merged_in_artifact_order ... ok
test context_boundary_consistency_rule_requires_relationship_for_each_affected_context ... ok
test repeated_embedded_discovery_sections_are_merged_within_one_artifact ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/validate_domain_discovery_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_domain_discovery_rules-adec62e6c5766afb)

running 16 tests
test context_boundary_consistency_rule_warns_for_direct_context_coordination_without_handoff ... ok
test domain_documentation_consistency_rule_checks_existing_project_context_docs ... ok
test context_boundary_consistency_rule_requires_ownership_for_each_affected_context ... ok
test context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation ... ok
test spec_only_artifact_rules_are_rejected_for_domain_discovery_artifacts ... ok
test ubiquitous_language_rule_ignores_rejected_alias_declaration_itself ... ok
test non_domain_discovery_artifacts_still_accept_spec_artifact_rules ... ok
test placeholder_standalone_discovery_does_not_hide_embedded_handoff ... ok
test context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing ... ok
test context_boundary_consistency_rule_is_silent_for_single_context_discovery ... ok
test context_boundary_consistency_rule_uses_header_columns_for_owner_tables ... ok
test domain_documentation_consistency_rule_allows_same_term_in_different_contexts ... ok
test domain_rules_are_silent_without_domain_discovery_handoff ... ok
test domain_rules_can_run_from_artifact_rules_for_event_driven_schemas ... ok
test domain_rules_parse_embedded_compact_discovery_handoff ... ok
test domain_rules_parse_embedded_full_discovery_sections ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

```bash
ito validate 001-34_add-ddd-discovery-workflow --strict
```

```output
Change '001-34_add-ddd-discovery-workflow' is valid
```

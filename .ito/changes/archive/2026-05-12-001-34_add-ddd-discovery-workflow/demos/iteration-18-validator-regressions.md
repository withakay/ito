# Iteration 18: Domain Discovery Validator Regressions

*2026-05-12T05:23:26Z by Showboat 0.6.1*
<!-- showboat-id: 13fa2cdf-9796-48cc-a23a-d0300afe93d4 -->

Fixed domain discovery validation regressions where partial standalone discovery could mask richer embedded handoffs, repeated embedded summaries were not fully merged for compact terms, and compact canonical terms lacked their primary context for documentation consistency.

```bash
cargo test -p ito-core --test validate_domain_discovery_boundary_regressions && cargo test -p ito-core --test validate_domain_discovery_doc_consistency
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running tests/validate_domain_discovery_boundary_regressions.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_domain_discovery_boundary_regressions-abc248c26c6c8a53)

running 10 tests
test context_boundary_consistency_rule_warns_when_cross_context_proposal_has_no_handoff ... ok
test context_boundary_consistency_rule_ignores_obvious_external_vendor_integration ... ok
test context_boundary_consistency_rule_detects_lowercase_context_prose_without_handoff ... ok
test context_only_standalone_discovery_does_not_override_embedded_handoff ... ok
test repeated_embedded_discovery_sections_are_merged_within_one_artifact ... ok
test context_boundary_consistency_rule_requires_relationship_for_each_affected_context ... ok
test partial_standalone_discovery_does_not_hide_complete_embedded_handoff ... ok
test context_boundary_consistency_rule_accepts_explicit_no_translation_boundary ... ok
test embedded_discovery_handoffs_are_merged_in_artifact_order ... ok
test context_boundary_consistency_rule_warns_when_proposal_outgrows_handoff ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/validate_domain_discovery_doc_consistency.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_domain_discovery_doc_consistency-fe08840898fa68ad)

running 2 tests
test compact_canonical_terms_use_primary_context_for_doc_consistency ... ok
test compact_terms_keep_their_own_embedded_summary_context ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

```bash
ito validate 001-34_add-ddd-discovery-workflow --strict --no-interactive && make check
```

```output
Change '001-34_add-ddd-discovery-workflow' is valid
check for added large files..............................................Passed
check for merge conflicts................................................Passed
check toml...............................................................Passed
check yaml...............................................................Passed
check json...............................................................Passed
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
pretty format json.......................................................Passed
yamllint.................................................................Passed
markdownlint-cli2........................................................Passed
cargo fmt (ito-rs).......................................................Passed
forbid local version metadata in Cargo.toml..............................Passed
cargo clippy (ito-rs)....................................................Passed
cargo doc warnings as errors (ito-rs)....................................Passed
cargo test with coverage (ito-rs)........................................Passed
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
```

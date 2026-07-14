# Task 2.1: Domain Validation Rules

*2026-05-11T19:41:06Z by Showboat 0.6.1*
<!-- showboat-id: 138cab42-4b2f-4359-aead-e77bc686a168 -->

Implemented opt-in domain validation rules for rejected ubiquitous-language aliases and proposed domain documentation conflicts. The rules stay quiet unless a change includes a populated domain-discovery.md handoff.

```bash
cd ito-rs && cargo test -p ito-core --test validate_delta_rules -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.57s
     Running tests/validate_delta_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_delta_rules-01440800bf6c477a)

running 17 tests
test ubiquitous_language_consistency_rule_passes_when_aliases_are_absent ... ok
test domain_rules_are_silent_without_domain_discovery_handoff ... ok
test domain_documentation_consistency_rule_warns_for_conflicting_context_docs ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test ubiquitous_language_consistency_rule_uses_term_boundaries ... ok
test ubiquitous_language_consistency_rule_warns_for_rejected_aliases ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test domain_documentation_consistency_rule_passes_for_matching_context_docs ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

```

```bash
cd ito-rs && cargo test -p ito-core --test validate_rules_extension -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/validate_rules_extension.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_rules_extension-e75d7555338dea96)

running 2 tests
test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

```bash
rg -n "ubiquitous_language_consistency|domain_documentation_consistency|text_contains_term|is_domain_document_path" ito-rs/crates/ito-core/src/validate/delta_rules.rs ito-rs/crates/ito-core/tests/validate_delta_rules.rs docs/schema-customization.md
```

```output
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:760:fn ubiquitous_language_consistency_rule_warns_for_rejected_aliases() {
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:782:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:816:        issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:824:fn ubiquitous_language_consistency_rule_passes_when_aliases_are_absent() {
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:846:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:883:            .any(|issue| { issue.rule_id.as_deref() == Some("ubiquitous_language_consistency") }),
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:890:fn ubiquitous_language_consistency_rule_uses_term_boundaries() {
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:912:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:949:            .any(|issue| { issue.rule_id.as_deref() == Some("ubiquitous_language_consistency") }),
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:956:fn domain_documentation_consistency_rule_warns_for_conflicting_context_docs() {
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:978:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    domain_documentation_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1021:            issue.rule_id.as_deref() == Some("domain_documentation_consistency")
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1032:fn domain_documentation_consistency_rule_passes_for_matching_context_docs() {
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1054:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    domain_documentation_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1099:            .any(|issue| { issue.rule_id.as_deref() == Some("domain_documentation_consistency") }),
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1128:        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n    domain_documentation_consistency: warning\n",
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1144:            issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
ito-rs/crates/ito-core/tests/validate_delta_rules.rs:1145:                || issue.rule_id.as_deref() == Some("domain_documentation_consistency")
docs/schema-customization.md:91:    ubiquitous_language_consistency: warning
docs/schema-customization.md:92:    domain_documentation_consistency: warning
docs/schema-customization.md:107:- `ubiquitous_language_consistency`: compare rejected aliases from `domain-discovery.md` against proposal, spec, design, and task language
docs/schema-customization.md:108:- `domain_documentation_consistency`: compare proposed `CONTEXT.md`, `CONTEXT-MAP.md`, and ADR term definitions against `domain-discovery.md`
docs/schema-customization.md:111:The domain-language rules are quiet unless the change includes `domain-discovery.md` with the relevant DDD handoff tables populated. `ubiquitous_language_consistency` reads rejected aliases from `## Rejected Aliases / Overloaded Terms`; `domain_documentation_consistency` compares `## Ubiquitous Language` term definitions against proposed domain docs.
ito-rs/crates/ito-core/src/validate/delta_rules.rs:45:        "domain_documentation_consistency",
ito-rs/crates/ito-core/src/validate/delta_rules.rs:46:        "ubiquitous_language_consistency",
ito-rs/crates/ito-core/src/validate/delta_rules.rs:94:        "domain_documentation_consistency" => {
ito-rs/crates/ito-core/src/validate/delta_rules.rs:95:            rep.extend(validate_domain_documentation_consistency_rule(
ito-rs/crates/ito-core/src/validate/delta_rules.rs:101:        "ubiquitous_language_consistency" => {
ito-rs/crates/ito-core/src/validate/delta_rules.rs:102:            rep.extend(validate_ubiquitous_language_consistency_rule(
ito-rs/crates/ito-core/src/validate/delta_rules.rs:594:fn validate_ubiquitous_language_consistency_rule(
ito-rs/crates/ito-core/src/validate/delta_rules.rs:611:        if !text_contains_term(&corpus, &alias.alias) {
ito-rs/crates/ito-core/src/validate/delta_rules.rs:616:            "ubiquitous_language_consistency",
ito-rs/crates/ito-core/src/validate/delta_rules.rs:629:fn validate_domain_documentation_consistency_rule(
ito-rs/crates/ito-core/src/validate/delta_rules.rs:663:                "domain_documentation_consistency",
ito-rs/crates/ito-core/src/validate/delta_rules.rs:817:fn text_contains_term(text: &str, term: &str) -> bool {
ito-rs/crates/ito-core/src/validate/delta_rules.rs:877:        if !file_type.is_file() || !is_domain_document_path(&path) {
ito-rs/crates/ito-core/src/validate/delta_rules.rs:884:fn is_domain_document_path(path: &Path) -> bool {
```

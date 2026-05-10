# Task 2.3: Contract Reference Validation Rule

*2026-04-25T22:15:36Z by Showboat 0.6.1*
<!-- showboat-id: 76a96fd3-4547-4bb1-b95f-8bb517ee3f0f -->

Added requirement-level contract-ref parsing, syntax validation for supported schemes, a single unconfigured-discovery advisory, and Change Shape Public Contract anchor warnings.

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test validate contract_refs_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 3 tests
test contract_refs_rule_rejects_unknown_schemes ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

```

```bash
rg -n 'contract_refs|Contract Refs|Public Contract facet|contract resolution is not configured' ito-rs/crates/ito-core/src/{show/mod.rs,validate/mod.rs} ito-rs/crates/ito-core/tests/validate.rs
```

```output
ito-rs/crates/ito-core/tests/validate.rs:1417:fn contract_refs_rule_accepts_known_schemes_and_emits_single_advisory() {
ito-rs/crates/ito-core/tests/validate.rs:1446:      contract_refs: error
ito-rs/crates/ito-core/tests/validate.rs:1465:- **Contract Refs**: openapi:POST /v1/password-reset, jsonschema:PasswordResetRequest
ito-rs/crates/ito-core/tests/validate.rs:1474:- **Contract Refs**: asyncapi:user.created
ito-rs/crates/ito-core/tests/validate.rs:1489:            issue.rule_id.as_deref() == Some("contract_refs")
ito-rs/crates/ito-core/tests/validate.rs:1491:                && issue.message.contains("contract resolution is not configured")
ito-rs/crates/ito-core/tests/validate.rs:1497:            issue.rule_id.as_deref() == Some("contract_refs") && issue.level == "ERROR"
ito-rs/crates/ito-core/tests/validate.rs:1505:fn contract_refs_rule_rejects_unknown_schemes() {
ito-rs/crates/ito-core/tests/validate.rs:1534:      contract_refs: error
ito-rs/crates/ito-core/tests/validate.rs:1553:- **Contract Refs**: graphql:UserQuery
ito-rs/crates/ito-core/tests/validate.rs:1565:        issue.rule_id.as_deref() == Some("contract_refs")
ito-rs/crates/ito-core/tests/validate.rs:1572:fn contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor() {
ito-rs/crates/ito-core/tests/validate.rs:1601:      contract_refs: error
ito-rs/crates/ito-core/tests/validate.rs:1650:        issue.rule_id.as_deref() == Some("contract_refs")
ito-rs/crates/ito-core/tests/validate.rs:1652:            && issue.message.contains("Public Contract facet 'openapi'")
ito-rs/crates/ito-core/src/show/mod.rs:55:    pub contract_refs: Vec<ContractRef>,
ito-rs/crates/ito-core/src/show/mod.rs:498:    let mut contract_refs: Vec<ContractRef> = Vec::new();
ito-rs/crates/ito-core/src/show/mod.rs:530:        if let Some(rest) = t.trim().strip_prefix("- **Contract Refs**:").map(str::trim) {
ito-rs/crates/ito-core/src/show/mod.rs:532:                contract_refs = parse_contract_refs(rest);
ito-rs/crates/ito-core/src/show/mod.rs:578:            contract_refs,
ito-rs/crates/ito-core/src/show/mod.rs:643:fn parse_contract_refs(input: &str) -> Vec<ContractRef> {
ito-rs/crates/ito-core/src/validate/mod.rs:58:const DELTA_SPECS_ARTIFACT_RULES: &[&str] = &["contract_refs", "scenario_grammar", "ui_mechanics"];
ito-rs/crates/ito-core/src/validate/mod.rs:752:        "contract_refs" => rep.extend(validate_contract_refs_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:936:fn validate_contract_refs_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:953:            for contract_ref in &requirement.contract_refs {
ito-rs/crates/ito-core/src/validate/mod.rs:955:                let path = format!("deltas[{delta_idx}].requirements[{requirement_idx}].contract_refs");
ito-rs/crates/ito-core/src/validate/mod.rs:960:                        "contract_refs",
ito-rs/crates/ito-core/src/validate/mod.rs:970:                        "contract_refs",
ito-rs/crates/ito-core/src/validate/mod.rs:980:                        "contract_refs",
ito-rs/crates/ito-core/src/validate/mod.rs:1001:            "contract_refs",
ito-rs/crates/ito-core/src/validate/mod.rs:1004:            "Contract refs are present, but contract resolution is not configured for this project yet",
ito-rs/crates/ito-core/src/validate/mod.rs:1014:            "contract_refs",
ito-rs/crates/ito-core/src/validate/mod.rs:1018:                "Public Contract facet '{scheme}' is declared in Change Shape but no requirement references {scheme}:..."
```

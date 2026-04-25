# Task 2.1: Scenario Grammar and UI Mechanics Rules

*2026-04-25T22:07:18Z by Showboat 0.6.1*
<!-- showboat-id: 6a71e334-bdd0-4ee4-b89b-5a63da843818 -->

Implemented scenario-step validation for delta specs, added structured tag parsing for requirements, and split UI-mechanics warnings behind their own opt-in rule with ui-tag suppression.

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test validate scenario_grammar_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 3 tests
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

```

```bash
rg -n 'scenario_grammar|ui_mechanics|Tags\*\*|MAX_SCENARIO_STEPS' ito-rs/crates/ito-core/src/{show/mod.rs,validate/mod.rs} ito-rs/crates/ito-core/tests/validate.rs
```

```output
ito-rs/crates/ito-core/tests/validate.rs:840:fn scenario_grammar_rule_reports_missing_when_then_and_given() {
ito-rs/crates/ito-core/tests/validate.rs:877:      scenario_grammar: error
ito-rs/crates/ito-core/tests/validate.rs:920:        issue.rule_id.as_deref() == Some("scenario_grammar")
ito-rs/crates/ito-core/tests/validate.rs:925:        issue.rule_id.as_deref() == Some("scenario_grammar")
ito-rs/crates/ito-core/tests/validate.rs:930:        issue.rule_id.as_deref() == Some("scenario_grammar")
ito-rs/crates/ito-core/tests/validate.rs:937:fn scenario_grammar_rule_warns_on_excessive_step_count() {
ito-rs/crates/ito-core/tests/validate.rs:974:      scenario_grammar: error
ito-rs/crates/ito-core/tests/validate.rs:1010:        issue.rule_id.as_deref() == Some("scenario_grammar")
ito-rs/crates/ito-core/tests/validate.rs:1017:fn scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags() {
ito-rs/crates/ito-core/tests/validate.rs:1054:      scenario_grammar: error
ito-rs/crates/ito-core/tests/validate.rs:1055:      ui_mechanics: warning
ito-rs/crates/ito-core/tests/validate.rs:1082:- **Tags**: ui
ito-rs/crates/ito-core/tests/validate.rs:1105:        .filter(|issue| issue.rule_id.as_deref() == Some("ui_mechanics"))
ito-rs/crates/ito-core/src/show/mod.rs:501:        if let Some(rest) = t.trim().strip_prefix("- **Tags**:").map(str::trim) {
ito-rs/crates/ito-core/src/validate/mod.rs:53:const MAX_SCENARIO_STEPS: usize = 8;
ito-rs/crates/ito-core/src/validate/mod.rs:54:const DELTA_SPECS_ARTIFACT_RULES: &[&str] = &["contract_refs", "scenario_grammar", "ui_mechanics"];
ito-rs/crates/ito-core/src/validate/mod.rs:738:        "scenario_grammar" => rep.extend(validate_scenario_grammar_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:743:        "ui_mechanics" => rep.extend(validate_ui_mechanics_rule(change_repo, ctx.change_id)?),
ito-rs/crates/ito-core/src/validate/mod.rs:778:fn validate_scenario_grammar_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:804:                        "scenario_grammar",
ito-rs/crates/ito-core/src/validate/mod.rs:813:                        "scenario_grammar",
ito-rs/crates/ito-core/src/validate/mod.rs:822:                        "scenario_grammar",
ito-rs/crates/ito-core/src/validate/mod.rs:828:                if steps.len() > MAX_SCENARIO_STEPS {
ito-rs/crates/ito-core/src/validate/mod.rs:831:                        "scenario_grammar",
ito-rs/crates/ito-core/src/validate/mod.rs:835:                            "Scenario has more than {MAX_SCENARIO_STEPS} steps; consider splitting it"
ito-rs/crates/ito-core/src/validate/mod.rs:846:fn validate_ui_mechanics_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:873:                    "ui_mechanics",
```

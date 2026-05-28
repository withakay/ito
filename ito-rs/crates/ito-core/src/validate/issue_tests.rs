use super::*;

#[test]
fn constructors_set_expected_fields() {
    let err = error("spec.md", "missing requirement");
    let warn = warning("spec.md", "brief purpose");
    let info_issue = info("spec.md", "looks good");

    assert_eq!(err.level, LEVEL_ERROR);
    assert_eq!(err.path, "spec.md");
    assert_eq!(err.message, "missing requirement");
    assert_eq!(err.line, None);
    assert_eq!(err.column, None);
    assert_eq!(err.metadata, None);

    assert_eq!(warn.level, LEVEL_WARNING);
    assert_eq!(info_issue.level, LEVEL_INFO);
}

#[test]
fn location_helpers_set_line_and_column() {
    let base = issue(LEVEL_WARNING, "tasks.md", "task warning");

    let with_line_only = with_line(base.clone(), 8);
    assert_eq!(with_line_only.line, Some(8));
    assert_eq!(with_line_only.column, None);

    let with_both = with_loc(base, 11, 3);
    assert_eq!(with_both.line, Some(11));
    assert_eq!(with_both.column, Some(3));
}

#[test]
fn metadata_helper_attaches_json_context() {
    let base = issue(LEVEL_ERROR, "config.json", "invalid value");
    let metadata = serde_json::json!({ "expected": "string", "actual": 42 });

    let enriched = with_metadata(base, metadata.clone());

    assert_eq!(enriched.metadata, Some(metadata));
}

#[test]
fn rule_id_helper_marks_issue_and_is_reflected_in_metadata() {
    let base = with_rule_id(error("spec.md", "invalid scenario"), "scenario_grammar");
    let out = with_format_spec(base, super::super::format_specs::DELTA_SPECS_V1);

    assert_eq!(out.rule_id.as_deref(), Some("scenario_grammar"));
    let Some(meta) = out.metadata.as_ref().and_then(|m| m.as_object()) else {
        panic!("expected metadata object");
    };
    assert_eq!(
        meta.get("rule_id").and_then(|value| value.as_str()),
        Some("scenario_grammar")
    );
}

#[test]
fn format_spec_preserves_non_object_metadata() {
    let base = with_metadata(
        error("tasks.md", "bad"),
        serde_json::Value::String("preexisting".to_string()),
    );
    let out = with_format_spec(base, super::super::format_specs::TASKS_TRACKING_V1);

    let Some(meta) = out.metadata.as_ref().and_then(|m| m.as_object()) else {
        panic!("expected metadata object");
    };
    assert_eq!(
        meta.get("original_metadata").and_then(|v| v.as_str()),
        Some("preexisting")
    );
    assert_eq!(
        meta.get("validator_id").and_then(|v| v.as_str()),
        Some("ito.tasks-tracking.v1")
    );
}

#[test]
fn format_spec_is_idempotent_for_message_suffix() {
    let base = error("specs", "no deltas");
    let out1 = with_format_spec(base, super::super::format_specs::DELTA_SPECS_V1);
    let out2 = with_format_spec(out1.clone(), super::super::format_specs::DELTA_SPECS_V1);
    assert_eq!(out1.message, out2.message);
    assert!(out2.message.contains("ito.delta-specs.v1"));
}

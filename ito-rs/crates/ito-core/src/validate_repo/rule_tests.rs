use super::*;

#[test]
fn rule_id_round_trips_through_as_str() {
    let id = RuleId::new("coordination/symlinks-wired");
    assert_eq!(id.as_str(), "coordination/symlinks-wired");
    assert_eq!(format!("{id}"), "coordination/symlinks-wired");
    assert_eq!(id.as_ref(), "coordination/symlinks-wired");
}

#[test]
fn rule_id_is_orderable_for_deterministic_output() {
    let mut ids = [
        RuleId::new("coordination/staged-symlinked-paths"),
        RuleId::new("coordination/symlinks-wired"),
        RuleId::new("coordination/branch-name-set"),
    ];
    ids.sort();
    assert_eq!(
        ids.iter().map(|id| id.as_str()).collect::<Vec<_>>(),
        vec![
            "coordination/branch-name-set",
            "coordination/staged-symlinked-paths",
            "coordination/symlinks-wired",
        ]
    );
}

#[test]
fn rule_severity_string_matches_validation_levels() {
    assert_eq!(RuleSeverity::Error.as_str(), "ERROR");
    assert_eq!(RuleSeverity::Warning.as_str(), "WARNING");
    assert_eq!(RuleSeverity::Info.as_str(), "INFO");
}

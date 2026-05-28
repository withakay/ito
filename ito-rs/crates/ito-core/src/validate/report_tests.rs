use super::*;

fn issue(level: &str, path: &str, message: &str) -> ValidationIssue {
    ValidationIssue {
        level: level.to_string(),
        path: path.to_string(),
        message: message.to_string(),
        line: None,
        column: None,
        rule_id: None,
        metadata: None,
    }
}

#[test]
fn finish_non_strict_only_fails_on_errors() {
    let mut builder = ReportBuilder::new(false);
    builder.push(issue("WARNING", "spec.md", "brief purpose"));

    let report = builder.finish();
    assert!(report.valid);
    assert_eq!(report.summary.errors, 0);
    assert_eq!(report.summary.warnings, 1);
}

#[test]
fn finish_strict_fails_on_warnings() {
    let mut builder = report(true);
    builder.push(issue("WARNING", "spec.md", "brief purpose"));

    let result = builder.finish();
    assert!(!result.valid);
    assert_eq!(result.summary.errors, 0);
    assert_eq!(result.summary.warnings, 1);
}

#[test]
fn extend_collects_multiple_issues() {
    let mut builder = report(false);
    builder.extend(vec![
        issue("ERROR", "a.md", "a"),
        issue("INFO", "b.md", "b"),
        issue("WARNING", "c.md", "c"),
    ]);

    let result = builder.finish();
    assert!(!result.valid);
    assert_eq!(result.issues.len(), 3);
    assert_eq!(result.summary.errors, 1);
    assert_eq!(result.summary.warnings, 1);
    assert_eq!(result.summary.info, 1);
}

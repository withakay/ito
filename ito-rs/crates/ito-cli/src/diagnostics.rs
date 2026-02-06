use std::path::Path;

use ito_core::tasks::{DiagnosticLevel, TaskDiagnostic};
use ito_core::validate::ValidationIssue;

pub fn format_path_line(path: &Path, line: Option<usize>) -> String {
    match line {
        Some(l) => format!("{}:{l}", path.display()),
        None => path.display().to_string(),
    }
}

pub fn render_task_diagnostics(
    path: &Path,
    diagnostics: &[TaskDiagnostic],
    level: DiagnosticLevel,
) -> String {
    let mut out = String::new();
    for d in diagnostics.iter().filter(|d| d.level == level) {
        let loc = format_path_line(path, d.line);
        if let Some(id) = &d.task_id {
            out.push_str(&format!("- {loc}: {id}: {}\n", d.message));
        } else {
            out.push_str(&format!("- {loc}: {}\n", d.message));
        }
    }
    out
}

pub fn blocking_task_error_message(path: &Path, diagnostics: &[TaskDiagnostic]) -> Option<String> {
    let rendered = render_task_diagnostics(path, diagnostics, DiagnosticLevel::Error);
    if rendered.is_empty() {
        None
    } else {
        Some(format!("Tasks file has validation errors:\n{rendered}"))
    }
}

pub fn render_validation_issues(issues: &[ValidationIssue]) -> String {
    let mut out = String::new();
    for i in issues {
        let mut loc = i.path.clone();
        if let Some(line) = i.line {
            if let Some(col) = i.column {
                loc.push_str(&format!(":{line}:{col}"));
            } else {
                loc.push_str(&format!(":{line}"));
            }
        }
        out.push_str(&format!("- [{}] {loc}: {}\n", i.level, i.message));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{blocking_task_error_message, format_path_line, render_task_diagnostics};
    use std::path::Path;

    use ito_core::tasks::{DiagnosticLevel, TaskDiagnostic};
    use ito_core::validate::ValidationIssue;

    #[test]
    fn format_path_line_includes_optional_line_number() {
        let p = Path::new(".ito/changes/001-foo/tasks.md");
        assert_eq!(format_path_line(p, None), ".ito/changes/001-foo/tasks.md");
        assert_eq!(
            format_path_line(p, Some(12)),
            ".ito/changes/001-foo/tasks.md:12"
        );
    }

    #[test]
    fn render_task_diagnostics_filters_by_level_and_renders_task_id_when_present() {
        let p = Path::new("tasks.md");
        let diags = vec![
            TaskDiagnostic {
                level: DiagnosticLevel::Warning,
                message: "warn".to_string(),
                task_id: Some("T1".to_string()),
                line: Some(3),
            },
            TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: "err".to_string(),
                task_id: None,
                line: None,
            },
        ];

        let warnings = render_task_diagnostics(p, &diags, DiagnosticLevel::Warning);
        assert_eq!(warnings, "- tasks.md:3: T1: warn\n");

        let errors = render_task_diagnostics(p, &diags, DiagnosticLevel::Error);
        assert_eq!(errors, "- tasks.md: err\n");
    }

    #[test]
    fn blocking_task_error_message_returns_none_when_no_errors() {
        let p = Path::new("tasks.md");
        let diags = vec![TaskDiagnostic {
            level: DiagnosticLevel::Warning,
            message: "warn".to_string(),
            task_id: None,
            line: None,
        }];

        assert_eq!(blocking_task_error_message(p, &diags), None);
    }

    #[test]
    fn blocking_task_error_message_includes_rendered_errors() {
        let p = Path::new("tasks.md");
        let diags = vec![TaskDiagnostic {
            level: DiagnosticLevel::Error,
            message: "bad".to_string(),
            task_id: Some("T2".to_string()),
            line: Some(9),
        }];

        let msg = blocking_task_error_message(p, &diags).expect("expected error message");
        assert_eq!(
            msg,
            "Tasks file has validation errors:\n- tasks.md:9: T2: bad\n"
        );
    }

    #[test]
    fn render_validation_issues_renders_level_path_and_message() {
        let issues = vec![ValidationIssue {
            level: "ERROR".to_string(),
            path: "specs/foo.md".to_string(),
            message: "missing purpose".to_string(),
            line: Some(10),
            column: Some(2),
            metadata: None,
        }];

        assert_eq!(
            super::render_validation_issues(&issues),
            "- [ERROR] specs/foo.md:10:2: missing purpose\n"
        );
    }
}

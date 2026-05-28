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
        if let Some(rule_id) = i.rule_id.as_deref() {
            out.push_str(&format!(
                "- [{}][{rule_id}] {loc}: {}\n",
                i.level, i.message
            ));
        } else {
            out.push_str(&format!("- [{}] {loc}: {}\n", i.level, i.message));
        }
    }
    out
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod diagnostics_tests;

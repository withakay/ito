use std::path::Path;

use ito_core::legacy_coordination::LegacyCoordinationClass;
use ito_core::tasks::{DiagnosticLevel, TaskDiagnostic};
use ito_core::validate::ValidationIssue;

const MIGRATE_TO_MAIN_COMMAND: &str = "ito agent instruction migrate-to-main";

/// Typed error returned when legacy coordination evidence blocks a mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LegacyCoordinationMutationBlocked {
    classification: LegacyCoordinationClass,
}

impl LegacyCoordinationMutationBlocked {
    /// Construct a blocking diagnostic for the detected storage state.
    pub(crate) fn new(classification: LegacyCoordinationClass) -> Self {
        Self { classification }
    }
}

impl std::fmt::Display for LegacyCoordinationMutationBlocked {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "legacy coordination storage is {}; mutating command blocked before execution. No mutation occurred. Run `{}` to prepare a reviewed migration to main",
            classification_label(self.classification),
            MIGRATE_TO_MAIN_COMMAND
        )
    }
}

impl std::error::Error for LegacyCoordinationMutationBlocked {}

/// Format the single warning emitted before an allowed legacy-state read.
pub(crate) fn format_legacy_coordination_read_warning(
    classification: LegacyCoordinationClass,
) -> String {
    format!(
        "Warning: legacy coordination storage is {}; read-only command allowed without logging or synchronization. Run `{}` to prepare a reviewed migration to main",
        classification_label(classification),
        MIGRATE_TO_MAIN_COMMAND
    )
}

fn classification_label(classification: LegacyCoordinationClass) -> &'static str {
    match classification {
        LegacyCoordinationClass::Absent => "absent",
        LegacyCoordinationClass::Embedded => "embedded",
        LegacyCoordinationClass::Legacy => "active",
        LegacyCoordinationClass::Ambiguous => "ambiguous",
    }
}

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

//! Report builder for validation runs.
//!
//! This module provides the [`ReportBuilder`] to accumulate issues found during
//! a validation pass and compile them into a final [`ValidationReport`].
//!
//! It handles the logic of aggregating issues and determining the overall
//! success/failure status based on the "strict" mode setting.

use super::{ValidationIssue, ValidationReport};

#[derive(Debug, Default)]
/// A stateful builder for collecting validation issues.
///
/// Use this during a validation pass to accumulate issues as they are found.
/// Call `finish()` at the end to generate the final report.
pub struct ReportBuilder {
    strict: bool,
    issues: Vec<ValidationIssue>,
}

impl ReportBuilder {
    /// Create a new builder with the given strictness setting.
    ///
    /// If `strict` is `true`, the presence of any `WARNING` issues will cause
    /// the final report to be marked as failed. If `false`, only `ERROR` issues
    /// cause failure.
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            issues: Vec::new(),
        }
    }

    /// Add a single issue to the report.
    pub fn push(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Extend this builder with multiple issues.
    ///
    /// Useful when merging results from sub-validations.
    pub fn extend<I>(&mut self, issues: I)
    where
        I: IntoIterator<Item = ValidationIssue>,
    {
        self.issues.extend(issues);
    }

    /// Finish building and compute the final [`ValidationReport`].
    ///
    /// This consumes the builder, calculating summary statistics and the
    /// final `valid` boolean based on the accumulated issues and strictness.
    pub fn finish(self) -> ValidationReport {
        ValidationReport::new(self.issues, self.strict)
    }
}

/// Convenience constructor for a [`ReportBuilder`].
///
/// Equivalent to `ReportBuilder::new(strict)`.
pub fn report(strict: bool) -> ReportBuilder {
    ReportBuilder::new(strict)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue(level: &str, path: &str, message: &str) -> ValidationIssue {
        ValidationIssue {
            level: level.to_string(),
            path: path.to_string(),
            message: message.to_string(),
            line: None,
            column: None,
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
}

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

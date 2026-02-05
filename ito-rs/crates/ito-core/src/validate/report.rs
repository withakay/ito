//! Report builder for validation runs.
//!
//! Validation functions accumulate multiple issues and then compute a summary and
//! validity flag (strict vs non-strict) when finishing.

use super::{ValidationIssue, ValidationReport};

#[derive(Debug, Default)]
/// Incrementally build a [`ValidationReport`].
pub struct ReportBuilder {
    strict: bool,
    issues: Vec<ValidationIssue>,
}

impl ReportBuilder {
    /// Create a new builder.
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            issues: Vec::new(),
        }
    }

    /// Add a single issue.
    pub fn push(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Extend this builder with multiple issues.
    pub fn extend<I>(&mut self, issues: I)
    where
        I: IntoIterator<Item = ValidationIssue>,
    {
        self.issues.extend(issues);
    }

    /// Finish building and compute the final report fields.
    pub fn finish(self) -> ValidationReport {
        ValidationReport::new(self.issues, self.strict)
    }
}

/// Convenience constructor for a [`ReportBuilder`].
pub fn report(strict: bool) -> ReportBuilder {
    ReportBuilder::new(strict)
}

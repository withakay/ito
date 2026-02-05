use super::{ValidationIssue, ValidationReport};

#[derive(Debug, Default)]
pub struct ReportBuilder {
    strict: bool,
    issues: Vec<ValidationIssue>,
}

impl ReportBuilder {
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            issues: Vec::new(),
        }
    }

    pub fn push(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn extend<I>(&mut self, issues: I)
    where
        I: IntoIterator<Item = ValidationIssue>,
    {
        self.issues.extend(issues);
    }

    pub fn finish(self) -> ValidationReport {
        ValidationReport::new(self.issues, self.strict)
    }
}

pub fn report(strict: bool) -> ReportBuilder {
    ReportBuilder::new(strict)
}

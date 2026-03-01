//! Backend-backed change repository adapter.
//!
//! Delegates change reads to a [`BackendChangeReader`] when backend mode is
//! enabled. The filesystem repository remains the fallback when backend mode
//! is disabled.

use ito_domain::backend::BackendChangeReader;
use ito_domain::changes::{
    Change, ChangeRepository as DomainChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions,
};
use ito_domain::errors::DomainResult;

/// Backend-backed change repository.
///
/// Wraps a [`BackendChangeReader`] implementation and delegates all read
/// operations to the backend. Change target resolution is performed against
/// the backend-supplied change list.
pub struct BackendChangeRepository<R: BackendChangeReader> {
    reader: R,
}

impl<R: BackendChangeReader> BackendChangeRepository<R> {
    /// Create a backend-backed change repository.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BackendChangeReader> DomainChangeRepository for BackendChangeRepository<R> {
    fn resolve_target_with_options(
        &self,
        input: &str,
        _options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let Ok(summaries) = self.reader.list_changes() else {
            return ChangeTargetResolution::NotFound;
        };

        let input = input.trim();
        let mut exact = Vec::new();
        let mut prefix = Vec::new();

        for summary in &summaries {
            if summary.id == input {
                exact.push(summary.id.clone());
            } else if summary.id.starts_with(input) {
                prefix.push(summary.id.clone());
            }
        }

        if exact.len() == 1 {
            return ChangeTargetResolution::Unique(exact.into_iter().next().unwrap());
        }
        if exact.is_empty() && prefix.len() == 1 {
            return ChangeTargetResolution::Unique(prefix.into_iter().next().unwrap());
        }
        if !exact.is_empty() || !prefix.is_empty() {
            let mut all = exact;
            all.extend(prefix);
            return ChangeTargetResolution::Ambiguous(all);
        }

        ChangeTargetResolution::NotFound
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        let Ok(summaries) = self.reader.list_changes() else {
            return Vec::new();
        };

        let ids: Vec<String> = summaries.iter().map(|s| s.id.clone()).collect();
        ito_common::match_::nearest_matches(input, &ids, max)
    }

    fn exists(&self, id: &str) -> bool {
        self.reader.get_change(id).is_ok()
    }

    fn get(&self, id: &str) -> DomainResult<Change> {
        self.reader.get_change(id)
    }

    fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.reader.list_changes()
    }

    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.reader.list_changes()?;
        let filtered = all
            .into_iter()
            .filter(|s| s.module_id.as_deref() == Some(module_id))
            .collect();
        Ok(filtered)
    }

    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.reader.list_changes()?;
        let filtered = all
            .into_iter()
            .filter(|s| s.completed_tasks < s.total_tasks || s.total_tasks == 0)
            .collect();
        Ok(filtered)
    }

    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.reader.list_changes()?;
        let filtered = all
            .into_iter()
            .filter(|s| s.total_tasks > 0 && s.completed_tasks == s.total_tasks)
            .collect();
        Ok(filtered)
    }

    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        let all = self.reader.list_changes()?;
        for summary in all {
            if summary.id == id {
                return Ok(summary);
            }
        }
        Err(ito_domain::errors::DomainError::not_found("change", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use ito_domain::tasks::TasksParseResult;

    /// In-memory backend reader for testing.
    struct FakeReader {
        changes: Vec<ChangeSummary>,
        full: Vec<Change>,
    }

    impl FakeReader {
        fn new(changes: Vec<ChangeSummary>, full: Vec<Change>) -> Self {
            Self { changes, full }
        }
    }

    impl BackendChangeReader for FakeReader {
        fn list_changes(&self) -> DomainResult<Vec<ChangeSummary>> {
            Ok(self.changes.clone())
        }

        fn get_change(&self, change_id: &str) -> DomainResult<Change> {
            for c in &self.full {
                if c.id == change_id {
                    return Ok(c.clone());
                }
            }
            Err(ito_domain::errors::DomainError::not_found(
                "change", change_id,
            ))
        }
    }

    fn make_summary(id: &str, completed: u32, total: u32) -> ChangeSummary {
        ChangeSummary {
            id: id.to_string(),
            module_id: None,
            completed_tasks: completed,
            shelved_tasks: 0,
            in_progress_tasks: 0,
            pending_tasks: total - completed,
            total_tasks: total,
            last_modified: Utc::now(),
            has_proposal: true,
            has_design: false,
            has_specs: true,
            has_tasks: true,
        }
    }

    fn make_change(id: &str) -> Change {
        Change {
            id: id.to_string(),
            module_id: None,
            path: std::path::PathBuf::from("/fake"),
            proposal: Some("# Proposal".to_string()),
            design: None,
            specs: vec![],
            tasks: TasksParseResult::empty(),
            last_modified: Utc::now(),
        }
    }

    #[test]
    fn list_returns_all_changes() {
        let summaries = vec![
            make_summary("001-01_a", 0, 2),
            make_summary("001-02_b", 1, 2),
        ];
        let reader = FakeReader::new(summaries.clone(), vec![]);
        let repo = BackendChangeRepository::new(reader);

        let result = repo.list().unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn resolve_target_exact_match() {
        let summaries = vec![make_summary("001-01_a", 0, 2)];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let resolution = DomainChangeRepository::resolve_target(&repo, "001-01_a");
        assert!(matches!(resolution, ChangeTargetResolution::Unique(id) if id == "001-01_a"));
    }

    #[test]
    fn resolve_target_prefix_match() {
        let summaries = vec![make_summary("001-01_something", 0, 2)];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let resolution = DomainChangeRepository::resolve_target(&repo, "001-01");
        assert!(
            matches!(resolution, ChangeTargetResolution::Unique(id) if id == "001-01_something")
        );
    }

    #[test]
    fn resolve_target_ambiguous() {
        let summaries = vec![
            make_summary("001-01_a", 0, 2),
            make_summary("001-01_b", 0, 2),
        ];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let resolution = DomainChangeRepository::resolve_target(&repo, "001-01");
        assert!(matches!(resolution, ChangeTargetResolution::Ambiguous(_)));
    }

    #[test]
    fn resolve_target_not_found() {
        let summaries = vec![make_summary("001-01_a", 0, 2)];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let resolution = DomainChangeRepository::resolve_target(&repo, "999");
        assert!(matches!(resolution, ChangeTargetResolution::NotFound));
    }

    #[test]
    fn get_delegates_to_reader() {
        let change = make_change("001-01_a");
        let reader = FakeReader::new(vec![], vec![change]);
        let repo = BackendChangeRepository::new(reader);

        let result = DomainChangeRepository::get(&repo, "001-01_a").unwrap();
        assert_eq!(result.id, "001-01_a");
    }

    #[test]
    fn list_incomplete_filters_correctly() {
        let summaries = vec![
            make_summary("001-01_a", 2, 2),
            make_summary("001-02_b", 1, 2),
        ];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let incomplete = DomainChangeRepository::list_incomplete(&repo).unwrap();
        assert_eq!(incomplete.len(), 1);
        assert_eq!(incomplete[0].id, "001-02_b");
    }

    #[test]
    fn list_complete_filters_correctly() {
        let summaries = vec![
            make_summary("001-01_a", 2, 2),
            make_summary("001-02_b", 1, 2),
        ];
        let reader = FakeReader::new(summaries, vec![]);
        let repo = BackendChangeRepository::new(reader);

        let complete = DomainChangeRepository::list_complete(&repo).unwrap();
        assert_eq!(complete.len(), 1);
        assert_eq!(complete[0].id, "001-01_a");
    }
}

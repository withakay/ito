//! Backend-backed change repository adapter.
//!
//! Delegates change reads to a [`BackendChangeReader`] when backend mode is
//! enabled. The filesystem repository remains the fallback when backend mode
//! is disabled.

use ito_domain::backend::BackendChangeReader;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository as DomainChangeRepository, ChangeSummary,
    ChangeTargetResolution, ResolveTargetOptions, parse_change_id, parse_module_id,
};
use ito_domain::errors::DomainResult;
use std::collections::BTreeSet;

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
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        let Ok(summaries) = self.reader.list_changes(options.lifecycle) else {
            return ChangeTargetResolution::NotFound;
        };

        let input = input.trim();
        if input.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let ids: Vec<String> = summaries.iter().map(|s| s.id.clone()).collect();

        // 1. Exact match.
        if ids.iter().any(|id| id == input) {
            return ChangeTargetResolution::Unique(input.to_string());
        }

        // 2. Numeric change-id match: `1-12` or `1-12_slug` → `001-12_*`.
        let numeric_selector = parse_change_id(input).or_else(|| {
            // Also handle bare `1-12` style via two-number extraction.
            extract_two_numbers_as_change_id(input)
        });
        if let Some((module_id, change_num)) = numeric_selector {
            let numeric_prefix = format!("{module_id}-{change_num}");
            let with_separator = format!("{numeric_prefix}_");
            let mut numeric_matches: BTreeSet<String> = BTreeSet::new();
            for id in &ids {
                if id == &numeric_prefix || id.starts_with(&with_separator) {
                    numeric_matches.insert(id.clone());
                }
            }
            if !numeric_matches.is_empty() {
                let numeric_matches: Vec<String> = numeric_matches.into_iter().collect();
                if numeric_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(numeric_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(numeric_matches);
            }
        }

        // 3. Module-scoped slug query: `1:setup` → changes in module 001 with "setup" in slug.
        if let Some((module_part, query)) = input.split_once(':') {
            let query = query.trim();
            if !query.is_empty() {
                let module_id = parse_module_id(module_part);
                let tokens = tokenize_query(query);
                let mut scoped_matches: BTreeSet<String> = BTreeSet::new();
                for id in &ids {
                    let Some((name_module, _name_change, slug)) = split_canonical_change_id(id)
                    else {
                        continue;
                    };
                    if name_module != module_id {
                        continue;
                    }
                    if slug_matches_tokens(slug, &tokens) {
                        scoped_matches.insert(id.clone());
                    }
                }
                if scoped_matches.is_empty() {
                    return ChangeTargetResolution::NotFound;
                }
                let scoped_matches: Vec<String> = scoped_matches.into_iter().collect();
                if scoped_matches.len() == 1 {
                    return ChangeTargetResolution::Unique(scoped_matches[0].clone());
                }
                return ChangeTargetResolution::Ambiguous(scoped_matches);
            }
        }

        // 4. Prefix match on full id string.
        let mut prefix_matches: BTreeSet<String> = BTreeSet::new();
        for id in &ids {
            if id.starts_with(input) {
                prefix_matches.insert(id.clone());
            }
        }

        if prefix_matches.is_empty() {
            // 5. Slug token match across all changes.
            let tokens = tokenize_query(input);
            for id in &ids {
                let Some((_module, _change, slug)) = split_canonical_change_id(id) else {
                    continue;
                };
                if slug_matches_tokens(slug, &tokens) {
                    prefix_matches.insert(id.clone());
                }
            }
        }

        if prefix_matches.is_empty() {
            return ChangeTargetResolution::NotFound;
        }

        let prefix_matches: Vec<String> = prefix_matches.into_iter().collect();
        if prefix_matches.len() == 1 {
            return ChangeTargetResolution::Unique(prefix_matches[0].clone());
        }
        ChangeTargetResolution::Ambiguous(prefix_matches)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        let Ok(summaries) = self.reader.list_changes(ChangeLifecycleFilter::Active) else {
            return Vec::new();
        };

        let ids: Vec<String> = summaries.iter().map(|s| s.id.clone()).collect();
        ito_common::match_::nearest_matches(input, &ids, max)
    }

    fn exists(&self, id: &str) -> bool {
        self.exists_with_filter(id, ChangeLifecycleFilter::Active)
    }

    fn exists_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> bool {
        self.reader.get_change(id, filter).is_ok()
    }

    fn get_with_filter(&self, id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        self.reader.get_change(id, filter)
    }

    fn list_with_filter(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        self.reader.list_changes(filter)
    }

    fn list_by_module_with_filter(
        &self,
        module_id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let normalized_id = parse_module_id(module_id);
        let all = self.reader.list_changes(filter)?;
        let mut filtered = Vec::new();
        for s in all {
            if s.module_id.as_deref() == Some(&normalized_id) {
                filtered.push(s);
            }
        }
        Ok(filtered)
    }

    fn list_incomplete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.reader.list_changes(filter)?;
        let filtered = all
            .into_iter()
            .filter(|s| s.completed_tasks < s.total_tasks || s.total_tasks == 0)
            .collect();
        Ok(filtered)
    }

    fn list_complete_with_filter(
        &self,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        let all = self.reader.list_changes(filter)?;
        let filtered = all
            .into_iter()
            .filter(|s| s.total_tasks > 0 && s.completed_tasks == s.total_tasks)
            .collect();
        Ok(filtered)
    }

    fn get_summary_with_filter(
        &self,
        id: &str,
        filter: ChangeLifecycleFilter,
    ) -> DomainResult<ChangeSummary> {
        let all = self.reader.list_changes(filter)?;
        for summary in all {
            if summary.id == id {
                return Ok(summary);
            }
        }
        Err(ito_domain::errors::DomainError::not_found("change", id))
    }
}

// ── Resolution helpers ─────────────────────────────────────────────
//
// These mirror the logic in `FsChangeRepository` so that backend and
// filesystem resolution behave identically for inputs like `1-12` or
// `1:slug`.

/// Split a canonical change id into `(module_id, change_num, slug)`.
fn split_canonical_change_id(name: &str) -> Option<(String, String, &str)> {
    let (module_id, change_num) = parse_change_id(name)?;
    let slug = name.split_once('_').map(|(_id, s)| s).unwrap_or("");
    Some((module_id, change_num, slug))
}

/// Tokenize a query string into lowercase words.
fn tokenize_query(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for s in input.split_whitespace() {
        let t = s.trim().to_lowercase();
        if !t.is_empty() {
            tokens.push(t);
        }
    }
    tokens
}

/// Return `true` when all tokens appear in the normalized slug text.
fn slug_matches_tokens(slug: &str, tokens: &[String]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let mut text = String::new();
    for ch in slug.chars() {
        if ch.is_ascii_alphanumeric() {
            text.push(ch.to_ascii_lowercase());
        } else {
            text.push(' ');
        }
    }
    for token in tokens {
        if !text.contains(token.as_str()) {
            return false;
        }
    }
    true
}

/// Extract two numbers from an input string and format as a change id prefix.
///
/// Handles inputs like `1-12` (already handled by `parse_change_id`) and
/// alternative separators like `1:12`.
fn extract_two_numbers_as_change_id(input: &str) -> Option<(String, String)> {
    use regex::Regex;
    let re = Regex::new(r"\d+").ok()?;
    let mut parts: Vec<&str> = Vec::new();
    for m in re.find_iter(input) {
        parts.push(m.as_str());
        if parts.len() > 2 {
            return None;
        }
    }
    if parts.len() != 2 {
        return None;
    }
    let parsed = format!("{}-{}", parts[0], parts[1]);
    parse_change_id(&parsed)
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
        fn list_changes(
            &self,
            _filter: ito_domain::changes::ChangeLifecycleFilter,
        ) -> DomainResult<Vec<ChangeSummary>> {
            Ok(self.changes.clone())
        }

        fn get_change(
            &self,
            change_id: &str,
            _filter: ito_domain::changes::ChangeLifecycleFilter,
        ) -> DomainResult<Change> {
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

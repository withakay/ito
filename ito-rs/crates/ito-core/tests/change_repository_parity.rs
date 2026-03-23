//! Parity tests for change repository implementations.
//!
//! Verifies that `BackendChangeRepository` and `SqliteChangeRepository` behave
//! identically to `FsChangeRepository` for inputs like `1-12`, `1:slug`,
//! empty strings, and lifecycle filters.

use chrono::Utc;
use ito_core::backend_change_repository::BackendChangeRepository;
use ito_core::sqlite_project_store::{SqliteBackendProjectStore, UpsertChangeParams};
use ito_domain::backend::BackendChangeReader;
use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::TasksParseResult;

// ── Fake backend reader ────────────────────────────────────────────

struct FakeReader {
    changes: Vec<ChangeSummary>,
}

impl FakeReader {
    fn new(changes: Vec<ChangeSummary>) -> Self {
        Self { changes }
    }
}

impl BackendChangeReader for FakeReader {
    fn list_changes(&self, _filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self.changes.clone())
    }

    fn get_change(&self, change_id: &str, _filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        for s in &self.changes {
            if s.id == change_id {
                return Ok(Change {
                    id: s.id.clone(),
                    module_id: s.module_id.clone(),
                    sub_module_id: s.sub_module_id.clone(),
                    path: std::path::PathBuf::new(),
                    proposal: None,
                    design: None,
                    specs: vec![],
                    tasks: TasksParseResult::empty(),
                    last_modified: Utc::now(),
                });
            }
        }
        Err(DomainError::not_found("change", change_id))
    }
}

fn make_summary(id: &str, module_id: Option<&str>) -> ChangeSummary {
    ChangeSummary {
        id: id.to_string(),
        module_id: module_id.map(|s| s.to_string()),
        sub_module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 1,
        total_tasks: 1,
        last_modified: Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: true,
        has_tasks: true,
    }
}

// ── BackendChangeRepository parity tests ──────────────────────────

#[test]
fn backend_resolve_empty_input_returns_not_found() {
    let reader = FakeReader::new(vec![make_summary("001-01_alpha", Some("001"))]);
    let repo = BackendChangeRepository::new(reader);

    let result = repo.resolve_target("");
    assert_eq!(result, ChangeTargetResolution::NotFound);

    let result = repo.resolve_target("   ");
    assert_eq!(result, ChangeTargetResolution::NotFound);
}

#[test]
fn backend_resolve_numeric_short_form_matches_canonical_id() {
    // `1-12` should resolve to `001-12_something` just like the filesystem repo.
    let reader = FakeReader::new(vec![make_summary("001-12_setup-wizard", Some("001"))]);
    let repo = BackendChangeRepository::new(reader);

    let result = repo.resolve_target("1-12");
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
    );
}

#[test]
fn backend_resolve_numeric_short_form_ambiguous() {
    let reader = FakeReader::new(vec![
        make_summary("001-12_first-change", Some("001")),
        make_summary("001-12_follow-up", Some("001")),
    ]);
    let repo = BackendChangeRepository::new(reader);

    let result = repo.resolve_target("1-12");
    assert!(
        matches!(result, ChangeTargetResolution::Ambiguous(ref ids) if ids.len() == 2),
        "expected Ambiguous with 2 matches, got {result:?}"
    );
}

#[test]
fn backend_resolve_module_scoped_slug_query() {
    // `1:setup` should resolve changes in module 001 whose slug contains "setup".
    let reader = FakeReader::new(vec![
        make_summary("001-12_setup-wizard", Some("001")),
        make_summary("002-12_setup-wizard", Some("002")),
    ]);
    let repo = BackendChangeRepository::new(reader);

    let result = repo.resolve_target("1:setup");
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
    );

    let result = repo.resolve_target("2:setup");
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("002-12_setup-wizard".to_string())
    );
}

#[test]
fn backend_resolve_module_scoped_slug_not_found() {
    let reader = FakeReader::new(vec![make_summary("001-12_setup-wizard", Some("001"))]);
    let repo = BackendChangeRepository::new(reader);

    // Module 002 has no changes.
    let result = repo.resolve_target("2:setup");
    assert_eq!(result, ChangeTargetResolution::NotFound);
}

#[test]
fn backend_list_by_module_normalizes_module_id() {
    // Module id `1` should be normalized to `001` before comparison.
    let reader = FakeReader::new(vec![
        make_summary("001-01_alpha", Some("001")),
        make_summary("002-01_beta", Some("002")),
    ]);
    let repo = BackendChangeRepository::new(reader);

    let result = repo
        .list_by_module_with_filter("1", ChangeLifecycleFilter::Active)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "001-01_alpha");

    // Normalized form also works.
    let result = repo
        .list_by_module_with_filter("001", ChangeLifecycleFilter::Active)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "001-01_alpha");
}

#[test]
fn backend_resolve_lifecycle_filter_respected() {
    // The FakeReader ignores the filter (returns all), but the repo should
    // still pass the filter through to the reader.
    let reader = FakeReader::new(vec![make_summary("001-01_alpha", Some("001"))]);
    let repo = BackendChangeRepository::new(reader);

    // With Active filter (default), the change should be found.
    let result = repo.resolve_target_with_options(
        "001-01_alpha",
        ResolveTargetOptions {
            lifecycle: ChangeLifecycleFilter::Active,
        },
    );
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-01_alpha".to_string())
    );
}

// ── SqliteChangeRepository parity tests ───────────────────────────

struct TempSqliteStore {
    store: SqliteBackendProjectStore,
    // Keep the TempDir alive so the file isn't deleted while the store is open.
    _tmp: tempfile::TempDir,
}

fn sqlite_store_with_changes(changes: &[(&str, Option<&str>)]) -> TempSqliteStore {
    let tmp = tempfile::TempDir::new().unwrap();
    let db_path = tmp.path().join("test.db");
    let store = SqliteBackendProjectStore::open(&db_path).unwrap();
    store.ensure_project("org", "repo").unwrap();
    for (change_id, module_id) in changes {
        store
            .upsert_change(&UpsertChangeParams {
                org: "org",
                repo: "repo",
                change_id,
                module_id: *module_id,
                sub_module_id: None,
                proposal: Some("# Proposal"),
                design: None,
                tasks_md: Some("- [ ] task"),
                specs: &[],
            })
            .unwrap();
    }
    TempSqliteStore { store, _tmp: tmp }
}

#[test]
fn sqlite_resolve_empty_input_returns_not_found() {
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target("");
    assert_eq!(result, ChangeTargetResolution::NotFound);

    let result = repo.resolve_target("   ");
    assert_eq!(result, ChangeTargetResolution::NotFound);
}

#[test]
fn sqlite_resolve_numeric_short_form_matches_canonical_id() {
    let ts = sqlite_store_with_changes(&[("001-12_setup-wizard", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target("1-12");
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-12_setup-wizard".to_string())
    );
}

#[test]
fn sqlite_resolve_numeric_short_form_ambiguous() {
    let ts = sqlite_store_with_changes(&[
        ("001-12_first-change", Some("001")),
        ("001-12_follow-up", Some("001")),
    ]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target("1-12");
    assert!(
        matches!(result, ChangeTargetResolution::Ambiguous(ref ids) if ids.len() == 2),
        "expected Ambiguous with 2 matches, got {result:?}"
    );
}

#[test]
fn sqlite_resolve_prefix_match() {
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target("001-01");
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-01_alpha".to_string())
    );
}

#[test]
fn sqlite_resolve_archived_filter_returns_not_found() {
    // SQLite has no archived concept; Archived-only queries always return NotFound.
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target_with_options(
        "001-01_alpha",
        ResolveTargetOptions {
            lifecycle: ChangeLifecycleFilter::Archived,
        },
    );
    assert_eq!(result, ChangeTargetResolution::NotFound);
}

#[test]
fn sqlite_resolve_all_filter_finds_active_changes() {
    // All filter should include active changes (which is all SQLite has).
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.resolve_target_with_options(
        "001-01_alpha",
        ResolveTargetOptions {
            lifecycle: ChangeLifecycleFilter::All,
        },
    );
    assert_eq!(
        result,
        ChangeTargetResolution::Unique("001-01_alpha".to_string())
    );
}

#[test]
fn sqlite_list_by_module_normalizes_module_id() {
    let ts =
        sqlite_store_with_changes(&[("001-01_alpha", Some("001")), ("002-01_beta", Some("002"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    // Short form `1` should normalize to `001`.
    let result = repo
        .list_by_module_with_filter("1", ChangeLifecycleFilter::Active)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "001-01_alpha");

    // Canonical form also works.
    let result = repo
        .list_by_module_with_filter("001", ChangeLifecycleFilter::Active)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "001-01_alpha");
}

#[test]
fn sqlite_list_archived_filter_returns_empty() {
    // SQLite has no archived changes; Archived filter should return empty.
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo
        .list_with_filter(ChangeLifecycleFilter::Archived)
        .unwrap();
    assert!(result.is_empty());
}

#[test]
fn sqlite_list_all_filter_returns_active_changes() {
    // All filter should return all SQLite changes (treated as active).
    let ts =
        sqlite_store_with_changes(&[("001-01_alpha", Some("001")), ("002-01_beta", Some("002"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.list_with_filter(ChangeLifecycleFilter::All).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn sqlite_get_with_all_filter_finds_change() {
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.get_with_filter("001-01_alpha", ChangeLifecycleFilter::All);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, "001-01_alpha");
}

#[test]
fn sqlite_get_with_archived_filter_returns_not_found() {
    let ts = sqlite_store_with_changes(&[("001-01_alpha", Some("001"))]);
    let repo = ts.store.change_repository("org", "repo").unwrap();

    let result = repo.get_with_filter("001-01_alpha", ChangeLifecycleFilter::Archived);
    assert!(result.is_err());
}

use std::path::Path;

use chrono::Utc;
use rusqlite::Connection;
use tempfile::TempDir;

use ito_core::BackendProjectStore;
use ito_core::backend_change_repository::BackendChangeRepository;
use ito_core::change_repository::FsChangeRepository;
use ito_core::repository_runtime::{PersistenceMode, RepositoryRuntimeBuilder, SqliteRuntime};
use ito_core::sqlite_project_store::{SqliteBackendProjectStore, UpsertChangeParams};
use ito_domain::backend::BackendChangeReader;
use ito_domain::changes::{
    Change, ChangeLifecycleFilter, ChangeRepository, ChangeSummary, ChangeTargetResolution,
    ResolveTargetOptions,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::TasksParseResult;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dirs should exist");
    }
    std::fs::write(path, contents).expect("test fixture should write");
}

fn make_change(root: &Path, id: &str) {
    write(
        root.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(
        root.join(".ito/changes").join(id).join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 todo\n",
    );
}

fn make_archived_change(root: &Path, id: &str) {
    write(
        root.join(".ito/changes/archive")
            .join(id)
            .join("proposal.md"),
        "## Why\narchived fixture\n",
    );
}

fn sqlite_runtime_with_changes(
    active_ids: &[&str],
    archived_ids: &[&str],
) -> ito_core::repository_runtime::RepositoryRuntime {
    let repo = TempDir::new().expect("temp repo");
    let db_path = repo.path().join("ito.db");
    let store = SqliteBackendProjectStore::open(&db_path).expect("sqlite store");
    store
        .ensure_project("local", "demo")
        .expect("ensure sqlite project");

    for change_id in active_ids.iter().chain(archived_ids.iter()) {
        store
            .upsert_change(&UpsertChangeParams {
                org: "local",
                repo: "demo",
                change_id,
                module_id: change_id.split('-').next(),
                sub_module_id: None,
                proposal: Some("# Proposal"),
                design: None,
                tasks_md: Some("## 1. Implementation\n- [ ] 1.1 Todo"),
                specs: &[],
            })
            .expect("upsert sqlite change");
    }

    if !archived_ids.is_empty() {
        let conn = Connection::open(&db_path).expect("open sqlite db");
        for change_id in archived_ids {
            conn.execute(
                "UPDATE changes SET archived_at = ?1 WHERE org = ?2 AND repo = ?3 AND change_id = ?4",
                ("2026-03-08T00:00:00Z", "local", "demo", change_id),
            )
            .expect("mark sqlite change archived");
        }
    }

    RepositoryRuntimeBuilder::new(repo.path().join(".ito"))
        .mode(PersistenceMode::Sqlite)
        .sqlite_runtime(SqliteRuntime {
            db_path,
            org: "local".to_string(),
            repo: "demo".to_string(),
        })
        .build()
        .expect("sqlite runtime")
}

fn summary(id: &str) -> ChangeSummary {
    ChangeSummary {
        id: id.to_string(),
        module_id: Some(id.split('-').next().unwrap_or_default().to_string()),
        sub_module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 1,
        total_tasks: 1,
        last_modified: Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: false,
        has_tasks: true,
        orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
    }
}

fn change(id: &str) -> Change {
    Change {
        id: id.to_string(),
        module_id: Some(id.split('-').next().unwrap_or_default().to_string()),
        sub_module_id: None,
        path: std::path::PathBuf::new(),
        proposal: Some("# Proposal".to_string()),
        design: None,
        specs: Vec::new(),
        tasks: TasksParseResult::empty(),
        orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
        last_modified: Utc::now(),
    }
}

struct FakeReader {
    changes: Vec<ChangeSummary>,
    full: Vec<Change>,
}

impl BackendChangeReader for FakeReader {
    fn list_changes(&self, _filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self.changes.clone())
    }

    fn get_change(&self, change_id: &str, _filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        self.full
            .iter()
            .find(|change| change.id == change_id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("change", change_id))
    }
}

#[test]
fn change_target_resolution_matches_across_repository_modes() {
    let change_ids = [
        "025-07_feature-one",
        "025-08_feature-two",
        "026-01_other-work",
    ];

    let fs_repo = TempDir::new().expect("temp repo");
    let fs_ito_path = fs_repo.path().join(".ito");
    for change_id in change_ids {
        make_change(fs_repo.path(), change_id);
    }
    let fs_repo = FsChangeRepository::new(&fs_ito_path);

    let sqlite_runtime = sqlite_runtime_with_changes(&change_ids, &[]);
    let sqlite_repo = sqlite_runtime.repositories().changes.clone();

    let remote_repo = BackendChangeRepository::new(FakeReader {
        changes: change_ids.iter().map(|id| summary(id)).collect(),
        full: change_ids.iter().map(|id| change(id)).collect(),
    });

    let cases = [
        (
            "025-07_feature-one",
            ChangeTargetResolution::Unique("025-07_feature-one".to_string()),
        ),
        (
            "025-07",
            ChangeTargetResolution::Unique("025-07_feature-one".to_string()),
        ),
        (
            "change 25 7",
            ChangeTargetResolution::Unique("025-07_feature-one".to_string()),
        ),
        (
            "025:feature one",
            ChangeTargetResolution::Unique("025-07_feature-one".to_string()),
        ),
        (
            "025",
            ChangeTargetResolution::Ambiguous(vec![
                "025-07_feature-one".to_string(),
                "025-08_feature-two".to_string(),
            ]),
        ),
        (
            "other",
            ChangeTargetResolution::Unique("026-01_other-work".to_string()),
        ),
        ("", ChangeTargetResolution::NotFound),
    ];

    for (input, expected) in cases {
        let options = ResolveTargetOptions::default();
        assert_eq!(
            fs_repo.resolve_target_with_options(input, options),
            expected
        );
        assert_eq!(
            sqlite_repo.resolve_target_with_options(input, options),
            expected
        );
        assert_eq!(
            remote_repo.resolve_target_with_options(input, options),
            expected
        );
    }
}

#[test]
fn sqlite_resolver_honors_archived_lifecycle_like_filesystem() {
    let archived_id = "025-09_archived-change";

    let fs_repo = TempDir::new().expect("temp repo");
    let fs_ito_path = fs_repo.path().join(".ito");
    make_change(fs_repo.path(), "025-07_feature-one");
    make_archived_change(fs_repo.path(), archived_id);
    let fs_repo = FsChangeRepository::new(&fs_ito_path);

    let sqlite_runtime = sqlite_runtime_with_changes(&["025-07_feature-one"], &[archived_id]);
    let sqlite_repo = sqlite_runtime.repositories().changes.clone();

    let default_options = ResolveTargetOptions::default();
    let archived_options = ResolveTargetOptions {
        lifecycle: ChangeLifecycleFilter::All,
    };

    assert_eq!(
        fs_repo.resolve_target_with_options("025-09", default_options),
        ChangeTargetResolution::NotFound
    );
    assert_eq!(
        sqlite_repo.resolve_target_with_options("025-09", default_options),
        ChangeTargetResolution::NotFound
    );

    assert_eq!(
        fs_repo.resolve_target_with_options("025-09", archived_options),
        ChangeTargetResolution::Unique(archived_id.to_string())
    );
    assert_eq!(
        sqlite_repo.resolve_target_with_options("025-09", archived_options),
        ChangeTargetResolution::Unique(archived_id.to_string())
    );
}

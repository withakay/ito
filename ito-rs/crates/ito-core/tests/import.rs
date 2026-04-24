use std::cell::RefCell;
use std::path::Path;

use chrono::Utc;
use ito_core::ArtifactBundle;
use ito_core::backend_import::{
    BackendImportSink, ImportAction, ImportLifecycle, ImportSummary, LocalImportChange,
    RepositoryBackedImportSink, import_local_changes, import_local_changes_with_options,
};
use ito_core::errors::{CoreError, CoreResult};
use ito_domain::backend::{
    ArchiveResult, BackendArchiveClient, BackendChangeReader, BackendError, BackendSyncClient,
    PushResult,
};
use ito_domain::changes::{Change, ChangeLifecycleFilter, ChangeSummary};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::TasksParseResult;
use tempfile::TempDir;

fn write_active_change(ito_path: &Path, change_id: &str) {
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(change_dir.join("specs/backend-import")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Active proposal\n").unwrap();
    std::fs::write(change_dir.join("design.md"), "# Active design\n").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [ ] pending\n").unwrap();
    std::fs::write(
        change_dir.join("specs/backend-import/spec.md"),
        "## ADDED Requirements\n",
    )
    .unwrap();
}

fn read_bundle(ito_path: &Path, change_id: &str) -> ArtifactBundle {
    let change_dir = ito_path.join("changes").join(change_id);
    read_bundle_from_dir(&change_dir, change_id)
}

fn read_bundle_from_dir(change_dir: &Path, change_id: &str) -> ArtifactBundle {
    let proposal = std::fs::read_to_string(change_dir.join("proposal.md")).ok();
    let design = std::fs::read_to_string(change_dir.join("design.md")).ok();
    let tasks = std::fs::read_to_string(change_dir.join("tasks.md")).ok();
    let mut specs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(change_dir.join("specs")) {
        for entry in entries.flatten() {
            let spec_file = entry.path().join("spec.md");
            if spec_file.is_file() {
                specs.push((
                    entry.file_name().to_string_lossy().to_string(),
                    std::fs::read_to_string(spec_file).unwrap(),
                ));
            }
        }
    }
    specs.sort_by(|left, right| left.0.cmp(&right.0));
    ArtifactBundle {
        change_id: change_id.to_string(),
        proposal,
        design,
        tasks,
        specs,
        revision: String::new(),
    }
}

fn write_archived_change(ito_path: &Path, archive_dir: &str, change_id: &str) {
    let change_dir = ito_path.join("changes").join("archive").join(archive_dir);
    std::fs::create_dir_all(change_dir.join("specs/backend-import")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Archived proposal\n").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [x] done\n").unwrap();
    std::fs::write(
        change_dir.join("specs/backend-import/spec.md"),
        format!("## Imported for {change_id}\n"),
    )
    .unwrap();
}

#[derive(Default)]
struct RecordingSink {
    calls: RefCell<Vec<(String, ImportLifecycle)>>,
}

impl BackendImportSink for RecordingSink {
    fn preview_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        self.calls
            .borrow_mut()
            .push((change.change_id.clone(), change.lifecycle));
        Ok(ImportAction::Previewed)
    }

    fn import_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        self.calls
            .borrow_mut()
            .push((change.change_id.clone(), change.lifecycle));
        Ok(ImportAction::Imported)
    }
}

struct FailingArchivedSink {
    calls: RefCell<Vec<(String, ImportLifecycle)>>,
}

impl BackendImportSink for FailingArchivedSink {
    fn preview_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        self.calls
            .borrow_mut()
            .push((change.change_id.clone(), change.lifecycle));
        Ok(ImportAction::Previewed)
    }

    fn import_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        self.calls
            .borrow_mut()
            .push((change.change_id.clone(), change.lifecycle));
        if change.lifecycle == ImportLifecycle::Archived {
            return Err(CoreError::process("backend write failed"));
        }
        Ok(ImportAction::Imported)
    }
}

fn assert_summary_counts(summary: &ImportSummary, imported: usize, skipped: usize, failed: usize) {
    assert_eq!(summary.imported, imported);
    assert_eq!(summary.previewed, 0);
    assert_eq!(summary.skipped, skipped);
    assert_eq!(summary.failed, failed);
}

#[test]
fn imports_active_and_archived_changes_with_lifecycle_fidelity() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");
    write_archived_change(
        &ito_path,
        "2026-03-10-024-17_archived-example",
        "024-17_archived-example",
    );

    let sink = RecordingSink::default();
    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_summary_counts(&summary, 2, 0, 0);
    assert_eq!(
        sink.calls.into_inner(),
        vec![
            (
                "024-17_archived-example".to_string(),
                ImportLifecycle::Archived,
            ),
            ("024-18_active-example".to_string(), ImportLifecycle::Active,),
        ]
    );
}

#[test]
fn import_summary_records_failures_without_aborting_remaining_changes() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");
    write_archived_change(
        &ito_path,
        "2026-03-10-024-17_archived-example",
        "024-17_archived-example",
    );

    let sink = FailingArchivedSink {
        calls: RefCell::new(Vec::new()),
    };
    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_summary_counts(&summary, 1, 0, 1);
    assert_eq!(sink.calls.into_inner().len(), 2);
}

#[test]
fn ignores_unrecognized_archive_directories_during_discovery() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");
    write_archived_change(
        &ito_path,
        "2026-03-10-024-17_archived-example",
        "024-17_archived-example",
    );
    std::fs::create_dir_all(ito_path.join("changes/archive/README")).unwrap();

    let sink = RecordingSink::default();
    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_summary_counts(&summary, 2, 0, 0);
    assert_eq!(sink.calls.into_inner().len(), 2);
}

#[test]
fn dry_run_previews_without_importing() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");

    let sink = RecordingSink::default();
    let summary =
        ito_core::backend_import::import_local_changes_with_options(&sink, &ito_path, true)
            .unwrap();

    assert_eq!(summary.imported, 0);
    assert_eq!(summary.previewed, 1);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(
        summary.results[0].status,
        ito_core::backend_import::ImportItemStatus::Previewed
    );
}

fn summary(id: &str) -> ChangeSummary {
    ChangeSummary {
        id: id.to_string(),
        module_id: Some("024".to_string()),
        sub_module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 0,
        last_modified: Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: true,
        has_tasks: true,
        orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
    }
}

fn change(id: &str) -> Change {
    Change {
        id: id.to_string(),
        module_id: Some("024".to_string()),
        sub_module_id: None,
        path: std::path::PathBuf::new(),
        proposal: Some("# Proposal\n".to_string()),
        design: None,
        specs: Vec::new(),
        tasks: TasksParseResult::empty(),
        orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
        last_modified: Utc::now(),
    }
}

struct FakeChangeReader {
    active: Vec<ChangeSummary>,
    archived: Vec<ChangeSummary>,
    list_calls: RefCell<Vec<ChangeLifecycleFilter>>,
    get_calls: RefCell<Vec<(String, ChangeLifecycleFilter)>>,
}

impl BackendChangeReader for FakeChangeReader {
    fn list_changes(&self, filter: ChangeLifecycleFilter) -> DomainResult<Vec<ChangeSummary>> {
        self.list_calls.borrow_mut().push(filter);
        Ok(match filter {
            ChangeLifecycleFilter::Active => self.active.clone(),
            ChangeLifecycleFilter::Archived => self.archived.clone(),
            ChangeLifecycleFilter::All => {
                let mut all = self.active.clone();
                all.extend(self.archived.clone());
                all
            }
        })
    }

    fn get_change(&self, change_id: &str, filter: ChangeLifecycleFilter) -> DomainResult<Change> {
        self.get_calls
            .borrow_mut()
            .push((change_id.to_string(), filter));

        let exists = match filter {
            ChangeLifecycleFilter::Active => {
                self.active.iter().any(|change| change.id == change_id)
            }
            ChangeLifecycleFilter::Archived => {
                self.archived.iter().any(|change| change.id == change_id)
            }
            ChangeLifecycleFilter::All => {
                self.active.iter().any(|change| change.id == change_id)
                    || self.archived.iter().any(|change| change.id == change_id)
            }
        };

        if exists {
            return Ok(change(change_id));
        }

        Err(DomainError::not_found("change", change_id))
    }
}

struct FakeSyncClient {
    pulled: Result<ArtifactBundle, BackendError>,
    pushes: RefCell<Vec<String>>,
    pulls: RefCell<Vec<String>>,
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, change_id: &str) -> Result<ArtifactBundle, BackendError> {
        self.pulls.borrow_mut().push(change_id.to_string());
        self.pulled.clone()
    }

    fn push(&self, change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        self.pushes.borrow_mut().push(change_id.to_string());
        Ok(PushResult {
            change_id: change_id.to_string(),
            new_revision: "rev-2".to_string(),
        })
    }
}

struct FakeArchiveClient {
    archives: RefCell<Vec<String>>,
}

impl BackendArchiveClient for FakeArchiveClient {
    fn mark_archived(&self, change_id: &str) -> Result<ArchiveResult, BackendError> {
        self.archives.borrow_mut().push(change_id.to_string());
        Ok(ArchiveResult {
            change_id: change_id.to_string(),
            archived_at: "2026-03-10T00:00:00Z".to_string(),
        })
    }
}

#[test]
fn skips_already_imported_active_change_when_remote_bundle_matches() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");
    let bundle = read_bundle(&ito_path, "024-18_active-example");

    let reader = FakeChangeReader {
        active: vec![summary("024-18_active-example")],
        archived: Vec::new(),
        list_calls: RefCell::new(Vec::new()),
        get_calls: RefCell::new(Vec::new()),
    };
    let sync = FakeSyncClient {
        pulled: Ok(bundle),
        pushes: RefCell::new(Vec::new()),
        pulls: RefCell::new(Vec::new()),
    };
    let archive = FakeArchiveClient {
        archives: RefCell::new(Vec::new()),
    };
    let sink = RepositoryBackedImportSink::new(&reader, &sync, &archive);

    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_eq!(summary.imported, 0);
    assert_eq!(summary.skipped, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(
        reader.list_calls.into_inner(),
        Vec::<ChangeLifecycleFilter>::new()
    );
    assert_eq!(
        reader.get_calls.into_inner(),
        vec![
            (
                "024-18_active-example".to_string(),
                ChangeLifecycleFilter::Active,
            ),
            (
                "024-18_active-example".to_string(),
                ChangeLifecycleFilter::Archived,
            ),
        ]
    );
    assert_eq!(sync.pushes.into_inner(), Vec::<String>::new());
    assert_eq!(archive.archives.into_inner(), Vec::<String>::new());
}

#[test]
fn rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_archived_change(
        &ito_path,
        "2026-03-10-024-17_archived-example",
        "024-17_archived-example",
    );
    let bundle = read_bundle_from_dir(
        &ito_path.join("changes/archive/2026-03-10-024-17_archived-example"),
        "024-17_archived-example",
    );

    let reader = FakeChangeReader {
        active: vec![summary("024-17_archived-example")],
        archived: Vec::new(),
        list_calls: RefCell::new(Vec::new()),
        get_calls: RefCell::new(Vec::new()),
    };
    let sync = FakeSyncClient {
        pulled: Ok(bundle),
        pushes: RefCell::new(Vec::new()),
        pulls: RefCell::new(Vec::new()),
    };
    let archive = FakeArchiveClient {
        archives: RefCell::new(Vec::new()),
    };
    let sink = RepositoryBackedImportSink::new(&reader, &sync, &archive);

    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_eq!(summary.imported, 1);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(sync.pushes.into_inner(), Vec::<String>::new());
    assert_eq!(
        archive.archives.into_inner(),
        vec!["024-17_archived-example".to_string()]
    );
}

#[test]
fn dry_run_uses_preview_logic_without_mutating_backend() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");

    let reader = FakeChangeReader {
        active: Vec::new(),
        archived: Vec::new(),
        list_calls: RefCell::new(Vec::new()),
        get_calls: RefCell::new(Vec::new()),
    };
    let sync = FakeSyncClient {
        pulled: Ok(read_bundle(&ito_path, "024-18_active-example")),
        pushes: RefCell::new(Vec::new()),
        pulls: RefCell::new(Vec::new()),
    };
    let archive = FakeArchiveClient {
        archives: RefCell::new(Vec::new()),
    };
    let sink = RepositoryBackedImportSink::new(&reader, &sync, &archive);

    let summary = import_local_changes_with_options(&sink, &ito_path, true).unwrap();

    assert_eq!(summary.previewed, 1);
    assert_eq!(sync.pushes.into_inner(), Vec::<String>::new());
    assert_eq!(archive.archives.into_inner(), Vec::<String>::new());
}

#[test]
fn pushes_when_remote_active_bundle_differs() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");

    let reader = FakeChangeReader {
        active: vec![summary("024-18_active-example")],
        archived: Vec::new(),
        list_calls: RefCell::new(Vec::new()),
        get_calls: RefCell::new(Vec::new()),
    };
    let sync = FakeSyncClient {
        pulled: Ok(ArtifactBundle {
            proposal: Some("# Old proposal\n".to_string()),
            ..read_bundle(&ito_path, "024-18_active-example")
        }),
        pushes: RefCell::new(Vec::new()),
        pulls: RefCell::new(Vec::new()),
    };
    let archive = FakeArchiveClient {
        archives: RefCell::new(Vec::new()),
    };
    let sink = RepositoryBackedImportSink::new(&reader, &sync, &archive);

    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_eq!(summary.imported, 1);
    assert_eq!(
        sync.pulls.into_inner(),
        vec!["024-18_active-example".to_string()]
    );
    assert_eq!(
        sync.pushes.into_inner(),
        vec!["024-18_active-example".to_string()]
    );
}

#[test]
fn active_local_change_fails_when_backend_only_has_archived_copy() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");

    let reader = FakeChangeReader {
        active: Vec::new(),
        archived: vec![summary("024-18_active-example")],
        list_calls: RefCell::new(Vec::new()),
        get_calls: RefCell::new(Vec::new()),
    };
    let sync = FakeSyncClient {
        pulled: Err(BackendError::NotFound("unused".to_string())),
        pushes: RefCell::new(Vec::new()),
        pulls: RefCell::new(Vec::new()),
    };
    let archive = FakeArchiveClient {
        archives: RefCell::new(Vec::new()),
    };
    let sink = RepositoryBackedImportSink::new(&reader, &sync, &archive);

    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_eq!(summary.imported, 0);
    assert_eq!(summary.failed, 1);
    assert!(
        summary.results[0]
            .message
            .as_deref()
            .unwrap_or_default()
            .contains("backend already contains archived change")
    );
}

#[test]
fn archived_directory_with_empty_canonical_change_id_is_ignored() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes")).unwrap();
    std::fs::create_dir_all(ito_path.join("changes/archive/2026-03-10-")).unwrap();
    write_active_change(&ito_path, "024-18_active-example");

    let sink = RecordingSink::default();
    let summary = import_local_changes(&sink, &ito_path).unwrap();

    assert_summary_counts(&summary, 1, 0, 0);
    assert_eq!(sink.calls.into_inner().len(), 1);
}

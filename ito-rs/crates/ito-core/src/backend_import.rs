//! Local-to-backend import orchestration.

use std::path::{Path, PathBuf};

use ito_common::fs::StdFs;
use ito_common::paths;
use ito_domain::backend::{BackendArchiveClient, BackendChangeReader, BackendSyncClient};
use ito_domain::changes::ChangeLifecycleFilter;
use ito_domain::discovery;

use crate::ArtifactBundle;
use crate::backend_sync::read_bundle_from_change_dir;
use crate::errors::{CoreError, CoreResult};

/// Lifecycle state for a locally discovered change import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportLifecycle {
    /// The change is still active in `.ito/changes/`.
    Active,
    /// The change is archived under `.ito/changes/archive/`.
    Archived,
}

/// A local change ready to be imported into backend-managed state.
#[derive(Debug, Clone)]
pub struct LocalImportChange {
    /// Canonical change identifier.
    pub change_id: String,
    /// Source path on disk.
    pub source_path: PathBuf,
    /// Lifecycle state of the source change.
    pub lifecycle: ImportLifecycle,
    /// Artifact bundle prepared from local files.
    pub bundle: ArtifactBundle,
}

/// Sink result for a single import item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportAction {
    /// The item was imported successfully.
    Imported,
    /// The item would be imported during a dry run.
    Previewed,
    /// The item was intentionally skipped.
    Skipped(String),
}

/// Final status for one imported item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportItemStatus {
    /// The change was imported.
    Imported,
    /// The change would be imported during a dry run.
    Previewed,
    /// The change was skipped.
    Skipped,
    /// The change failed to import.
    Failed,
}

/// Outcome for a single processed import item.
#[derive(Debug, Clone)]
pub struct ImportItemResult {
    /// Canonical change identifier.
    pub change_id: String,
    /// Lifecycle state of the imported change.
    pub lifecycle: ImportLifecycle,
    /// Final status.
    pub status: ImportItemStatus,
    /// Additional detail for skipped or failed items.
    pub message: Option<String>,
}

/// Summary returned by the import orchestration.
#[derive(Debug, Clone)]
pub struct ImportSummary {
    /// Per-item results in deterministic processing order.
    pub results: Vec<ImportItemResult>,
    /// Number of imported items.
    pub imported: usize,
    /// Number of previewed items during a dry run.
    pub previewed: usize,
    /// Number of skipped items.
    pub skipped: usize,
    /// Number of failed items.
    pub failed: usize,
}

/// Output port for applying one imported change to backend-managed state.
pub trait BackendImportSink {
    /// Preview how a single local change would be handled without mutating backend state.
    fn preview_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction>;

    /// Import a single local change.
    fn import_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ImportPlan {
    Skip(String),
    Import {
        push_bundle: bool,
        mark_archived: bool,
    },
}

/// Import sink backed by the existing backend read/sync/archive ports.
pub struct RepositoryBackedImportSink<'a, R, S, A> {
    change_reader: &'a R,
    sync_client: &'a S,
    archive_client: &'a A,
}

impl<'a, R, S, A> RepositoryBackedImportSink<'a, R, S, A> {
    /// Create a sink backed by existing backend repository clients.
    pub fn new(change_reader: &'a R, sync_client: &'a S, archive_client: &'a A) -> Self {
        Self {
            change_reader,
            sync_client,
            archive_client,
        }
    }
}

impl<R, S, A> BackendImportSink for RepositoryBackedImportSink<'_, R, S, A>
where
    R: BackendChangeReader,
    S: BackendSyncClient,
    A: BackendArchiveClient,
{
    fn preview_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        match evaluate_import_plan(self.change_reader, self.sync_client, change)? {
            ImportPlan::Skip(message) => Ok(ImportAction::Skipped(message)),
            ImportPlan::Import { .. } => Ok(ImportAction::Previewed),
        }
    }

    fn import_change(&self, change: &LocalImportChange) -> CoreResult<ImportAction> {
        match evaluate_import_plan(self.change_reader, self.sync_client, change)? {
            ImportPlan::Import {
                push_bundle,
                mark_archived,
            } => {
                if push_bundle {
                    let mut bundle = change.bundle.clone();
                    bundle.revision.clear();
                    self.sync_client
                        .push(&change.change_id, &bundle)
                        .map_err(|err| CoreError::process(err.to_string()))?;
                }

                if mark_archived {
                    self.archive_client
                        .mark_archived(&change.change_id)
                        .map_err(|err| CoreError::process(err.to_string()))?;
                }

                Ok(ImportAction::Imported)
            }
            ImportPlan::Skip(message) => Ok(ImportAction::Skipped(message)),
        }
    }
}

/// Discover local active and archived changes, then import them through a sink.
pub fn import_local_changes(
    sink: &dyn BackendImportSink,
    ito_path: &Path,
) -> CoreResult<ImportSummary> {
    import_local_changes_with_options(sink, ito_path, false)
}

/// Discover local changes, then import or preview them through a sink.
pub fn import_local_changes_with_options(
    sink: &dyn BackendImportSink,
    ito_path: &Path,
    dry_run: bool,
) -> CoreResult<ImportSummary> {
    let changes = discover_local_import_changes(ito_path)?;
    let mut results = Vec::with_capacity(changes.len());
    let mut imported = 0;
    let mut previewed = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for change in changes {
        let outcome = if dry_run {
            sink.preview_change(&change)
        } else {
            sink.import_change(&change)
        };

        match outcome {
            Ok(ImportAction::Imported) => {
                imported += 1;
                results.push(ImportItemResult {
                    change_id: change.change_id,
                    lifecycle: change.lifecycle,
                    status: ImportItemStatus::Imported,
                    message: None,
                });
            }
            Ok(ImportAction::Previewed) => {
                previewed += 1;
                results.push(ImportItemResult {
                    change_id: change.change_id,
                    lifecycle: change.lifecycle,
                    status: ImportItemStatus::Previewed,
                    message: None,
                });
            }
            Ok(ImportAction::Skipped(message)) => {
                skipped += 1;
                results.push(ImportItemResult {
                    change_id: change.change_id,
                    lifecycle: change.lifecycle,
                    status: ImportItemStatus::Skipped,
                    message: Some(message),
                });
            }
            Err(err) => {
                failed += 1;
                results.push(ImportItemResult {
                    change_id: change.change_id,
                    lifecycle: change.lifecycle,
                    status: ImportItemStatus::Failed,
                    message: Some(err.to_string()),
                });
            }
        }
    }

    Ok(ImportSummary {
        results,
        imported,
        previewed,
        skipped,
        failed,
    })
}

fn evaluate_import_plan<R, S>(
    change_reader: &R,
    sync_client: &S,
    change: &LocalImportChange,
) -> CoreResult<ImportPlan>
where
    R: BackendChangeReader,
    S: BackendSyncClient,
{
    let active_exists = exists_in_backend(
        change_reader,
        &change.change_id,
        ChangeLifecycleFilter::Active,
    )?;
    let archived_exists = exists_in_backend(
        change_reader,
        &change.change_id,
        ChangeLifecycleFilter::Archived,
    )?;

    match change.lifecycle {
        ImportLifecycle::Active => {
            if archived_exists {
                return Err(CoreError::validation(format!(
                    "backend already contains archived change '{}' while local copy is active",
                    change.change_id
                )));
            }
            if !active_exists {
                return Ok(ImportPlan::Import {
                    push_bundle: true,
                    mark_archived: false,
                });
            }

            if remote_bundle_matches(sync_client, change)? {
                return Ok(ImportPlan::Skip("already imported".to_string()));
            }

            Ok(ImportPlan::Import {
                push_bundle: true,
                mark_archived: false,
            })
        }
        ImportLifecycle::Archived => {
            if archived_exists {
                return Ok(ImportPlan::Skip("already archived".to_string()));
            }

            if !active_exists {
                return Ok(ImportPlan::Import {
                    push_bundle: true,
                    mark_archived: true,
                });
            }

            let push_bundle = !remote_bundle_matches(sync_client, change)?;
            Ok(ImportPlan::Import {
                push_bundle,
                mark_archived: true,
            })
        }
    }
}

fn exists_in_backend<R>(
    change_reader: &R,
    change_id: &str,
    filter: ChangeLifecycleFilter,
) -> CoreResult<bool>
where
    R: BackendChangeReader,
{
    let changes = change_reader
        .list_changes(filter)
        .map_err(CoreError::from)?;
    Ok(changes.into_iter().any(|change| change.id == change_id))
}

fn remote_bundle_matches<S>(sync_client: &S, change: &LocalImportChange) -> CoreResult<bool>
where
    S: BackendSyncClient,
{
    let remote = sync_client
        .pull(&change.change_id)
        .map_err(|err| CoreError::process(err.to_string()))?;
    Ok(artifact_bundles_match(&remote, &change.bundle))
}

fn artifact_bundles_match(left: &ArtifactBundle, right: &ArtifactBundle) -> bool {
    left.change_id == right.change_id
        && left.proposal == right.proposal
        && left.design == right.design
        && left.tasks == right.tasks
        && left.specs == right.specs
}

fn discover_local_import_changes(ito_path: &Path) -> CoreResult<Vec<LocalImportChange>> {
    let fs = StdFs;
    let mut changes = Vec::new();

    for change_id in discovery::list_change_dir_names(&fs, ito_path)? {
        let path = paths::change_dir(ito_path, &change_id);
        let bundle = read_bundle_from_change_dir(&path, &change_id)?;
        changes.push(LocalImportChange {
            change_id,
            source_path: path,
            lifecycle: ImportLifecycle::Active,
            bundle,
        });
    }

    let archive_dir = paths::changes_archive_dir(ito_path);
    for archived_name in discovery::list_dir_names(&fs, &archive_dir)? {
        let canonical_change_id = canonical_archived_change_id(&archived_name)?;
        let path = archive_dir.join(&archived_name);
        let bundle = read_bundle_from_change_dir(&path, &canonical_change_id)?;
        changes.push(LocalImportChange {
            change_id: canonical_change_id,
            source_path: path,
            lifecycle: ImportLifecycle::Archived,
            bundle,
        });
    }

    changes.sort_by(|left, right| left.change_id.cmp(&right.change_id));
    Ok(changes)
}

fn canonical_archived_change_id(archived_name: &str) -> CoreResult<String> {
    let parts: Vec<&str> = archived_name.splitn(4, '-').collect();
    if parts.len() != 4
        || parts[0].len() != 4
        || parts[1].len() != 2
        || parts[2].len() != 2
        || !parts[0].chars().all(|ch| ch.is_ascii_digit())
        || !parts[1].chars().all(|ch| ch.is_ascii_digit())
        || !parts[2].chars().all(|ch| ch.is_ascii_digit())
    {
        return Err(CoreError::validation(format!(
            "Archived change directory has unexpected format: {archived_name}"
        )));
    }
    Ok(parts[3].to_string())
}

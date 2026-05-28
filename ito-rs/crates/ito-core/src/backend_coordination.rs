//! Backend coordination use-cases for CLI commands.
//!
//! Provides the business logic for claim, release, allocate, and sync
//! operations that the CLI adapter calls. Each function accepts trait
//! objects for the backend clients so the CLI can inject the concrete
//! implementation.

use std::path::Path;

use ito_domain::backend::{
    AllocateResult, ArchiveResult, ArtifactBundle, BackendArchiveClient, BackendLeaseClient,
    BackendSyncClient, ClaimResult, PushResult, ReleaseResult,
};

use crate::backend_sync::map_backend_error;
use crate::errors::{CoreError, CoreResult};

/// Claim a change lease through the backend.
pub fn claim_change(
    lease_client: &dyn BackendLeaseClient,
    change_id: &str,
) -> CoreResult<ClaimResult> {
    lease_client
        .claim(change_id)
        .map_err(|e| map_backend_error(e, "claim"))
}

/// Release a change lease through the backend.
pub fn release_change(
    lease_client: &dyn BackendLeaseClient,
    change_id: &str,
) -> CoreResult<ReleaseResult> {
    lease_client
        .release(change_id)
        .map_err(|e| map_backend_error(e, "release"))
}

/// Allocate the next available change through the backend.
pub fn allocate_change(lease_client: &dyn BackendLeaseClient) -> CoreResult<AllocateResult> {
    lease_client
        .allocate()
        .map_err(|e| map_backend_error(e, "allocate"))
}

/// Pull artifacts from the backend for a change.
pub fn sync_pull(
    sync_client: &dyn BackendSyncClient,
    ito_path: &std::path::Path,
    change_id: &str,
    backup_dir: &std::path::Path,
) -> CoreResult<ArtifactBundle> {
    crate::backend_sync::pull_artifacts(sync_client, ito_path, change_id, backup_dir)
}

/// Push local artifacts to the backend for a change.
pub fn sync_push(
    sync_client: &dyn BackendSyncClient,
    ito_path: &std::path::Path,
    change_id: &str,
    backup_dir: &std::path::Path,
) -> CoreResult<PushResult> {
    crate::backend_sync::push_artifacts(sync_client, ito_path, change_id, backup_dir)
}

/// Result of a backend-mode archive orchestration.
#[derive(Debug)]
pub struct BackendArchiveOutcome {
    /// Spec IDs that were copied to the main specs tree.
    pub specs_updated: Vec<String>,
    /// The archive folder name (date-prefixed).
    pub archive_name: String,
    /// Backend archive result with timestamp.
    pub backend_result: ArchiveResult,
}

/// Orchestrate a backend-mode archive for a change.
///
/// The flow is:
/// 1. Pull the final artifact bundle from the backend.
/// 2. Copy spec deltas to the main specs tree (unless `skip_specs`).
/// 3. Move the change to the archive directory.
/// 4. Mark the change as archived on the backend.
///
/// If step 4 fails, the local archive is already committed — the caller
/// should report the backend error but NOT roll back the local archive
/// (the local state is correct; the backend can be retried).
pub fn archive_with_backend(
    sync_client: &dyn BackendSyncClient,
    archive_client: &dyn BackendArchiveClient,
    ito_path: &Path,
    change_id: &str,
    backup_dir: &Path,
    skip_specs: bool,
) -> CoreResult<BackendArchiveOutcome> {
    // Step 1: Pull final artifacts from backend
    crate::backend_sync::pull_artifacts(sync_client, ito_path, change_id, backup_dir)?;

    // Step 2: Copy spec deltas to main specs tree
    let specs_updated = if skip_specs {
        Vec::new()
    } else {
        let spec_names = crate::archive::discover_change_specs(ito_path, change_id)?;
        crate::archive::copy_specs_to_main(ito_path, change_id, &spec_names)?
    };

    // Step 3: Move to archive
    let archive_name = crate::archive::generate_archive_name(change_id);
    crate::archive::move_to_archive(ito_path, change_id, &archive_name)?;

    // Step 4: Mark archived on backend
    let backend_result = archive_client
        .mark_archived(change_id)
        .map_err(|e| map_backend_error(e, "archive"))?;

    Ok(BackendArchiveOutcome {
        specs_updated,
        archive_name,
        backend_result,
    })
}

/// Check whether the given `CoreError` represents a backend availability failure.
///
/// The CLI can use this to suggest fallback to filesystem mode.
pub fn is_backend_unavailable(err: &CoreError) -> bool {
    match err {
        CoreError::Process(msg) => msg.contains("Backend unavailable"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "backend_coordination_tests.rs"]
mod backend_coordination_tests;

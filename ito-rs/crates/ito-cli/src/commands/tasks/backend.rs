use crate::cli::SyncAction;
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::backend_client::{BackendRuntime, resolve_backend_runtime};
use ito_core::backend_coordination;
use ito_core::backend_http::BackendHttpClient;
use ito_core::repository_runtime::PersistenceMode;

use super::support::{print_json, resolve_change_id};

/// Best-effort push of task artifacts to the backend after a mutation.
///
/// If backend mode is not enabled, this is a no-op. If the push fails,
/// the error is reported as a warning but does not fail the command
/// (the local mutation already succeeded).
pub(super) fn sync_after_mutation(rt: &Runtime, change_id: &str) {
    if let Ok(runtime) = rt.repository_runtime()
        && runtime.mode() == PersistenceMode::Remote
    {
        return;
    }
    let runtime = match try_backend_runtime(rt) {
        Ok(Some(runtime)) => runtime,
        Ok(None) => return, // Backend not enabled
        Err(e) => {
            eprintln!("Warning: backend sync skipped: {e}");
            return;
        }
    };

    let client = BackendHttpClient::new(runtime.clone());
    let ito_path = rt.ito_path();

    if let Err(err) =
        backend_coordination::sync_push(&client, ito_path, change_id, &runtime.backup_dir)
    {
        let msg = err.to_string();
        if msg.contains("not yet available on backend") {
            // Backend endpoints are not available yet; don't spam warnings on every mutation.
            return;
        }

        eprintln!(
            "Warning: backend sync after task mutation failed: {}. \
             Run 'ito tasks sync push {change_id}' manually.",
            err
        );
    }
}

/// Resolve backend runtime config, failing if backend mode is not enabled.
fn require_backend_runtime(rt: &Runtime) -> CliResult<BackendRuntime> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let config: ItoConfig = serde_json::from_value(merged)
        .map_err(|e| CliError::msg(format!("Invalid merged Ito config: {e}")))?;

    if !config.backend.enabled {
        return Err(CliError::msg(
            "Backend mode is not enabled. Set 'backend.enabled=true' in your Ito config.",
        ));
    }

    resolve_backend_runtime(&config.backend)
        .map_err(to_cli_error)?
        .ok_or_else(|| CliError::msg("Backend mode is enabled but runtime could not be resolved."))
}

/// Resolve backend runtime config, returning None if backend mode is not enabled.
fn try_backend_runtime(rt: &Runtime) -> CliResult<Option<BackendRuntime>> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let config: ItoConfig = match serde_json::from_value(merged) {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!("Skipping backend integration due to invalid config: {e}");
            eprintln!("Warning: backend integration skipped due to invalid config: {e}");
            return Ok(None);
        }
    };

    if !config.backend.enabled {
        return Ok(None);
    }

    resolve_backend_runtime(&config.backend).map_err(to_cli_error)
}

/// Stub backend lease client that returns not-implemented errors.
///
/// The actual HTTP client will be implemented when the backend adds
/// lease/allocation endpoints. For now the CLI surface is wired up
/// and will report a clear error.
struct StubLeaseClient;

impl ito_core::BackendLeaseClient for StubLeaseClient {
    fn claim(&self, change_id: &str) -> Result<ito_core::ClaimResult, ito_core::BackendError> {
        Err(ito_core::BackendError::Other(format!(
            "Lease endpoints not yet available on backend for change '{change_id}'"
        )))
    }

    fn release(&self, change_id: &str) -> Result<ito_core::ReleaseResult, ito_core::BackendError> {
        Err(ito_core::BackendError::Other(format!(
            "Lease endpoints not yet available on backend for change '{change_id}'"
        )))
    }

    fn allocate(&self) -> Result<ito_core::AllocateResult, ito_core::BackendError> {
        Err(ito_core::BackendError::Other(
            "Allocation endpoints not yet available on backend".to_string(),
        ))
    }
}

pub(super) fn handle_backend_claim(
    rt: &Runtime,
    change_id: &str,
    want_json: bool,
) -> CliResult<()> {
    let _runtime = require_backend_runtime(rt)?;
    let change_repo = rt
        .repository_runtime()
        .map_err(to_cli_error)?
        .repositories()
        .changes
        .as_ref();
    let change_id = resolve_change_id(change_repo, change_id)?;
    let client = StubLeaseClient;

    let result = backend_coordination::claim_change(&client, &change_id).map_err(to_cli_error)?;

    if want_json {
        return print_json(&serde_json::json!({
            "action": "claim",
            "change_id": result.change_id,
            "holder": result.holder,
            "expires_at": result.expires_at,
        }));
    }

    eprintln!(
        "✔ Change \"{}\" claimed by \"{}\"",
        result.change_id, result.holder
    );
    Ok(())
}

pub(super) fn handle_backend_release(
    rt: &Runtime,
    change_id: &str,
    want_json: bool,
) -> CliResult<()> {
    let _runtime = require_backend_runtime(rt)?;
    let change_repo = rt
        .repository_runtime()
        .map_err(to_cli_error)?
        .repositories()
        .changes
        .as_ref();
    let change_id = resolve_change_id(change_repo, change_id)?;
    let client = StubLeaseClient;

    let result = backend_coordination::release_change(&client, &change_id).map_err(to_cli_error)?;

    if want_json {
        return print_json(&serde_json::json!({
            "action": "release",
            "change_id": result.change_id,
        }));
    }

    eprintln!("✔ Change \"{}\" released", result.change_id);
    Ok(())
}

pub(super) fn handle_backend_allocate(rt: &Runtime, want_json: bool) -> CliResult<()> {
    let _runtime = require_backend_runtime(rt)?;
    let client = StubLeaseClient;

    let result = backend_coordination::allocate_change(&client).map_err(to_cli_error)?;

    if let Some(claim) = &result.claim {
        if want_json {
            return print_json(&serde_json::json!({
                "action": "allocate",
                "allocated": true,
                "change_id": claim.change_id,
                "holder": claim.holder,
                "expires_at": claim.expires_at,
            }));
        }

        eprintln!(
            "✔ Allocated change \"{}\" to \"{}\"",
            claim.change_id, claim.holder
        );
        return Ok(());
    }

    if want_json {
        return print_json(&serde_json::json!({
            "action": "allocate",
            "allocated": false,
        }));
    }

    eprintln!("No changes available for allocation.");
    Ok(())
}

pub(super) fn handle_backend_sync(
    rt: &Runtime,
    action: &SyncAction,
    want_json: bool,
) -> CliResult<()> {
    let runtime = require_backend_runtime(rt)?;
    let ito_path = rt.ito_path();
    let client = BackendHttpClient::new(runtime.clone());
    let change_repo = rt
        .repository_runtime()
        .map_err(to_cli_error)?
        .repositories()
        .changes
        .as_ref();

    match action {
        SyncAction::Pull { change_id } => {
            let change_id = resolve_change_id(change_repo, change_id)?;
            let bundle =
                backend_coordination::sync_pull(&client, ito_path, &change_id, &runtime.backup_dir)
                    .map_err(to_cli_error)?;

            if want_json {
                return print_json(&serde_json::json!({
                    "action": "sync_pull",
                    "change_id": bundle.change_id,
                    "revision": bundle.revision,
                    "artifacts": {
                        "proposal": bundle.proposal.is_some(),
                        "design": bundle.design.is_some(),
                        "tasks": bundle.tasks.is_some(),
                        "specs": bundle.specs.len(),
                    },
                }));
            }

            eprintln!(
                "✔ Pulled artifacts for \"{}\" (revision: {})",
                bundle.change_id, bundle.revision
            );
            Ok(())
        }
        SyncAction::Push { change_id } => {
            let change_id = resolve_change_id(change_repo, change_id)?;
            let result =
                backend_coordination::sync_push(&client, ito_path, &change_id, &runtime.backup_dir)
                    .map_err(to_cli_error)?;

            if want_json {
                return print_json(&serde_json::json!({
                    "action": "sync_push",
                    "change_id": result.change_id,
                    "new_revision": result.new_revision,
                }));
            }

            eprintln!(
                "✔ Pushed artifacts for \"{}\" (new revision: {})",
                result.change_id, result.new_revision
            );
            Ok(())
        }
    }
}

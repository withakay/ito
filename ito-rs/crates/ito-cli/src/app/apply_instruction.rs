use std::path::{Component, Path};

use ito_config::ConfigContext;
use ito_core::implementation_readiness::{
    AuthoritativeChangeSource, ReadinessPhase, ReadinessReport, ReadinessRequest,
    evaluate_execute_from_prepare,
};
use ito_core::templates::{self as core_templates, ApplyInstructionsResponse, TemplatesError};

use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;

pub(super) struct PreparedApplySource {
    report: ReadinessReport,
    source: AuthoritativeChangeSource,
}

impl PreparedApplySource {
    pub(super) fn source(&self) -> &AuthoritativeChangeSource {
        &self.source
    }
}

pub(super) fn reject_schema_override(artifact: &str, schema: Option<&str>) -> CliResult<()> {
    if artifact == "apply" && schema.is_some() {
        return fail(
            "apply instructions do not accept --schema; the accepted proposal's authoritative .ito.yaml selects the schema",
        );
    }
    Ok(())
}

pub(super) fn prepare_source(
    rt: &Runtime,
    change: &str,
    json: bool,
) -> CliResult<PreparedApplySource> {
    let report =
        super::change::require_runtime_readiness(rt, change, ReadinessPhase::Prepare, json)?;
    from_prepare(rt, report, &["apply"], json)
}

pub(super) fn from_prepare(
    rt: &Runtime,
    report: ReadinessReport,
    guidance_artifacts: &[&str],
    json: bool,
) -> CliResult<PreparedApplySource> {
    let source =
        super::change::require_authoritative_render_source(rt, &report, guidance_artifacts, json)?;
    Ok(PreparedApplySource { report, source })
}

pub(super) fn compute(
    prepared: &mut PreparedApplySource,
    rt: &Runtime,
) -> CliResult<ApplyInstructionsResponse> {
    let ctx = rt.ctx();
    let mut apply = compute_from_authority(prepared.source(), ctx)?;
    if overlay_execute_ready_tracking(prepared, rt, &apply)? {
        apply = compute_from_authority(prepared.source(), ctx)?;
    }
    make_worktree_relative_paths(
        &mut apply,
        prepared.source().ito_path(),
        prepared.source().change_id(),
    )?;
    Ok(apply)
}

fn compute_from_authority(
    source: &AuthoritativeChangeSource,
    ctx: &ConfigContext,
) -> CliResult<ApplyInstructionsResponse> {
    let mut authority_ctx = ctx.clone();
    authority_ctx.project_dir = Some(source.project_root().to_path_buf());
    let apply = match core_templates::compute_apply_instructions(
        source.ito_path(),
        source.change_id(),
        None,
        &authority_ctx,
    ) {
        Ok(response) => response,
        Err(TemplatesError::InvalidChangeName) => return fail("Invalid change name"),
        Err(TemplatesError::ChangeNotFound(name)) => {
            return fail(format!("Change '{name}' not found"));
        }
        Err(TemplatesError::SchemaNotFound(name)) => {
            return fail(super::common::schema_not_found_message(ctx, &name));
        }
        Err(error) => return Err(to_cli_error(error)),
    };
    Ok(apply)
}

fn overlay_execute_ready_tracking(
    prepared: &mut PreparedApplySource,
    rt: &Runtime,
    apply: &ApplyInstructionsResponse,
) -> CliResult<bool> {
    let Some(tracks_file) = apply.tracks_file.as_deref() else {
        return Ok(false);
    };
    if !safe_tracking_filename(tracks_file) {
        return fail(format!(
            "Authority schema selected unsafe apply tracking filename '{tracks_file}'"
        ));
    }

    let request = ReadinessRequest::new(
        prepared.source.change_id(),
        ReadinessPhase::Execute,
        rt.cwd(),
    )
    .with_current_checkout(rt.cwd());
    let config = rt.typed_config().map_err(to_cli_error)?;
    let execute = evaluate_execute_from_prepare(&prepared.report, &request, &config);
    if !execute.ready {
        return Ok(false);
    }

    let checkout_tracking = rt
        .ito_path()
        .join("changes")
        .join(prepared.source.change_id())
        .join(tracks_file);
    let authority_tracking = prepared
        .source
        .ito_path()
        .join("changes")
        .join(prepared.source.change_id())
        .join(tracks_file);
    match std::fs::symlink_metadata(&checkout_tracking) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            let contents = std::fs::read(&checkout_tracking).map_err(|error| {
                to_cli_error(format!(
                    "Cannot read execute-ready tracking file '{}': {error}",
                    checkout_tracking.display()
                ))
            })?;
            std::fs::write(&authority_tracking, contents).map_err(|error| {
                to_cli_error(format!(
                    "Cannot stage live task progress for apply instructions: {error}"
                ))
            })?;
        }
        Ok(_) => {
            return fail(format!(
                "Execute-ready tracking path '{}' must be a regular file",
                checkout_tracking.display()
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match std::fs::remove_file(&authority_tracking) {
                Ok(()) => {}
                Err(remove_error) if remove_error.kind() == std::io::ErrorKind::NotFound => {}
                Err(remove_error) => {
                    return fail(format!(
                        "Cannot stage missing live task progress for apply instructions: {remove_error}"
                    ));
                }
            }
        }
        Err(error) => {
            return fail(format!(
                "Cannot inspect execute-ready tracking file '{}': {error}",
                checkout_tracking.display()
            ));
        }
    }
    Ok(true)
}

fn safe_tracking_filename(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && !value.contains('/')
        && !value.contains('\\')
        && !value.contains("..")
        && !value.contains('\0')
}

fn make_worktree_relative_paths(
    apply: &mut ApplyInstructionsResponse,
    source_ito_path: &Path,
    change_id: &str,
) -> CliResult<()> {
    let source_change = source_ito_path.join("changes").join(change_id);
    let worktree_change = Path::new(".ito").join("changes").join(change_id);
    let rebase = |path: &str| worktree_relative_path(&source_change, &worktree_change, path);
    apply.change_dir = worktree_change.to_string_lossy().to_string();
    apply.tracks_path = apply.tracks_path.as_deref().map(&rebase).transpose()?;
    for path in apply.context_files.values_mut() {
        *path = rebase(path)?;
    }
    Ok(())
}

fn worktree_relative_path(
    source_change: &Path,
    worktree_change: &Path,
    value: &str,
) -> CliResult<String> {
    let path = Path::new(value);
    let relative = path.strip_prefix(source_change).map_err(|_| {
        to_cli_error(format!(
            "Authoritative apply path '{}' escaped the accepted change directory",
            path.display()
        ))
    })?;
    if relative.as_os_str().is_empty()
        || !relative
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return fail(format!(
            "Authoritative apply path '{}' contains unsafe relative components",
            path.display()
        ));
    }
    Ok(worktree_change.join(relative).to_string_lossy().to_string())
}

#[cfg(test)]
#[path = "apply_instruction_tests.rs"]
mod tests;

use crate::cli::ArchiveArgs;
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::backend_client::{BackendRuntime, resolve_backend_runtime};
use ito_core::backend_coordination;
use ito_core::backend_http::BackendHttpClient;
use ito_core::paths as core_paths;

pub(crate) fn handle_archive(rt: &Runtime, args: &[String]) -> CliResult<()> {
    use ito_core::archive;

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["archive"], "ito archive")
        );
        return Ok(());
    }

    let ito_path = rt.ito_path();
    let changes_dir = core_paths::changes_dir(ito_path);
    let repository_runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let remote_mode =
        repository_runtime.mode() == ito_core::repository_runtime::PersistenceMode::Remote;

    if !changes_dir.exists() && !remote_mode {
        return fail("No Ito changes directory found. Run 'ito init' first.");
    }

    // Parse options
    let skip_validation = args.iter().any(|a| a == "--no-validate");
    let skip_specs = args.iter().any(|a| a == "--skip-specs");
    let auto_confirm = args.iter().any(|a| a == "--yes" || a == "-y");

    // Get change name (first positional arg)
    let change_name = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(|s| s.as_str());

    // If no change specified, list available changes and prompt for selection
    let runtime = repository_runtime;
    let change_repo = runtime.repositories().changes.as_ref();
    let change_name = if let Some(name) = change_name {
        match super::common::resolve_change_target(change_repo, name) {
            Ok(resolved) => resolved,
            Err(msg) => return fail(msg),
        }
    } else {
        let available = change_repo.list().unwrap_or_default();
        if available.is_empty() {
            return fail("No changes found to archive.");
        }

        println!("Available changes:");
        for (idx, change) in available.iter().enumerate() {
            println!("  {}. {}", idx + 1, change.id);
        }
        println!();

        // Simple selection (in a real implementation, would use interactive prompt)
        // For now, just fail with message
        return fail("Please specify a change name: ito archive <change-name>");
    };

    // Verify change exists
    if !change_repo.exists(&change_name) {
        return fail(format!("Change '{}' not found", change_name));
    }

    // Check task completion unless skipping validation
    if !skip_validation {
        let task_repo = runtime.repositories().tasks.as_ref();
        let (completed, total) = task_repo.get_task_counts(&change_name).unwrap_or((0, 0));
        if total > 0 {
            if completed < total {
                let pending = total - completed;
                println!(
                    "Warning: Change has {} incomplete tasks out of {}",
                    pending, total
                );
                if !auto_confirm {
                    println!("Continue with archive anyway? [y/N]: ");
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| CliError::msg("Failed to read input"))?;
                    let input = input.trim().to_lowercase();
                    if input != "y" && input != "yes" {
                        println!("Archive cancelled.");
                        return Ok(());
                    }
                }
            } else {
                eprintln!("✔ All tasks complete");
            }
        }
    }

    // Check for backend mode — if enabled, run the repository-backed archive flow.
    if let Some(runtime) = try_backend_runtime(rt)? {
        return handle_backend_archive(rt, ito_path, &change_name, skip_specs, &runtime);
    }

    // ── Filesystem-only archive flow ───────────────────────────────

    // Generate archive name
    let archive_name = archive::generate_archive_name(&change_name);

    // Check if archive already exists
    if archive::archive_exists(ito_path, &archive_name) {
        return fail(format!("Archive '{}' already exists", archive_name));
    }

    let mut specs_updated: Vec<String> = Vec::new();

    // Handle spec updates unless skipped
    if !skip_specs {
        let spec_names =
            archive::discover_change_specs(ito_path, &change_name).map_err(to_cli_error)?;

        if !spec_names.is_empty() {
            let (new_specs, existing_specs) = archive::categorize_specs(ito_path, &spec_names);

            // Show confirmation
            if !new_specs.is_empty() || !existing_specs.is_empty() {
                println!("The following specs will be updated:");
                println!();

                if !new_specs.is_empty() {
                    println!("NEW specs to be created:");
                    for spec in &new_specs {
                        println!("  - {}", spec);
                    }
                    println!();
                }

                if !existing_specs.is_empty() {
                    println!("EXISTING specs to be updated:");
                    for spec in &existing_specs {
                        println!("  - {}", spec);
                    }
                    println!();
                }

                if !auto_confirm {
                    println!(
                        "Update {} specs and archive '{}'? [y/N]: ",
                        spec_names.len(),
                        change_name
                    );
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| CliError::msg("Failed to read input"))?;
                    let input = input.trim().to_lowercase();
                    if input != "y" && input != "yes" {
                        println!("Skipping spec updates, continuing with archive...");
                    } else {
                        // Copy specs to main
                        specs_updated =
                            archive::copy_specs_to_main(ito_path, &change_name, &spec_names)
                                .map_err(to_cli_error)?;
                        eprintln!("✔ Updated {} specs", specs_updated.len());
                    }
                } else {
                    // Copy specs to main
                    specs_updated =
                        archive::copy_specs_to_main(ito_path, &change_name, &spec_names)
                            .map_err(to_cli_error)?;
                    eprintln!("✔ Updated {} specs", specs_updated.len());
                }
            }
        }
    }

    // Audit pre-check: warn about drift but don't block archiving
    {
        let audit_report = ito_core::audit::run_reconcile(ito_path, Some(&change_name), false);
        if !audit_report.drifts.is_empty() {
            eprintln!(
                "Warning: {} audit drift items detected for '{}'. Run 'ito audit reconcile --change {} --fix' to resolve.",
                audit_report.drifts.len(),
                change_name,
                change_name
            );
        }
    }

    // Emit audit events BEFORE the directory move
    // change.archive event
    if let Some(event) = AuditEventBuilder::new()
        .entity(EntityType::Change)
        .entity_id(&change_name)
        .op(ops::CHANGE_ARCHIVE)
        .actor(Actor::Cli)
        .by(rt.user_identity())
        .meta(serde_json::json!({
            "archive_name": archive_name,
        }))
        .ctx(rt.event_context().clone())
        .build()
    {
        rt.emit_audit_event(&event);
    }

    // module.change_completed event
    // Extract module_id from change_name (format: "NNN-NN_slug")
    if let Some(module_id) = change_name.split('-').next()
        && let Some(event) = AuditEventBuilder::new()
            .entity(EntityType::Module)
            .entity_id(module_id)
            .op(ops::MODULE_CHANGE_COMPLETED)
            .actor(Actor::Cli)
            .by(rt.user_identity())
            .meta(serde_json::json!({
                "change_id": change_name,
            }))
            .ctx(rt.event_context().clone())
            .build()
    {
        rt.emit_audit_event(&event);
    }

    // Move to archive
    archive::mark_change_complete_in_module_markdown(ito_path, &change_name)
        .map_err(to_cli_error)?;
    archive::move_to_archive(ito_path, &change_name, &archive_name).map_err(to_cli_error)?;

    eprintln!("✔ Archived '{}' as '{}'", change_name, archive_name);
    if !specs_updated.is_empty() {
        eprintln!("  Updated specs: {}", specs_updated.join(", "));
    }

    Ok(())
}

/// Dispatches the `ito archive` command from parsed clap arguments.
///
/// Routes to batch mode when `--completed` is set; otherwise archives a single change
/// via the legacy raw-args handler.
///
/// # Examples
///
/// ```no_run
/// # use crate::{Runtime, ArchiveArgs, handle_archive_clap};
/// let rt = Runtime::new();
/// let args = ArchiveArgs { completed: false, change: Some("module-123".into()), ..Default::default() };
/// handle_archive_clap(&rt, &args).unwrap();
/// ```
pub(crate) fn handle_archive_clap(rt: &Runtime, args: &ArchiveArgs) -> CliResult<()> {
    if args.completed {
        return handle_archive_completed(rt, args);
    }

    let change_id = args.change_flag.as_deref().or(args.change.as_deref());
    let argv = build_single_archive_argv(change_id, args);
    handle_archive(rt, &argv)
}

/// Build the argv vector for a single-change archive invocation.
///
/// Combines an optional change id with the shared flags (`-y`, `--skip-specs`,
/// `--no-validate`) from `ArchiveArgs`.
fn build_single_archive_argv(change_id: Option<&str>, args: &ArchiveArgs) -> Vec<String> {
    let mut argv: Vec<String> = Vec::new();
    if let Some(id) = change_id {
        argv.push(id.to_string());
    }
    if args.yes {
        argv.push("-y".to_string());
    }
    if args.skip_specs {
        argv.push("--skip-specs".to_string());
    }
    if args.no_validate {
        argv.push("--no-validate".to_string());
    }
    argv
}

// ── Backend-mode helpers ────────────────────────────────────────────

/// Try to resolve backend runtime from config.
///
/// Returns `Ok(None)` if backend mode is disabled (no error). Returns
/// `Err` only if backend is enabled but misconfigured.
fn try_backend_runtime(rt: &Runtime) -> CliResult<Option<BackendRuntime>> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let config: ItoConfig = serde_json::from_value(merged).map_err(to_cli_error)?;

    if !config.backend.enabled {
        return Ok(None);
    }

    resolve_backend_runtime(&config.backend).map_err(to_cli_error)
}

/// Backend-mode archive: pull from backend, archive locally, mark archived on backend.
fn handle_backend_archive(
    rt: &Runtime,
    ito_path: &std::path::Path,
    change_name: &str,
    skip_specs: bool,
    runtime: &BackendRuntime,
) -> CliResult<()> {
    eprintln!("Backend mode enabled — syncing from backend before archiving...");

    let client = BackendHttpClient::new(runtime.clone());

    // Audit pre-check
    {
        let audit_report = ito_core::audit::run_reconcile(ito_path, Some(change_name), false);
        if !audit_report.drifts.is_empty() {
            eprintln!(
                "Warning: {} audit drift items detected for '{}'. Run 'ito audit reconcile --change {} --fix' to resolve.",
                audit_report.drifts.len(),
                change_name,
                change_name
            );
        }
    }

    // Run the backend-mode archive orchestration
    let outcome = backend_coordination::archive_with_backend(
        &client,
        &client,
        ito_path,
        change_name,
        &runtime.backup_dir,
        skip_specs,
    )
    .map_err(to_cli_error)?;

    // Emit audit events
    if let Some(event) = AuditEventBuilder::new()
        .entity(EntityType::Change)
        .entity_id(change_name)
        .op(ops::CHANGE_ARCHIVE)
        .actor(Actor::Cli)
        .by(rt.user_identity())
        .meta(serde_json::json!({
            "archive_name": outcome.archive_name,
            "backend_archived_at": outcome.backend_result.archived_at,
        }))
        .ctx(rt.event_context().clone())
        .build()
    {
        rt.emit_audit_event(&event);
    }

    if let Some(module_id) = change_name.split('-').next()
        && let Some(event) = AuditEventBuilder::new()
            .entity(EntityType::Module)
            .entity_id(module_id)
            .op(ops::MODULE_CHANGE_COMPLETED)
            .actor(Actor::Cli)
            .by(rt.user_identity())
            .meta(serde_json::json!({
                "change_id": change_name,
            }))
            .ctx(rt.event_context().clone())
            .build()
    {
        rt.emit_audit_event(&event);
    }

    // Report success
    eprintln!(
        "✔ Archived '{}' as '{}' (backend archived at {})",
        change_name, outcome.archive_name, outcome.backend_result.archived_at
    );
    if !outcome.specs_updated.is_empty() {
        eprintln!("  Updated specs: {}", outcome.specs_updated.join(", "));
    }

    // Post-archive commit reminder
    eprintln!();
    eprintln!("Next steps:");
    eprintln!(
        "  git add .ito/changes/archive/{} .ito/specs/",
        outcome.archive_name
    );
    eprintln!("  git commit -m \"chore: archive {}\"", change_name);

    Ok(())
}

/// Archive all changes with `ChangeStatus::Complete`.
///
/// Discovers completed changes via the repository runtime, then
/// archives each one sequentially using the existing single-change flow.
/// Reports per-change progress and a summary on completion.
fn handle_archive_completed(rt: &Runtime, args: &ArchiveArgs) -> CliResult<()> {
    let ito_path = rt.ito_path();
    let changes_dir = core_paths::changes_dir(ito_path);
    let runtime = rt.repository_runtime().map_err(to_cli_error)?;
    let remote_mode = runtime.mode() == ito_core::repository_runtime::PersistenceMode::Remote;

    if !changes_dir.exists() && !remote_mode {
        return fail("No Ito changes directory found. Run 'ito init' first.");
    }

    let completed = runtime
        .repositories()
        .changes
        .list_complete()
        .map_err(to_cli_error)?;

    if completed.is_empty() {
        eprintln!("No completed changes to archive.");
        return Ok(());
    }

    if !args.yes {
        eprintln!("Completed changes ready to archive:");
        for summary in &completed {
            eprintln!("  - {}", summary.id);
        }
        eprint!("Archive {} completed change(s)? [y/N]: ", completed.len());

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|_| CliError::msg("Failed to read input"))?;
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            eprintln!("Archive cancelled.");
            return Ok(());
        }
    }

    let mut archived: Vec<String> = Vec::new();
    let mut failed: Vec<(String, String)> = Vec::new();

    for summary in &completed {
        let change_id = &summary.id;
        eprintln!("Archiving '{}'...", change_id);

        let argv = build_single_archive_argv(Some(change_id), args);
        match handle_archive(rt, &argv) {
            Ok(()) => archived.push(change_id.clone()),
            Err(e) => {
                let msg = format!("{e}");
                eprintln!("  ✖ Failed to archive '{}': {}", change_id, msg);
                failed.push((change_id.clone(), msg));
            }
        }
    }

    // Print summary.
    if failed.is_empty() {
        eprintln!("Archived {} change(s).", archived.len());
    } else {
        eprintln!(
            "Archived {} change(s), {} failed.",
            archived.len(),
            failed.len()
        );
    }

    if !failed.is_empty() {
        return fail(format!("Failed to archive {} change(s)", failed.len()));
    }

    Ok(())
}

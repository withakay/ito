use crate::cli::{CreateAction, CreateArgs, NewAction, NewArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::commands::sync::best_effort_sync_coordination;
use crate::runtime::Runtime;
use crate::util::{parse_string_flag, split_csv};
use ito_config::{load_cascading_project_config, resolve_coordination_branch_settings};
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::coordination_worktree::maybe_auto_commit_coordination;
use ito_core::create::{create_change_in_sub_module, create_sub_module};
use ito_core::git::reserve_change_on_coordination_branch;
use ito_core::repository_runtime::PersistenceMode;
use ito_core::{create as core_create, templates as core_templates};
use std::path::Path;

/// Return an error if the runtime is in remote persistence mode.
///
/// Several creation operations (sub-module changes, sub-module creation) write
/// directly to the local filesystem and must not be used when a remote backend
/// is active. Call this guard before performing any local-only write.
fn guard_local_only(rt: &Runtime, operation: &str) -> CliResult<()> {
    if let Ok(repo_rt) = rt.repository_runtime()
        && repo_rt.mode() == PersistenceMode::Remote
    {
        return fail(format!(
            "{operation} is a local-only operation and cannot be used when remote persistence is \
             active.\nTo proceed locally, disable backend mode by removing 'backend.enabled' from \
             your .ito/config.json."
        ));
    }
    Ok(())
}

fn auto_commit_after_change_creation(ito_path: &Path, change_id: &str) {
    let project_root = ito_path.parent().unwrap_or(ito_path);
    if let Err(err) = maybe_auto_commit_coordination(
        project_root,
        ito_path,
        &format!("chore: create change {change_id}"),
    ) {
        eprintln!("Warning: auto-commit to coordination worktree failed: {err}");
    }
}

fn coordination_branch_settings(rt: &Runtime) -> (bool, String) {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    resolve_coordination_branch_settings(&merged)
}

/// Prints a concise, structured summary about a newly created change to stderr.
///
/// The message includes the change path under the repository `changes` directory,
/// the schema, the files created, module information, and suggested next steps.
///
/// # Parameters
///
/// - `ito_path`: Absolute path to the repository root that contains the `changes` directory.
/// - `change_id`: Identifier of the newly created change.
/// - `schema`: Schema name associated with the change.
/// - `module_id`: Module identifier associated with the change (e.g., "000" for default).
/// - `module_was_explicit`: `true` if the module was explicitly provided via `--module`, `false` if the default module was used.
/// - `has_description`: `true` if a README.md description file was created for the change.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // Prints a summary for change "chg-123" under "/tmp/ito"
/// print_change_created_message(Path::new("/tmp/ito"), "chg-123", "default_schema", "000", false, true);
/// ```
fn print_change_created_message(
    ito_path: &Path,
    change_id: &str,
    schema: &str,
    module_id: &str,
    module_was_explicit: bool,
    has_description: bool,
) {
    debug_assert!(
        ito_path.is_absolute(),
        "ito_path should already be absolute"
    );
    let changes_dir = ito_path.join("changes").join(change_id);
    eprintln!("✔ Created change '{change_id}'");
    eprintln!("  Path: {}", changes_dir.display());
    eprintln!("  Schema: {schema}");
    eprintln!("  Created files:");
    eprintln!("    - .ito.yaml");
    if has_description {
        eprintln!("    - README.md");
    }
    if module_was_explicit {
        eprintln!("  Module: {module_id} (from --module)");
    } else {
        eprintln!("  Module: {module_id} (default)");
    }
    eprintln!("  Next steps:");
    eprintln!("    1) ito agent instruction proposal --change {change_id}");
    eprintln!("    2) ito agent instruction tasks --change {change_id}");
    eprintln!("    3) ito validate {change_id} --strict");
}

pub(crate) fn handle_create_clap(rt: &Runtime, args: &CreateArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        // Preserve legacy behavior: `ito create` errors.
        return fail("Missing required argument <type>");
    };

    let forwarded: Vec<String> = match action {
        CreateAction::Module {
            name,
            scope,
            depends_on,
            description,
        } => {
            let mut out = vec!["module".to_string()];
            if let Some(name) = name {
                out.push(name.clone());
            }
            if let Some(scope) = scope {
                out.push("--scope".to_string());
                out.push(scope.clone());
            }
            if let Some(depends_on) = depends_on {
                out.push("--depends-on".to_string());
                out.push(depends_on.clone());
            }
            if let Some(description) = description {
                out.push("--description".to_string());
                out.push(description.clone());
            }
            out
        }
        CreateAction::Change {
            name,
            schema,
            module,
            sub_module,
            description,
        } => {
            let mut out = vec!["change".to_string()];
            if let Some(name) = name {
                out.push(name.clone());
            }
            if let Some(schema) = schema {
                out.push("--schema".to_string());
                out.push(schema.clone());
            }
            if let Some(module) = module {
                out.push("--module".to_string());
                out.push(module.clone());
            }
            if let Some(sub_module) = sub_module {
                out.push("--sub-module".to_string());
                out.push(sub_module.clone());
            }
            if let Some(description) = description {
                out.push("--description".to_string());
                out.push(description.clone());
            }
            out
        }
        CreateAction::SubModule {
            name,
            module,
            description,
        } => {
            let mut out = vec!["sub-module".to_string()];
            if let Some(name) = name {
                out.push(name.clone());
            }
            if let Some(module) = module {
                out.push("--module".to_string());
                out.push(module.clone());
            }
            if let Some(description) = description {
                out.push("--description".to_string());
                out.push(description.clone());
            }
            out
        }
        CreateAction::External(rest) => rest.clone(),
    };

    handle_create(rt, &forwarded)
}

/// Create a module or a change based on the provided CLI tokens.
///
/// Processes a slice of tokens where the first token selects the create kind (`"module"` or `"change"`)
/// and subsequent tokens provide the name and optional flags. When invoked it will create the requested
/// entity, emit audit events, print a concise summary to stdout/stderr, and — for changes — optionally
/// coordinate with a remote coordination branch (reservation) when enabled.
///
/// # Examples
///
/// ```no_run
/// // Invocation shape example:
/// let runtime = /* obtain Runtime */;
/// handle_create(&runtime, &["change".to_string(), "my-change".to_string(), "--schema".to_string(), "api".to_string()]);
/// ```
pub(crate) fn handle_create(rt: &Runtime, args: &[String]) -> CliResult<()> {
    let Some(kind) = args.first().map(|s| s.as_str()) else {
        return fail("Missing required argument <type>");
    };

    let ito_path = rt.ito_path();

    match kind {
        "module" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if name.is_empty() || name.starts_with('-') {
                return fail("Missing required argument <name>");
            }
            let scope = parse_string_flag(args, "--scope")
                .map(|raw| split_csv(&raw))
                .unwrap_or_else(|| vec!["*".to_string()]);
            let depends_on = parse_string_flag(args, "--depends-on")
                .map(|raw| split_csv(&raw))
                .unwrap_or_default();
            let description = parse_string_flag(args, "--description");

            let r = core_create::create_module(
                ito_path,
                name,
                scope,
                depends_on,
                description.as_deref(),
            )
            .map_err(to_cli_error)?;
            if !r.created {
                println!("Module \"{}\" already exists as {}", name, r.folder_name);
                return Ok(());
            }

            // Emit audit event for module creation
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Module)
                .entity_id(&r.folder_name)
                .op(ops::MODULE_CREATE)
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            println!("Created module: {}", r.folder_name);
            println!("  Path: {}", r.module_dir.display());
            println!("  Edit: {}", r.module_dir.join("module.md").display());
            Ok(())
        }
        "change" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if name.is_empty() || name.starts_with('-') {
                return fail("Missing required argument <name>");
            }
            let schema_opt = parse_string_flag(args, "--schema");
            let schema = schema_opt
                .clone()
                .unwrap_or_else(|| core_templates::default_schema_name().to_string());
            let module = parse_string_flag(args, "--module");
            let sub_module = parse_string_flag(args, "--sub-module");
            let description = parse_string_flag(args, "--description");

            // --module and --sub-module are mutually exclusive (belt-and-suspenders
            // guard in case the token-forwarding path bypasses clap validation).
            if module.is_some() && sub_module.is_some() {
                return fail("--module and --sub-module are mutually exclusive");
            }

            // Sub-module change creation is a local-only operation: it writes
            // directly to the filesystem. Reject it when remote persistence is
            // active so the user gets an actionable error instead of a silent
            // no-op or a confusing partial write.
            if sub_module.is_some() {
                guard_local_only(rt, "ito create change --sub-module")?;
            }

            // Derive a display-friendly namespace label for the spinner message.
            let (namespace_display, module_id_for_audit) = if let Some(ref sm) = sub_module {
                (sm.clone(), sm.clone())
            } else {
                let mid = module
                    .as_deref()
                    .and_then(|m| {
                        ito_core::parse_module_id(m)
                            .ok()
                            .map(|p| p.module_id.to_string())
                    })
                    .unwrap_or_else(|| "000".to_string());
                (mid.clone(), mid)
            };

            let schema_display = if schema_opt.is_some() {
                format!(" with schema '{}'", schema)
            } else {
                String::new()
            };

            // Match TS/ora: spinner output is written to stderr.
            eprintln!(
                "- Creating change '{}' in module {}{}...",
                name, namespace_display, schema_display
            );

            let (coord_enabled, coord_branch) = coordination_branch_settings(rt);
            best_effort_sync_coordination(rt, "before create");

            let create_result = if let Some(ref sm) = sub_module {
                create_change_in_sub_module(ito_path, name, &schema, sm, description.as_deref())
            } else {
                core_create::create_change(
                    ito_path,
                    name,
                    &schema,
                    module.as_deref(),
                    description.as_deref(),
                )
            };

            match create_result {
                Ok(r) => {
                    // Emit audit event for change creation
                    if let Some(event) = AuditEventBuilder::new()
                        .entity(EntityType::Change)
                        .entity_id(&r.change_id)
                        .op(ops::CHANGE_CREATE)
                        .actor(Actor::Cli)
                        .by(rt.user_identity())
                        .meta(serde_json::json!({
                            "schema": schema,
                            "module": module_id_for_audit,
                        }))
                        .ctx(rt.event_context().clone())
                        .build()
                    {
                        rt.emit_audit_event(&event);
                    }

                    // Emit module.change_added event if change belongs to a non-default module
                    if module_id_for_audit != "000"
                        && let Some(event) = AuditEventBuilder::new()
                            .entity(EntityType::Module)
                            .entity_id(&module_id_for_audit)
                            .op(ops::MODULE_CHANGE_ADDED)
                            .to(&r.change_id)
                            .actor(Actor::Cli)
                            .by(rt.user_identity())
                            .meta(serde_json::json!({
                                "change_id": r.change_id,
                            }))
                            .ctx(rt.event_context().clone())
                            .build()
                    {
                        rt.emit_audit_event(&event);
                    }

                    print_change_created_message(
                        ito_path,
                        &r.change_id,
                        &schema,
                        &module_id_for_audit,
                        module.is_some() || sub_module.is_some(),
                        description.is_some(),
                    );

                    if coord_enabled {
                        let project_root = ito_path.parent().unwrap_or(ito_path);
                        if let Err(err) = reserve_change_on_coordination_branch(
                            project_root,
                            ito_path,
                            &r.change_id,
                            &coord_branch,
                        ) {
                            return fail(format!(
                                "Created local change '{}' but failed to reserve it on coordination branch '{}': {}",
                                r.change_id, coord_branch, err.message
                            ));
                        }
                    }

                    // Best-effort auto-commit to coordination worktree.
                    auto_commit_after_change_creation(ito_path, &r.change_id);
                    best_effort_sync_coordination(rt, "after create");

                    Ok(())
                }
                Err(e) => Err(to_cli_error(e)),
            }
        }
        "sub-module" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if name.is_empty() || name.starts_with('-') {
                return fail("Missing required argument <name>");
            }
            let module = parse_string_flag(args, "--module");
            let description = parse_string_flag(args, "--description");

            let Some(parent_module) = module.as_deref() else {
                return fail("Missing required flag --module <id>");
            };

            // sub-module creation is a local-only operation.
            guard_local_only(rt, "ito create sub-module")?;

            eprintln!(
                "- Creating sub-module '{}' under module {}...",
                name, parent_module
            );

            match create_sub_module(ito_path, name, parent_module, description.as_deref()) {
                Ok(r) => {
                    eprintln!(
                        "✔ Created sub-module '{}' ({}) under module {}",
                        r.sub_module_id, r.sub_module_name, r.parent_module_id
                    );
                    eprintln!("  Path: {}", r.sub_module_dir.display());
                    eprintln!("  Edit: {}", r.sub_module_dir.join("module.md").display());
                    eprintln!("  Next steps:");
                    eprintln!(
                        "    1) ito create change <name> --sub-module {}",
                        r.sub_module_id
                    );
                    eprintln!("    2) ito show sub-module {}", r.sub_module_id);
                    Ok(())
                }
                Err(e) => Err(to_cli_error(e)),
            }
        }
        _ => fail(format!("Unknown create type '{kind}'")),
    }
}

pub(crate) fn handle_new(rt: &Runtime, args: &[String]) -> CliResult<()> {
    let Some(kind) = args.first().map(|s| s.as_str()) else {
        return fail("Missing required argument <type>");
    };
    if kind != "change" {
        return fail(format!("Unknown new type '{kind}'"));
    }

    let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
    if name.is_empty() || name.starts_with('-') {
        return fail("Missing required argument <name>");
    }

    let schema_opt = parse_string_flag(args, "--schema");
    let schema = schema_opt
        .clone()
        .unwrap_or_else(|| core_templates::default_schema_name().to_string());
    let module = parse_string_flag(args, "--module");
    let description = parse_string_flag(args, "--description");

    let ito_path = rt.ito_path();

    let module_id = module
        .as_deref()
        .and_then(|m| {
            ito_core::parse_module_id(m)
                .ok()
                .map(|p| p.module_id.to_string())
        })
        .unwrap_or_else(|| "000".to_string());
    let schema_display = if schema_opt.is_some() {
        format!(" with schema '{}'", schema)
    } else {
        String::new()
    };
    eprintln!(
        "- Creating change '{}' in module {}{}...",
        name, module_id, schema_display
    );

    let (coord_enabled, coord_branch) = coordination_branch_settings(rt);
    best_effort_sync_coordination(rt, "before create");

    match core_create::create_change(
        ito_path,
        name,
        &schema,
        module.as_deref(),
        description.as_deref(),
    ) {
        Ok(r) => {
            // Emit audit event for change creation (via `ito new`)
            if let Some(event) = AuditEventBuilder::new()
                .entity(EntityType::Change)
                .entity_id(&r.change_id)
                .op(ops::CHANGE_CREATE)
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .meta(serde_json::json!({
                    "schema": schema,
                    "module": module_id,
                }))
                .ctx(rt.event_context().clone())
                .build()
            {
                rt.emit_audit_event(&event);
            }

            // Emit module.change_added event if change belongs to a non-default module
            if module_id != "000"
                && let Some(event) = AuditEventBuilder::new()
                    .entity(EntityType::Module)
                    .entity_id(&module_id)
                    .op(ops::MODULE_CHANGE_ADDED)
                    .to(&r.change_id)
                    .actor(Actor::Cli)
                    .by(rt.user_identity())
                    .meta(serde_json::json!({
                        "change_id": r.change_id,
                    }))
                    .ctx(rt.event_context().clone())
                    .build()
            {
                rt.emit_audit_event(&event);
            }

            print_change_created_message(
                ito_path,
                &r.change_id,
                &schema,
                &module_id,
                module.is_some(),
                description.is_some(),
            );

            if coord_enabled {
                let project_root = ito_path.parent().unwrap_or(ito_path);
                if let Err(err) = reserve_change_on_coordination_branch(
                    project_root,
                    ito_path,
                    &r.change_id,
                    &coord_branch,
                ) {
                    return fail(format!(
                        "Created local change '{}' but failed to reserve it on coordination branch '{}': {}",
                        r.change_id, coord_branch, err.message
                    ));
                }
            }

            // Best-effort auto-commit to coordination worktree.
            auto_commit_after_change_creation(ito_path, &r.change_id);
            best_effort_sync_coordination(rt, "after create");

            Ok(())
        }
        Err(e) => Err(to_cli_error(e)),
    }
}

pub(crate) fn handle_new_clap(rt: &Runtime, args: &NewArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return fail("Missing required argument <type>");
    };

    let forwarded: Vec<String> = match action {
        NewAction::Change {
            name,
            schema,
            module,
            description,
        } => {
            let mut out = vec!["change".to_string()];
            if let Some(name) = name {
                out.push(name.clone());
            }
            if let Some(schema) = schema {
                out.push("--schema".to_string());
                out.push(schema.clone());
            }
            if let Some(module) = module {
                out.push("--module".to_string());
                out.push(module.clone());
            }
            if let Some(description) = description {
                out.push("--description".to_string());
                out.push(description.clone());
            }
            out
        }
        NewAction::External(rest) => rest.clone(),
    };

    handle_new(rt, &forwarded)
}

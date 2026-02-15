use crate::cli::{CreateAction, CreateArgs, NewAction, NewArgs};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::{parse_string_flag, split_csv};
use ito_config::{load_cascading_project_config, resolve_coordination_branch_settings};
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::git::{
    CoordinationGitErrorKind, fetch_coordination_branch, reserve_change_on_coordination_branch,
};
use ito_core::{create as core_create, templates as core_templates};
use std::path::Path;

fn coordination_branch_settings(rt: &Runtime) -> (bool, String) {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    resolve_coordination_branch_settings(&merged)
}

fn sync_coordination_if_enabled(ito_path: &Path, coord_enabled: bool, coord_branch: &str) {
    if !coord_enabled {
        return;
    }

    let project_root = ito_path.parent().unwrap_or(ito_path);
    if let Err(err) = fetch_coordination_branch(project_root, coord_branch)
        && err.kind != CoordinationGitErrorKind::RemoteMissing
    {
        eprintln!(
            "Warning: failed to sync coordination branch '{}' before create: {}",
            coord_branch, err.message
        );
    }
}

fn print_change_created_message(
    ito_path: &Path,
    change_id: &str,
    schema: &str,
    module_id: &str,
    module_was_explicit: bool,
    has_description: bool,
) {
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
            out
        }
        CreateAction::Change {
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
        CreateAction::External(rest) => rest.clone(),
    };

    handle_create(rt, &forwarded)
}

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

            let r = core_create::create_module(ito_path, name, scope, depends_on)
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
            let description = parse_string_flag(args, "--description");

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

            // Match TS/ora: spinner output is written to stderr.
            eprintln!(
                "- Creating change '{}' in module {}{}...",
                name, module_id, schema_display
            );

            let (coord_enabled, coord_branch) = coordination_branch_settings(rt);
            sync_coordination_if_enabled(ito_path, coord_enabled, &coord_branch);

            match core_create::create_change(
                ito_path,
                name,
                &schema,
                module.as_deref(),
                description.as_deref(),
            ) {
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
    sync_coordination_if_enabled(ito_path, coord_enabled, &coord_branch);

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

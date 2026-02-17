use crate::cli::{ConfigArgs, ConfigCommand};
use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_core::audit::{Actor, AuditEventBuilder, EntityType, ops};
use ito_core::config as core_config;
use std::path::{Path, PathBuf};

pub(crate) fn handle_config(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            crate::app::common::render_command_long_help(&["config"], "ito config")
        );
        return Ok(());
    }

    let sub = args.first().map(|s| s.as_str()).unwrap_or("");

    if sub == "schema" {
        let output = args
            .iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .map(PathBuf::from);

        return handle_config_schema(output.as_deref());
    }

    let Some(path) = ito_config::global_config_path(rt.ctx()) else {
        return fail("No Ito config directory found");
    };

    match sub {
        "path" => {
            println!("{}", path.display());
            Ok(())
        }
        "list" => {
            let v = core_config::read_json_config(&path).map_err(to_cli_error)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".to_string())
            );
            Ok(())
        }
        "get" => {
            let key = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if key.is_empty() || key.starts_with('-') {
                return fail("Missing required argument <key>");
            }
            let v = core_config::read_json_config(&path).map_err(to_cli_error)?;
            let parts = core_config::json_split_path(key);
            let Some(value) = core_config::json_get_path(&v, &parts) else {
                return fail("Key not found");
            };
            println!("{}", json_render_value(value));
            Ok(())
        }
        "set" => {
            let key = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if key.is_empty() || key.starts_with('-') {
                return fail("Missing required argument <key>");
            }
            let raw = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if raw.is_empty() {
                return fail("Missing required argument <value>");
            }
            let force_string = args.iter().any(|a| a == "--string");

            let mut v = core_config::read_json_config(&path).map_err(to_cli_error)?;

            // Capture previous value for audit event
            let parts = core_config::json_split_path(key);
            let prev_value = core_config::json_get_path(&v, &parts).map(json_render_value);

            let value = core_config::parse_json_value_arg(raw, force_string);
            core_config::validate_config_value(&parts, &value).map_err(to_cli_error)?;
            core_config::json_set_path(&mut v, &parts, value).map_err(to_cli_error)?;
            core_config::write_json_config(&path, &v).map_err(to_cli_error)?;

            // Emit audit event for config set
            let mut builder = AuditEventBuilder::new()
                .entity(EntityType::Config)
                .entity_id(key)
                .op(ops::CONFIG_SET)
                .to(raw)
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone());
            if let Some(prev) = prev_value {
                builder = builder.from(prev);
            }
            if let Some(event) = builder.build() {
                rt.emit_audit_event(&event);
            }

            Ok(())
        }
        "unset" => {
            let key = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if key.is_empty() || key.starts_with('-') {
                return fail("Missing required argument <key>");
            }

            let mut v = core_config::read_json_config(&path).map_err(to_cli_error)?;
            let parts = core_config::json_split_path(key);

            // Capture previous value for audit event
            let prev_value = core_config::json_get_path(&v, &parts).map(json_render_value);

            core_config::json_unset_path(&mut v, &parts).map_err(to_cli_error)?;
            core_config::write_json_config(&path, &v).map_err(to_cli_error)?;

            // Emit audit event for config unset
            let mut builder = AuditEventBuilder::new()
                .entity(EntityType::Config)
                .entity_id(key)
                .op(ops::CONFIG_UNSET)
                .actor(Actor::Cli)
                .by(rt.user_identity())
                .ctx(rt.event_context().clone());
            if let Some(prev) = prev_value {
                builder = builder.from(prev);
            }
            if let Some(event) = builder.build() {
                rt.emit_audit_event(&event);
            }

            Ok(())
        }
        _ => fail(format!("Unknown config subcommand '{sub}'")),
    }
}

pub(crate) fn handle_config_clap(rt: &Runtime, args: &ConfigArgs) -> CliResult<()> {
    match &args.command {
        None => {
            let argv: Vec<String> = Vec::new();
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::Path(common)) => {
            let mut argv: Vec<String> = vec!["path".to_string()];
            if common.string {
                argv.push("--string".to_string());
            }
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::List(common)) => {
            let mut argv: Vec<String> = vec!["list".to_string()];
            if common.string {
                argv.push("--string".to_string());
            }
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::Get { key, common }) => {
            let mut argv: Vec<String> = vec!["get".to_string(), key.clone()];
            if common.string {
                argv.push("--string".to_string());
            }
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::Set { key, value, common }) => {
            let mut argv: Vec<String> = vec!["set".to_string(), key.clone(), value.clone()];
            if common.string {
                argv.push("--string".to_string());
            }
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::Unset { key, common }) => {
            let mut argv: Vec<String> = vec!["unset".to_string(), key.clone()];
            if common.string {
                argv.push("--string".to_string());
            }
            handle_config(rt, &argv)
        }
        Some(ConfigCommand::Schema { output }) => handle_config_schema(output.as_deref()),
        Some(ConfigCommand::External(v)) => {
            let sub = v.first().map(|s| s.as_str()).unwrap_or("");
            let argv: Vec<String> = vec![sub.to_string()];
            handle_config(rt, &argv)
        }
    }
}

fn handle_config_schema(output: Option<&Path>) -> CliResult<()> {
    let schema = ito_config::schema::config_schema_pretty_json();

    let Some(output) = output else {
        println!("{schema}");
        return Ok(());
    };

    if let Some(parent) = output.parent() {
        ito_common::io::create_dir_all_std(parent).map_err(to_cli_error)?;
    }

    let mut bytes = schema.into_bytes();
    bytes.push(b'\n');
    ito_common::io::write_atomic_std(output, bytes).map_err(to_cli_error)?;
    Ok(())
}

fn json_render_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            serde_json::to_string_pretty(v).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

#[cfg(test)]
mod config_tests;

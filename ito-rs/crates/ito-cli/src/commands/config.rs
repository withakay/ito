use crate::cli::{ConfigArgs, ConfigCommand};
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
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
            let v = read_json_object_or_empty(&path)?;
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
            let v = read_json_object_or_empty(&path)?;
            let Some(value) = json_get_path(&v, key) else {
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

            let mut v = read_json_object_or_empty(&path)?;
            let value = parse_json_value_arg(raw, force_string)?;
            json_set_path(&mut v, key, value)?;

            let bytes = serde_json::to_vec_pretty(&v).map_err(to_cli_error)?;
            let mut bytes = bytes;
            bytes.push(b'\n');
            ito_common::io::write_atomic_std(&path, bytes).map_err(to_cli_error)?;
            Ok(())
        }
        "unset" => {
            let key = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if key.is_empty() || key.starts_with('-') {
                return fail("Missing required argument <key>");
            }

            let mut v = read_json_object_or_empty(&path)?;
            json_unset_path(&mut v, key)?;

            let bytes = serde_json::to_vec_pretty(&v).map_err(to_cli_error)?;
            let mut bytes = bytes;
            bytes.push(b'\n');
            ito_common::io::write_atomic_std(&path, bytes).map_err(to_cli_error)?;
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

fn read_json_object_or_empty(path: &Path) -> CliResult<serde_json::Value> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    };
    let v: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| CliError::msg(format!("Invalid JSON in {}: {e}", path.display())))?;
    match v {
        serde_json::Value::Object(_) => Ok(v),
        _ => Err(CliError::msg(format!(
            "Expected JSON object in {}",
            path.display()
        ))),
    }
}

fn parse_json_value_arg(raw: &str, force_string: bool) -> CliResult<serde_json::Value> {
    if force_string {
        return Ok(serde_json::Value::String(raw.to_string()));
    }
    match serde_json::from_str::<serde_json::Value>(raw) {
        Ok(v) => Ok(v),
        Err(_) => Ok(serde_json::Value::String(raw.to_string())),
    }
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

fn json_split_path(path: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    for part in path.split('.') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        out.push(part);
    }
    out
}

fn json_get_path<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let parts = json_split_path(path);
    let mut cur = root;
    for p in parts {
        let serde_json::Value::Object(map) = cur else {
            return None;
        };
        let next = map.get(p)?;
        cur = next;
    }
    Some(cur)
}

fn json_set_path(
    root: &mut serde_json::Value,
    path: &str,
    value: serde_json::Value,
) -> CliResult<()> {
    let parts = json_split_path(path);
    if parts.is_empty() {
        return Err(CliError::msg("Invalid empty path"));
    }

    let mut cur = root;
    for (i, key) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();

        let is_object = matches!(cur, serde_json::Value::Object(_));
        if !is_object {
            *cur = serde_json::Value::Object(serde_json::Map::new());
        }

        let serde_json::Value::Object(map) = cur else {
            return Err(CliError::msg("Failed to set path"));
        };

        if is_last {
            map.insert((*key).to_string(), value);
            return Ok(());
        }

        let needs_object = match map.get(*key) {
            Some(serde_json::Value::Object(_)) => false,
            Some(_) => true,
            None => true,
        };
        if needs_object {
            map.insert(
                (*key).to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        let Some(next) = map.get_mut(*key) else {
            return Err(CliError::msg("Failed to set path"));
        };
        cur = next;
    }

    Ok(())
}

fn json_unset_path(root: &mut serde_json::Value, path: &str) -> CliResult<()> {
    let parts = json_split_path(path);
    if parts.is_empty() {
        return Err(CliError::msg("Invalid empty path"));
    }

    let mut cur = root;
    for (i, p) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();
        let serde_json::Value::Object(map) = cur else {
            return Ok(());
        };

        if is_last {
            map.remove(*p);
            return Ok(());
        }

        let Some(next) = map.get_mut(*p) else {
            return Ok(());
        };
        cur = next;
    }

    Ok(())
}

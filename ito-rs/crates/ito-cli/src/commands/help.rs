use crate::cli::HelpArgs;
use crate::cli_error::CliResult;
use clap::CommandFactory;

/// Collects the stable set of CLI command paths (as parts) for which help text should be emitted.
///
/// This returns the base command plus a curated list of top-level and nested commands
/// that exist in the current clap command tree. Deprecated aliases are intentionally
/// excluded so the output remains stable and user-facing.
///
/// # Returns
///
/// A vector of command paths where each entry is a `Vec<String>` of path parts.
/// The first entry is an empty `Vec` representing the base command; subsequent entries
/// are the subcommand path segments (e.g., `["agent", "instruction"]`).
///
/// # Examples
///
/// ```
/// let parts = help_all_parts();
/// // first entry is the base command
/// assert!(parts.first().map(|p| p.is_empty()).unwrap_or(false));
/// // each entry is a vector of strings representing the command path
/// for entry in parts {
///     let _path: Vec<String> = entry;
/// }
/// ```
fn help_all_parts() -> Vec<Vec<String>> {
    // Keep output stable and user-facing (exclude deprecated aliases like
    // `templates`, `instructions`, `loop`, etc.), while still deriving help
    // text from clap.
    let mut out: Vec<Vec<String>> = Vec::new();
    out.push(Vec::new());

    let cmd = crate::cli::Cli::command();
    let names: &[&[&str]] = &[
        &["init"],
        &["update"],
        &["tasks"],
        &["plan"],
        &["state"],
        &["workflow"],
        &["list"],
        &["archive"],
        &["config"],
        &["create"],
        &["validate"],
        &["show"],
        &["agent"],
        &["agent", "instruction"],
        &["ralph"],
        &["status"],
        &["completions"],
        &["stats"],
        &["agent-config"],
    ];

    for path in names {
        // Only include paths that exist in the current clap tree.
        let mut current = cmd.clone();
        let mut ok = true;
        for part in *path {
            let Some(found) = current.find_subcommand_mut(part) else {
                ok = false;
                break;
            };
            current = found.clone();
        }

        if ok {
            out.push(path.iter().map(|s| s.to_string()).collect());
        }
    }

    out
}

fn render_help(parts: &[&str], bin_name: &str) -> String {
    crate::app::common::render_command_long_help(parts, bin_name)
}

pub(crate) fn handle_help_clap(args: &HelpArgs) -> CliResult<()> {
    if args.all {
        return handle_help_all_flags(args.json);
    }

    if !args.command.is_empty() {
        let mut bin_name = "ito".to_string();
        for p in &args.command {
            bin_name.push(' ');
            bin_name.push_str(p);
        }
        let parts: Vec<&str> = args.command.iter().map(|s| s.as_str()).collect();
        print!("{}", render_help(&parts, &bin_name));
        return Ok(());
    }

    print!("{}", render_help(&[], "ito"));
    Ok(())
}

pub(crate) fn handle_help_all(args: &[String]) -> CliResult<()> {
    let json_output = args.iter().any(|a| a == "--json");

    let entries = help_all_parts();

    if json_output {
        let commands: Vec<serde_json::Value> = entries
            .iter()
            .map(|parts| {
                let path = if parts.is_empty() {
                    "ito".to_string()
                } else {
                    format!("ito {}", parts.join(" "))
                };
                let bin_name = path.clone();
                let parts_ref: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
                serde_json::json!({
                    "path": path,
                    "help": render_help(&parts_ref, &bin_name),
                })
            })
            .collect();

        let output = serde_json::json!({
            "version": "1.0",
            "commands": commands,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).unwrap_or_default()
        );
        return Ok(());
    }

    println!("================================================================================");
    println!("ITO CLI REFERENCE");
    println!("================================================================================\n");

    for (i, parts) in entries.iter().enumerate() {
        let path = if parts.is_empty() {
            "ito".to_string()
        } else {
            format!("ito {}", parts.join(" "))
        };

        let bin_name = path.clone();
        let parts_ref: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

        if i > 0 {
            println!(
                "\n--------------------------------------------------------------------------------\n"
            );
        }
        println!("{path}");
        println!("{}", "-".repeat(path.len()));
        println!("{}", render_help(&parts_ref, &bin_name));
    }

    println!("\n================================================================================");
    println!("Run 'ito <command> -h' for detailed command help.");
    println!("================================================================================");

    Ok(())
}

pub(crate) fn handle_help_all_flags(json_output: bool) -> CliResult<()> {
    if json_output {
        return handle_help_all(&["--json".to_string()]);
    }
    handle_help_all(&[])
}
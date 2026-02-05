use crate::cli::InitArgs;
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_config::output;
use ito_core::installers::{InitOptions, InstallMode, install_default_templates};
use std::collections::BTreeSet;

pub(super) fn handle_init(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["init"], "ito init")
        );
        return Ok(());
    }

    let force = args.iter().any(|a| a == "--force" || a == "-f");
    let tools_arg = parse_string_flag(args, "--tools");

    // Positional path (defaults to current directory).
    let target = super::common::last_positional(args).unwrap_or_else(|| ".".to_string());
    let target_path = std::path::Path::new(&target);
    let ctx = rt.ctx();

    let all_ids = ito_core::installers::available_tool_ids();

    let tools: BTreeSet<String> = if let Some(raw) = tools_arg.as_deref() {
        let raw = raw.trim();
        if raw.is_empty() {
            return fail("--tools cannot be empty");
        }

        if raw == "none" {
            BTreeSet::new()
        } else if raw == "all" {
            all_ids.iter().map(|s| (*s).to_string()).collect()
        } else {
            let valid = all_ids.join(", ");
            let mut selected: BTreeSet<String> = BTreeSet::new();
            for part in raw.split(',') {
                let id = part.trim();
                if id.is_empty() {
                    continue;
                }
                if all_ids.contains(&id) {
                    selected.insert(id.to_string());
                } else {
                    return fail(format!("Unknown tool id '{id}'. Valid tool ids: {valid}"));
                }
            }
            selected
        }
    } else {
        use std::io::BufRead;
        use std::io::{IsTerminal, stdin, stdout};

        // Match TS semantics: prompt only when interactive; otherwise require explicit --tools.
        let ui = output::resolve_ui_options(
            false,
            std::env::var("NO_COLOR").ok().as_deref(),
            false,
            std::env::var("ITO_INTERACTIVE").ok().as_deref(),
        );
        let is_tty = stdin().is_terminal() && stdout().is_terminal();
        if !(ui.interactive && is_tty) {
            return fail(
                "Non-interactive init requires --tools (all, none, or comma-separated ids).",
            );
        }

        println!(
            "Welcome to Ito!\n\nStep 1/3\n\nConfigure your Ito tooling\nPress Enter to continue."
        );
        {
            let mut line = String::new();
            let mut locked = stdin().lock();
            let _ = locked.read_line(&mut line);
        }

        println!(
            "\nStep 2/3\n\nWhich natively supported AI tools do you use?\nUse ↑/↓ to move · Space to toggle · Enter reviews.\n"
        );

        let mut detected: BTreeSet<&'static str> = BTreeSet::new();
        if target_path.join("CLAUDE.md").exists() || target_path.join(".claude").exists() {
            detected.insert(ito_core::installers::TOOL_CLAUDE);
        }
        if target_path.join(".opencode").exists() {
            detected.insert(ito_core::installers::TOOL_OPENCODE);
        }
        if target_path.join(".github").exists() {
            detected.insert(ito_core::installers::TOOL_GITHUB_COPILOT);
        }
        if target_path.join(".codex").exists() {
            detected.insert(ito_core::installers::TOOL_CODEX);
        }

        let tool_items: Vec<(&'static str, &str)> = vec![
            (ito_core::installers::TOOL_CLAUDE, "Claude Code"),
            (ito_core::installers::TOOL_CODEX, "Codex"),
            (ito_core::installers::TOOL_GITHUB_COPILOT, "GitHub Copilot"),
            (ito_core::installers::TOOL_OPENCODE, "OpenCode"),
        ];
        let labels: Vec<String> = tool_items
            .iter()
            .map(|(id, label)| format!("{label} ({id})"))
            .collect();
        let defaults: Vec<bool> = tool_items
            .iter()
            .map(|(id, _)| detected.contains(id))
            .collect();

        let indices =
            match dialoguer::MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Select AI tools to configure")
                .items(&labels)
                .defaults(&defaults)
                .interact()
            {
                Ok(v) => v,
                Err(e) => {
                    return Err(CliError::msg(format!("Failed to prompt for tools: {e}")));
                }
            };

        println!("\nStep 3/3\n\nReview selections\nPress Enter to confirm.");
        {
            let mut line = String::new();
            let mut locked = stdin().lock();
            let _ = locked.read_line(&mut line);
        }

        indices
            .into_iter()
            .map(|i| tool_items[i].0.to_string())
            .collect()
    };

    let opts = InitOptions::new(tools, force);
    install_default_templates(target_path, ctx, InstallMode::Init, &opts).map_err(to_cli_error)?;

    print_post_init_guidance(target_path);

    Ok(())
}

fn print_post_init_guidance(target_path: &std::path::Path) {
    let target_display = if target_path == std::path::Path::new(".") {
        "current directory".to_string()
    } else {
        format!("{}", target_path.display())
    };

    println!(
        r#"
Ito initialized in {target_display}

Next step: Run /ito-project-setup in your AI assistant to configure the project.

Or manually edit:
  .ito/project.md        Project overview, tech stack, architecture
  .ito/user-guidance.md  Your coding standards and preferences
  .ito/config.json       Tool settings and defaults

Learn more: ito --help | ito agent instruction --help
"#
    );
}

pub(crate) fn handle_init_clap(rt: &Runtime, args: &InitArgs) -> CliResult<()> {
    if let Some(home) = &args.home {
        // For parity/testing.
        unsafe {
            std::env::set_var("HOME", home);
        }
    }

    let mut argv: Vec<String> = Vec::new();
    if let Some(tools) = &args.tools {
        argv.push("--tools".to_string());
        argv.push(tools.clone());
    }
    if args.force {
        argv.push("--force".to_string());
    }
    if let Some(path) = &args.path {
        argv.push(path.clone());
    }
    handle_init(rt, &argv)
}

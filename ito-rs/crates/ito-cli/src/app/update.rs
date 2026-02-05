use crate::cli::UpdateArgs;
use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::installers::{InitOptions, InstallMode, install_default_templates};
use std::collections::BTreeSet;

pub(super) fn handle_update(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["update"], "ito update")
        );
        return Ok(());
    }

    // `--json` is accepted for parity with TS but not implemented yet.
    let _want_json = args.iter().any(|a| a == "--json");
    let target = super::common::last_positional(args).unwrap_or_else(|| ".".to_string());
    let target_path = std::path::Path::new(&target);
    let ctx = rt.ctx();

    let tools: BTreeSet<String> = ito_core::installers::available_tool_ids()
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let opts = InitOptions::new(tools, true);

    install_default_templates(target_path, ctx, InstallMode::Update, &opts)
        .map_err(to_cli_error)?;

    Ok(())
}

pub(crate) fn handle_update_clap(rt: &Runtime, args: &UpdateArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    if args.json {
        argv.push("--json".to_string());
    }
    if let Some(path) = &args.path {
        argv.push(path.clone());
    }
    handle_update(rt, &argv)
}

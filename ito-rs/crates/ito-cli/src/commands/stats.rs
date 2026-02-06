use crate::cli::StatsArgs;
use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;

pub(crate) fn handle_stats_clap(rt: &Runtime, _args: &StatsArgs) -> CliResult<()> {
    let Some(config_dir) = ito_config::ito_config_dir(rt.ctx()) else {
        println!("No Ito config directory found.");
        return Ok(());
    };

    let log_dir = config_dir
        .join("logs")
        .join("execution")
        .join("v1")
        .join("projects");

    let stats = ito_core::stats::compute_command_stats(&log_dir).map_err(to_cli_error)?;

    println!("Ito Stats");
    println!("────────────────────────────────────────");
    println!("command_id: count");
    for (id, count) in stats.counts {
        println!("{id}: {count}");
    }

    Ok(())
}

use crate::cli::{PlanAction, PlanArgs};
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::domain::planning as wf_planning;
use ito_core::planning_init;

pub(crate) fn handle_plan_clap(rt: &Runtime, args: &PlanArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return Err(CliError::msg("Missing required plan subcommand"));
    };

    let ito_path = rt.ito_path();
    let ito_dir = ito_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| ".ito".to_string());
    let current_date = ito_core::time::now_date();

    match action {
        PlanAction::Init => {
            planning_init::init_planning_structure(ito_path, &current_date, &ito_dir)
                .map_err(to_cli_error)?;
            eprintln!("✔ Planning structure initialized");
            println!("Created:");
            println!("  - {}/planning/PROJECT.md", ito_dir);
            println!("  - {}/planning/ROADMAP.md", ito_dir);
            println!("  - {}/planning/STATE.md", ito_dir);
            Ok(())
        }
        PlanAction::Status => {
            let roadmap_path = wf_planning::planning_dir(ito_path).join("ROADMAP.md");
            let contents = ito_common::io::read_to_string(&roadmap_path).map_err(|_| {
                CliError::msg("ROADMAP.md not found. Run \"ito init\" or \"ito plan init\" first.")
            })?;

            let Some((milestone, status, phase)) = wf_planning::read_current_progress(&contents)
            else {
                return Err(CliError::msg(
                    "Could not find current milestone section in ROADMAP.md",
                ));
            };
            let phases = wf_planning::read_phase_rows(&contents);

            println!("Current Progress");
            println!("────────────────────────────────────────");
            println!("Milestone: {milestone}");
            println!("Status: {status}");
            println!("Phase: {phase}");
            println!();
            println!("Phases");
            println!("────────────────────────────────────────");
            for (num, name, st, _changes) in phases {
                let icon = if st.eq_ignore_ascii_case("Complete") {
                    "✓"
                } else if st.eq_ignore_ascii_case("In Progress") {
                    "●"
                } else {
                    "○"
                };
                println!("  {icon} Phase {num}: {name} [{st}]");
            }
            Ok(())
        }
    }
}

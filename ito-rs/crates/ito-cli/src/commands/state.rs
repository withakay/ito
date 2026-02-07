use crate::cli::{StateAction, StateArgs};
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::state::{self, StateAction as CoreStateAction};

pub(crate) fn handle_state_clap(rt: &Runtime, args: &StateArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return Err(CliError::msg("Missing required state subcommand"));
    };

    let ito_path = rt.ito_path();
    let ito_dir = ito_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| ".ito".to_string());
    let state_path = ito_path.join("planning").join("STATE.md");
    if !state_path.exists() {
        return Err(CliError::msg(format!(
            "STATE.md not found. Run \"ito init\" first or create {}/planning/STATE.md",
            ito_dir
        )));
    }

    if matches!(action, StateAction::Show) {
        let contents = state::read_state(ito_path).map_err(to_cli_error)?;
        print!("{contents}");
        return Ok(());
    }

    let text = match action {
        StateAction::Show => String::new(),
        StateAction::Decision { text }
        | StateAction::Blocker { text }
        | StateAction::Note { text }
        | StateAction::Focus { text }
        | StateAction::Question { text } => text.join(" "),
    };

    let core_action = match action {
        StateAction::Show => unreachable!("handled above"),
        StateAction::Decision { .. } => CoreStateAction::AddDecision { text: text.clone() },
        StateAction::Blocker { .. } => CoreStateAction::AddBlocker { text: text.clone() },
        StateAction::Question { .. } => CoreStateAction::AddQuestion { text: text.clone() },
        StateAction::Focus { .. } => CoreStateAction::SetFocus { text: text.clone() },
        StateAction::Note { .. } => CoreStateAction::AddNote { text: text.clone() },
    };

    state::update_state(ito_path, core_action).map_err(to_cli_error)?;

    match action {
        StateAction::Show => {}
        StateAction::Decision { .. } => eprintln!("✔ Decision recorded: {text}"),
        StateAction::Blocker { .. } => eprintln!("✔ Blocker recorded: {text}"),
        StateAction::Note { .. } => eprintln!("✔ Note recorded: {text}"),
        StateAction::Focus { .. } => eprintln!("✔ Focus updated: {text}"),
        StateAction::Question { .. } => eprintln!("✔ Question added: {text}"),
    }

    Ok(())
}

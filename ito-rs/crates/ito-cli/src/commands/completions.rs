use clap::CommandFactory;

use crate::cli::{Cli, CompletionShell};
use crate::cli_error::CliResult;

pub(crate) fn handle_completions(shell: CompletionShell) -> CliResult<()> {
    let mut cmd = Cli::command();

    let shell = match shell {
        CompletionShell::Bash => clap_complete::Shell::Bash,
        CompletionShell::Zsh => clap_complete::Shell::Zsh,
        CompletionShell::Fish => clap_complete::Shell::Fish,
        CompletionShell::PowerShell => clap_complete::Shell::PowerShell,
    };

    clap_complete::generate(shell, &mut cmd, "ito", &mut std::io::stdout());
    Ok(())
}

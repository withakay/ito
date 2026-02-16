use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug, Clone)]
pub enum ValidateCommand {
    /// Validate a module
    Module {
        /// Module id
        #[arg(value_name = "MODULE")]
        module_id: Option<String>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ValidateItemType {
    Change,
    Spec,
    Module,
}

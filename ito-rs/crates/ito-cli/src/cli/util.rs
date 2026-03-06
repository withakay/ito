use clap::{Args, Subcommand};

/// Utility commands for Ito scripting and agent tooling.
#[derive(Args, Debug, Clone)]
#[command(subcommand_required = true, arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
pub struct UtilArgs {
    #[command(subcommand)]
    pub command: Option<UtilCommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UtilCommand {
    /// Parse an Ito ID argument and emit structured JSON output.
    ///
    /// Classifies the input as a change ID, module ID, or continue-ready intent
    /// and prints a JSON object with `"mode"` and (for change/module) `"id"`.
    ///
    /// Output modes:
    ///   {"mode":"change","id":"005-01_add-auth"}
    ///   {"mode":"module","id":"012"}
    ///   {"mode":"continue-ready"}
    ///
    /// Examples:
    ///   ito util parse-id 005-01_add-auth
    ///   ito util parse-id 012
    ///   ito util parse-id next
    ///   ito util parse-id
    #[command(verbatim_doc_comment)]
    ParseId(ParseIdArgs),
}

/// Arguments for `ito util parse-id`.
#[derive(Args, Debug, Clone)]
pub struct ParseIdArgs {
    /// The input to classify (change ID, module ID, or intent keyword).
    ///
    /// If omitted, defaults to continue-ready mode.
    #[arg(trailing_var_arg = true)]
    pub input: Vec<String>,
}

use clap::Args;

/// Search Ito change artifacts using a regular expression.
///
/// When searching a single change, provide two positional arguments:
///
/// ```text
/// ito grep <CHANGE_ID> <PATTERN>
/// ```
///
/// When searching a module or all changes, only the pattern is needed:
///
/// ```text
/// ito grep --module <ID> <PATTERN>
/// ito grep --all <PATTERN>
/// ```
#[derive(Args, Debug, Clone)]
pub struct GrepArgs {
    /// Search all changes in a module (by module ID)
    #[arg(short = 'm', long, conflicts_with = "all")]
    pub module: Option<String>,

    /// Search all changes in the project
    #[arg(short = 'a', long, conflicts_with = "module")]
    pub all: bool,

    /// Maximum number of matching lines to print (0 = unlimited)
    #[arg(short = 'l', long, default_value_t = 0)]
    pub limit: usize,

    /// Positional arguments: `[CHANGE_ID] PATTERN`.
    ///
    /// With `--module` or `--all`: only `PATTERN` is required.
    /// Without flags: `CHANGE_ID` and `PATTERN` are both required.
    #[arg(value_name = "ARGS", required = true, num_args = 1..)]
    pub args: Vec<String>,
}

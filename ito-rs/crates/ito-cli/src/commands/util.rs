use crate::cli::{ParseIdArgs, UtilArgs, UtilCommand};
use crate::cli_error::{CliResult, fail, to_cli_error};
use ito_common::id::{parse_change_id, parse_module_id};

/// Dispatches `ito util` subcommands.
///
/// # Returns
///
/// `CliResult<()>` — `Ok(())` on success, or an error if the subcommand is missing.
pub(crate) fn handle_util_clap(args: &UtilArgs) -> CliResult<()> {
    let Some(cmd) = &args.command else {
        return fail("Missing required subcommand");
    };

    match cmd {
        UtilCommand::ParseId(args) => handle_parse_id(args),
    }
}

/// Parse an Ito ID from the given arguments and print structured JSON.
///
/// Classifies the joined input as one of:
/// - A change ID (`NNN-NN_name`) → `{"mode":"change","id":"<canonical>"}`
/// - A module ID (`NNN`) → `{"mode":"module","id":"<canonical>"}`
/// - A continue-ready intent (keywords or empty) → `{"mode":"continue-ready"}`
///
/// Keyword phrases that imply continue-ready include "next", "continue",
/// "ready", "pick", and any empty or whitespace-only input.
///
/// # Examples
///
/// ```text
/// // With args.input = ["005-01_add-auth"]:
/// // Output: {"mode":"change","id":"005-01_add-auth"}
///
/// // With args.input = ["012"]:
/// // Output: {"mode":"module","id":"012"}
///
/// // With args.input = ["next"]:
/// // Output: {"mode":"continue-ready"}
///
/// // With args.input = []:
/// // Output: {"mode":"continue-ready"}
/// ```
fn handle_parse_id(args: &ParseIdArgs) -> CliResult<()> {
    let raw = args.input.join(" ");
    let input = raw.trim();

    // Try parsing as a change ID first (most specific pattern)
    if let Ok(parsed) = parse_change_id(input) {
        let v = serde_json::json!({
            "mode": "change",
            "id": parsed.canonical.as_str(),
        });
        let out = serde_json::to_string(&v).map_err(to_cli_error)?;
        println!("{out}");
        return Ok(());
    }

    // Try parsing as a module ID (numeric-only input)
    if let Ok(parsed) = parse_module_id(input) {
        let v = serde_json::json!({
            "mode": "module",
            "id": parsed.module_id.as_str(),
        });
        let out = serde_json::to_string(&v).map_err(to_cli_error)?;
        println!("{out}");
        return Ok(());
    }

    // Fall back to continue-ready for empty input or known intent keywords
    let lower = input.to_ascii_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let is_continue_ready = input.is_empty()
        || words
            .iter()
            .any(|w| matches!(*w, "next" | "continue" | "ready" | "pick"));

    if is_continue_ready {
        let v = serde_json::json!({ "mode": "continue-ready" });
        let out = serde_json::to_string(&v).map_err(to_cli_error)?;
        println!("{out}");
        return Ok(());
    }

    fail(format!(
        "Could not classify input as a change ID, module ID, or continue-ready intent: \"{input}\"\n\
         Hint: pass a change ID (e.g. \"005-01_add-auth\"), a module ID (e.g. \"012\"), \
         or a keyword like \"next\"."
    ))
}

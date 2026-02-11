use crate::cli::{TemplatesAction, TemplatesArgs, TemplatesSchemasAction};
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::workflow;

/// Handle top-level `templates` CLI subcommands and perform the requested action.
///
/// Currently supports exporting embedded schemas. When exporting, prints the export destination and the counts of written and skipped files. If files were skipped and `force` is false, prints a hint to use `--force`.
///
/// # Examples
///
/// ```no_run
/// use crate::runtime::Runtime;
/// use crate::cli::{TemplatesArgs, TemplatesAction, TemplatesSchemasAction};
///
/// let rt = Runtime::new();
/// // construct args so that args.action is Some(TemplatesAction::Schemas(...)))
/// let args = TemplatesArgs { /* populate with Schemas -> Export */ };
/// handle_templates_clap(&rt, &args).unwrap();
/// ```
pub(crate) fn handle_templates_clap(rt: &Runtime, args: &TemplatesArgs) -> CliResult<()> {
    let Some(action) = &args.action else {
        return Err(CliError::msg("Missing required templates subcommand"));
    };

    match action {
        TemplatesAction::Schemas(schemas) => {
            let Some(schema_action) = &schemas.action else {
                return Err(CliError::msg("Missing required schemas subcommand"));
            };

            match schema_action {
                TemplatesSchemasAction::Export { to, force } => {
                    let result =
                        workflow::export_embedded_schemas(to, *force).map_err(to_cli_error)?;
                    println!("Exported schemas to {}", to.display());
                    println!("Written: {}", result.written);
                    println!("Skipped: {}", result.skipped);
                    if !force && result.skipped > 0 {
                        println!("Use --force to overwrite skipped files.");
                    }
                    let _ = rt;
                    Ok(())
                }
            }
        }
    }
}

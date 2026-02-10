use crate::cli::{TemplatesAction, TemplatesArgs, TemplatesSchemasAction};
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::workflow;

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

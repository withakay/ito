use crate::cli::ArchiveArgs;
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use ito_common::paths as core_paths;
use ito_core::change_repository::FsChangeRepository;
use ito_domain::tasks::TaskRepository;

pub(crate) fn handle_archive(rt: &Runtime, args: &[String]) -> CliResult<()> {
    use ito_core::archive;

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["archive"], "ito archive")
        );
        return Ok(());
    }

    let ito_path = rt.ito_path();
    let changes_dir = core_paths::changes_dir(ito_path);

    if !changes_dir.exists() {
        return fail("No Ito changes directory found. Run 'ito init' first.");
    }

    // Parse options
    let skip_validation = args.iter().any(|a| a == "--no-validate");
    let skip_specs = args.iter().any(|a| a == "--skip-specs");
    let auto_confirm = args.iter().any(|a| a == "--yes" || a == "-y");

    // Get change name (first positional arg)
    let change_name = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(|s| s.as_str());

    // If no change specified, list available changes and prompt for selection
    let change_repo = FsChangeRepository::new(ito_path);
    let change_name = if let Some(name) = change_name {
        match super::common::resolve_change_target(ito_path, name) {
            Ok(resolved) => resolved,
            Err(msg) => return fail(msg),
        }
    } else {
        let available = change_repo.list().unwrap_or_default();
        if available.is_empty() {
            return fail("No changes found to archive.");
        }

        println!("Available changes:");
        for (idx, change) in available.iter().enumerate() {
            println!("  {}. {}", idx + 1, change.id);
        }
        println!();

        // Simple selection (in a real implementation, would use interactive prompt)
        // For now, just fail with message
        return fail("Please specify a change name: ito archive <change-name>");
    };

    // Verify change exists
    if !change_repo.exists(&change_name) {
        return fail(format!("Change '{}' not found", change_name));
    }

    // Check task completion unless skipping validation
    if !skip_validation {
        let task_repo = TaskRepository::new(ito_path);
        let (completed, total) = task_repo.get_task_counts(&change_name).unwrap_or((0, 0));
        if total > 0 {
            if completed < total {
                let pending = total - completed;
                println!(
                    "Warning: Change has {} incomplete tasks out of {}",
                    pending, total
                );
                if !auto_confirm {
                    println!("Continue with archive anyway? [y/N]: ");
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| CliError::msg("Failed to read input"))?;
                    let input = input.trim().to_lowercase();
                    if input != "y" && input != "yes" {
                        println!("Archive cancelled.");
                        return Ok(());
                    }
                }
            } else {
                eprintln!("✔ All tasks complete");
            }
        }
    }

    // Generate archive name
    let archive_name = archive::generate_archive_name(&change_name);

    // Check if archive already exists
    if archive::archive_exists(ito_path, &archive_name) {
        return fail(format!("Archive '{}' already exists", archive_name));
    }

    let mut specs_updated: Vec<String> = Vec::new();

    // Handle spec updates unless skipped
    if !skip_specs {
        let spec_names =
            archive::discover_change_specs(ito_path, &change_name).map_err(to_cli_error)?;

        if !spec_names.is_empty() {
            let (new_specs, existing_specs) = archive::categorize_specs(ito_path, &spec_names);

            // Show confirmation
            if !new_specs.is_empty() || !existing_specs.is_empty() {
                println!("The following specs will be updated:");
                println!();

                if !new_specs.is_empty() {
                    println!("NEW specs to be created:");
                    for spec in &new_specs {
                        println!("  - {}", spec);
                    }
                    println!();
                }

                if !existing_specs.is_empty() {
                    println!("EXISTING specs to be updated:");
                    for spec in &existing_specs {
                        println!("  - {}", spec);
                    }
                    println!();
                }

                if !auto_confirm {
                    println!(
                        "Update {} specs and archive '{}'? [y/N]: ",
                        spec_names.len(),
                        change_name
                    );
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| CliError::msg("Failed to read input"))?;
                    let input = input.trim().to_lowercase();
                    if input != "y" && input != "yes" {
                        println!("Skipping spec updates, continuing with archive...");
                    } else {
                        // Copy specs to main
                        specs_updated =
                            archive::copy_specs_to_main(ito_path, &change_name, &spec_names)
                                .map_err(to_cli_error)?;
                        eprintln!("✔ Updated {} specs", specs_updated.len());
                    }
                } else {
                    // Copy specs to main
                    specs_updated =
                        archive::copy_specs_to_main(ito_path, &change_name, &spec_names)
                            .map_err(to_cli_error)?;
                    eprintln!("✔ Updated {} specs", specs_updated.len());
                }
            }
        }
    }

    // Move to archive
    archive::move_to_archive(ito_path, &change_name, &archive_name).map_err(to_cli_error)?;

    eprintln!("✔ Archived '{}' as '{}'", change_name, archive_name);
    if !specs_updated.is_empty() {
        eprintln!("  Updated specs: {}", specs_updated.join(", "));
    }

    Ok(())
}

pub(crate) fn handle_archive_clap(rt: &Runtime, args: &ArchiveArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();
    if let Some(change) = &args.change {
        argv.push(change.clone());
    }
    if args.yes {
        argv.push("-y".to_string());
    }
    if args.skip_specs {
        argv.push("--skip-specs".to_string());
    }
    if args.no_validate {
        argv.push("--no-validate".to_string());
    }
    handle_archive(rt, &argv)
}

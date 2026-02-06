use crate::cli::{ShowArgs, ShowCommand, ShowItemType};
use crate::cli_error::{CliError, CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::parse_string_flag;
use ito_common::match_::nearest_matches;
use ito_common::paths as core_paths;
use ito_config::output;
use ito_core::change_repository::FsChangeRepository;
use ito_core::module_repository::FsModuleRepository;
use ito_core::show as core_show;

pub(crate) fn handle_show(rt: &Runtime, args: &[String]) -> CliResult<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "{}",
            super::common::render_command_long_help(&["show"], "ito show")
        );
        return Ok(());
    }

    // Parse subcommand: `ito show module <id>`
    if args.first().map(|s| s.as_str()) == Some("module") {
        return handle_show_module(rt, &args[1..]);
    }

    let want_json = args.iter().any(|a| a == "--json");
    let typ = parse_string_flag(args, "--type");
    let cli_no_interactive = args.iter().any(|a| a == "--no-interactive");
    let ui = output::resolve_ui_options(
        false,
        std::env::var("NO_COLOR").ok().as_deref(),
        cli_no_interactive,
        std::env::var("ITO_INTERACTIVE").ok().as_deref(),
    );

    let deltas_only = args.iter().any(|a| a == "--deltas-only");
    let requirements_only = args.iter().any(|a| a == "--requirements-only");

    let requirements = args.iter().any(|a| a == "--requirements");
    let scenarios = !args.iter().any(|a| a == "--no-scenarios");
    let requirement_idx = parse_string_flag(args, "--requirement")
        .or_else(|| parse_string_flag(args, "-r"))
        .and_then(|s| s.parse::<usize>().ok());

    let item = super::common::last_positional(args);
    if item.is_none() {
        if ui.interactive {
            // Interactive selection is not implemented yet.
        }
        return fail(
            "Nothing to show. Try one of:\n  ito show <item>\n  ito show (for interactive selection)\nOr run in an interactive terminal.",
        );
    }
    let item = item.expect("checked");

    let ito_path = rt.ito_path();

    let explicit = typ.as_deref();
    let resolved_type = match explicit {
        Some("change") | Some("spec") => explicit.unwrap().to_string(),
        Some(_) => return fail("Invalid type. Expected 'change' or 'spec'."),
        None => super::common::detect_item_type(rt, &item),
    };

    if resolved_type == "ambiguous" {
        return fail(format!(
            "Ambiguous item '{item}' matches both a change and a spec.\nUse --type change or --type spec to disambiguate."
        ));
    }
    if resolved_type == "unknown" {
        let candidates = super::common::list_candidate_items(rt);
        let suggestions = nearest_matches(&item, &candidates, 5);
        return fail(super::common::unknown_with_suggestions(
            "item",
            &item,
            &suggestions,
        ));
    }

    // Warn on ignored flags (matches TS behavior closely).
    if want_json {
        let ignored = ignored_show_flags(
            &resolved_type,
            deltas_only,
            requirements_only,
            requirements,
            scenarios,
            requirement_idx,
        );
        if !ignored.is_empty() {
            eprintln!(
                "Warning: Ignoring flags not applicable to {resolved_type}: {}",
                ignored.join(", ")
            );
        }
    }

    match resolved_type.as_str() {
        "spec" => {
            let spec_path = core_paths::spec_markdown_path(ito_path, &item);
            let md = ito_common::io::read_to_string(&spec_path).map_err(|_| {
                CliError::msg(format!(
                    "Spec '{item}' not found at {}",
                    spec_path.display()
                ))
            })?;
            if want_json {
                if requirements && requirement_idx.is_some() {
                    return fail("Cannot use --requirement with --requirements");
                }
                let mut json = core_show::parse_spec_show_json(&item, &md);

                // Apply filters
                if requirements || !scenarios {
                    for r in &mut json.requirements {
                        r.scenarios.clear();
                    }
                }
                if let Some(one_based) = requirement_idx {
                    if one_based == 0 || one_based > json.requirements.len() {
                        return fail(format!(
                            "Requirement index out of range. Expected 1..={}",
                            json.requirements.len()
                        ));
                    }
                    json.requirements = vec![json.requirements[one_based - 1].clone()];
                    json.requirement_count = json.requirements.len() as u32;
                }
                let rendered = serde_json::to_string_pretty(&json).expect("json should serialize");
                println!("{rendered}");
            } else {
                print!("{md}");
            }
            Ok(())
        }
        "change" => {
            let resolved_change = match super::common::resolve_change_target(ito_path, &item) {
                Ok(id) => id,
                Err(msg) => return fail(msg),
            };
            let change_repo = FsChangeRepository::new(ito_path);
            if want_json {
                let files = core_show::read_change_delta_spec_files(&change_repo, &resolved_change)
                    .unwrap_or_default();
                let json = core_show::parse_change_show_json(&resolved_change, &files);
                let rendered = serde_json::to_string_pretty(&json).expect("json should serialize");
                println!("{rendered}");
            } else {
                let md = core_show::read_change_proposal_markdown(&change_repo, &resolved_change)
                    .map_err(to_cli_error)?
                    .unwrap_or_default();
                print!("{md}");
            }
            Ok(())
        }
        _ => fail("Unhandled show type"),
    }
}

pub(crate) fn handle_show_clap(rt: &Runtime, args: &ShowArgs) -> CliResult<()> {
    let mut argv: Vec<String> = Vec::new();

    if args.json {
        argv.push("--json".to_string());
    }
    if let Some(typ) = args.typ {
        let s = match typ {
            ShowItemType::Change => "change",
            ShowItemType::Spec => "spec",
        };
        argv.push("--type".to_string());
        argv.push(s.to_string());
    }
    if args.no_interactive {
        argv.push("--no-interactive".to_string());
    }
    if args.deltas_only {
        argv.push("--deltas-only".to_string());
    }
    if args.requirements_only {
        argv.push("--requirements-only".to_string());
    }
    if args.requirements {
        argv.push("--requirements".to_string());
    }
    if args.no_scenarios {
        argv.push("--no-scenarios".to_string());
    }
    if let Some(idx) = args.requirement {
        argv.push("--requirement".to_string());
        argv.push(idx.to_string());
    }

    match &args.command {
        Some(ShowCommand::Module(m)) => {
            argv.push("module".to_string());
            if m.json {
                argv.push("--json".to_string());
            }
            argv.push(m.module_id.clone());
            return handle_show(rt, &argv);
        }
        None => {}
    }

    if let Some(item) = &args.item {
        argv.push(item.clone());
    }

    handle_show(rt, &argv)
}

fn ignored_show_flags(
    typ: &str,
    deltas_only: bool,
    requirements_only: bool,
    requirements: bool,
    scenarios: bool,
    requirement_idx: Option<usize>,
) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    if typ == "spec" {
        if deltas_only {
            out.push("deltasOnly");
        }
        if requirements_only {
            out.push("requirementsOnly");
        }
    } else if typ == "change" {
        // Commander sets `scenarios` default true; TS warns even when not specified.
        if scenarios {
            out.push("scenarios");
        }
        if requirements {
            out.push("requirements");
        }
        if requirement_idx.is_some() {
            out.push("requirement");
        }
    }
    out
}

fn handle_show_module(rt: &Runtime, args: &[String]) -> CliResult<()> {
    // Minimal module show: print module.md if present.
    let want_json = args.iter().any(|a| a == "--json");
    if want_json {
        return fail("Module JSON output is not implemented");
    }
    let module_id = super::common::last_positional(args);
    if module_id.is_none() {
        return fail(
            "Nothing to show. Try one of:\n  ito show module <module-id>\nOr run in an interactive terminal.",
        );
    }
    let module_id = module_id.expect("checked");

    let ito_path = rt.ito_path();

    let module_repo = FsModuleRepository::new(ito_path);
    let module = module_repo.get(&module_id).map_err(to_cli_error)?;

    let module_md_path = module.path.join("module.md");
    let md = ito_common::io::read_to_string_or_default(&module_md_path);
    print!("{md}");

    Ok(())
}

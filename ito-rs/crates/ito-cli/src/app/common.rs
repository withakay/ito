use crate::cli::Cli;
use crate::runtime::Runtime;
use clap::ColorChoice;
use clap::CommandFactory;
use ito_config::ConfigContext;
use ito_core::paths as core_paths;
use ito_core::workflow as core_workflow;
use ito_core::{ChangeRepository, ChangeTargetResolution};
use std::path::Path;

pub(crate) fn schema_not_found_message(ctx: &ConfigContext, name: &str) -> String {
    let schemas = core_workflow::list_available_schemas(ctx);
    let mut msg = format!("Schema '{name}' not found");
    if !schemas.is_empty() {
        msg.push_str(&format!(". Available schemas:\n  {}", schemas.join("\n  ")));
    }
    msg
}

pub(crate) fn render_command_long_help(path: &[&str], bin_name: &str) -> String {
    let mut cmd = Cli::command();
    cmd = cmd.color(ColorChoice::Never);

    if path.is_empty() {
        return cmd.bin_name(bin_name).render_long_help().to_string();
    }

    let mut current = cmd;
    for (i, part) in path.iter().enumerate() {
        let Some(found) = current.find_subcommand_mut(part) else {
            return format!("Usage: {bin_name}\n\n(Help unavailable)");
        };

        let mut found = found.clone().color(ColorChoice::Never);
        if i + 1 == path.len() {
            found = found.bin_name(bin_name);
            return found.render_long_help().to_string();
        }
        current = found;
    }

    format!("Usage: {bin_name}\n\n(Help unavailable)")
}

pub(crate) fn unknown_with_suggestions(kind: &str, item: &str, suggestions: &[String]) -> String {
    let mut msg = format!("Unknown {kind} '{item}'");
    if !suggestions.is_empty() {
        msg.push_str(&format!("\nDid you mean: {}?", suggestions.join(", ")));
    }
    msg
}

pub(crate) fn detect_item_type(
    change_repo: &impl ChangeRepository,
    ito_path: &Path,
    idx: &ito_core::repo_index::RepoIndex,
    item: &str,
) -> String {
    let is_change = match change_repo.resolve_target(item) {
        ChangeTargetResolution::Unique(_) => true,
        ChangeTargetResolution::Ambiguous(_) | ChangeTargetResolution::NotFound => {
            change_repo.exists(item)
        }
    };
    let is_spec = idx.spec_dir_names.iter().any(|n| n == item)
        && core_paths::spec_markdown_path(ito_path, item).exists();
    match (is_change, is_spec) {
        (true, true) => "ambiguous".to_string(),
        (true, false) => "change".to_string(),
        (false, true) => "spec".to_string(),
        _ => "unknown".to_string(),
    }
}

pub(crate) fn list_spec_ids(rt: &Runtime) -> Vec<String> {
    list_spec_ids_from_index(rt.ito_path(), rt.repo_index())
}

pub(crate) fn list_change_ids(change_repo: &impl ChangeRepository) -> Vec<String> {
    change_repo
        .list()
        .map(|changes| changes.into_iter().map(|c| c.id).collect())
        .unwrap_or_default()
}

pub(crate) fn resolve_change_target(
    change_repo: &impl ChangeRepository,
    input: &str,
) -> Result<String, String> {
    match change_repo.resolve_target(input) {
        ChangeTargetResolution::Unique(id) => Ok(id),
        ChangeTargetResolution::Ambiguous(matches) => {
            let mut msg = format!("Change '{input}' is ambiguous. Matches:\n");
            for id in matches.iter().take(8) {
                msg.push_str(&format!("  {id}\n"));
            }
            if matches.len() > 8 {
                msg.push_str(&format!("  ... and {} more\n", matches.len() - 8));
            }
            msg.push_str("Use a longer prefix or the full canonical change ID.");
            Err(msg)
        }
        ChangeTargetResolution::NotFound => {
            let mut msg = format!("Change '{input}' not found");
            let suggestions = change_repo.suggest_targets(input, 5);
            if !suggestions.is_empty() {
                msg.push_str("\n\nDid you mean:\n");
                for suggestion in suggestions {
                    msg.push_str(&format!("  {}\n", suggestion));
                }
            } else {
                let changes = change_repo.list().unwrap_or_default();
                if !changes.is_empty() {
                    msg.push_str("\n\nAvailable changes:\n");
                    for c in changes {
                        msg.push_str(&format!("  {}\n", c.id));
                    }
                }
            }
            Err(msg)
        }
    }
}

pub(crate) fn list_candidate_items(
    change_repo: &impl ChangeRepository,
    rt: &Runtime,
) -> Vec<String> {
    let mut items = list_spec_ids(rt);
    items.extend(list_change_ids(change_repo));
    items
}

pub(crate) fn list_spec_ids_from_index(
    ito_path: &Path,
    idx: &ito_core::repo_index::RepoIndex,
) -> Vec<String> {
    let specs_dir = core_paths::specs_dir(ito_path);
    let mut ids: Vec<String> = Vec::new();
    for id in &idx.spec_dir_names {
        if specs_dir.join(id).join("spec.md").exists() {
            ids.push(id.clone());
        }
    }
    ids.sort();
    ids
}

pub(crate) fn last_positional(args: &[String]) -> Option<String> {
    let mut last: Option<String> = None;
    let mut skip_next = false;
    for a in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if a == "--type"
            || a == "--sort"
            || a == "--module"
            || a == "--concurrency"
            || a == "--requirement"
            || a == "--tools"
            || a == "--schema"
            || a == "-r"
        {
            skip_next = true;
            continue;
        }
        if a.starts_with('-') {
            continue;
        }
        last = Some(a.clone());
    }
    last
}

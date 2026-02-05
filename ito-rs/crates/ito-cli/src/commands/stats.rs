use crate::cli::StatsArgs;
use crate::cli_error::CliResult;
use crate::runtime::Runtime;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub(crate) fn handle_stats_clap(rt: &Runtime, _args: &StatsArgs) -> CliResult<()> {
    let Some(config_dir) = ito_config::ito_config_dir(rt.ctx()) else {
        println!("No Ito config directory found.");
        return Ok(());
    };

    let root = config_dir
        .join("logs")
        .join("execution")
        .join("v1")
        .join("projects");

    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for id in known_command_ids() {
        counts.insert(id.to_string(), 0);
    }

    let mut files: Vec<PathBuf> = Vec::new();
    collect_jsonl_files(&root, &mut files);

    for path in files {
        let Ok(f) = std::fs::File::open(&path) else {
            continue;
        };
        let reader = std::io::BufReader::new(f);
        for line in reader.lines() {
            let Ok(line) = line else {
                continue;
            };
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            #[derive(serde::Deserialize)]
            struct Event {
                event_type: Option<String>,
                command_id: Option<String>,
            }

            let Ok(ev) = serde_json::from_str::<Event>(line) else {
                continue;
            };
            let Some(event_type) = ev.event_type else {
                continue;
            };
            if event_type != "command_end" {
                continue;
            }
            let Some(command_id) = ev.command_id else {
                continue;
            };

            let entry = counts.entry(command_id).or_insert(0);
            *entry = entry.saturating_add(1);
        }
    }

    println!("Ito Stats");
    println!("────────────────────────────────────────");
    println!("command_id: count");
    for (id, count) in counts {
        println!("{id}: {count}");
    }

    Ok(())
}

fn collect_jsonl_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries {
        let Ok(e) = e else {
            continue;
        };
        let path = e.path();
        if path.is_dir() {
            collect_jsonl_files(&path, out);
            continue;
        }
        let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
            continue;
        };
        if ext == "jsonl" {
            out.push(path);
        }
    }
}

fn known_command_ids() -> Vec<&'static str> {
    vec![
        "ito.init",
        "ito.update",
        "ito.list",
        "ito.config.path",
        "ito.config.list",
        "ito.config.get",
        "ito.config.set",
        "ito.config.unset",
        "ito.agent_config.init",
        "ito.agent_config.summary",
        "ito.agent_config.get",
        "ito.agent_config.set",
        "ito.create.module",
        "ito.create.change",
        "ito.new.change",
        "ito.plan.init",
        "ito.plan.status",
        "ito.state.show",
        "ito.state.decision",
        "ito.state.blocker",
        "ito.state.note",
        "ito.state.focus",
        "ito.state.question",
        "ito.tasks.init",
        "ito.tasks.status",
        "ito.tasks.next",
        "ito.tasks.start",
        "ito.tasks.complete",
        "ito.tasks.shelve",
        "ito.tasks.unshelve",
        "ito.tasks.add",
        "ito.tasks.show",
        "ito.workflow.init",
        "ito.workflow.list",
        "ito.workflow.show",
        "ito.status",
        "ito.stats",
        "ito.templates",
        "ito.instructions",
        "ito.x_instructions",
        "ito.agent.instruction",
        "ito.show",
        "ito.validate",
        "ito.ralph",
        "ito.loop",
    ]
}

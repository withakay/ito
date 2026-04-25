//! Memory instruction artifacts (`memory-capture`, `memory-search`,
//! `memory-query`).
//!
//! Each artifact:
//!
//! 1. Parses operation-specific flags from the raw arg list.
//! 2. Loads the merged Ito config and extracts `memory.<op>` if present.
//! 3. Renders one of three branches via [`ito_core::memory`]:
//!    - `Command` — emits the rendered shell command line.
//!    - `Skill`   — emits a "invoke this skill with these inputs" directive.
//!    - `NotConfigured` — emits operator setup guidance.
//!
//! Output is plain text by default, or a JSON envelope when `--json` is
//! present.

use std::collections::BTreeMap;

use ito_config::load_cascading_project_config;
use ito_config::types::ItoConfig;
use ito_core::memory::{
    self, CaptureInputs, Operation, QueryInputs, RenderedInstruction, SearchInputs,
};
use serde::Serialize;
use serde_json::Value;

use crate::cli_error::{CliResult, fail, to_cli_error};
use crate::runtime::Runtime;
use crate::util::{parse_repeated_string_flag, parse_string_flag};

/// Dispatch any memory-* instruction. Returns `Ok(true)` if the artifact
/// matched a memory artifact (and was handled), `Ok(false)` if no match (so
/// the caller can keep looking).
pub(crate) fn try_handle(
    rt: &Runtime,
    artifact: &str,
    args: &[String],
    want_json: bool,
) -> CliResult<bool> {
    match artifact {
        "memory-capture" => {
            handle_memory_capture(rt, args, want_json)?;
            Ok(true)
        }
        "memory-search" => {
            handle_memory_search(rt, args, want_json)?;
            Ok(true)
        }
        "memory-query" => {
            handle_memory_query(rt, args, want_json)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn handle_memory_capture(rt: &Runtime, args: &[String], want_json: bool) -> CliResult<()> {
    let inputs = CaptureInputs {
        context: parse_string_flag(args, "--context"),
        files: parse_repeated_string_flag(args, "--file"),
        folders: parse_repeated_string_flag(args, "--folder"),
    };
    let config = load_memory_config(rt)?;
    let rendered = memory::render_capture(config.as_ref(), &inputs);
    emit_rendered(rendered, "memory-capture", want_json)
}

fn handle_memory_search(rt: &Runtime, args: &[String], want_json: bool) -> CliResult<()> {
    let Some(query) = parse_string_flag(args, "--query") else {
        return fail("Missing required option --query for memory-search");
    };
    let limit = match parse_string_flag(args, "--limit") {
        Some(s) => match s.parse::<u64>() {
            Ok(n) if n > 0 => Some(n),
            _ => {
                return fail(format!(
                    "Invalid value for --limit ('{s}'). Provide a positive integer."
                ));
            }
        },
        None => None,
    };
    let inputs = SearchInputs {
        query,
        limit,
        scope: parse_string_flag(args, "--scope"),
    };
    let config = load_memory_config(rt)?;
    let rendered = memory::render_search(config.as_ref(), &inputs);
    emit_rendered(rendered, "memory-search", want_json)
}

fn handle_memory_query(rt: &Runtime, args: &[String], want_json: bool) -> CliResult<()> {
    let Some(query) = parse_string_flag(args, "--query") else {
        return fail("Missing required option --query for memory-query");
    };
    let inputs = QueryInputs { query };
    let config = load_memory_config(rt)?;
    let rendered = memory::render_query(config.as_ref(), &inputs);
    emit_rendered(rendered, "memory-query", want_json)
}

fn load_memory_config(rt: &Runtime) -> CliResult<Option<ito_config::types::MemoryConfig>> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let typed: ItoConfig = serde::Deserialize::deserialize(&merged).map_err(|e| {
        to_cli_error(format!(
            "Failed to parse merged Ito config while preparing memory instruction.\n\
             \n\
             Why: The merged config contains an invalid value or type, so the memory provider cannot be resolved.\n\
             \n\
             How to fix: Inspect your config files (or run `ito config check`) and correct the invalid field, then retry.\n\
             \n\
             Underlying error: {e}"
        ))
    })?;
    Ok(typed.memory)
}

#[derive(Serialize)]
struct OutputEnvelope<'a> {
    artifact: &'a str,
    branch: &'a str,
    instruction: String,
    skill: Option<&'a str>,
    inputs: Option<&'a BTreeMap<String, Value>>,
    options: Option<&'a Value>,
    operation: Option<&'a str>,
}

fn emit_rendered(
    rendered: RenderedInstruction,
    artifact: &str,
    want_json: bool,
) -> CliResult<()> {
    match rendered {
        RenderedInstruction::Command { line } => {
            let body = render_command_text(artifact, &line);
            if want_json {
                let env = OutputEnvelope {
                    artifact,
                    branch: "command",
                    instruction: body,
                    skill: None,
                    inputs: None,
                    options: None,
                    operation: None,
                };
                let json = serde_json::to_string_pretty(&env)
                    .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
                println!("{json}");
            } else {
                println!("{body}");
            }
            Ok(())
        }
        RenderedInstruction::Skill {
            skill_id,
            inputs,
            options,
        } => {
            let body = render_skill_text(artifact, &skill_id, &inputs, options.as_ref());
            if want_json {
                let env = OutputEnvelope {
                    artifact,
                    branch: "skill",
                    instruction: body,
                    skill: Some(&skill_id),
                    inputs: Some(&inputs),
                    options: options.as_ref(),
                    operation: None,
                };
                let json = serde_json::to_string_pretty(&env)
                    .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
                println!("{json}");
            } else {
                println!("{body}");
            }
            Ok(())
        }
        RenderedInstruction::NotConfigured { operation } => {
            let body = render_not_configured_text(operation);
            if want_json {
                let env = OutputEnvelope {
                    artifact,
                    branch: "not_configured",
                    instruction: body,
                    skill: None,
                    inputs: None,
                    options: None,
                    operation: Some(operation.as_key()),
                };
                let json = serde_json::to_string_pretty(&env)
                    .map_err(|e| to_cli_error(format!("serializing response: {e}")))?;
                println!("{json}");
            } else {
                println!("{body}");
            }
            Ok(())
        }
    }
}

fn render_command_text(artifact: &str, line: &str) -> String {
    let header = match artifact {
        "memory-capture" => "Capture this memory by running:",
        "memory-search" => "Search memory by running:",
        "memory-query" => "Query memory by running:",
        _ => "Run this command:",
    };
    format!("{header}\n\n```bash\n{line}\n```\n")
}

fn render_skill_text(
    artifact: &str,
    skill_id: &str,
    inputs: &BTreeMap<String, Value>,
    options: Option<&Value>,
) -> String {
    let header = match artifact {
        "memory-capture" => "Capture this memory by invoking the configured skill:",
        "memory-search" => "Search memory by invoking the configured skill:",
        "memory-query" => "Query memory by invoking the configured skill:",
        _ => "Invoke the configured skill:",
    };
    let mut out = format!("{header}\n\n- Skill: `{skill_id}`\n- Inputs:\n");
    for (key, value) in inputs {
        let pretty = serde_json::to_string(value)
            .unwrap_or_else(|_| "<unrenderable>".to_string());
        out.push_str(&format!("  - `{key}` = `{pretty}`\n"));
    }
    if let Some(options) = options {
        let pretty = serde_json::to_string_pretty(options)
            .unwrap_or_else(|_| "<unrenderable>".to_string());
        out.push_str(&format!(
            "- Options (passed through verbatim):\n```json\n{pretty}\n```\n"
        ));
    } else {
        out.push_str("- Options: (none configured)\n");
    }
    out
}

fn render_not_configured_text(operation: Operation) -> String {
    let op = operation.as_key();
    let (cmd_example, skill_example) = match operation {
        Operation::Capture => (
            "{ \"kind\": \"command\", \"command\": \"brv curate \\\"{context}\\\" {files} {folders}\" }",
            "{ \"kind\": \"skill\", \"skill\": \"ito-memory-markdown\", \"options\": { \"root\": \".ito/memories\" } }",
        ),
        Operation::Search => (
            "{ \"kind\": \"command\", \"command\": \"brv search \\\"{query}\\\" --limit {limit}{scope}\" }",
            "{ \"kind\": \"skill\", \"skill\": \"byterover-explore\" }",
        ),
        Operation::Query => (
            "{ \"kind\": \"command\", \"command\": \"brv query \\\"{query}\\\"\" }",
            "{ \"kind\": \"skill\", \"skill\": \"byterover-explore\" }",
        ),
    };

    format!(
        "Memory `{op}` is not configured.\n\n\
Configure it by adding `memory.{op}` to your Ito config (`.ito/config.json`).\
 The value picks one of two shapes:\n\n\
- **Command** template — Ito renders a shell command line:\n\n\
```json\n\"memory\": {{ \"{op}\": {cmd_example} }}\n```\n\n\
- **Skill** delegation — Ito directs the agent to invoke an installed skill:\n\n\
```json\n\"memory\": {{ \"{op}\": {skill_example} }}\n```\n\n\
There is no default provider; an unconfigured operation always renders this guidance.\n",
    )
}

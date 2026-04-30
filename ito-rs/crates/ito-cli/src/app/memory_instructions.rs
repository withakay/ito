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
use ito_config::types::{ItoConfig, MemoryConfig, MemoryOpConfig};
use ito_core::memory::{
    self, CaptureInputs, Operation, QueryInputs, RenderedInstruction, SearchInputs,
};
use serde::Serialize;
use serde_json::Value;

/// Per-operation flag exposed to the apply / finish instruction templates.
///
/// Templates use `{% if memory.<op>.configured %}` guards to decide whether
/// to render memory-related reminders.
#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct MemoryOpTemplateState {
    /// `true` when the operation has a configured provider (`skill` or `command`).
    pub(crate) configured: bool,
}

/// Memory state surfaced to instruction templates.
///
/// Each operation tracks whether it is configured. The `instructions.rs`
/// renderers feed this into the `apply.md.j2` and `finish.md.j2` contexts
/// so the templates can independently decide which reminders to show.
#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct MemoryTemplateConfig {
    pub(crate) capture: MemoryOpTemplateState,
    pub(crate) search: MemoryOpTemplateState,
    pub(crate) query: MemoryOpTemplateState,
}

/// Build a [`MemoryTemplateConfig`] from a merged Ito config value.
///
/// Returns the all-`false` default when the merged config either has no
/// `memory` section or fails to deserialize as [`ItoConfig`]. The latter is
/// rare (the strict `ito validate` command would surface it first), so we
/// degrade silently here and let the template render the not-configured
/// branch rather than failing instruction generation.
pub(crate) fn memory_template_config_from_merged(
    merged: &serde_json::Value,
) -> MemoryTemplateConfig {
    let typed: ItoConfig = serde::Deserialize::deserialize(merged).unwrap_or_default();
    let memory: Option<MemoryConfig> = typed.memory;
    let configured = |op: &Option<MemoryOpConfig>| op.is_some();
    match memory {
        Some(m) => MemoryTemplateConfig {
            capture: MemoryOpTemplateState {
                configured: configured(&m.capture),
            },
            search: MemoryOpTemplateState {
                configured: configured(&m.search),
            },
            query: MemoryOpTemplateState {
                configured: configured(&m.query),
            },
        },
        None => MemoryTemplateConfig::default(),
    }
}

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

fn emit_rendered(rendered: RenderedInstruction, artifact: &str, want_json: bool) -> CliResult<()> {
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

pub(crate) fn render_memory_instruction_text(
    rendered: &RenderedInstruction,
    artifact: &str,
) -> Option<String> {
    match rendered {
        RenderedInstruction::Command { line } => Some(render_command_text(artifact, line)),
        RenderedInstruction::Skill {
            skill_id,
            inputs,
            options,
        } => Some(render_skill_text(
            artifact,
            skill_id,
            inputs,
            options.as_ref(),
        )),
        RenderedInstruction::NotConfigured { operation: _ } => None,
    }
}

fn render_command_text(artifact: &str, line: &str) -> String {
    let header = match artifact {
        "memory-capture" => "Capture this memory by running:",
        "memory-search" => "Search memory by running:",
        "memory-query" => "Query memory by running:",
        _ => "Run this command:",
    };
    let guidance = provider_operation_guidance(artifact, "command");
    format!("{guidance}{header}\n\n```bash\n{line}\n```\n")
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
    let guidance = provider_operation_guidance(artifact, "skill");
    let mut out = format!(
        "{guidance}{header}\n\n- Invoke the skill exactly as named; pass inputs/options without inventing provider-specific defaults.\n- Skill: `{skill_id}`\n- Inputs:\n"
    );
    for (key, value) in inputs {
        let pretty = serde_json::to_string(value).unwrap_or_else(|_| "<unrenderable>".to_string());
        out.push_str(&format!("  - `{key}` = `{pretty}`\n"));
    }
    if let Some(options) = options {
        let pretty =
            serde_json::to_string_pretty(options).unwrap_or_else(|_| "<unrenderable>".to_string());
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
    let artifact = artifact_for_operation(operation);
    let guidance = provider_operation_guidance(artifact, "not-configured");
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
        "{guidance}Memory `{op}` is not configured.\n\n\
Configure it by adding `memory.{op}` to your Ito config (`.ito/config.json`).\
 The value picks one of two shapes:\n\n\
- **Command** template — Ito renders a shell command line:\n\n\
```json\n\"memory\": {{ \"{op}\": {cmd_example} }}\n```\n\n\
- **Skill** delegation — Ito directs the agent to invoke an installed skill:\n\n\
```json\n\"memory\": {{ \"{op}\": {skill_example} }}\n```\n\n\
There is no default provider; an unconfigured operation always renders this guidance.\n\n\
## Fallback guidance\n\n\
Continue with normal repository inspection when this operation is unavailable, and mention that Ito memory `{op}` is not configured if it affects the task.\n",
    )
}

fn provider_operation_guidance(artifact: &str, branch: &str) -> String {
    let (operation, required_inputs) = match artifact {
        "memory-capture" => (
            "capture",
            "Required inputs: `--context`; optional repeatable `--file` and `--folder`",
        ),
        "memory-search" => (
            "search",
            "Required inputs: `--query`; optional `--limit` and `--scope`",
        ),
        "memory-query" => ("query", "Required inputs: `--query`"),
        _ => ("unknown", "Required inputs: see the artifact help"),
    };

    format!(
        "## Provider Operation Guidance\n\n\
- Artifact: `{artifact}`\n\
- Operation: `{operation}`\n\
- Configured branch: `{branch}`\n\
- Provider routing is resolved from `.ito/config.json` (`memory.{operation}`).\n\
- {required_inputs}\n\n"
    )
}

fn artifact_for_operation(operation: Operation) -> &'static str {
    match operation {
        Operation::Capture => "memory-capture",
        Operation::Search => "memory-search",
        Operation::Query => "memory-query",
    }
}

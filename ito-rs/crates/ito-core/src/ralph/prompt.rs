//! Prompt construction for Ralph loop iterations.
//!
//! The Ralph loop assembles a single prompt string that includes optional Ito
//! context (change proposal + module), the user's base prompt, and a fixed
//! preamble describing the iteration rules.

use crate::validate;
use miette::{Result, miette};
use std::path::Path;

use ito_common::fs::StdFs;
use ito_common::paths;

/// Options that control which context is embedded into a Ralph prompt.
pub struct BuildPromptOptions {
    /// Optional change id (e.g. `014-01_add-rust-crate-documentation`).
    pub change_id: Option<String>,
    /// Optional module id (e.g. `014`).
    pub module_id: Option<String>,

    /// Iteration number to display in the preamble.
    pub iteration: Option<u32>,
    /// Optional maximum number of iterations (used only for display).
    pub max_iterations: Option<u32>,
    /// Minimum iteration count required before a completion promise is honored.
    pub min_iterations: u32,

    /// The completion promise token (e.g. `COMPLETE`).
    pub completion_promise: String,

    /// Optional additional context injected mid-loop.
    pub context_content: Option<String>,

    /// Optional validation failure output from the previous iteration.
    ///
    /// When present, the prompt includes a section explaining completion was rejected.
    pub validation_failure: Option<String>,
}

/// Build the standard Ralph preamble for a given iteration.
///
/// This is the outer wrapper around the task content; it communicates the loop
/// rules and the completion promise the harness must emit.
pub fn build_prompt_preamble(
    iteration: u32,
    max_iterations: Option<u32>,
    min_iterations: u32,
    completion_promise: &str,
    context_content: Option<&str>,
    validation_failure: Option<&str>,
    task: &str,
) -> String {
    let has_finite_max = max_iterations.is_some_and(|v| v > 0);
    let normalized_context = context_content.unwrap_or("").trim();
    let context_section = if normalized_context.is_empty() {
        String::new()
    } else {
        format!(
            "\n## Additional Context (added by user mid-loop)\n\n{c}\n\n---\n",
            c = normalized_context
        )
    };

    let normalized_validation = validation_failure.unwrap_or("").trim();
    let validation_section = if normalized_validation.is_empty() {
        String::new()
    } else {
        format!(
            "\n## Validation Failure (completion rejected)\n\nRalph detected a completion promise, but it was rejected because validation failed. Fix the issues below and try again.\n\n{v}\n\n---\n",
            v = normalized_validation
        )
    };

    let max_str = if has_finite_max {
        format!(" / {}", max_iterations.unwrap())
    } else {
        " (unlimited)".to_string()
    };

    format!(
        "# Ralph Wiggum Loop - Iteration {iteration}\n\nYou are in an iterative development loop. Work on the task below until you can genuinely complete it.\n\nImportant: Ralph validates completion promises before exiting (tasks + project checks/tests).\n{context_section}{validation_section}## Your Task\n\n{task}\n\n## Instructions\n\n1. Read the current state of files to understand what's been done\n2. **Update your todo list** - Use the TodoWrite tool to track progress and plan remaining work\n3. Make progress on the task\n4. Run tests/verification if applicable\n5. When the task is GENUINELY COMPLETE, output:\n   <promise>{completion_promise}</promise>\n\n## Critical Rules\n\n- ONLY output <promise>{completion_promise}</promise> when the task is truly done\n- Do NOT lie or output false promises to exit the loop\n- If stuck, try a different approach\n- Check your work before claiming completion\n- The loop will continue until you succeed\n- **IMPORTANT**: Update your todo list at the start of each iteration to show progress\n\n## AUTONOMY REQUIREMENTS (CRITICAL)\n\n- **DO NOT ASK QUESTIONS** - This is an autonomous loop with no human interaction\n- **DO NOT USE THE QUESTION TOOL** - Work independently without prompting for input\n- Make reasonable assumptions when information is missing\n- Use your best judgment to resolve ambiguities\n- If multiple approaches exist, choose the most reasonable one and proceed\n- The orchestrator cannot respond to questions - you must be self-sufficient\n- Trust your training and make decisions autonomously\n\n## Current Iteration: {iteration}{max_str} (min: {min_iterations})\n\nNow, work on the task autonomously. Good luck!",
        iteration = iteration,
        context_section = context_section,
        validation_section = validation_section,
        task = task,
        completion_promise = completion_promise,
        max_str = max_str,
        min_iterations = min_iterations,
    )
}

/// Build a full Ralph prompt with optional change/module context.
///
/// When `options.iteration` is set, this includes the iteration preamble.
pub fn build_ralph_prompt(
    ito_path: &Path,
    user_prompt: &str,
    options: BuildPromptOptions,
) -> Result<String> {
    let mut sections: Vec<String> = Vec::new();

    if let Some(change_id) = options.change_id.as_deref()
        && let Some(ctx) = load_change_context(ito_path, change_id)?
    {
        sections.push(ctx);
    }

    if let Some(module_id) = options.module_id.as_deref()
        && let Some(ctx) = load_module_context(ito_path, module_id)?
    {
        sections.push(ctx);
    }

    sections.push(user_prompt.to_string());
    let task = sections.join("\n\n---\n\n");

    if let Some(iteration) = options.iteration {
        Ok(build_prompt_preamble(
            iteration,
            options.max_iterations,
            options.min_iterations,
            &options.completion_promise,
            options.context_content.as_deref(),
            options.validation_failure.as_deref(),
            &task,
        )
        .trim()
        .to_string())
    } else {
        Ok(task)
    }
}

fn load_change_context(ito_path: &Path, change_id: &str) -> Result<Option<String>> {
    let changes_dir = paths::changes_dir(ito_path);
    let resolved = resolve_change_id(&changes_dir, change_id)?;
    let Some(resolved) = resolved else {
        return Ok(None);
    };

    let proposal_path = changes_dir.join(&resolved).join("proposal.md");
    if !proposal_path.exists() {
        return Ok(None);
    }

    let proposal = ito_common::io::read_to_string(&proposal_path)?;
    Ok(Some(format!(
        "## Change Proposal ({id})\n\n{proposal}",
        id = resolved,
        proposal = proposal
    )))
}

fn resolve_change_id(changes_dir: &Path, input: &str) -> Result<Option<String>> {
    let direct = changes_dir.join(input);
    if direct.exists() {
        return Ok(Some(input.to_string()));
    }

    if !changes_dir.exists() {
        return Ok(None);
    }

    let mut matches: Vec<String> = Vec::new();
    let fs = StdFs;
    for name in ito_domain::discovery::list_dir_names(&fs, changes_dir)? {
        if name == "archive" {
            continue;
        }
        if name.starts_with(input) {
            matches.push(name);
        }
    }

    matches.sort();
    matches.dedup();

    match matches.len() {
        0 => Ok(None),
        1 => Ok(Some(matches[0].clone())),
        _ => Err(miette!(
            "Ambiguous change id '{input}'. Matches: {matches}",
            input = input,
            matches = matches.join(", ")
        )),
    }
}

fn load_module_context(ito_path: &Path, module_id: &str) -> Result<Option<String>> {
    let resolved = validate::resolve_module(ito_path, module_id)?;
    let Some(resolved) = resolved else {
        return Ok(None);
    };

    if !resolved.module_md.exists() {
        return Ok(None);
    }

    let module_content = ito_common::io::read_to_string(&resolved.module_md)?;
    Ok(Some(format!(
        "## Module ({id})\n\n{content}",
        id = resolved.id,
        content = module_content
    )))
}

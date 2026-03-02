//! Handler for `ito util` subcommands.

use crate::cli::{ParseIdArgs, UtilArgs, UtilCommand};
use crate::cli_error::CliResult;
use ito_common::id::{
    looks_like_change_id, looks_like_module_id, parse_change_id, parse_module_id,
};

/// Keywords that signal "continue-ready" intent (pick the next ready change).
const CONTINUE_READY_KEYWORDS: &[&str] =
    &["next", "continue", "ready", "continue-ready", "pick", "go"];

/// Dispatch `ito util <subcommand>`.
pub(crate) fn handle_util_clap(args: &UtilArgs) -> CliResult<()> {
    match &args.action {
        UtilCommand::ParseId(parse_args) => handle_parse_id(parse_args),
    }
}

/// Classify a single input string and return a JSON value.
///
/// Classification order:
/// 1. Try `parse_change_id` (strict) — produces `"change"` type.
/// 2. Try `parse_module_id` (strict) — produces `"module"` type.
/// 3. Check for continue-ready keywords — produces `"continue_ready"` type.
/// 4. Fall through to `"unknown"`.
fn classify_input(input: &str) -> serde_json::Value {
    let trimmed = input.trim();

    // 1. Change ID — must look plausible first (avoids false positives on bare numbers).
    if looks_like_change_id(trimmed)
        && let Ok(parsed) = parse_change_id(trimmed)
    {
        return serde_json::json!({
            "input": input,
            "type": "change",
            "change_id": parsed.canonical.as_str(),
            "module_id": parsed.module_id.as_str(),
            "change_num": parsed.change_num,
            "name": parsed.name,
        });
    }

    // 2. Module ID — digit-prefixed strings like "012" or "005_my-module".
    if looks_like_module_id(trimmed)
        && let Ok(parsed) = parse_module_id(trimmed)
    {
        let mut obj = serde_json::json!({
            "input": input,
            "type": "module",
            "module_id": parsed.module_id.as_str(),
        });
        if let Some(name) = &parsed.module_name {
            obj["module_name"] = serde_json::json!(name);
        }
        return obj;
    }

    // 3. Continue-ready keywords (case-insensitive, check each word).
    let lower = trimmed.to_ascii_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let is_continue_ready = words.iter().any(|w| CONTINUE_READY_KEYWORDS.contains(w));
    if is_continue_ready {
        return serde_json::json!({
            "input": input,
            "type": "continue_ready",
        });
    }

    // 4. Unknown.
    serde_json::json!({
        "input": input,
        "type": "unknown",
    })
}

/// Handle `ito util parse-id <inputs...>`.
fn handle_parse_id(args: &ParseIdArgs) -> CliResult<()> {
    let results: Vec<serde_json::Value> = args.inputs.iter().map(|i| classify_input(i)).collect();

    let output = if results.len() == 1 {
        serde_json::to_string_pretty(&results[0]).expect("json should serialize")
    } else {
        serde_json::to_string_pretty(&results).expect("json should serialize")
    };

    println!("{output}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_change_id() {
        let result = classify_input("005-01_add-auth");
        assert_eq!(result["type"], "change");
        assert_eq!(result["change_id"], "005-01_add-auth");
        assert_eq!(result["module_id"], "005");
        assert_eq!(result["change_num"], "01");
        assert_eq!(result["name"], "add-auth");
    }

    #[test]
    fn classify_change_id_flexible_padding() {
        let result = classify_input("5-1_foo");
        assert_eq!(result["type"], "change");
        assert_eq!(result["change_id"], "005-01_foo");
    }

    #[test]
    fn classify_module_id_bare_number() {
        let result = classify_input("012");
        assert_eq!(result["type"], "module");
        assert_eq!(result["module_id"], "012");
        assert!(result.get("module_name").is_none());
    }

    #[test]
    fn classify_module_id_with_name() {
        let result = classify_input("005_my-module");
        assert_eq!(result["type"], "module");
        assert_eq!(result["module_id"], "005");
        assert_eq!(result["module_name"], "my-module");
    }

    #[test]
    fn classify_module_id_flexible_padding() {
        let result = classify_input("5");
        assert_eq!(result["type"], "module");
        assert_eq!(result["module_id"], "005");
    }

    #[test]
    fn classify_continue_ready_keyword_next() {
        let result = classify_input("next");
        assert_eq!(result["type"], "continue_ready");
    }

    #[test]
    fn classify_continue_ready_keyword_continue() {
        let result = classify_input("continue");
        assert_eq!(result["type"], "continue_ready");
    }

    #[test]
    fn classify_continue_ready_phrase() {
        let result = classify_input("pick next ready");
        assert_eq!(result["type"], "continue_ready");
    }

    #[test]
    fn classify_continue_ready_case_insensitive() {
        let result = classify_input("NEXT");
        assert_eq!(result["type"], "continue_ready");
    }

    #[test]
    fn classify_unknown_input() {
        let result = classify_input("something-random");
        assert_eq!(result["type"], "unknown");
    }

    #[test]
    fn classify_empty_string() {
        let result = classify_input("");
        assert_eq!(result["type"], "unknown");
    }

    #[test]
    fn classify_preserves_original_input() {
        let result = classify_input("  012  ");
        assert_eq!(result["input"], "  012  ");
        assert_eq!(result["type"], "module");
        assert_eq!(result["module_id"], "012");
    }
}

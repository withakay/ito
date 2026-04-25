//! Placeholder rendering for the `command` shape of memory operations.
//!
//! Rules (from the `agent-memory-abstraction` spec):
//!
//! - **Scalar string** (`{context}`, `{query}`, `{scope}`) — replaced with
//!   a single shell-quoted token. Missing values render as the empty
//!   string.
//! - **Scalar integer** (`{limit}`) — replaced with the decimal literal,
//!   or empty when unset and no operation default applies.
//! - **List** (`{files}`, `{folders}`) — expanded as repeated flags. The
//!   flag name is fixed by the placeholder name (`{files}` → `--file`,
//!   `{folders}` → `--folder`). Empty lists render as empty strings.
//! - **Unknown placeholder** (e.g. `{foo}`) — preserved literally.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::{CaptureInputs, DEFAULT_SEARCH_LIMIT, QueryInputs, SearchInputs};

/// POSIX single-quote shell quoting.
///
/// Wraps `value` in single quotes and escapes any embedded single quote with
/// the canonical `'\''` sequence. The empty string renders as `''`. Pasting
/// the output into a POSIX shell preserves the original byte sequence.
#[must_use]
pub fn shell_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Render the `memory-capture` command template with placeholder
/// substitution.
pub(super) fn render_capture_command(template: &str, inputs: &CaptureInputs) -> String {
    let context_quoted = shell_quote(inputs.context.as_deref().unwrap_or(""));
    let files = render_repeated_flag("--file", &inputs.files);
    let folders = render_repeated_flag("--folder", &inputs.folders);
    substitute(template, |name| match name {
        "context" => Some(context_quoted.clone()),
        "files" => Some(files.clone()),
        "folders" => Some(folders.clone()),
        _ => None,
    })
}

/// Render the `memory-search` command template with placeholder
/// substitution.
pub(super) fn render_search_command(template: &str, inputs: &SearchInputs) -> String {
    let query_quoted = shell_quote(&inputs.query);
    let limit_value = inputs
        .limit
        .unwrap_or(DEFAULT_SEARCH_LIMIT)
        .to_string();
    let scope_quoted = inputs
        .scope
        .as_ref()
        .map(|s| shell_quote(s))
        .unwrap_or_default();
    substitute(template, |name| match name {
        "query" => Some(query_quoted.clone()),
        "limit" => Some(limit_value.clone()),
        "scope" => Some(scope_quoted.clone()),
        _ => None,
    })
}

/// Render the `memory-query` command template with placeholder substitution.
pub(super) fn render_query_command(template: &str, inputs: &QueryInputs) -> String {
    let query_quoted = shell_quote(&inputs.query);
    substitute(template, |name| match name {
        "query" => Some(query_quoted.clone()),
        _ => None,
    })
}

/// Convert capture inputs to the structured map passed to a delegated skill.
pub(super) fn capture_inputs_as_structured(inputs: &CaptureInputs) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    out.insert(
        "context".to_string(),
        inputs
            .context
            .as_ref()
            .map_or(Value::Null, |s| Value::String(s.clone())),
    );
    out.insert("files".to_string(), json!(inputs.files));
    out.insert("folders".to_string(), json!(inputs.folders));
    out
}

/// Convert search inputs to the structured map passed to a delegated skill.
pub(super) fn search_inputs_as_structured(inputs: &SearchInputs) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    out.insert("query".to_string(), Value::String(inputs.query.clone()));
    out.insert(
        "limit".to_string(),
        Value::Number(inputs.limit.unwrap_or(DEFAULT_SEARCH_LIMIT).into()),
    );
    out.insert(
        "scope".to_string(),
        inputs
            .scope
            .as_ref()
            .map_or(Value::Null, |s| Value::String(s.clone())),
    );
    out
}

/// Convert query inputs to the structured map passed to a delegated skill.
pub(super) fn query_inputs_as_structured(inputs: &QueryInputs) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    out.insert("query".to_string(), Value::String(inputs.query.clone()));
    out
}

/// Render `--<flag> 'value'` pairs for each value, joined by single spaces.
/// Empty `values` returns an empty string.
fn render_repeated_flag(flag: &str, values: &[String]) -> String {
    let mut out = String::new();
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(flag);
        out.push(' ');
        out.push_str(&shell_quote(value));
    }
    out
}

/// Walk `template` and replace `{name}` placeholders using `lookup`.
///
/// When `lookup(name)` returns `Some(value)`, the entire `{name}` token is
/// replaced with `value`. When `lookup` returns `None`, the placeholder is
/// preserved literally (including its braces) — this matches the spec's
/// "unknown placeholders pass through" rule.
fn substitute<F>(template: &str, lookup: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    let bytes = template.as_bytes();
    let mut out = String::with_capacity(template.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            // Find a matching closing brace within the same input. We don't
            // support nested braces — they're not part of the spec.
            let rest = &template[i + 1..];
            if let Some(end) = rest.find('}') {
                let name = &rest[..end];
                if is_placeholder_name(name) {
                    if let Some(replacement) = lookup(name) {
                        out.push_str(&replacement);
                    } else {
                        // Unknown placeholder: preserve literally.
                        out.push('{');
                        out.push_str(name);
                        out.push('}');
                    }
                    i += 1 + end + 1; // consume "{name}"
                    continue;
                }
            }
        }
        // Default: copy this character literally. Use char_indices to handle
        // multi-byte UTF-8 correctly.
        let ch = template[i..].chars().next().expect("non-empty by loop guard");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// A valid placeholder name is non-empty and consists only of ASCII
/// alphanumerics and underscores. This lets the substituter tell the
/// difference between `{files}` (a placeholder) and `{}` or `{foo bar}`
/// (literal text).
fn is_placeholder_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

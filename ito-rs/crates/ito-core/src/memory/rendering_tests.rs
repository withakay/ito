use super::rendering::shell_quote;
use super::*;
use ito_config::types::{MemoryConfig, MemoryOpConfig};
use serde_json::{Value, json};

fn capture_command_cfg(template: &str) -> MemoryConfig {
    MemoryConfig {
        capture: Some(MemoryOpConfig::Command {
            command: template.to_string(),
        }),
        search: None,
        query: None,
    }
}

fn search_command_cfg(template: &str) -> MemoryConfig {
    MemoryConfig {
        capture: None,
        search: Some(MemoryOpConfig::Command {
            command: template.to_string(),
        }),
        query: None,
    }
}

fn query_command_cfg(template: &str) -> MemoryConfig {
    MemoryConfig {
        capture: None,
        search: None,
        query: Some(MemoryOpConfig::Command {
            command: template.to_string(),
        }),
    }
}

#[track_caller]
fn assert_command(rendered: &RenderedInstruction, expected: &str) {
    match rendered {
        RenderedInstruction::Command { line } => assert_eq!(line, expected),
        other => panic!("expected Command branch, got {other:?}"),
    }
}

// ---- Shell quoting ----------------------------------------------------------

#[test]
fn shell_quote_handles_empty_string() {
    assert_eq!(shell_quote(""), "''");
}

#[test]
fn shell_quote_wraps_simple_strings_in_single_quotes() {
    assert_eq!(shell_quote("foo"), "'foo'");
    assert_eq!(shell_quote("foo bar"), "'foo bar'");
}

#[test]
fn shell_quote_escapes_embedded_single_quotes() {
    assert_eq!(shell_quote("it's"), "'it'\\''s'");
    assert_eq!(shell_quote("'"), "''\\'''");
}

#[test]
fn shell_quote_preserves_unicode_bytes() {
    assert_eq!(shell_quote("naïve résumé"), "'naïve résumé'");
}

// ---- Placeholder substitution: capture --------------------------------------

#[test]
fn capture_command_substitutes_context_with_quoting() {
    let cfg = capture_command_cfg("brv curate {context}");
    let inputs = CaptureInputs {
        context: Some("decision X".to_string()),
        ..Default::default()
    };
    assert_command(
        &render_capture(Some(&cfg), &inputs),
        "brv curate 'decision X'",
    );
}

#[test]
fn capture_command_substitutes_missing_context_with_empty_quoted_string() {
    let cfg = capture_command_cfg("brv curate {context}");
    let inputs = CaptureInputs::default();
    assert_command(&render_capture(Some(&cfg), &inputs), "brv curate ''");
}

#[test]
fn capture_command_expands_files_as_repeated_flags() {
    let cfg = capture_command_cfg("brv curate {context} {files}");
    let inputs = CaptureInputs {
        context: Some("notes".to_string()),
        files: vec!["a.md".to_string(), "b.md".to_string()],
        folders: vec![],
    };
    assert_command(
        &render_capture(Some(&cfg), &inputs),
        "brv curate 'notes' --file 'a.md' --file 'b.md'",
    );
}

#[test]
fn capture_command_expands_folders_with_explicit_flag_name() {
    let cfg = capture_command_cfg("brv curate {context} {folders}");
    let inputs = CaptureInputs {
        context: Some("dir notes".to_string()),
        folders: vec!["docs/".to_string()],
        files: vec![],
    };
    assert_command(
        &render_capture(Some(&cfg), &inputs),
        "brv curate 'dir notes' --folder 'docs/'",
    );
}

#[test]
fn capture_command_empty_lists_render_as_empty_strings() {
    let cfg = capture_command_cfg("brv curate {context} {files} {folders}");
    let inputs = CaptureInputs {
        context: Some("ctx".to_string()),
        files: vec![],
        folders: vec![],
    };
    assert_command(&render_capture(Some(&cfg), &inputs), "brv curate 'ctx'  ");
}

#[test]
fn capture_command_preserves_unknown_placeholders_literally() {
    let cfg = capture_command_cfg("brv curate {foo} --note {context}");
    let inputs = CaptureInputs {
        context: Some("hi".to_string()),
        ..Default::default()
    };
    assert_command(
        &render_capture(Some(&cfg), &inputs),
        "brv curate {foo} --note 'hi'",
    );
}

#[test]
fn capture_command_quotes_shell_metacharacters() {
    let cfg = capture_command_cfg("brv curate {context}");
    let inputs = CaptureInputs {
        context: Some("foo $(rm -rf /); echo 'gotcha'".to_string()),
        ..Default::default()
    };
    assert_command(
        &render_capture(Some(&cfg), &inputs),
        "brv curate 'foo $(rm -rf /); echo '\\''gotcha'\\'''",
    );
}

// ---- Placeholder substitution: search ---------------------------------------

#[test]
fn search_command_substitutes_query_and_default_limit() {
    let cfg = search_command_cfg("brv search {query} --limit {limit}");
    let inputs = SearchInputs {
        query: "coordination".to_string(),
        limit: None,
        scope: None,
    };
    assert_command(
        &render_search(Some(&cfg), &inputs),
        "brv search 'coordination' --limit 10",
    );
}

#[test]
fn search_command_uses_supplied_limit_when_present() {
    let cfg = search_command_cfg("brv search {query} --limit {limit}");
    let inputs = SearchInputs {
        query: "x".to_string(),
        limit: Some(3),
        scope: None,
    };
    assert_command(
        &render_search(Some(&cfg), &inputs),
        "brv search 'x' --limit 3",
    );
}

#[test]
fn search_command_renders_scope_as_empty_quoted_token_when_absent() {
    // {scope} always shell-quotes — even when unset — so that flag-prefixed
    // templates like `--scope {scope}` produce a valid shell argument
    // (`--scope ''`) instead of a dangling flag with no value. This matches
    // the {context} placeholder's behavior for capture.
    let cfg = search_command_cfg("brv search {query} --scope {scope}");
    let inputs = SearchInputs {
        query: "x".to_string(),
        limit: None,
        scope: None,
    };
    assert_command(
        &render_search(Some(&cfg), &inputs),
        "brv search 'x' --scope ''",
    );
}

#[test]
fn search_command_renders_scope_as_quoted_value() {
    let cfg = search_command_cfg("brv search {query} --scope {scope}");
    let inputs = SearchInputs {
        query: "x".to_string(),
        limit: None,
        scope: Some("auth/".to_string()),
    };
    assert_command(
        &render_search(Some(&cfg), &inputs),
        "brv search 'x' --scope 'auth/'",
    );
}

// ---- Placeholder substitution: query ----------------------------------------

#[test]
fn query_command_substitutes_query() {
    let cfg = query_command_cfg("brv query {query}");
    let inputs = QueryInputs {
        query: "How does coordination work?".to_string(),
    };
    assert_command(
        &render_query(Some(&cfg), &inputs),
        "brv query 'How does coordination work?'",
    );
}

// ---- Skill branch -----------------------------------------------------------

#[test]
fn capture_skill_emits_structured_inputs_and_options() {
    let cfg = MemoryConfig {
        capture: Some(MemoryOpConfig::Skill {
            skill: "ito-memory-markdown".to_string(),
            options: Some(json!({ "root": ".ito/memories" })),
        }),
        ..Default::default()
    };
    let inputs = CaptureInputs {
        context: Some("decision X".to_string()),
        files: vec!["a.md".to_string()],
        folders: vec![],
    };
    let rendered = render_capture(Some(&cfg), &inputs);
    let RenderedInstruction::Skill {
        skill_id,
        inputs,
        options,
    } = rendered
    else {
        panic!("expected Skill branch, got {rendered:?}");
    };
    assert_eq!(skill_id, "ito-memory-markdown");
    assert_eq!(
        inputs.get("context"),
        Some(&Value::String("decision X".to_string()))
    );
    assert_eq!(inputs.get("files"), Some(&json!(["a.md"])));
    assert_eq!(inputs.get("folders"), Some(&json!(Vec::<String>::new())));
    assert_eq!(options, Some(json!({ "root": ".ito/memories" })),);
}

#[test]
fn search_skill_includes_default_limit_in_structured_inputs() {
    let cfg = MemoryConfig {
        search: Some(MemoryOpConfig::Skill {
            skill: "byterover-explore".to_string(),
            options: None,
        }),
        ..Default::default()
    };
    let inputs = SearchInputs {
        query: "coordination".to_string(),
        limit: None,
        scope: None,
    };
    let rendered = render_search(Some(&cfg), &inputs);
    let RenderedInstruction::Skill {
        skill_id,
        inputs,
        options,
    } = rendered
    else {
        panic!("expected Skill branch, got {rendered:?}");
    };
    assert_eq!(skill_id, "byterover-explore");
    assert_eq!(
        inputs.get("query"),
        Some(&Value::String("coordination".to_string()))
    );
    assert_eq!(inputs.get("limit"), Some(&json!(10)));
    assert_eq!(inputs.get("scope"), Some(&Value::Null));
    assert!(options.is_none());
}

// ---- Not-configured branch --------------------------------------------------

#[test]
fn capture_not_configured_when_memory_section_absent() {
    let rendered = render_capture(None, &CaptureInputs::default());
    assert_eq!(
        rendered,
        RenderedInstruction::NotConfigured {
            operation: Operation::Capture
        }
    );
}

#[test]
fn capture_not_configured_when_only_search_is_set() {
    let cfg = search_command_cfg("brv search {query}");
    let rendered = render_capture(Some(&cfg), &CaptureInputs::default());
    assert_eq!(
        rendered,
        RenderedInstruction::NotConfigured {
            operation: Operation::Capture
        }
    );
}

#[test]
fn search_not_configured_when_only_capture_is_set() {
    let cfg = capture_command_cfg("brv curate {context}");
    let rendered = render_search(
        Some(&cfg),
        &SearchInputs {
            query: "x".to_string(),
            limit: None,
            scope: None,
        },
    );
    assert_eq!(
        rendered,
        RenderedInstruction::NotConfigured {
            operation: Operation::Search
        }
    );
}

// ---- Mixed shapes -----------------------------------------------------------

#[test]
fn mixed_shapes_render_independently() {
    let cfg = MemoryConfig {
        capture: Some(MemoryOpConfig::Skill {
            skill: "ito-memory-markdown".to_string(),
            options: None,
        }),
        search: Some(MemoryOpConfig::Command {
            command: "rg {query} .ito/memories".to_string(),
        }),
        query: None,
    };
    assert!(matches!(
        render_capture(Some(&cfg), &CaptureInputs::default()),
        RenderedInstruction::Skill { .. }
    ));
    assert!(matches!(
        render_search(
            Some(&cfg),
            &SearchInputs {
                query: "auth".to_string(),
                limit: None,
                scope: None
            }
        ),
        RenderedInstruction::Command { .. }
    ));
    assert!(matches!(
        render_query(
            Some(&cfg),
            &QueryInputs {
                query: "anything".to_string()
            }
        ),
        RenderedInstruction::NotConfigured {
            operation: Operation::Query
        }
    ));
}

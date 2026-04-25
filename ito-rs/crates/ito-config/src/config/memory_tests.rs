use super::*;
use serde_json::json;

#[test]
fn memory_default_is_absent_on_ito_config() {
    let config = ItoConfig::default();
    assert!(config.memory.is_none());
}

#[test]
fn memory_section_round_trips_when_absent() {
    let json = "{}";
    let parsed: ItoConfig = serde_json::from_str(json).unwrap();
    assert!(parsed.memory.is_none());

    let serialized = serde_json::to_string(&parsed).unwrap();
    assert!(!serialized.contains("memory"));
}

#[test]
fn memory_section_accepts_capture_only() {
    let json = json!({
        "memory": {
            "capture": { "kind": "command", "command": "brv curate \"{context}\"" }
        }
    });
    let parsed: ItoConfig = serde_json::from_value(json).unwrap();
    let memory = parsed.memory.expect("memory section present");

    assert!(matches!(
        memory.capture,
        Some(MemoryOpConfig::Command { ref command }) if command == "brv curate \"{context}\""
    ));
    assert!(memory.search.is_none());
    assert!(memory.query.is_none());
}

#[test]
fn memory_section_accepts_skill_with_options() {
    let json = json!({
        "memory": {
            "capture": {
                "kind": "skill",
                "skill": "ito-memory-markdown",
                "options": { "root": ".ito/memories" }
            }
        }
    });
    let parsed: ItoConfig = serde_json::from_value(json).unwrap();
    let capture = parsed
        .memory
        .expect("memory section present")
        .capture
        .expect("capture present");

    match capture {
        MemoryOpConfig::Skill {
            skill,
            options: Some(options),
        } => {
            assert_eq!(skill, "ito-memory-markdown");
            assert_eq!(
                options.get("root").and_then(|v| v.as_str()),
                Some(".ito/memories")
            );
        }
        other => panic!("expected Skill variant with options, got {other:?}"),
    }
}

#[test]
fn memory_section_skill_options_are_optional() {
    let json = json!({
        "memory": {
            "capture": { "kind": "skill", "skill": "byterover-explore" }
        }
    });
    let parsed: ItoConfig = serde_json::from_value(json).unwrap();
    let capture = parsed
        .memory
        .expect("memory section present")
        .capture
        .expect("capture present");

    match capture {
        MemoryOpConfig::Skill { skill, options } => {
            assert_eq!(skill, "byterover-explore");
            assert!(options.is_none());
        }
        other => panic!("expected Skill variant, got {other:?}"),
    }
}

#[test]
fn memory_section_supports_mixed_per_op_shapes() {
    let json = json!({
        "memory": {
            "capture": { "kind": "skill", "skill": "ito-memory-markdown" },
            "search":  { "kind": "command", "command": "rg \"{query}\" .ito/memories" },
            "query":   { "kind": "command", "command": "brv query \"{query}\"" }
        }
    });
    let parsed: ItoConfig = serde_json::from_value(json).unwrap();
    let memory = parsed.memory.expect("memory section present");

    assert!(matches!(memory.capture, Some(MemoryOpConfig::Skill { .. })));
    let Some(search) = memory.search else {
        panic!("expected memory.search to be present");
    };
    match search {
        MemoryOpConfig::Command { command: _ } => {}
        MemoryOpConfig::Skill { .. } => panic!("expected memory.search to be a command op"),
    }
    assert!(matches!(memory.query, Some(MemoryOpConfig::Command { .. })));
}

#[test]
fn memory_section_round_trips_full_config() {
    let original = MemoryConfig {
        capture: Some(MemoryOpConfig::Skill {
            skill: "ito-memory-markdown".to_string(),
            options: Some(json!({ "root": ".ito/memories" })),
        }),
        search: Some(MemoryOpConfig::Command {
            command: "brv search \"{query}\" --limit {limit}".to_string(),
        }),
        query: Some(MemoryOpConfig::Command {
            command: "brv query \"{query}\"".to_string(),
        }),
    };

    let serialized = serde_json::to_value(&original).unwrap();
    let roundtripped: MemoryConfig = serde_json::from_value(serialized).unwrap();

    assert!(matches!(
        roundtripped.capture,
        Some(MemoryOpConfig::Skill { ref skill, options: Some(_) }) if skill == "ito-memory-markdown"
    ));
    assert!(matches!(
        roundtripped.search,
        Some(MemoryOpConfig::Command { ref command }) if command.contains("{query}")
    ));
    let Some(query) = roundtripped.query else {
        panic!("expected roundtripped.query to be present");
    };
    match query {
        MemoryOpConfig::Command { command: _ } => {}
        MemoryOpConfig::Skill { .. } => panic!("expected roundtripped.query to be a command op"),
    }
}

#[test]
fn memory_section_omits_absent_ops_when_serialized() {
    let memory = MemoryConfig {
        capture: Some(MemoryOpConfig::Command {
            command: "brv curate \"{context}\"".to_string(),
        }),
        search: None,
        query: None,
    };
    let json = serde_json::to_string(&memory).unwrap();
    assert!(json.contains("\"capture\""));
    assert!(!json.contains("\"search\""));
    assert!(!json.contains("\"query\""));
}

#[test]
fn memory_op_config_unknown_kind_is_rejected() {
    let json = json!({
        "memory": {
            "capture": { "kind": "magic", "command": "noop" }
        }
    });
    let result: Result<ItoConfig, _> = serde_json::from_value(json);
    assert!(result.is_err(), "expected error for unknown kind");
}

#[test]
fn memory_op_config_skill_variant_requires_skill_field() {
    let json = json!({
        "memory": {
            "capture": { "kind": "skill" }
        }
    });
    let result: Result<ItoConfig, _> = serde_json::from_value(json);
    assert!(result.is_err(), "expected error when skill field missing");
}

#[test]
fn memory_op_config_command_variant_requires_command_field() {
    let json = json!({
        "memory": {
            "capture": { "kind": "command" }
        }
    });
    let result: Result<ItoConfig, _> = serde_json::from_value(json);
    assert!(result.is_err(), "expected error when command field missing");
}

#[test]
fn memory_section_unknown_op_key_is_rejected() {
    let json = json!({
        "memory": {
            "curate": { "kind": "command", "command": "noop" }
        }
    });
    let result: Result<ItoConfig, _> = serde_json::from_value(json);
    assert!(
        result.is_err(),
        "expected error for unknown operation key 'curate'"
    );
}

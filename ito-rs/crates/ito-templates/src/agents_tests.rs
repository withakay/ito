use super::*;

#[test]
fn render_template_replaces_model() {
    let template = r#"---
model: "{{model}}"
---
Instructions"#;

    let config = AgentConfig {
        model: "anthropic/claude-haiku-4-5".to_string(),
        ..Default::default()
    };

    let result = render_agent_template(template, &config);
    assert!(result.contains("model: \"anthropic/claude-haiku-4-5\""));
}

#[test]
fn render_template_replaces_variant() {
    let template = r#"---
model: "{{model}}"
variant: "{{variant}}"
---"#;

    let config = AgentConfig {
        model: "openai/gpt-5.2-codex".to_string(),
        variant: Some("high".to_string()),
        ..Default::default()
    };

    let result = render_agent_template(template, &config);
    assert!(result.contains("variant: \"high\""));
}

#[test]
fn render_template_removes_variant_line_if_not_set() {
    let template = r#"---
model: "{{model}}"
variant: "{{variant}}"
---"#;

    let config = AgentConfig {
        model: "anthropic/claude-haiku-4-5".to_string(),
        variant: None,
        ..Default::default()
    };

    let result = render_agent_template(template, &config);
    assert!(!result.contains("variant"));
}

#[test]
fn default_configs_has_all_combinations() {
    let configs = default_agent_configs();

    for harness in Harness::all() {
        for tier in AgentTier::all() {
            assert!(
                configs.contains_key(&(*harness, *tier)),
                "Missing config for {:?}/{:?}",
                harness,
                tier
            );
        }
    }
}

#[test]
fn agent_surface_inventory_defines_activation_boundaries() {
    let inventory = agent_surface_inventory();

    let expected = [
        ("ito-quick", AgentActivationMode::DelegatedRole),
        ("ito-general", AgentActivationMode::DirectEntryPoint),
        ("ito-thinking", AgentActivationMode::DirectEntryPoint),
        ("ito-orchestrator", AgentActivationMode::DirectEntryPoint),
        ("ito-planner", AgentActivationMode::DelegatedRole),
        ("ito-researcher", AgentActivationMode::DelegatedRole),
        ("ito-worker", AgentActivationMode::DelegatedRole),
        ("ito-reviewer", AgentActivationMode::DelegatedRole),
        ("ito-test-runner", AgentActivationMode::DelegatedRole),
    ];

    assert_eq!(inventory.len(), expected.len());
    for (name, activation) in expected {
        let surface = inventory
            .iter()
            .find(|surface| surface.name == name)
            .unwrap_or_else(|| panic!("missing {name} in agent surface inventory"));
        assert_eq!(surface.activation, activation);
    }
}

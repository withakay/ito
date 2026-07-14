use super::*;

#[test]
fn schema_contains_expected_sections() {
    let v = config_schema_json();
    let props = v
        .get("properties")
        .and_then(|p| p.as_object())
        .or_else(|| {
            v.get("schema")
                .and_then(|s| s.get("properties"))
                .and_then(|p| p.as_object())
        })
        .expect("schema properties");

    assert!(props.contains_key("projectPath"));
    assert!(props.contains_key("harnesses"));
    assert!(props.contains_key("cache"));
    assert!(props.contains_key("defaults"));
    assert!(
        !props.contains_key("tools"),
        "the removed tmux-only tools namespace must not remain in the schema"
    );
    assert!(props.contains_key("$schema"));
}

#[test]
fn schema_describes_proposal_integration_mode_and_default() {
    let schema = config_schema_json();
    let property = &schema["definitions"]["ProposalConfig"]["properties"]["integration_mode"];
    assert_eq!(property["default"], "pull_request");

    let variants = &schema["definitions"]["ProposalIntegrationMode"]["oneOf"];
    let encoded = serde_json::to_string(variants).unwrap();
    assert!(encoded.contains("pull_request"));
    assert!(encoded.contains("direct_merge"));
    assert!(!encoded.contains("pull_request_auto_merge"));
}

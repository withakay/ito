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
    assert!(props.contains_key("tools"));
    assert!(props.contains_key("$schema"));
}

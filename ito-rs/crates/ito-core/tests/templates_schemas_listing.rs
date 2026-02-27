use ito_config::ConfigContext;
use ito_core::templates::{SchemaListResponse, list_schemas_detail};

fn default_ctx() -> ConfigContext {
    ConfigContext {
        xdg_config_home: None,
        project_dir: None,
        home_dir: None,
    }
}

#[test]
fn list_schemas_detail_returns_all_embedded_schemas() {
    let response = list_schemas_detail(&default_ctx());

    let names: Vec<&str> = response.schemas.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"spec-driven"), "missing spec-driven");
    assert!(names.contains(&"minimalist"), "missing minimalist");
    assert!(names.contains(&"tdd"), "missing tdd");
    assert!(names.contains(&"event-driven"), "missing event-driven");
}

#[test]
fn list_schemas_detail_is_sorted() {
    let response = list_schemas_detail(&default_ctx());

    let names: Vec<&str> = response.schemas.iter().map(|s| s.name.as_str()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "schemas should be sorted alphabetically");
}

#[test]
fn list_schemas_detail_recommended_default_is_spec_driven() {
    let response = list_schemas_detail(&default_ctx());
    assert_eq!(response.recommended_default, "spec-driven");
}

#[test]
fn list_schemas_detail_entries_have_descriptions() {
    let response = list_schemas_detail(&default_ctx());

    for schema in &response.schemas {
        assert!(
            !schema.description.is_empty(),
            "schema '{}' should have a description",
            schema.name
        );
    }
}

#[test]
fn list_schemas_detail_entries_have_artifacts() {
    let response = list_schemas_detail(&default_ctx());

    for schema in &response.schemas {
        assert!(
            !schema.artifacts.is_empty(),
            "schema '{}' should have at least one artifact",
            schema.name
        );
    }
}

#[test]
fn list_schemas_detail_all_sources_are_embedded() {
    let response = list_schemas_detail(&default_ctx());

    for schema in &response.schemas {
        assert_eq!(
            schema.source, "embedded",
            "schema '{}' should have source 'embedded' (no project/user dirs configured)",
            schema.name
        );
    }
}

#[test]
fn list_schemas_detail_json_round_trips() {
    let response = list_schemas_detail(&default_ctx());

    let json = serde_json::to_string_pretty(&response).expect("should serialize to JSON");
    let parsed: SchemaListResponse =
        serde_json::from_str(&json).expect("should deserialize from JSON");

    assert_eq!(parsed.schemas.len(), response.schemas.len());
    assert_eq!(parsed.recommended_default, response.recommended_default);

    for (orig, rt) in response.schemas.iter().zip(parsed.schemas.iter()) {
        assert_eq!(orig.name, rt.name);
        assert_eq!(orig.description, rt.description);
        assert_eq!(orig.artifacts, rt.artifacts);
        assert_eq!(orig.source, rt.source);
    }
}

#[test]
fn list_schemas_detail_spec_driven_has_expected_artifacts() {
    let response = list_schemas_detail(&default_ctx());

    let spec_driven = response
        .schemas
        .iter()
        .find(|s| s.name == "spec-driven")
        .expect("spec-driven should be present");

    assert!(spec_driven.artifacts.contains(&"proposal".to_string()));
    assert!(spec_driven.artifacts.contains(&"specs".to_string()));
    assert!(spec_driven.artifacts.contains(&"design".to_string()));
    assert!(spec_driven.artifacts.contains(&"tasks".to_string()));
}

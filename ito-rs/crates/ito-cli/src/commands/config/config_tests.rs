use super::*;

#[test]
fn json_render_value_renders_common_json_types() {
    assert_eq!(
        json_render_value(&serde_json::Value::String("hi".to_string())),
        "hi"
    );
    assert_eq!(json_render_value(&serde_json::json!(123)), "123");
    assert_eq!(json_render_value(&serde_json::json!(true)), "true");
    assert_eq!(json_render_value(&serde_json::Value::Null), "null");

    let obj = json_render_value(&serde_json::json!({"a": 1}));
    assert!(obj.contains('"'));
    assert!(obj.contains("a"));

    let arr = json_render_value(&serde_json::json!([1, 2]));
    assert!(arr.contains('['));
    assert!(arr.contains(']'));
}

#[test]
fn handle_config_schema_writes_file_when_output_is_set() {
    let td = tempfile::tempdir().expect("tempdir");
    let path = td.path().join("schema.json");
    handle_config_schema(Some(&path)).expect("schema write");

    let s = std::fs::read_to_string(&path).expect("read schema");
    assert!(s.ends_with('\n'));
    let _: serde_json::Value = serde_json::from_str(&s).expect("json");
}

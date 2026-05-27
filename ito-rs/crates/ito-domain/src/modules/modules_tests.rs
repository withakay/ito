use super::*;

#[test]
fn test_module_creation() {
    let module = Module {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        description: Some("Development tooling".to_string()),
        path: PathBuf::from("/test"),
        sub_modules: Vec::new(),
    };

    assert_eq!(module.id, "005");
    assert_eq!(module.name, "dev-tooling");
    assert!(module.sub_modules.is_empty());
}

#[test]
fn test_module_summary() {
    let summary = ModuleSummary {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        change_count: 3,
        sub_modules: Vec::new(),
    };

    assert_eq!(summary.change_count, 3);
    assert!(summary.sub_modules.is_empty());
}

#[test]
fn test_sub_module_creation() {
    let sub = SubModule {
        id: "005.01".to_string(),
        parent_module_id: "005".to_string(),
        sub_id: "01".to_string(),
        name: "core-api".to_string(),
        description: Some("Core API sub-module".to_string()),
        change_count: 2,
        path: PathBuf::from("/test/005.01_core-api"),
    };

    assert_eq!(sub.id, "005.01");
    assert_eq!(sub.parent_module_id, "005");
    assert_eq!(sub.sub_id, "01");
    assert_eq!(sub.name, "core-api");
    assert_eq!(sub.change_count, 2);
}

#[test]
fn test_sub_module_summary_creation() {
    let summary = SubModuleSummary {
        id: "005.01".to_string(),
        name: "core-api".to_string(),
        change_count: 2,
    };

    assert_eq!(summary.id, "005.01");
    assert_eq!(summary.name, "core-api");
    assert_eq!(summary.change_count, 2);
}

#[test]
fn test_module_with_sub_modules() {
    let sub = SubModule {
        id: "005.01".to_string(),
        parent_module_id: "005".to_string(),
        sub_id: "01".to_string(),
        name: "core-api".to_string(),
        description: None,
        change_count: 1,
        path: PathBuf::from("/test/005.01_core-api"),
    };

    let module = Module {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        description: None,
        path: PathBuf::from("/test"),
        sub_modules: vec![sub],
    };

    assert_eq!(module.sub_modules.len(), 1);
    assert_eq!(module.sub_modules[0].id, "005.01");
}

#[test]
fn test_module_summary_with_sub_modules() {
    let sub_summary = SubModuleSummary {
        id: "005.01".to_string(),
        name: "core-api".to_string(),
        change_count: 2,
    };

    let summary = ModuleSummary {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        change_count: 5,
        sub_modules: vec![sub_summary],
    };

    assert_eq!(summary.sub_modules.len(), 1);
    assert_eq!(summary.sub_modules[0].change_count, 2);
}

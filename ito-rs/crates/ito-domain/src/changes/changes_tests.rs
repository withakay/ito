use super::*;

#[test]
fn test_normalize_id() {
    assert_eq!(normalize_id("5", 3), "005");
    assert_eq!(normalize_id("05", 3), "005");
    assert_eq!(normalize_id("005", 3), "005");
    assert_eq!(normalize_id("0005", 3), "005");
    assert_eq!(normalize_id("1", 2), "01");
    assert_eq!(normalize_id("01", 2), "01");
    assert_eq!(normalize_id("001", 2), "01");
}

#[test]
fn test_parse_change_id() {
    assert_eq!(
        parse_change_id("005-01_my-change"),
        Some(("005".to_string(), "01".to_string()))
    );
    assert_eq!(
        parse_change_id("5-1_whatever"),
        Some(("005".to_string(), "01".to_string()))
    );
    assert_eq!(
        parse_change_id("1-2"),
        Some(("001".to_string(), "02".to_string()))
    );
    assert_eq!(
        parse_change_id("001-000002_foo"),
        Some(("001".to_string(), "02".to_string()))
    );
    assert_eq!(parse_change_id("invalid"), None);
}

#[test]
fn test_parse_module_id() {
    assert_eq!(parse_module_id("005"), "005");
    assert_eq!(parse_module_id("5"), "005");
    assert_eq!(parse_module_id("005_dev-tooling"), "005");
    assert_eq!(parse_module_id("5_dev-tooling"), "005");
}

#[test]
fn test_extract_module_id() {
    assert_eq!(
        extract_module_id("005-01_my-change"),
        Some("005".to_string())
    );
    assert_eq!(extract_module_id("013-18_cleanup"), Some("013".to_string()));
    assert_eq!(extract_module_id("5-1_foo"), Some("005".to_string()));
    assert_eq!(extract_module_id("invalid"), None);
    // Sub-module format: strip sub-module component
    assert_eq!(extract_module_id("024.01-03_foo"), Some("024".to_string()));
    assert_eq!(extract_module_id("5.1-2_bar"), Some("005".to_string()));
}

#[test]
fn test_extract_sub_module_id() {
    assert_eq!(
        extract_sub_module_id("024.01-03_foo"),
        Some("024.01".to_string())
    );
    assert_eq!(
        extract_sub_module_id("5.1-2_bar"),
        Some("005.01".to_string())
    );
    assert_eq!(extract_sub_module_id("005-01_my-change"), None);
    assert_eq!(extract_sub_module_id("invalid"), None);
}

#[test]
fn test_parse_change_id_sub_module_format() {
    assert_eq!(
        parse_change_id("024.01-03_foo"),
        Some(("024".to_string(), "03".to_string()))
    );
    assert_eq!(
        parse_change_id("5.1-2_bar"),
        Some(("005".to_string(), "02".to_string()))
    );
}

#[test]
fn test_change_sub_module_id_field() {
    let summary = ChangeSummary {
        id: "005.01-03_my-change".to_string(),
        module_id: Some("005".to_string()),
        sub_module_id: Some("005.01".to_string()),
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 0,
        last_modified: Utc::now(),
        has_proposal: false,
        has_design: false,
        has_specs: false,
        has_tasks: false,
        orchestrate: ChangeOrchestrateMetadata::default(),
    };

    assert_eq!(summary.sub_module_id.as_deref(), Some("005.01"));
}

#[test]
fn test_change_status_display() {
    assert_eq!(ChangeStatus::NoTasks.to_string(), "no-tasks");
    assert_eq!(ChangeStatus::InProgress.to_string(), "in-progress");
    assert_eq!(ChangeStatus::Complete.to_string(), "complete");
}

#[test]
fn test_change_summary_status() {
    let mut summary = ChangeSummary {
        id: "test".to_string(),
        module_id: None,
        sub_module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 0,
        last_modified: Utc::now(),
        has_proposal: false,
        has_design: false,
        has_specs: false,
        has_tasks: false,
        orchestrate: ChangeOrchestrateMetadata::default(),
    };

    assert_eq!(summary.status(), ChangeStatus::NoTasks);

    summary.total_tasks = 5;
    summary.completed_tasks = 3;
    assert_eq!(summary.status(), ChangeStatus::InProgress);

    summary.completed_tasks = 5;
    assert_eq!(summary.status(), ChangeStatus::Complete);
}

#[test]
fn test_change_work_status() {
    let mut summary = ChangeSummary {
        id: "test".to_string(),
        module_id: None,
        sub_module_id: None,
        completed_tasks: 0,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: 0,
        total_tasks: 0,
        last_modified: Utc::now(),
        has_proposal: false,
        has_design: false,
        has_specs: false,
        has_tasks: false,
        orchestrate: ChangeOrchestrateMetadata::default(),
    };

    assert_eq!(summary.work_status(), ChangeWorkStatus::Draft);

    summary.has_proposal = true;
    summary.has_specs = true;
    summary.has_tasks = true;
    summary.total_tasks = 3;
    summary.pending_tasks = 3;

    assert_eq!(summary.work_status(), ChangeWorkStatus::Ready);

    summary.in_progress_tasks = 1;
    summary.pending_tasks = 2;
    assert_eq!(summary.work_status(), ChangeWorkStatus::InProgress);

    summary.in_progress_tasks = 0;
    summary.pending_tasks = 0;
    summary.shelved_tasks = 1;
    summary.completed_tasks = 2;
    assert_eq!(summary.work_status(), ChangeWorkStatus::Paused);

    summary.shelved_tasks = 0;
    summary.completed_tasks = 3;
    assert_eq!(summary.work_status(), ChangeWorkStatus::Complete);
}

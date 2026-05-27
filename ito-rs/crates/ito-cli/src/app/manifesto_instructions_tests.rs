use super::*;
use ito_core::{ProgressInfo, TaskItem, TasksFormat, TasksParseResult};

fn draft_change_with_pending_task() -> Change {
    Change {
        id: "000-01_test-change".to_string(),
        module_id: Some("000".to_string()),
        sub_module_id: None,
        path: std::path::PathBuf::from("/tmp/000-01_test-change"),
        proposal: Some("## Why\nCustom workflow.\n".to_string()),
        design: None,
        specs: Vec::new(),
        tasks: TasksParseResult {
            format: TasksFormat::Checkbox,
            tasks: Vec::<TaskItem>::new(),
            waves: Vec::new(),
            diagnostics: Vec::new(),
            progress: ProgressInfo {
                total: 1,
                complete: 0,
                shelved: 0,
                in_progress: 0,
                pending: 1,
                remaining: 1,
            },
        },
        orchestrate: Default::default(),
        last_modified: Utc::now(),
    }
}

#[test]
fn manifesto_state_uses_schema_status_for_custom_artifacts() {
    let change = Some(draft_change_with_pending_task());
    let schema_status = core_templates::ChangeStatus {
        change_name: "000-01_test-change".to_string(),
        schema_name: "custom".to_string(),
        is_complete: true,
        apply_requires: vec!["proposal".to_string()],
        artifacts: vec![core_templates::ArtifactStatus {
            id: "proposal".to_string(),
            output_path: "proposal.md".to_string(),
            status: "done".to_string(),
            missing_deps: Vec::new(),
        }],
    };

    let state = resolve_manifesto_state(
        false,
        &change,
        Some(&schema_status),
        "unknown",
        "not-requested",
    );

    assert_eq!(state, "apply-ready");
}

#[test]
fn manifesto_state_reports_drafting_for_incomplete_schema_artifacts() {
    let change = Some(draft_change_with_pending_task());
    let schema_status = core_templates::ChangeStatus {
        change_name: "000-01_test-change".to_string(),
        schema_name: "custom".to_string(),
        is_complete: false,
        apply_requires: vec!["proposal".to_string()],
        artifacts: Vec::new(),
    };

    let state = resolve_manifesto_state(
        false,
        &change,
        Some(&schema_status),
        "unknown",
        "not-requested",
    );

    assert_eq!(state, "proposal-drafting");
}

#[test]
fn manifesto_state_uses_apply_requirements_not_all_required_artifacts() {
    let change = Some(draft_change_with_pending_task());
    let schema_status = core_templates::ChangeStatus {
        change_name: "000-01_test-change".to_string(),
        schema_name: "custom".to_string(),
        is_complete: false,
        apply_requires: vec!["proposal".to_string()],
        artifacts: vec![
            core_templates::ArtifactStatus {
                id: "proposal".to_string(),
                output_path: "proposal.md".to_string(),
                status: "done".to_string(),
                missing_deps: Vec::new(),
            },
            core_templates::ArtifactStatus {
                id: "analysis".to_string(),
                output_path: "analysis.md".to_string(),
                status: "ready".to_string(),
                missing_deps: Vec::new(),
            },
        ],
    };

    let state = resolve_manifesto_state(
        false,
        &change,
        Some(&schema_status),
        "unknown",
        "not-requested",
    );

    assert_eq!(state, "apply-ready");
}

#[test]
fn manifesto_state_treats_empty_apply_requirements_as_apply_ready() {
    let change = Some(draft_change_with_pending_task());
    let schema_status = core_templates::ChangeStatus {
        change_name: "000-01_test-change".to_string(),
        schema_name: "custom".to_string(),
        is_complete: false,
        apply_requires: Vec::new(),
        artifacts: vec![core_templates::ArtifactStatus {
            id: "proposal".to_string(),
            output_path: "proposal.md".to_string(),
            status: "ready".to_string(),
            missing_deps: Vec::new(),
        }],
    };

    let state = resolve_manifesto_state(
        false,
        &change,
        Some(&schema_status),
        "unknown",
        "not-requested",
    );

    assert_eq!(state, "apply-ready");
}

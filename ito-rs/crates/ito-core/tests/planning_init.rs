use ito_core::planning_init;
use ito_domain::planning::{planning_dir, research_dir};
use tempfile::tempdir;

#[test]
fn init_planning_structure_creates_only_workspace() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    planning_init::init_planning_structure(ito_path).unwrap();

    let plan_dir = planning_dir(ito_path);
    assert!(plan_dir.exists(), "planning dir should exist");
    // Regression guard: the old planning bootstrap created these fixed files.
    assert!(!plan_dir.join("PROJECT.md").exists());
    assert!(!plan_dir.join("ROADMAP.md").exists());
    assert!(!plan_dir.join("STATE.md").exists());
}

#[test]
fn init_planning_structure_preserves_existing_plan_documents() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();
    let plan_dir = planning_dir(ito_path);
    std::fs::create_dir_all(&plan_dir).unwrap();
    std::fs::write(plan_dir.join("existing.md"), "# Existing plan\n").unwrap();

    planning_init::init_planning_structure(ito_path).unwrap();

    assert_eq!(
        std::fs::read_to_string(plan_dir.join("existing.md")).unwrap(),
        "# Existing plan\n"
    );
    assert!(!plan_dir.join("PROJECT.md").exists());
    assert!(!plan_dir.join("ROADMAP.md").exists());
    assert!(!plan_dir.join("STATE.md").exists());
}

#[test]
fn init_planning_structure_errors_when_planning_path_is_a_file() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    std::fs::write(planning_dir(ito_path), "not a directory\n").unwrap();

    assert!(planning_init::init_planning_structure(ito_path).is_err());
}

#[test]
fn read_planning_workspace_status_lists_plan_documents() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();
    let plan_dir = planning_dir(ito_path);
    std::fs::create_dir_all(&plan_dir).unwrap();
    std::fs::write(plan_dir.join("topic.md"), "# Topic\n").unwrap();
    std::fs::write(plan_dir.join("alpha.md"), "# Alpha\n").unwrap();
    std::fs::write(plan_dir.join("BETA.MD"), "# Beta\n").unwrap();
    std::fs::write(plan_dir.join("notes.txt"), "not a plan").unwrap();
    std::fs::create_dir(plan_dir.join("nested.md")).unwrap();

    let status = planning_init::read_planning_workspace_status(ito_path).expect("status");
    assert!(status.planning_exists);
    assert!(!status.planning_invalid);
    assert!(!status.research_exists);
    assert!(!status.research_invalid);
    assert_eq!(
        status.planning_documents,
        vec![
            plan_dir.join("BETA.MD"),
            plan_dir.join("alpha.md"),
            plan_dir.join("topic.md")
        ]
    );
}

#[test]
fn read_planning_workspace_status_allows_missing_workspace() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    let status = planning_init::read_planning_workspace_status(ito_path).expect("status");
    assert!(!status.planning_exists);
    assert!(!status.planning_invalid);
    assert!(!status.research_exists);
    assert!(!status.research_invalid);
    assert!(status.planning_documents.is_empty());
}

#[test]
fn read_planning_workspace_status_reports_conflicting_file() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    std::fs::write(planning_dir(ito_path), "not a directory\n").unwrap();

    let status = planning_init::read_planning_workspace_status(ito_path).expect("status");
    assert!(!status.planning_exists);
    assert!(status.planning_invalid);
    assert!(status.planning_documents.is_empty());
}

#[test]
fn read_planning_workspace_status_reports_conflicting_research_file() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    std::fs::write(research_dir(ito_path), "not a directory\n").unwrap();

    let status = planning_init::read_planning_workspace_status(ito_path).expect("status");
    assert!(!status.planning_exists);
    assert!(!status.planning_invalid);
    assert!(status.planning_documents.is_empty());
    assert!(!status.research_exists);
    assert!(status.research_invalid);
}

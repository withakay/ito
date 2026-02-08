use ito_core::planning_init;
use ito_domain::planning::planning_dir;
use tempfile::tempdir;

#[test]
fn init_planning_structure_writes_files() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    planning_init::init_planning_structure(ito_path, "test-module", "test-change").unwrap();

    let plan_dir = planning_dir(ito_path);
    assert!(plan_dir.exists(), "planning dir should exist");
    assert!(plan_dir.join("project.md").exists());
    assert!(plan_dir.join("roadmap.md").exists());
    assert!(plan_dir.join("state.md").exists());
}

#[test]
fn read_planning_status_returns_contents_for_existing_roadmap() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();
    let roadmap_content = "# Roadmap\n\n## Current Milestone: v1\n";

    let plan_dir = planning_dir(ito_path);
    std::fs::create_dir_all(&plan_dir).unwrap();
    std::fs::write(plan_dir.join("ROADMAP.md"), roadmap_content).unwrap();

    let result = planning_init::read_planning_status(ito_path).expect("should read ROADMAP.md");
    assert_eq!(result, roadmap_content);
}

#[test]
fn read_planning_status_returns_error_for_missing_roadmap() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();
    // Do NOT create planning/ROADMAP.md

    let result = planning_init::read_planning_status(ito_path);
    assert!(result.is_err(), "should fail for missing ROADMAP.md");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("ROADMAP.md"),
        "error should mention ROADMAP.md, got: {msg}"
    );
}

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

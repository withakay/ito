use ito_core::workflow_templates;
use ito_domain::workflow::{count_tasks, workflows_dir};
use tempfile::tempdir;

#[test]
fn init_workflow_structure_writes_expected_files() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    workflow_templates::init_workflow_structure(ito_path).unwrap();

    let wf_dir = workflows_dir(ito_path);
    assert!(wf_dir.exists(), "workflows dir should exist");
    assert!(wf_dir.join("research.yaml").exists());
    assert!(wf_dir.join("execute.yaml").exists());
    assert!(wf_dir.join("review.yaml").exists());

    let workflows = workflow_templates::list_workflows(ito_path);
    assert_eq!(workflows.len(), 3);
    assert!(workflows.contains(&"research".to_string()));
    assert!(workflows.contains(&"execute".to_string()));
    assert!(workflows.contains(&"review".to_string()));
}

#[test]
fn load_workflow_parses_and_counts_tasks() {
    let tmp = tempdir().unwrap();
    let ito_path = tmp.path();

    workflow_templates::init_workflow_structure(ito_path).unwrap();
    let wf = workflow_templates::load_workflow(ito_path, "research").unwrap();

    assert_eq!(wf.id, "research");
    assert!(count_tasks(&wf) > 0, "research workflow should have tasks");
}

use ito_domain::workflow;

#[test]
fn init_workflow_structure_writes_expected_files() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito_path = td.path().join(".ito");
    workflow::init_workflow_structure(&ito_path).expect("init");

    assert!(workflow::workflows_dir(&ito_path).exists());
    assert!(workflow::workflow_state_dir(&ito_path).exists());
    assert!(workflow::commands_dir(&ito_path).exists());
    assert!(workflow::workflow_file_path(&ito_path, "research").exists());
    assert!(workflow::workflow_file_path(&ito_path, "execute").exists());
    assert!(workflow::workflow_file_path(&ito_path, "review").exists());

    let names = workflow::list_workflows(&ito_path);
    assert_eq!(names, vec!["execute", "research", "review"]);
}

#[test]
fn load_workflow_parses_and_counts_tasks() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito_path = td.path().join(".ito");
    workflow::init_workflow_structure(&ito_path).expect("init");

    let wf = workflow::load_workflow(&ito_path, "research").expect("load");
    assert_eq!(wf.id, "research");
    assert!(!wf.waves.is_empty());
    assert!(workflow::count_tasks(&wf) >= 1);
}

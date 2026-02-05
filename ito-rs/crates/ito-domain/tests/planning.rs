use ito_domain::planning;

#[test]
fn roadmap_parsing_extracts_current_progress_and_phases() {
    let roadmap = planning::roadmap_md_template();
    let (milestone, status, phase) = planning::read_current_progress(&roadmap).expect("progress");
    assert_eq!(milestone, "v1-core");
    assert_eq!(status, "Not Started");
    assert_eq!(phase, "0 of 0");

    let phases = planning::read_phase_rows(&roadmap);
    assert_eq!(phases.len(), 1);
    assert_eq!(phases[0].0, "1");
    assert_eq!(phases[0].2, "Pending");
}

#[test]
fn init_planning_structure_writes_files() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito_path = td.path().join(".ito");
    let date = "2026-01-28";

    planning::init_planning_structure(&ito_path, date, ".ito").expect("init");

    let project = ito_path.join("planning").join("PROJECT.md");
    let roadmap = ito_path.join("planning").join("ROADMAP.md");
    let state = ito_path.join("planning").join("STATE.md");
    assert!(project.exists());
    assert!(roadmap.exists());
    assert!(state.exists());

    let state_contents = std::fs::read_to_string(state).expect("read");
    assert!(state_contents.contains("Last Updated: 2026-01-28"));
    assert!(state_contents.contains("`.ito/changes/`"));
}

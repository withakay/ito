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

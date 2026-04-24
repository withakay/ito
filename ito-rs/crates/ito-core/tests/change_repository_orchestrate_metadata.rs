use ito_core::change_repository::ChangeRepository;
use ito_domain::changes::{ChangeLifecycleFilter, ChangeRepository as _};
use tempfile::TempDir;

#[test]
fn change_repository_exposes_orchestrate_metadata_from_ito_yaml() {
    let tmp = TempDir::new().expect("temp dir");
    let ito_path = tmp.path().join(".ito");
    let change_id = "001-01_demo";
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(&change_dir).expect("create change dir");

    std::fs::write(
        change_dir.join(".ito.yaml"),
        r#"schema: spec-driven
orchestrate:
  depends_on:
    - 001-02_other
    - 002-01_else
  preferred_gates:
    - plan
    - dispatch
"#,
    )
    .expect("write .ito.yaml");
    std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write proposal");
    std::fs::write(change_dir.join("tasks.md"), "- [ ] one\n").expect("write tasks");

    let repo = ChangeRepository::new(&ito_path);

    let change = repo
        .get_with_filter(change_id, ChangeLifecycleFilter::Active)
        .expect("get change");
    assert_eq!(
        change.orchestrate.depends_on,
        vec!["001-02_other".to_string(), "002-01_else".to_string()]
    );
    assert_eq!(
        change.orchestrate.preferred_gates,
        vec!["plan".to_string(), "dispatch".to_string()]
    );

    let summaries = repo
        .list_with_filter(ChangeLifecycleFilter::Active)
        .expect("list");
    let summary = summaries
        .iter()
        .find(|s| s.id == change_id)
        .expect("summary exists");
    assert_eq!(
        summary.orchestrate.depends_on,
        vec!["001-02_other".to_string(), "002-01_else".to_string()]
    );
    assert_eq!(
        summary.orchestrate.preferred_gates,
        vec!["plan".to_string(), "dispatch".to_string()]
    );
}

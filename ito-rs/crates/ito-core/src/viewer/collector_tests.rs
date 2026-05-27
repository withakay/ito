use super::*;
use tempfile::TempDir;

#[test]
fn collect_proposal_artifacts_orders_sections_and_preserves_content() {
    let temp_dir = TempDir::new().unwrap();
    let ito_root = temp_dir.path().join(".ito");
    let change_dir = ito_root.join("changes/001-29_test-change");
    std::fs::create_dir_all(change_dir.join("specs/auth")).unwrap();
    std::fs::create_dir_all(change_dir.join("specs/zebra")).unwrap();

    std::fs::write(change_dir.join("proposal.md"), "# Proposal\nbody\n").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "# Tasks\n- [ ] verify\n").unwrap();
    std::fs::write(change_dir.join("specs/auth/spec.md"), "## ADDED\nauth\n").unwrap();
    std::fs::write(
        change_dir.join("specs/zebra/spec.md"),
        "## MODIFIED\nzebra\n",
    )
    .unwrap();

    let bundled = collect_proposal_artifacts("001-29_test-change", &ito_root).unwrap();

    let expected = [
        "---\n# proposal.md\n\n# Proposal\nbody",
        "---\n# tasks.md\n\n# Tasks\n- [ ] verify",
        "---\n# specs/auth/spec.md\n\n## ADDED\nauth",
        "---\n# specs/zebra/spec.md\n\n## MODIFIED\nzebra",
    ]
    .join("\n\n");

    assert_eq!(bundled, expected);
}

#[test]
fn collect_proposal_artifacts_skips_missing_optional_files() {
    let temp_dir = TempDir::new().unwrap();
    let ito_root = temp_dir.path().join(".ito");
    let change_dir = ito_root.join("changes/001-29_test-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "only proposal\n").unwrap();

    let bundled = collect_proposal_artifacts("001-29_test-change", &ito_root).unwrap();

    assert_eq!(bundled, "---\n# proposal.md\n\nonly proposal");
}

#[test]
fn collect_proposal_artifacts_errors_for_unknown_change() {
    let temp_dir = TempDir::new().unwrap();
    let ito_root = temp_dir.path().join(".ito");
    std::fs::create_dir_all(ito_root.join("changes")).unwrap();

    let error = collect_proposal_artifacts("001-29_missing", &ito_root).unwrap_err();

    assert_eq!(error.to_string(), "Change '001-29_missing' not found");
}

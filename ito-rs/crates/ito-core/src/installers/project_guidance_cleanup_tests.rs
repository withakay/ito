use super::*;

#[test]
fn removes_exact_retired_default_guidance_and_preserves_other_content() {
    let root = tempfile::tempdir().expect("root");
    let path = root.path().join("AGENTS.md");
    std::fs::write(
        &path,
        format!("before\n{RETIRED_DEFAULT_GUIDANCE}\n\ncustom guidance\n"),
    )
    .expect("fixture");

    assert!(remove_retired_default_guidance(&path).expect("cleanup"));
    let updated = std::fs::read_to_string(&path).expect("updated");
    assert_eq!(updated, "before\n\n\ncustom guidance\n");
}

#[test]
fn preserves_customized_guidance_block() {
    let root = tempfile::tempdir().expect("root");
    let path = root.path().join("AGENTS.md");
    let customized = RETIRED_DEFAULT_GUIDANCE.replace(
        "multi-agent: explore multiple approaches and synthesize",
        "multi-agent: project-specific policy",
    );
    std::fs::write(&path, &customized).expect("fixture");

    assert!(!remove_retired_default_guidance(&path).expect("cleanup"));
    assert_eq!(
        std::fs::read_to_string(&path).expect("preserved"),
        customized
    );
}

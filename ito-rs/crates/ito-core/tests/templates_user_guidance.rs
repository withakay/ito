use ito_core::templates::{
    WorkflowError, load_composed_user_guidance, load_user_guidance, load_user_guidance_for_artifact,
};

#[test]
fn load_user_guidance_prefers_user_prompts_guidance_file() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::create_dir_all(ito_path.join("user-prompts")).expect("create dir should succeed");
    std::fs::write(ito_path.join("user-guidance.md"), "Legacy guidance")
        .expect("legacy write should succeed");
    std::fs::write(
        ito_path.join("user-prompts/guidance.md"),
        "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\nNew shared guidance",
    )
    .expect("new write should succeed");

    let guidance = load_user_guidance(ito_path)
        .expect("load should succeed")
        .expect("should be present");

    assert_eq!(guidance, "New shared guidance");
}

/// Ensures that managed ITO header blocks (ITO:START/ITO:END) are removed from loaded user guidance.
///
/// # Examples
///
/// ```
/// // Create a temporary directory, write a guidance file containing a managed header block,
/// // then call `load_user_guidance` and observe the header block is stripped.
/// ```
#[test]
fn load_user_guidance_strips_managed_header_block() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::write(
        ito_path.join("user-guidance.md"),
        "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\nPrefer BDD.\n",
    )
    .expect("write should succeed");

    let guidance = load_user_guidance(ito_path)
        .expect("load should succeed")
        .expect("should be present");

    assert_eq!(guidance, "Prefer BDD.");
}

#[test]
fn load_user_guidance_strips_ito_internal_comment_block() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::write(
        ito_path.join("user-guidance.md"),
        "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\n<!-- ITO:INTERNAL:START -->\n## Your Guidance\n\n(Add your defaults here.)\n<!-- ITO:INTERNAL:END -->\n\nPrefer BDD.\n",
    )
    .expect("write should succeed");

    let guidance = load_user_guidance(ito_path)
        .expect("load should succeed")
        .expect("should be present");

    assert_eq!(guidance, "Prefer BDD.");
}

/// Verifies that a scoped artifact guidance file at `user-prompts/<artifact>.md` is read and returned as-is.
///
/// Creates a temporary directory containing `user-prompts/proposal.md` with the text
/// "Proposal-specific guidance" and asserts `load_user_guidance_for_artifact(path, "proposal")`
/// yields that exact content.
///
/// # Examples
///
/// ```no_run
/// // Setup a temp dir and write user-prompts/proposal.md with "Proposal-specific guidance",
/// // then call `load_user_guidance_for_artifact(path, "proposal")` and assert the returned string.
/// ```
#[test]
fn load_user_guidance_for_artifact_reads_scoped_file() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::create_dir_all(ito_path.join("user-prompts")).expect("create dir should succeed");
    std::fs::write(
        ito_path.join("user-prompts/proposal.md"),
        "Proposal-specific guidance",
    )
    .expect("write should succeed");

    let guidance = load_user_guidance_for_artifact(ito_path, "proposal")
        .expect("load should succeed")
        .expect("should be present");

    assert_eq!(guidance, "Proposal-specific guidance");
}

#[test]
fn load_user_guidance_for_artifact_strips_managed_header_block() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::create_dir_all(ito_path.join("user-prompts")).expect("create dir should succeed");
    std::fs::write(
        ito_path.join("user-prompts/proposal.md"),
        "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\nProposal body guidance",
    )
    .expect("write should succeed");

    let guidance = load_user_guidance_for_artifact(ito_path, "proposal")
        .expect("load should succeed")
        .expect("should be present");

    assert_eq!(guidance, "Proposal body guidance");
}

#[test]
fn load_composed_user_guidance_combines_scoped_and_shared() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    std::fs::create_dir_all(ito_path.join("user-prompts")).expect("create dir should succeed");
    std::fs::write(
        ito_path.join("user-prompts/proposal.md"),
        "Proposal guidance",
    )
    .expect("scoped write should succeed");
    std::fs::write(ito_path.join("user-prompts/guidance.md"), "Shared guidance")
        .expect("shared write should succeed");

    let guidance = load_composed_user_guidance(ito_path, "proposal")
        .expect("load should succeed")
        .expect("should be present");

    assert!(guidance.contains("Scoped Guidance (proposal)"));
    assert!(guidance.contains("Proposal guidance"));
    assert!(guidance.contains("Shared Guidance"));
    assert!(guidance.contains("Shared guidance"));
}

#[test]
fn load_user_guidance_for_artifact_rejects_path_traversal_ids() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = dir.path();

    let err = load_user_guidance_for_artifact(ito_path, "../secrets")
        .expect_err("path traversal id should be rejected");
    assert!(matches!(err, WorkflowError::InvalidArtifactId(_)));

    let err = load_user_guidance_for_artifact(ito_path, "proposal/extra")
        .expect_err("separator in id should be rejected");
    assert!(matches!(err, WorkflowError::InvalidArtifactId(_)));
}

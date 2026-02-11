use std::fs;
use std::path::PathBuf;

#[test]
fn user_guidance_template_exists_and_has_markers() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/default/project");
    let path = root.join(".ito/user-guidance.md");
    assert!(path.exists(), "missing template file: {path:?}");

    let contents = fs::read_to_string(&path).expect("read should succeed");
    assert!(contents.contains("<!-- ITO:START -->"));
    assert!(contents.contains("<!-- ITO:END -->"));
    assert!(
        contents.contains("## Your Guidance"),
        "expected guidance section header"
    );
}

#[test]
fn user_prompt_stub_templates_exist() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/default/project");
    let files = [
        ".ito/user-prompts/guidance.md",
        ".ito/user-prompts/proposal.md",
        ".ito/user-prompts/apply.md",
        ".ito/user-prompts/tasks.md",
    ];

    for rel in files {
        let path = root.join(rel);
        assert!(path.exists(), "missing template file: {path:?}");
        let contents = fs::read_to_string(&path).expect("read should succeed");
        assert!(
            !contents.trim().is_empty(),
            "template should not be empty: {path:?}"
        );
        assert!(
            contents.contains("<!-- ITO:START -->"),
            "expected managed start marker in {path:?}"
        );
        assert!(
            contents.contains("<!-- ITO:END -->"),
            "expected managed end marker in {path:?}"
        );
    }
}

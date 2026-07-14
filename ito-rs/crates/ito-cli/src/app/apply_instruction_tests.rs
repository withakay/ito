use super::*;

#[test]
fn worktree_relative_path_rejects_parent_component_escape() {
    let source = Path::new("/authority/.ito/changes/031-02_change");
    let worktree = Path::new(".ito/changes/031-02_change");
    let escaped = "/authority/.ito/changes/031-02_change/../../templates/schema.yaml";

    let error = worktree_relative_path(source, worktree, escaped).unwrap_err();

    assert!(error.to_string().contains("unsafe relative components"));
}

#[test]
fn worktree_relative_path_preserves_nested_normal_components() {
    let source = Path::new("/authority/.ito/changes/031-02_change");
    let worktree = Path::new(".ito/changes/031-02_change");
    let artifact = "/authority/.ito/changes/031-02_change/specs/main/spec.md";

    let path = worktree_relative_path(source, worktree, artifact).unwrap();

    assert_eq!(path, ".ito/changes/031-02_change/specs/main/spec.md");
}

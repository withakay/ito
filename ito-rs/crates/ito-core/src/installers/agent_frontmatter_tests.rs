use super::*;

#[test]
fn update_agent_model_field_updates_frontmatter_when_present() {
    let td = tempfile::tempdir().unwrap();
    let path = td.path().join("agent.md");
    std::fs::write(&path, "---\nname: test\nmodel: \"old\"\n---\nbody\n").unwrap();
    update_agent_model_field(&path, "new").unwrap();
    let s = std::fs::read_to_string(&path).unwrap();
    assert!(s.contains("model: \"new\""));

    let path = td.path().join("no-frontmatter.md");
    std::fs::write(&path, "no frontmatter\n").unwrap();
    update_agent_model_field(&path, "newer").unwrap();
    let s = std::fs::read_to_string(&path).unwrap();
    assert_eq!(s, "no frontmatter\n");
}

#[test]
fn activation_field_is_copied_from_rendered_template() {
    let td = tempfile::tempdir().unwrap();
    let path = td.path().join("agent.md");
    std::fs::write(&path, "---\nname: test\n---\nbody\n").unwrap();
    update_agent_activation_field_from_rendered(
        &path,
        b"---\nname: test\nactivation: delegated\n---\nrendered\n",
    )
    .unwrap();
    let s = std::fs::read_to_string(&path).unwrap();
    assert!(s.contains("activation: delegated"));
}

#[test]
fn mode_field_is_removed_for_direct_activation() {
    let td = tempfile::tempdir().unwrap();
    let path = td.path().join("agent.md");
    std::fs::write(
            &path,
            "---\nname: test\nmode: subagent\nmode_extra: keep\nnested:\n  mode: keep\nmodel: old\n---\nbody\n",
        )
        .unwrap();
    remove_agent_mode_field_for_direct_activation(
        &path,
        b"---\nname: test\nactivation: direct\n---\nrendered\n",
    )
    .unwrap();
    let s = std::fs::read_to_string(&path).unwrap();
    assert!(!s.contains("mode: subagent"));
    assert!(s.contains("mode_extra: keep"));
    assert!(s.contains("  mode: keep"));
    assert!(s.contains("model: old"));
}

#[test]
fn mode_field_removal_drops_continuation_lines() {
    let yaml = "name: test\nmode: |\n  stale\n  stale again\nmodel: old";
    let updated = remove_yaml_field(yaml, "mode");
    assert_eq!(updated, "name: test\nmodel: old");
}

#[test]
fn update_yaml_field_replaces_or_inserts() {
    let yaml = "name: test\nmodel: \"old\"\n";
    let updated = update_yaml_field(yaml, "model", "\"new\"");
    assert!(updated.contains("model: \"new\""));

    let yaml = "name: test\n";
    let updated = update_yaml_field(yaml, "model", "\"new\"");
    assert!(updated.contains("model: \"new\""));
}

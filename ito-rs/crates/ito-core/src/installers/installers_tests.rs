use super::*;

#[test]
fn gitignore_created_when_missing() {
    let td = tempfile::tempdir().unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, ".ito/session.json\n");
}

#[test]
fn gitignore_noop_when_already_present() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(td.path().join(".gitignore"), ".ito/session.json\n").unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, ".ito/session.json\n");
}

#[test]
fn gitignore_does_not_duplicate_on_repeated_calls() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(td.path().join(".gitignore"), "node_modules\n").unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, "node_modules\n.ito/session.json\n");
}

#[test]
fn gitignore_audit_session_added() {
    let td = tempfile::tempdir().unwrap();
    ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert!(s.contains(".ito/.state/audit/.session"));
}

#[test]
fn gitignore_both_session_entries() {
    let td = tempfile::tempdir().unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert!(s.contains(".ito/session.json"));
    assert!(s.contains(".ito/.state/audit/.session"));
}

#[test]
fn gitignore_preserves_existing_content_and_adds_newline_if_missing() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(td.path().join(".gitignore"), "node_modules").unwrap();
    ensure_repo_gitignore_ignores_session_json(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, "node_modules\n.ito/session.json\n");
}

#[test]
fn gitignore_legacy_audit_events_unignore_removed() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(
        td.path().join(".gitignore"),
        ".ito/.state/\n!.ito/.state/audit/\n",
    )
    .unwrap();
    remove_repo_gitignore_unignores_audit_events(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, ".ito/.state/\n");
}

#[test]
fn gitignore_legacy_audit_events_unignore_noop_when_absent() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(td.path().join(".gitignore"), ".ito/.state/\n").unwrap();
    remove_repo_gitignore_unignores_audit_events(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert_eq!(s, ".ito/.state/\n");
}

#[test]
fn gitignore_full_audit_setup() {
    let td = tempfile::tempdir().unwrap();
    // Simulate a broad .state/ ignore
    std::fs::write(
        td.path().join(".gitignore"),
        ".ito/.state/\n!.ito/.state/audit/\n",
    )
    .unwrap();
    ensure_repo_gitignore_ignores_audit_session(td.path(), ".ito").unwrap();
    remove_repo_gitignore_unignores_audit_events(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert!(s.contains(".ito/.state/audit/.session"));
    assert!(!s.contains("!.ito/.state/audit/"));
}

#[test]
fn gitignore_ignores_local_configs() {
    let td = tempfile::tempdir().unwrap();
    ensure_repo_gitignore_ignores_local_configs(td.path(), ".ito").unwrap();
    let s = std::fs::read_to_string(td.path().join(".gitignore")).unwrap();
    assert!(s.contains(".ito/config.local.json"));
    assert!(s.contains(".local/ito/config.json"));
}

#[test]
fn gitignore_exact_line_matching_trims_whitespace() {
    assert!(gitignore_has_exact_line("  foo  \nbar\n", "foo"));
    assert!(!gitignore_has_exact_line("foo\n", "bar"));
}

#[test]
fn should_install_project_rel_filters_by_tool_id() {
    let mut tools = BTreeSet::new();
    tools.insert(TOOL_OPENCODE.to_string());

    assert!(should_install_project_rel("AGENTS.md", &tools));
    assert!(should_install_project_rel(".ito/config.json", &tools));
    assert!(should_install_project_rel(".opencode/config.json", &tools));
    assert!(!should_install_project_rel(".claude/settings.json", &tools));
    assert!(!should_install_project_rel(".codex/config.json", &tools));
    assert!(!should_install_project_rel(
        ".github/workflows/x.yml",
        &tools
    ));
    assert!(!should_install_project_rel(".pi/settings.json", &tools));
}

#[test]
fn should_install_project_rel_filters_pi() {
    let mut tools = BTreeSet::new();
    tools.insert(TOOL_PI.to_string());

    // Pi-specific files install when Pi is selected
    assert!(should_install_project_rel(".pi/settings.json", &tools));
    assert!(should_install_project_rel(
        ".pi/extensions/ito-skills.ts",
        &tools
    ));

    // Common files always install
    assert!(should_install_project_rel("AGENTS.md", &tools));
    assert!(should_install_project_rel(".ito/config.json", &tools));

    // Other harness files do not install
    assert!(!should_install_project_rel(".opencode/config.json", &tools));
    assert!(!should_install_project_rel(".claude/settings.json", &tools));
    assert!(!should_install_project_rel(".codex/config.json", &tools));
}

#[test]
fn release_tag_is_prefixed_with_v() {
    let tag = release_tag();
    assert!(tag.starts_with('v'));
}

#[test]
fn write_one_non_marker_files_skip_on_init_update_mode() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("plain.txt");
    std::fs::write(&target, "existing").unwrap();

    let opts = InitOptions::new(BTreeSet::new(), false, true);
    write_one(
        &target,
        b"new",
        InstallMode::Init,
        &opts,
        FileOwnership::UserOwned,
    )
    .unwrap();
    let s = std::fs::read_to_string(&target).unwrap();
    assert_eq!(s, "existing");
}

#[test]
fn write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("plain.txt");
    std::fs::write(&target, "existing").unwrap();

    let opts = InitOptions::new(BTreeSet::new(), false, true);
    write_one(
        &target,
        b"new",
        InstallMode::Init,
        &opts,
        FileOwnership::ItoManaged,
    )
    .unwrap();
    let s = std::fs::read_to_string(&target).unwrap();
    assert_eq!(s, "new");
}

#[test]
fn write_one_non_marker_user_owned_files_preserve_on_update_mode() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("plain.txt");
    std::fs::write(&target, "existing").unwrap();

    let opts = InitOptions::new(BTreeSet::new(), false, true);
    write_one(
        &target,
        b"new",
        InstallMode::Update,
        &opts,
        FileOwnership::UserOwned,
    )
    .unwrap();
    let s = std::fs::read_to_string(&target).unwrap();
    assert_eq!(s, "existing");
}

#[test]
fn write_one_marker_managed_files_refuse_overwrite_without_markers() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("managed.md");
    std::fs::write(&target, "existing without markers\n").unwrap();

    let template = format!(
        "before\n{}\nmanaged\n{}\nafter\n",
        ito_templates::ITO_START_MARKER,
        ito_templates::ITO_END_MARKER
    );
    let opts = InitOptions::new(BTreeSet::new(), false, false);
    let err = write_one(
        &target,
        template.as_bytes(),
        InstallMode::Init,
        &opts,
        FileOwnership::ItoManaged,
    )
    .unwrap_err();
    assert!(err.to_string().contains("Refusing to overwrite"));
}

#[test]
fn write_one_marker_managed_files_update_existing_markers() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("managed.md");
    let existing = format!(
        "before\n{}\nold\n{}\nafter\n",
        ito_templates::ITO_START_MARKER,
        ito_templates::ITO_END_MARKER
    );
    std::fs::write(&target, existing).unwrap();

    let template = format!(
        "before\n{}\nnew\n{}\nafter\n",
        ito_templates::ITO_START_MARKER,
        ito_templates::ITO_END_MARKER
    );
    let opts = InitOptions::new(BTreeSet::new(), false, false);
    write_one(
        &target,
        template.as_bytes(),
        InstallMode::Init,
        &opts,
        FileOwnership::ItoManaged,
    )
    .unwrap();
    let s = std::fs::read_to_string(&target).unwrap();
    assert!(s.contains("new"));
    assert!(!s.contains("old"));
}

#[test]
fn write_one_marker_managed_files_error_when_markers_missing_in_update_mode() {
    let td = tempfile::tempdir().unwrap();
    let target = td.path().join("managed.md");
    // One marker present, one missing -> update should error.
    std::fs::write(
        &target,
        format!(
            "{}\nexisting without end marker\n",
            ito_templates::ITO_START_MARKER
        ),
    )
    .unwrap();

    let template = format!(
        "before\n{}\nmanaged\n{}\nafter\n",
        ito_templates::ITO_START_MARKER,
        ito_templates::ITO_END_MARKER
    );
    let opts = InitOptions::new(BTreeSet::new(), false, true);
    let err = write_one(
        &target,
        template.as_bytes(),
        InstallMode::Init,
        &opts,
        FileOwnership::ItoManaged,
    )
    .unwrap_err();
    assert!(err.to_string().contains("Failed to update markers"));
}

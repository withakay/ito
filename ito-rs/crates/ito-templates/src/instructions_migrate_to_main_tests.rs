use super::{MIGRATE_TO_MAIN_TEMPLATE_PATH, render_instruction_template};

#[derive(serde::Serialize)]
struct MigrationContext<'a> {
    project_root: &'a str,
    ito_root: &'a str,
    coordination_branch_name: &'a str,
    coordination_enabled: bool,
    coordination_storage: &'a str,
    expected_coordination_ito_root: &'a str,
    expected_managed_paths: [&'a str; 5],
    observed_evidence_json: &'a str,
    main_integration_mode: &'a str,
}

#[test]
fn migrate_to_main_template_encodes_reversible_reviewed_migration() {
    let rendered = render_instruction_template(
        MIGRATE_TO_MAIN_TEMPLATE_PATH,
        &MigrationContext {
            project_root: "/repo",
            ito_root: "/repo/.ito",
            coordination_branch_name: "ito/internal/changes",
            coordination_enabled: true,
            coordination_storage: "worktree",
            expected_coordination_ito_root: "/coordination/.ito",
            expected_managed_paths: [
                "/repo/.ito/changes",
                "/repo/.ito/specs",
                "/repo/.ito/modules",
                "/repo/.ito/workflows",
                "/repo/.ito/audit",
            ],
            observed_evidence_json: "{\n  \"classification\": { \"kind\": \"legacy\" }\n}",
            main_integration_mode: "pull_request",
        },
    )
    .expect("render migration instruction");

    assert!(rendered.contains("# Migrate Ito coordination state to main"));
    assert!(rendered.contains("/coordination/.ito"));
    assert!(rendered.contains("ito/internal/changes"));
    assert!(rendered.contains("/repo/.ito/audit"));
    assert!(rendered.contains("inventory"));
    assert!(rendered.contains("hash"));
    assert!(rendered.contains("entry type"));
    assert!(rendered.contains("executable bits"));
    assert!(rendered.contains("symlink targets"));
    assert!(rendered.contains("Git does not represent empty directories"));
    assert!(rendered.contains("fresh checkout"));
    assert!(rendered.contains("unsupported special entry"));
    assert!(rendered.contains("stop and report the conflict"));
    assert!(rendered.contains("storage` to `embedded"));
    assert!(rendered.contains("enabled` to `false"));
    assert!(rendered.contains("ito validate --all --strict"));
    assert!(rendered.contains("pull_request"));
    assert!(rendered.contains("Never delete or rewrite the source coordination worktree"));
}

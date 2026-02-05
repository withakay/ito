use ito_core::distribution::{
    AssetType, claude_manifests, codex_manifests, github_manifests, install_manifests,
    opencode_manifests,
};
use std::path::Path;

#[test]
fn opencode_manifests_includes_plugin_and_skills() {
    let config_dir = Path::new("/tmp/test/.opencode");
    let manifests = opencode_manifests(config_dir);

    // Should include the plugin adapter
    let plugin = manifests
        .iter()
        .find(|m| m.source == "opencode/ito-skills.js");
    assert!(plugin.is_some(), "should include opencode plugin adapter");
    let plugin = plugin.unwrap();
    assert_eq!(plugin.asset_type, AssetType::Adapter);
    assert!(plugin.dest.ends_with("plugins/ito-skills.js"));

    // Should include skills with ito- prefix
    let skills: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .collect();
    assert!(!skills.is_empty(), "should include skills");

    // All skills should have ito prefix in dest (either "ito-" or just "ito/")
    for skill in &skills {
        let dest_str = skill.dest.to_string_lossy();
        assert!(
            dest_str.contains("/ito"),
            "skill dest should have ito prefix: {}",
            dest_str
        );
    }
}

#[test]
fn claude_manifests_includes_session_start_and_skills() {
    let project_root = Path::new("/tmp/test");
    let manifests = claude_manifests(project_root);

    // Should include session-start.sh adapter
    let adapter = manifests
        .iter()
        .find(|m| m.source == "claude/session-start.sh");
    assert!(adapter.is_some(), "should include claude session-start.sh");
    let adapter = adapter.unwrap();
    assert_eq!(adapter.asset_type, AssetType::Adapter);
    assert!(adapter.dest.ends_with(".claude/session-start.sh"));

    // Should include skills
    let skills: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .collect();
    assert!(!skills.is_empty(), "should include skills");

    // Skills should go under .claude/skills/
    for skill in &skills {
        let dest_str = skill.dest.to_string_lossy();
        assert!(
            dest_str.contains(".claude/skills/"),
            "skill should be under .claude/skills/: {}",
            dest_str
        );
    }
}

#[test]
fn codex_manifests_includes_bootstrap_and_skills() {
    let project_root = Path::new("/tmp/test");
    let manifests = codex_manifests(project_root);

    // Should include bootstrap adapter
    let adapter = manifests
        .iter()
        .find(|m| m.source == "codex/ito-skills-bootstrap.md");
    assert!(adapter.is_some(), "should include codex bootstrap");
    let adapter = adapter.unwrap();
    assert_eq!(adapter.asset_type, AssetType::Adapter);
    assert!(
        adapter
            .dest
            .ends_with(".codex/instructions/ito-skills-bootstrap.md")
    );

    // Should include skills
    let skills: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .collect();
    assert!(!skills.is_empty(), "should include skills");

    // Skills should go under .codex/skills/
    for skill in &skills {
        let dest_str = skill.dest.to_string_lossy();
        assert!(
            dest_str.contains(".codex/skills/"),
            "skill should be under .codex/skills/: {}",
            dest_str
        );
    }
}

#[test]
fn github_manifests_includes_skills_and_commands() {
    let project_root = Path::new("/tmp/test");
    let manifests = github_manifests(project_root);

    // Should include skills and commands (no special adapter files)
    let skills: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .collect();
    let commands: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Command)
        .collect();

    assert!(!skills.is_empty(), "should include skills");
    assert!(!commands.is_empty(), "should include commands");

    // Skills should go under .github/skills/
    for skill in &skills {
        let dest_str = skill.dest.to_string_lossy();
        assert!(
            dest_str.contains(".github/skills/"),
            "skill should be under .github/skills/: {}",
            dest_str
        );
    }

    // Commands should go under .github/prompts/ with .prompt.md suffix
    for cmd in &commands {
        let dest_str = cmd.dest.to_string_lossy();
        assert!(
            dest_str.contains(".github/prompts/"),
            "command should be under .github/prompts/: {}",
            dest_str
        );
        assert!(
            dest_str.ends_with(".prompt.md"),
            "github prompts should have .prompt.md suffix: {}",
            dest_str
        );
    }
}

#[test]
fn install_manifests_writes_files_to_disk() {
    let td = tempfile::tempdir().unwrap();
    let config_dir = td.path().join(".opencode");

    let manifests = opencode_manifests(&config_dir);
    install_manifests(&manifests).unwrap();

    // Check plugin was installed
    assert!(
        config_dir.join("plugins").join("ito-skills.js").exists(),
        "plugin should be installed"
    );

    // Check at least one skill was installed
    let skills_dir = config_dir.join("skills");
    assert!(skills_dir.exists(), "skills directory should exist");

    // Should have ito-brainstorming skill
    assert!(
        skills_dir
            .join("ito-brainstorming")
            .join("SKILL.md")
            .exists(),
        "brainstorming skill should be installed"
    );
}

#[test]
fn install_manifests_creates_parent_directories() {
    let td = tempfile::tempdir().unwrap();
    let deep_path = td.path().join("a").join("b").join("c").join(".claude");

    let manifests = claude_manifests(deep_path.parent().unwrap());
    install_manifests(&manifests).unwrap();

    // Parent directories should be created
    assert!(deep_path.join("session-start.sh").exists());
}

#[test]
fn all_manifests_use_embedded_assets() {
    // Verify that all manifest generators produce valid manifests
    // that can be installed from embedded assets
    let td = tempfile::tempdir().unwrap();

    // OpenCode
    let oc = td.path().join("opencode");
    let manifests = opencode_manifests(&oc);
    assert!(
        install_manifests(&manifests).is_ok(),
        "opencode manifests should install successfully"
    );

    // Claude
    let claude = td.path().join("claude");
    let manifests = claude_manifests(&claude);
    assert!(
        install_manifests(&manifests).is_ok(),
        "claude manifests should install successfully"
    );

    // Codex
    let codex = td.path().join("codex");
    let manifests = codex_manifests(&codex);
    assert!(
        install_manifests(&manifests).is_ok(),
        "codex manifests should install successfully"
    );

    // GitHub
    let github = td.path().join("github");
    let manifests = github_manifests(&github);
    assert!(
        install_manifests(&manifests).is_ok(),
        "github manifests should install successfully"
    );
}

use ito_core::distribution::{
    claude_manifests, codex_manifests, github_manifests, install_manifests, opencode_manifests,
    AssetType,
};
use ito_templates::project_templates::WorktreeTemplateContext;
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
fn claude_manifests_includes_hooks_and_skills() {
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

    // Should include pre-tool audit hook adapter
    let hook = manifests
        .iter()
        .find(|m| m.source == "claude/hooks/ito-audit.sh");
    assert!(hook.is_some(), "should include claude pre-tool audit hook");
    let hook = hook.unwrap();
    assert_eq!(hook.asset_type, AssetType::Adapter);
    assert!(hook.dest.ends_with(".claude/hooks/ito-audit.sh"));

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
    assert!(adapter
        .dest
        .ends_with(".codex/instructions/ito-skills-bootstrap.md"));

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
    install_manifests(&manifests, None).unwrap();

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
    install_manifests(&manifests, None).unwrap();

    // Parent directories should be created
    assert!(deep_path.join("session-start.sh").exists());
    assert!(deep_path.join("hooks/ito-audit.sh").exists());
}

#[test]
fn install_manifests_renders_worktree_skill_with_context() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path().join("project");

    // Use claude manifests (any harness works — they all install skills)
    let manifests = claude_manifests(&project_root);

    // Install with a disabled worktree context (the most common case)
    let ctx = WorktreeTemplateContext::default();
    install_manifests(&manifests, Some(&ctx)).unwrap();

    // The using-git-worktrees skill should exist and be rendered (no Jinja2 syntax)
    let worktree_skill = project_root.join(".claude/skills/ito-using-git-worktrees/SKILL.md");
    assert!(
        worktree_skill.exists(),
        "worktree skill should be installed"
    );

    let content = std::fs::read_to_string(&worktree_skill).unwrap();
    assert!(
        !content.contains("{%"),
        "rendered skill should not contain Jinja2 block syntax"
    );
    assert!(
        content.contains("Worktrees are not configured for this project."),
        "disabled context should render explicit non-worktree guidance"
    );
}

#[test]
fn install_manifests_renders_worktree_skill_enabled() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path().join("project");

    let manifests = claude_manifests(&project_root);

    let ctx = WorktreeTemplateContext {
        enabled: true,
        strategy: "checkout_subdir".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
        project_root: "/home/user/project".to_string(),
    };
    install_manifests(&manifests, Some(&ctx)).unwrap();

    let worktree_skill = project_root.join(".claude/skills/ito-using-git-worktrees/SKILL.md");
    let content = std::fs::read_to_string(&worktree_skill).unwrap();
    assert!(
        !content.contains("{%"),
        "rendered skill should not contain Jinja2 block syntax"
    );
    assert!(
        content.contains("**Configured strategy:** `checkout_subdir`"),
        "enabled context should render strategy-specific guidance"
    );
}

#[test]
fn install_manifests_keeps_non_worktree_placeholders_verbatim() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path().join("project");

    let manifests = claude_manifests(&project_root);
    let ctx = WorktreeTemplateContext::default();
    install_manifests(&manifests, Some(&ctx)).unwrap();

    let research_skill = project_root.join(".claude/skills/ito-research/research-stack.md");
    assert!(
        research_skill.exists(),
        "research skill file should be installed"
    );

    let content = std::fs::read_to_string(research_skill).unwrap();
    assert!(
        content.contains("{{topic}}"),
        "non-worktree template placeholders should remain verbatim"
    );
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
        install_manifests(&manifests, None).is_ok(),
        "opencode manifests should install successfully"
    );

    // Claude
    let claude = td.path().join("claude");
    let manifests = claude_manifests(&claude);
    assert!(
        install_manifests(&manifests, None).is_ok(),
        "claude manifests should install successfully"
    );

    // Codex
    let codex = td.path().join("codex");
    let manifests = codex_manifests(&codex);
    assert!(
        install_manifests(&manifests, None).is_ok(),
        "codex manifests should install successfully"
    );

    // GitHub
    let github = td.path().join("github");
    let manifests = github_manifests(&github);
    assert!(
        install_manifests(&manifests, None).is_ok(),
        "github manifests should install successfully"
    );
}

use ito_core::distribution::{
    AssetType, MIGRATE_TO_MAIN_PROMPT, claude_manifests, codex_manifests, github_manifests,
    install_manifests, opencode_manifests, pi_manifests,
};
use ito_core::installers::{InitOptions, InstallMode};
use ito_templates::project_templates::WorktreeTemplateContext;
use std::collections::BTreeSet;
use std::path::Path;

/// Default init mode + opts used by every legacy `install_manifests` test
/// caller in this file. These tests originally pre-dated the mode/opts
/// signature change in 023-09 and assume "fresh init, no force, no update".
fn legacy_init_args() -> (InstallMode, InitOptions) {
    (
        InstallMode::Init,
        InitOptions::new(BTreeSet::new(), false, false),
    )
}

#[test]
fn opencode_manifests_includes_plugin_and_skills() {
    let config_dir = Path::new("/tmp/test/.opencode");
    let manifests = opencode_manifests(config_dir);

    // Should include the plugin adapter
    let mut plugin = None;
    for manifest in &manifests {
        if manifest.source == "opencode/ito-skills.js" {
            plugin = Some(manifest);
            break;
        }
    }
    let plugin = plugin.expect("should include opencode plugin adapter");
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

    let has_ito_loop = manifests.iter().any(|m| {
        m.asset_type == AssetType::Skill
            && m.source == "ito-loop/SKILL.md"
            && m.dest
                .to_string_lossy()
                .ends_with("/skills/ito-loop/SKILL.md")
    });
    assert!(has_ito_loop, "should include ito-loop skill");

    let has_ito_orchestrate = manifests.iter().any(|m| {
        m.asset_type == AssetType::Skill
            && m.source == "ito-orchestrate/SKILL.md"
            && m.dest
                .to_string_lossy()
                .ends_with("/skills/ito-orchestrate/SKILL.md")
    });
    assert!(has_ito_orchestrate, "should include ito-orchestrate skill");

    // Should include the renamed ito-loop command.
    let has_loop = manifests.iter().any(|m| {
        m.asset_type == AssetType::Command
            && m.source == "ito-loop.md"
            && m.dest.to_string_lossy().ends_with("/commands/ito-loop.md")
    });
    assert!(has_loop, "should include ito-loop command");

    let has_orchestrate = manifests.iter().any(|m| {
        m.asset_type == AssetType::Command
            && m.source == "ito-orchestrate.md"
            && m.dest
                .to_string_lossy()
                .ends_with("/commands/ito-orchestrate.md")
    });
    assert!(has_orchestrate, "should include ito-orchestrate command");
}

#[test]
fn claude_manifests_includes_hooks_and_skills() {
    let project_root = Path::new("/tmp/test");
    let manifests = claude_manifests(project_root);

    // Should include session-start.sh adapter
    let mut adapter = None;
    for manifest in &manifests {
        if manifest.source == "claude/session-start.sh" {
            adapter = Some(manifest);
            break;
        }
    }
    let adapter = adapter.expect("should include claude session-start.sh");
    assert_eq!(adapter.asset_type, AssetType::Adapter);
    assert!(adapter.dest.ends_with(".claude/session-start.sh"));

    // Should include pre-tool audit hook adapter
    let mut hook = None;
    for manifest in &manifests {
        if manifest.source == "claude/hooks/ito-audit.sh" {
            hook = Some(manifest);
            break;
        }
    }
    let hook = hook.expect("should include claude pre-tool audit hook");
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
    let mut adapter = None;
    for manifest in &manifests {
        if manifest.source == "codex/ito-skills-bootstrap.md" {
            adapter = Some(manifest);
            break;
        }
    }
    let adapter = adapter.expect("should include codex bootstrap");
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
fn wiki_skills_are_distributed_to_all_harnesses() {
    let project_root = Path::new("/tmp/test");

    for (harness, manifests) in [
        ("opencode", opencode_manifests(project_root)),
        ("claude", claude_manifests(project_root)),
        ("codex", codex_manifests(project_root)),
        ("github", github_manifests(project_root)),
    ] {
        for source in ["ito-wiki/SKILL.md", "ito-wiki-search/SKILL.md"] {
            assert!(
                manifests.iter().any(|manifest| manifest.source == source),
                "expected {source} in {harness} manifests"
            );
        }
    }
}

#[test]
fn migrate_to_main_prompt_is_distributed_to_all_harnesses() {
    let project_root = Path::new("/tmp/test");

    for (harness, manifests) in [
        ("opencode", opencode_manifests(project_root)),
        ("claude", claude_manifests(project_root)),
        ("codex", codex_manifests(project_root)),
        ("github", github_manifests(project_root)),
        ("pi", pi_manifests(project_root)),
    ] {
        let prompt = manifests.iter().find(|manifest| {
            manifest.asset_type == AssetType::Command && manifest.source == MIGRATE_TO_MAIN_PROMPT
        });
        let prompt =
            prompt.unwrap_or_else(|| panic!("missing migrate-to-main prompt for {harness}"));
        let destination = prompt.dest.to_string_lossy();
        assert!(
            destination.contains("migrate-to-main"),
            "unexpected {harness} destination: {destination}"
        );

        let td = tempfile::tempdir().expect("harness install root");
        let installed = match harness {
            "opencode" => opencode_manifests(&td.path().join(".opencode")),
            "claude" => claude_manifests(td.path()),
            "codex" => codex_manifests(td.path()),
            "github" => github_manifests(td.path()),
            "pi" => pi_manifests(td.path()),
            _ => unreachable!("known harness"),
        };
        let prompt = installed
            .into_iter()
            .find(|manifest| manifest.source == MIGRATE_TO_MAIN_PROMPT)
            .expect("installed migration prompt");
        let (mode, opts) = legacy_init_args();
        install_manifests(std::slice::from_ref(&prompt), None, mode, &opts)
            .expect("install migration prompt");
        let content = std::fs::read_to_string(&prompt.dest).expect("read installed prompt");
        assert!(content.contains("ito agent instruction migrate-to-main"));
    }
}

#[test]
fn install_manifests_writes_files_to_disk() {
    let td = tempfile::tempdir().unwrap();
    let config_dir = td.path().join(".opencode");

    let manifests = opencode_manifests(&config_dir);
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, None, mode, &opts).unwrap();

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
fn install_manifests_do_not_install_tmux_skill_assets() {
    let td = tempfile::tempdir().unwrap();
    let config_dir = td.path().join(".opencode");

    let manifests = opencode_manifests(&config_dir);
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, None, mode, &opts).unwrap();

    assert!(!config_dir.join("skills/ito-tmux/SKILL.md").exists());
    assert!(
        !config_dir
            .join("skills/ito-tmux/scripts/wait-for-text.sh")
            .exists()
    );
    assert!(
        !config_dir
            .join("skills/ito-tmux/scripts/find-sessions.sh")
            .exists()
    );
}

#[test]
fn install_manifests_creates_parent_directories() {
    let td = tempfile::tempdir().unwrap();
    let deep_path = td.path().join("a").join("b").join("c").join(".claude");

    let manifests = claude_manifests(deep_path.parent().unwrap());
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, None, mode, &opts).unwrap();

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
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, Some(&ctx), mode, &opts).unwrap();

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
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, Some(&ctx), mode, &opts).unwrap();

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
    let (mode, opts) = legacy_init_args();
    install_manifests(&manifests, Some(&ctx), mode, &opts).unwrap();

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
        {
            let (mode, opts) = legacy_init_args();
            install_manifests(&manifests, None, mode, &opts).is_ok()
        },
        "opencode manifests should install successfully"
    );

    // Claude
    let claude = td.path().join("claude");
    let manifests = claude_manifests(&claude);
    assert!(
        {
            let (mode, opts) = legacy_init_args();
            install_manifests(&manifests, None, mode, &opts).is_ok()
        },
        "claude manifests should install successfully"
    );

    // Codex
    let codex = td.path().join("codex");
    let manifests = codex_manifests(&codex);
    assert!(
        {
            let (mode, opts) = legacy_init_args();
            install_manifests(&manifests, None, mode, &opts).is_ok()
        },
        "codex manifests should install successfully"
    );

    // GitHub
    let github = td.path().join("github");
    let manifests = github_manifests(&github);
    assert!(
        {
            let (mode, opts) = legacy_init_args();
            install_manifests(&manifests, None, mode, &opts).is_ok()
        },
        "github manifests should install successfully"
    );
}

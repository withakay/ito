use super::*;

#[test]
fn manifest_includes_project_and_codex_entries() {
    let entries = generate_manifest(&[HarnessTool::Codex]);

    assert!(entries.iter().any(|entry| {
        entry.relative_path == "AGENTS.md"
            && entry.source == ManifestSource::Project
            && entry.harness.is_none()
    }));
    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".codex/instructions/ito-skills-bootstrap.md"
            && entry.source == ManifestSource::Adapter
            && entry.harness == Some(HarnessTool::Codex)
    }));
    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".codex/skills/ito-plan/SKILL.md"
            && entry.source == ManifestSource::Skill
            && entry.harness == Some(HarnessTool::Codex)
    }));
    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".codex/prompts/ito-plan.md"
            && entry.source == ManifestSource::Command
            && entry.harness == Some(HarnessTool::Codex)
    }));
}

#[test]
fn github_commands_use_prompt_suffix() {
    let entries = generate_manifest(&[HarnessTool::GitHubCopilot]);

    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".github/prompts/ito-plan.prompt.md"
            && entry.source_path == "ito-plan.md"
            && entry.harness == Some(HarnessTool::GitHubCopilot)
    }));
}

#[test]
fn every_harness_manifest_excludes_removed_tmux_assets() {
    let tools = [
        HarnessTool::ClaudeCode,
        HarnessTool::OpenCode,
        HarnessTool::Codex,
        HarnessTool::GitHubCopilot,
        HarnessTool::Pi,
    ];

    for tool in tools {
        let entries = generate_manifest(&[tool]);
        assert!(
            entries.iter().all(|entry| {
                !entry.relative_path.contains("ito-tmux") && !entry.source_path.contains("ito-tmux")
            }),
            "{tool:?} manifest still contains removed tmux assets"
        );
    }
}

use super::*;
use std::collections::BTreeSet;

#[test]
fn skill_inventory_is_exact_for_every_harness_manifest() {
    for tool in [
        HarnessTool::ClaudeCode,
        HarnessTool::OpenCode,
        HarnessTool::Codex,
        HarnessTool::GitHubCopilot,
        HarnessTool::Pi,
    ] {
        let entrypoints = generate_manifest(&[tool])
            .into_iter()
            .filter(|entry| entry.source == ManifestSource::Skill)
            .filter(|entry| entry.source_path.ends_with("/SKILL.md"))
            .filter_map(|entry| entry.source_path.split('/').next().map(str::to_owned))
            .collect::<Vec<_>>();
        let actual = entrypoints.iter().cloned().collect::<BTreeSet<_>>();
        let expected = crate::LIFECYCLE_SKILL_NAMES
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();

        assert_eq!(
            entrypoints.len(),
            7,
            "duplicate or extra skill for {tool:?}"
        );
        assert_eq!(actual, expected, "unexpected skill inventory for {tool:?}");
        assert!(
            actual.contains("ito-loop"),
            "ito-loop must remain installed"
        );
    }
}

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
        entry.relative_path == ".codex/skills/ito-proposal/SKILL.md"
            && entry.source == ManifestSource::Skill
            && entry.harness == Some(HarnessTool::Codex)
    }));
    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".codex/prompts/ito-proposal.md"
            && entry.source == ManifestSource::Command
            && entry.harness == Some(HarnessTool::Codex)
    }));
}

#[test]
fn github_commands_use_prompt_suffix() {
    let entries = generate_manifest(&[HarnessTool::GitHubCopilot]);

    assert!(entries.iter().any(|entry| {
        entry.relative_path == ".github/prompts/ito-proposal.prompt.md"
            && entry.source_path == "ito-proposal.md"
            && entry.harness == Some(HarnessTool::GitHubCopilot)
    }));
}

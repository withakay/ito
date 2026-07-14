use super::*;
use std::collections::BTreeSet;

#[test]
fn retired_surface_map_is_complete_and_unique() {
    let expected_skills = [
        "ito-brainstorming",
        "ito-cleanup",
        "ito-commit",
        "ito-feature",
        "ito-finish",
        "ito-fix",
        "ito-list",
        "ito-memory",
        "ito-orchestrate",
        "ito-orchestrate-setup",
        "ito-orchestrator-workflow",
        "ito-path",
        "ito-plan",
        "ito-proposal-intake",
        "ito-subagent-driven-development",
        "ito-tasks",
        "ito-test-with-subagent",
        "ito-tmux",
        "ito-update-repo",
        "ito-using-git-worktrees",
        "ito-using-ito-skills",
        "ito-verification-before-completion",
        "ito-wiki",
        "ito-wiki-search",
        "ito-workflow",
    ];
    let actual_skills = RETIRED_SKILLS
        .iter()
        .map(|entry| entry.name)
        .collect::<BTreeSet<_>>();
    let expected_skills = expected_skills.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(RETIRED_SKILLS.len(), expected_skills.len());
    assert_eq!(actual_skills, expected_skills);

    let expected_commands = [
        "ito-feature",
        "ito-fix",
        "ito-list",
        "loop",
        "ito-orchestrate",
        "ito-plan",
        "ito-proposal-intake",
        "ito-project-setup",
        "ito-update-repo",
    ];
    let actual_commands = RETIRED_COMMANDS
        .iter()
        .map(|entry| entry.name)
        .collect::<BTreeSet<_>>();
    let expected_commands = expected_commands.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(RETIRED_COMMANDS.len(), expected_commands.len());
    assert_eq!(actual_commands, expected_commands);

    let expected_historical_skills = [
        "ito-apply-change-proposal",
        "ito-dispatching-parallel-agents",
        "ito-finishing-a-development-branch",
        "ito-receiving-code-review",
        "ito-requesting-code-review",
        "ito-systematic-debugging",
        "ito-test-driven-development",
        "ito-write-change-proposal",
        "ito-writing-skills",
        "test-with-subagent",
        "tmux",
        "using-ito-skills",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let actual_historical_skills = HISTORICAL_RETIRED_SKILLS
        .iter()
        .map(|entry| entry.name)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        HISTORICAL_RETIRED_SKILLS.len(),
        expected_historical_skills.len()
    );
    assert_eq!(actual_historical_skills, expected_historical_skills);

    assert_eq!(
        RETIRED_SKILLS
            .iter()
            .filter(|entry| entry.replacement.is_none())
            .map(|entry| entry.name)
            .collect::<Vec<_>>(),
        ["ito-tmux"]
    );
}

#[test]
fn legacy_entries_include_required_examples() {
    assert!(LEGACY_ENTRIES.len() >= 17);

    assert!(LEGACY_ENTRIES.iter().any(|entry| {
        entry.old_path == "ito-apply-change-proposal/SKILL.md"
            && entry.new_path == Some("ito-apply/SKILL.md")
            && entry.entry_type == LegacyEntryType::Renamed
    }));
    assert!(
        LEGACY_ENTRIES
            .iter()
            .any(|entry| entry.old_path == ".ito/planning/"
                && entry.entry_type == LegacyEntryType::Removed)
    );
    assert!(LEGACY_ENTRIES.iter().any(|entry| {
        entry.old_path == ".opencode/command/"
            && entry.new_path == Some(".opencode/commands/")
            && entry.entry_type == LegacyEntryType::Relocated
    }));
    assert!(LEGACY_ENTRIES.iter().any(|entry| {
        entry.old_path == "ito-tmux/"
            && entry.new_path.is_none()
            && entry.entry_type == LegacyEntryType::Removed
    }));
}

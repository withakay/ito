//! Registry of known legacy Ito-managed files and directories.

/// Classification for a legacy Ito-managed path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyEntryType {
    /// The artifact was renamed in place.
    Renamed,
    /// The artifact was removed and has no current replacement.
    Removed,
    /// The artifact moved to a different directory or harness surface.
    Relocated,
}

/// One known legacy Ito-managed path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyEntry {
    /// Path that may exist in older repositories.
    pub old_path: &'static str,
    /// Replacement path when one exists.
    pub new_path: Option<&'static str>,
    /// How this path changed.
    pub entry_type: LegacyEntryType,
    /// Human-readable cleanup context.
    pub description: &'static str,
}

/// One retired user-facing skill or command and its supported replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetiredSurface {
    /// Retired activation name without a file extension.
    pub name: &'static str,
    /// Retained lifecycle entrypoint, direct CLI route, or `None` when the
    /// integration was deliberately removed.
    pub replacement: Option<&'static str>,
}

/// Skills consolidated into the seven lifecycle entrypoints.
pub const RETIRED_SKILLS: &[RetiredSurface] = &[
    RetiredSurface {
        name: "ito-brainstorming",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-cleanup",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-commit",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-feature",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-finish",
        replacement: Some("ito-archive"),
    },
    RetiredSurface {
        name: "ito-fix",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-list",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-memory",
        replacement: Some("ito-research + ito-archive"),
    },
    RetiredSurface {
        name: "ito-orchestrate",
        replacement: Some("ito-loop"),
    },
    RetiredSurface {
        name: "ito-orchestrate-setup",
        replacement: Some("ito-loop"),
    },
    RetiredSurface {
        name: "ito-orchestrator-workflow",
        replacement: Some("ito-loop"),
    },
    RetiredSurface {
        name: "ito-path",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-plan",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-proposal-intake",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-subagent-driven-development",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-tasks",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-test-with-subagent",
        replacement: Some("ito-review"),
    },
    RetiredSurface {
        name: "ito-tmux",
        replacement: None,
    },
    RetiredSurface {
        name: "ito-update-repo",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-using-git-worktrees",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-using-ito-skills",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-verification-before-completion",
        replacement: Some("ito-review"),
    },
    RetiredSurface {
        name: "ito-wiki",
        replacement: Some("ito-archive"),
    },
    RetiredSurface {
        name: "ito-wiki-search",
        replacement: Some("ito-research"),
    },
    RetiredSurface {
        name: "ito-workflow",
        replacement: Some("ito"),
    },
];

/// Older Ito-managed skills that may remain when a repository skipped one or
/// more upgrade generations.
pub const HISTORICAL_RETIRED_SKILLS: &[RetiredSurface] = &[
    RetiredSurface {
        name: "ito-apply-change-proposal",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-write-change-proposal",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-dispatching-parallel-agents",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-finishing-a-development-branch",
        replacement: Some("ito-archive"),
    },
    RetiredSurface {
        name: "ito-receiving-code-review",
        replacement: Some("ito-review"),
    },
    RetiredSurface {
        name: "ito-requesting-code-review",
        replacement: Some("ito-review"),
    },
    RetiredSurface {
        name: "ito-systematic-debugging",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-test-driven-development",
        replacement: Some("ito-apply"),
    },
    RetiredSurface {
        name: "ito-writing-skills",
        replacement: None,
    },
    RetiredSurface {
        name: "tmux",
        replacement: None,
    },
    RetiredSurface {
        name: "test-with-subagent",
        replacement: Some("ito-review"),
    },
    RetiredSurface {
        name: "using-ito-skills",
        replacement: Some("ito"),
    },
];

/// Command wrappers removed in favor of lifecycle entrypoints or direct CLI
/// commands.
pub const RETIRED_COMMANDS: &[RetiredSurface] = &[
    RetiredSurface {
        name: "ito-feature",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-fix",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-list",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "loop",
        replacement: Some("ito-loop"),
    },
    RetiredSurface {
        name: "ito-orchestrate",
        replacement: Some("ito-loop"),
    },
    RetiredSurface {
        name: "ito-plan",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-proposal-intake",
        replacement: Some("ito-proposal"),
    },
    RetiredSurface {
        name: "ito-project-setup",
        replacement: Some("ito"),
    },
    RetiredSurface {
        name: "ito-update-repo",
        replacement: Some("ito"),
    },
];

/// Known legacy Ito-managed files and directories from previous releases.
pub const LEGACY_ENTRIES: &[LegacyEntry] = &[
    LegacyEntry {
        old_path: "ito-tmux/",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Ito no longer distributes or manages a tmux skill.",
    },
    LegacyEntry {
        old_path: ".ito/planning/",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Removed planning workspace directory; planning now uses configured plan/research paths.",
    },
    LegacyEntry {
        old_path: ".opencode/command/",
        new_path: Some(".opencode/commands/"),
        entry_type: LegacyEntryType::Relocated,
        description: "OpenCode command directory was renamed to the plural commands directory.",
    },
    LegacyEntry {
        old_path: ".opencode/agent/",
        new_path: Some(".opencode/agents/"),
        entry_type: LegacyEntryType::Relocated,
        description: "OpenCode agent directory was renamed to the plural agents directory.",
    },
    LegacyEntry {
        old_path: ".claude/commands/loop.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy unprefixed loop command was removed in favor of Ito-prefixed commands.",
    },
    LegacyEntry {
        old_path: ".opencode/commands/loop.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy unprefixed loop command was removed in favor of Ito-prefixed commands.",
    },
    LegacyEntry {
        old_path: ".codex/prompts/loop.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy unprefixed loop prompt was removed in favor of Ito-prefixed prompts.",
    },
    LegacyEntry {
        old_path: ".github/prompts/loop.prompt.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy unprefixed loop prompt was removed in favor of Ito-prefixed prompts.",
    },
    LegacyEntry {
        old_path: ".pi/commands/loop.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy unprefixed loop command was removed in favor of Ito-prefixed commands.",
    },
    LegacyEntry {
        old_path: "ito-apply-change-proposal/SKILL.md",
        new_path: Some("ito-apply/SKILL.md"),
        entry_type: LegacyEntryType::Renamed,
        description: "Apply skill was renamed to ito-apply.",
    },
    LegacyEntry {
        old_path: "ito-write-change-proposal/SKILL.md",
        new_path: Some("ito-proposal/SKILL.md"),
        entry_type: LegacyEntryType::Renamed,
        description: "Proposal-writing skill was renamed to ito-proposal.",
    },
    LegacyEntry {
        old_path: "ito-dispatching-parallel-agents/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy parallel-agent dispatch skill was removed.",
    },
    LegacyEntry {
        old_path: "ito-finishing-a-development-branch/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy finishing skill was replaced by current Ito finish workflow guidance.",
    },
    LegacyEntry {
        old_path: "ito-receiving-code-review/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy receiving-code-review skill was removed.",
    },
    LegacyEntry {
        old_path: "ito-requesting-code-review/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy requesting-code-review skill was removed.",
    },
    LegacyEntry {
        old_path: "ito-systematic-debugging/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy systematic-debugging skill was removed.",
    },
    LegacyEntry {
        old_path: "ito-test-driven-development/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy TDD skill was removed.",
    },
    LegacyEntry {
        old_path: "ito-writing-skills/SKILL.md",
        new_path: None,
        entry_type: LegacyEntryType::Removed,
        description: "Legacy skill-authoring skill was removed.",
    },
];

#[cfg(test)]
#[path = "legacy_tests.rs"]
mod legacy_tests;

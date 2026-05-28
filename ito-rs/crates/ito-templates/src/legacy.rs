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

/// Known legacy Ito-managed files and directories from previous releases.
pub const LEGACY_ENTRIES: &[LegacyEntry] = &[
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

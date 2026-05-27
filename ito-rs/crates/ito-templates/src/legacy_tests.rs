use super::*;

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
}

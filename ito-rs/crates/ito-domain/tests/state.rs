use ito_domain::state;

fn sample_state_md(date: &str) -> String {
    format!(
        "# Project State\n\nLast Updated: {date}\n\n## Current Focus\n[What we're working on right now]\n\n## Recent Decisions\n- {date}: Project initialized\n\n## Open Questions\n- [ ] [Question needing resolution]\n\n## Blockers\n[None currently]\n\n## Session Notes\n### {date} - Initial Setup\n- Completed: Project planning structure initialized\n- Next: Define project vision and first milestone\n\n---\n"
    )
}

#[test]
fn add_decision_inserts_and_updates_last_updated() {
    let original = sample_state_md("2026-01-01");
    let out = state::add_decision(&original, "2026-01-28", "Decide thing").unwrap();
    assert!(out.contains("Last Updated: 2026-01-28"));
    assert!(out.contains("- 2026-01-28: Decide thing"));
}

#[test]
fn add_question_inserts_checkbox() {
    let original = sample_state_md("2026-01-01");
    let out = state::add_question(&original, "2026-01-28", "What now?").unwrap();
    assert!(out.contains("- [ ] What now?"));
}

#[test]
fn add_blocker_replaces_none_currently() {
    let original = sample_state_md("2026-01-01");
    let out = state::add_blocker(&original, "2026-01-28", "Blocked thing").unwrap();
    assert!(!out.contains("[None currently]"));
    assert!(out.contains("- Blocked thing"));
}

#[test]
fn set_focus_replaces_section_body() {
    let original = sample_state_md("2026-01-01");
    let out = state::set_focus(&original, "2026-01-28", "Ship it").unwrap();
    assert!(out.contains("## Current Focus\nShip it"));
    assert!(!out.contains("[What we're working on right now]"));
}

#[test]
fn add_note_inserts_new_session_when_missing() {
    let original = sample_state_md("2026-01-01");
    let out = state::add_note(&original, "2026-01-28", "12:34:56", "Note text").unwrap();
    assert!(out.contains("## Session Notes\n### 2026-01-28 Session\n- 12:34:56: Note text"));
}

#[test]
fn add_note_appends_to_existing_session() {
    let original = "# Project State\n\nLast Updated: 2026-01-01\n\n## Session Notes\n### 2026-01-28 Session\n- 10:00:00: First\n\n---\n";
    let out = state::add_note(original, "2026-01-28", "10:01:02", "Second").unwrap();
    assert!(out.contains("### 2026-01-28 Session\n- 10:01:02: Second"));
}

#[test]
fn add_decision_errors_when_missing_section() {
    let original = "# Project State\n\nLast Updated: 2026-01-01\n\n---\n";
    assert!(state::add_decision(original, "2026-01-28", "X").is_err());
}

#[test]
fn add_blocker_inserts_when_list_already_present() {
    let original = "# Project State\n\nLast Updated: 2026-01-01\n\n## Blockers\n- Existing\n\n## Session Notes\n---\n";
    let out = state::add_blocker(original, "2026-01-28", "New blocker").unwrap();
    assert!(out.contains("## Blockers\n- Existing\n- New blocker"));
}

#[test]
fn update_last_updated_noops_when_missing() {
    let original = "# Project State\n\n## Current Focus\nX\n";
    let out = state::update_last_updated(original, "2026-01-28");
    assert_eq!(out, original);
}

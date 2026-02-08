//! State management operations for `planning/STATE.md`.
//!
//! Wraps the pure domain functions in `ito_domain::state` with filesystem I/O.

use crate::errors::{CoreError, CoreResult};
use std::path::Path;

/// Actions that can be performed on STATE.md.
pub enum StateAction {
    /// Record a decision with associated text.
    AddDecision {
        /// The decision text to record.
        text: String,
    },
    /// Record a blocker with associated text.
    AddBlocker {
        /// The blocker text to record.
        text: String,
    },
    /// Record a timestamped note with associated text.
    AddNote {
        /// The note text to record.
        text: String,
    },
    /// Update the current focus area.
    SetFocus {
        /// The focus text to set.
        text: String,
    },
    /// Record an open question.
    AddQuestion {
        /// The question text to record.
        text: String,
    },
}

/// Read the contents of `planning/STATE.md`.
pub fn read_state(ito_path: &Path) -> CoreResult<String> {
    let state_path = ito_path.join("planning").join("STATE.md");
    ito_common::io::read_to_string(&state_path)
        .map_err(|e| CoreError::io("reading STATE.md", std::io::Error::other(e)))
}

/// Apply a state action to `planning/STATE.md` and write it back.
pub fn update_state(ito_path: &Path, action: StateAction) -> CoreResult<()> {
    let state_path = ito_path.join("planning").join("STATE.md");
    let contents = ito_common::io::read_to_string(&state_path)
        .map_err(|e| CoreError::io("reading STATE.md", std::io::Error::other(e)))?;
    let date = crate::time::now_date();

    let updated = match action {
        StateAction::AddDecision { ref text } => {
            ito_domain::state::add_decision(&contents, &date, text)
        }
        StateAction::AddBlocker { ref text } => {
            ito_domain::state::add_blocker(&contents, &date, text)
        }
        StateAction::AddNote { ref text } => {
            let time = crate::time::now_time();
            ito_domain::state::add_note(&contents, &date, &time, text)
        }
        StateAction::SetFocus { ref text } => ito_domain::state::set_focus(&contents, &date, text),
        StateAction::AddQuestion { ref text } => {
            ito_domain::state::add_question(&contents, &date, text)
        }
    };

    let updated = updated.map_err(CoreError::validation)?;

    ito_common::io::write(&state_path, updated.as_bytes())
        .map_err(|e| CoreError::io("writing STATE.md", std::io::Error::other(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: write a file creating parent directories as needed.
    fn write_file(path: &std::path::Path, contents: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent dirs");
        }
        std::fs::write(path, contents).expect("write fixture");
    }

    /// Minimal STATE.md that has every section the domain functions expect.
    fn minimal_state_md(date: &str) -> String {
        format!(
            "# Project State\n\n\
             Last Updated: {date}\n\n\
             ## Current Focus\n\
             [placeholder]\n\n\
             ## Recent Decisions\n\
             - {date}: Project initialized\n\n\
             ## Open Questions\n\
             - [ ] placeholder\n\n\
             ## Blockers\n\
             [None currently]\n\n\
             ## Session Notes\n\
             ### {date} - Initial Setup\n\
             - Completed: init\n"
        )
    }

    #[test]
    fn read_state_returns_error_for_missing_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        // Do NOT create planning/STATE.md

        let result = read_state(ito_path);
        assert!(result.is_err(), "read_state should fail for missing file");
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("STATE.md"),
            "error should mention STATE.md, got: {msg}"
        );
    }

    #[test]
    fn read_state_returns_contents_for_existing_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        let contents = minimal_state_md("2025-01-01");
        write_file(&ito_path.join("planning").join("STATE.md"), &contents);

        let result = read_state(ito_path).expect("read_state should succeed");
        assert_eq!(result, contents);
    }

    #[test]
    fn update_state_add_decision() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        write_file(
            &ito_path.join("planning").join("STATE.md"),
            &minimal_state_md("2025-01-01"),
        );

        update_state(
            ito_path,
            StateAction::AddDecision {
                text: "Use Rust".to_string(),
            },
        )
        .expect("add decision should succeed");

        let updated =
            std::fs::read_to_string(ito_path.join("planning").join("STATE.md")).expect("read back");
        assert!(
            updated.contains("Use Rust"),
            "decision text should appear in STATE.md"
        );
    }

    #[test]
    fn update_state_add_blocker() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        write_file(
            &ito_path.join("planning").join("STATE.md"),
            &minimal_state_md("2025-01-01"),
        );

        update_state(
            ito_path,
            StateAction::AddBlocker {
                text: "Waiting on API access".to_string(),
            },
        )
        .expect("add blocker should succeed");

        let updated =
            std::fs::read_to_string(ito_path.join("planning").join("STATE.md")).expect("read back");
        assert!(
            updated.contains("Waiting on API access"),
            "blocker text should appear in STATE.md"
        );
        // The "[None currently]" placeholder should have been replaced
        assert!(
            !updated.contains("[None currently]"),
            "placeholder should be replaced"
        );
    }

    #[test]
    fn update_state_set_focus() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        write_file(
            &ito_path.join("planning").join("STATE.md"),
            &minimal_state_md("2025-01-01"),
        );

        update_state(
            ito_path,
            StateAction::SetFocus {
                text: "Implement auth module".to_string(),
            },
        )
        .expect("set focus should succeed");

        let updated =
            std::fs::read_to_string(ito_path.join("planning").join("STATE.md")).expect("read back");
        assert!(
            updated.contains("Implement auth module"),
            "focus text should appear in STATE.md"
        );
    }

    #[test]
    fn update_state_add_question() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        write_file(
            &ito_path.join("planning").join("STATE.md"),
            &minimal_state_md("2025-01-01"),
        );

        update_state(
            ito_path,
            StateAction::AddQuestion {
                text: "Should we use gRPC?".to_string(),
            },
        )
        .expect("add question should succeed");

        let updated =
            std::fs::read_to_string(ito_path.join("planning").join("STATE.md")).expect("read back");
        assert!(
            updated.contains("Should we use gRPC?"),
            "question text should appear in STATE.md"
        );
    }

    #[test]
    fn update_state_add_note() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        write_file(
            &ito_path.join("planning").join("STATE.md"),
            &minimal_state_md("2025-01-01"),
        );

        update_state(
            ito_path,
            StateAction::AddNote {
                text: "Reviewed the design".to_string(),
            },
        )
        .expect("add note should succeed");

        let updated =
            std::fs::read_to_string(ito_path.join("planning").join("STATE.md")).expect("read back");
        assert!(
            updated.contains("Reviewed the design"),
            "note text should appear in STATE.md"
        );
    }

    #[test]
    fn update_state_returns_error_for_missing_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path();
        // Do NOT create planning/STATE.md

        let result = update_state(
            ito_path,
            StateAction::AddDecision {
                text: "test".to_string(),
            },
        );
        assert!(result.is_err(), "update_state should fail for missing file");
    }
}

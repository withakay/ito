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

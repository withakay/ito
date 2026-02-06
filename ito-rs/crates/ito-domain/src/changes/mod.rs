//! Change domain models and repository.
//!
//! This module provides domain models for Ito changes and a repository
//! for loading and querying change data.

mod repository;

pub use repository::{ChangeRepository, ChangeTargetResolution};

use chrono::{DateTime, Utc};
use std::path::PathBuf;

use crate::tasks::{ProgressInfo, TasksParseResult};

/// A specification within a change.
#[derive(Debug, Clone)]
pub struct Spec {
    /// Spec name (directory name under specs/)
    pub name: String,
    /// Spec content (raw markdown)
    pub content: String,
}

/// Status of a change based on task completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeStatus {
    /// No tasks defined
    NoTasks,
    /// Some tasks incomplete
    InProgress,
    /// All tasks complete
    Complete,
}

/// Work status of a change.
///
/// This is a derived status intended for UX and filtering. It is NOT a persisted
/// lifecycle state.
///
/// Semantics:
/// - `Draft`: missing required planning artifacts (proposal + specs + tasks)
/// - `Ready`: planning artifacts exist and there is remaining work, with no in-progress tasks
/// - `InProgress`: at least one task is in-progress
/// - `Paused`: no remaining work, but at least one task is shelved (i.e. all tasks are done or shelved)
/// - `Complete`: all tasks are complete (shelved tasks do NOT count as complete)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeWorkStatus {
    /// Missing required planning artifacts (proposal + specs + tasks).
    Draft,
    /// Ready to start work (planning artifacts exist, remaining work, nothing in-progress).
    Ready,
    /// At least one task is in-progress.
    InProgress,
    /// No remaining work, but at least one task is shelved.
    ///
    /// This distinguishes "we're finished but chose to shelve something" from `Complete`.
    Paused,
    /// All tasks complete.
    ///
    /// Note: shelved tasks do NOT count as complete.
    Complete,
}

impl std::fmt::Display for ChangeWorkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeWorkStatus::Draft => write!(f, "draft"),
            ChangeWorkStatus::Ready => write!(f, "ready"),
            ChangeWorkStatus::InProgress => write!(f, "in-progress"),
            ChangeWorkStatus::Paused => write!(f, "paused"),
            ChangeWorkStatus::Complete => write!(f, "complete"),
        }
    }
}

impl std::fmt::Display for ChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeStatus::NoTasks => write!(f, "no-tasks"),
            ChangeStatus::InProgress => write!(f, "in-progress"),
            ChangeStatus::Complete => write!(f, "complete"),
        }
    }
}

/// Full change with all artifacts loaded.
#[derive(Debug, Clone)]
pub struct Change {
    /// Change identifier (e.g., "005-01_my-change")
    pub id: String,
    /// Module ID extracted from the change ID (e.g., "005")
    pub module_id: Option<String>,
    /// Path to the change directory
    pub path: PathBuf,
    /// Proposal content (raw markdown)
    pub proposal: Option<String>,
    /// Design content (raw markdown)
    pub design: Option<String>,
    /// Specifications
    pub specs: Vec<Spec>,
    /// Parsed tasks
    pub tasks: TasksParseResult,
    /// Last modification time of any artifact
    pub last_modified: DateTime<Utc>,
}

impl Change {
    /// Get the status of this change based on task completion.
    pub fn status(&self) -> ChangeStatus {
        let progress = &self.tasks.progress;
        if progress.total == 0 {
            ChangeStatus::NoTasks
        } else if progress.complete >= progress.total {
            ChangeStatus::Complete
        } else {
            ChangeStatus::InProgress
        }
    }

    /// Derived work status for UX and filtering.
    pub fn work_status(&self) -> ChangeWorkStatus {
        let ProgressInfo {
            total,
            complete,
            shelved,
            in_progress,
            pending,
            remaining: _,
        } = self.tasks.progress;

        // Planning artifacts required to start work.
        let has_planning_artifacts = self.proposal.is_some() && !self.specs.is_empty() && total > 0;
        if !has_planning_artifacts {
            return ChangeWorkStatus::Draft;
        }

        if complete == total {
            return ChangeWorkStatus::Complete;
        }
        if in_progress > 0 {
            return ChangeWorkStatus::InProgress;
        }

        let done_or_shelved = complete + shelved;
        if pending == 0 && shelved > 0 && done_or_shelved == total {
            return ChangeWorkStatus::Paused;
        }

        ChangeWorkStatus::Ready
    }

    /// Check if all required artifacts are present.
    pub fn artifacts_complete(&self) -> bool {
        self.proposal.is_some()
            && self.design.is_some()
            && !self.specs.is_empty()
            && self.tasks.progress.total > 0
    }

    /// Get task progress as (completed, total).
    pub fn task_progress(&self) -> (u32, u32) {
        (
            self.tasks.progress.complete as u32,
            self.tasks.progress.total as u32,
        )
    }

    /// Get the progress info for this change.
    pub fn progress(&self) -> &ProgressInfo {
        &self.tasks.progress
    }
}

/// Lightweight change summary for listings.
#[derive(Debug, Clone)]
pub struct ChangeSummary {
    /// Change identifier
    pub id: String,
    /// Module ID extracted from the change ID
    pub module_id: Option<String>,
    /// Number of completed tasks
    pub completed_tasks: u32,
    /// Number of shelved tasks (enhanced tasks only)
    pub shelved_tasks: u32,
    /// Number of in-progress tasks
    pub in_progress_tasks: u32,
    /// Number of pending tasks
    pub pending_tasks: u32,
    /// Total number of tasks
    pub total_tasks: u32,
    /// Last modification time
    pub last_modified: DateTime<Utc>,
    /// Whether proposal.md exists
    pub has_proposal: bool,
    /// Whether design.md exists
    pub has_design: bool,
    /// Whether specs/ directory has content
    pub has_specs: bool,
    /// Whether tasks.md exists and has tasks
    pub has_tasks: bool,
}

impl ChangeSummary {
    /// Get the status of this change based on task counts.
    pub fn status(&self) -> ChangeStatus {
        if self.total_tasks == 0 {
            ChangeStatus::NoTasks
        } else if self.completed_tasks >= self.total_tasks {
            ChangeStatus::Complete
        } else {
            ChangeStatus::InProgress
        }
    }

    /// Derived work status for UX and filtering.
    pub fn work_status(&self) -> ChangeWorkStatus {
        let has_planning_artifacts = self.has_proposal && self.has_specs && self.has_tasks;
        if !has_planning_artifacts {
            return ChangeWorkStatus::Draft;
        }

        if self.total_tasks > 0 && self.completed_tasks == self.total_tasks {
            return ChangeWorkStatus::Complete;
        }
        if self.in_progress_tasks > 0 {
            return ChangeWorkStatus::InProgress;
        }

        let done_or_shelved = self.completed_tasks + self.shelved_tasks;
        if self.pending_tasks == 0 && self.shelved_tasks > 0 && done_or_shelved == self.total_tasks
        {
            return ChangeWorkStatus::Paused;
        }

        ChangeWorkStatus::Ready
    }

    /// Check if this change is ready for implementation.
    ///
    /// A change is "ready" when it has all required planning artifacts and has remaining work
    /// with no in-progress tasks.
    pub fn is_ready(&self) -> bool {
        self.work_status() == ChangeWorkStatus::Ready
    }
}

/// Extract module ID from a change ID.
///
/// Change IDs follow the pattern `NNN-NN_name` where `NNN` is the module ID.
/// Handles various formats:
/// - `005-01_my-change` -> `005`
/// - `5-1_whatever` -> `005`
/// - `1-000002` -> `001`
pub fn extract_module_id(change_id: &str) -> Option<String> {
    let parts: Vec<&str> = change_id.split('-').collect();
    if parts.len() >= 2 {
        Some(normalize_id(parts[0], 3))
    } else {
        None
    }
}

/// Normalize an ID to a fixed width with zero-padding.
///
/// - `"5"` with width 3 -> `"005"`
/// - `"005"` with width 3 -> `"005"`
/// - `"0005"` with width 3 -> `"005"` (strips leading zeros beyond width)
pub fn normalize_id(id: &str, width: usize) -> String {
    // Parse as number to strip leading zeros, then reformat
    let num: u32 = id.parse().unwrap_or(0);
    format!("{:0>width$}", num, width = width)
}

/// Parse a change identifier and return the normalized module ID and change number.
///
/// Handles various formats:
/// - `005-01_my-change` -> `("005", "01")`
/// - `5-1_whatever` -> `("005", "01")`
/// - `1-2` -> `("001", "02")`
/// - `001-000002_foo` -> `("001", "02")`
pub fn parse_change_id(input: &str) -> Option<(String, String)> {
    // Remove the name suffix if present (everything after underscore)
    let id_part = input.split('_').next().unwrap_or(input);

    let parts: Vec<&str> = id_part.split('-').collect();
    if parts.len() >= 2 {
        let module_id = normalize_id(parts[0], 3);
        let change_num = normalize_id(parts[1], 2);
        Some((module_id, change_num))
    } else {
        None
    }
}

/// Parse a module identifier and return the normalized module ID.
///
/// Handles various formats:
/// - `005` -> `"005"`
/// - `5` -> `"005"`
/// - `005_dev-tooling` -> `"005"`
/// - `5_dev-tooling` -> `"005"`
pub fn parse_module_id(input: &str) -> String {
    // Remove the name suffix if present (everything after underscore)
    let id_part = input.split('_').next().unwrap_or(input);
    normalize_id(id_part, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_id() {
        assert_eq!(normalize_id("5", 3), "005");
        assert_eq!(normalize_id("05", 3), "005");
        assert_eq!(normalize_id("005", 3), "005");
        assert_eq!(normalize_id("0005", 3), "005");
        assert_eq!(normalize_id("1", 2), "01");
        assert_eq!(normalize_id("01", 2), "01");
        assert_eq!(normalize_id("001", 2), "01");
    }

    #[test]
    fn test_parse_change_id() {
        assert_eq!(
            parse_change_id("005-01_my-change"),
            Some(("005".to_string(), "01".to_string()))
        );
        assert_eq!(
            parse_change_id("5-1_whatever"),
            Some(("005".to_string(), "01".to_string()))
        );
        assert_eq!(
            parse_change_id("1-2"),
            Some(("001".to_string(), "02".to_string()))
        );
        assert_eq!(
            parse_change_id("001-000002_foo"),
            Some(("001".to_string(), "02".to_string()))
        );
        assert_eq!(parse_change_id("invalid"), None);
    }

    #[test]
    fn test_parse_module_id() {
        assert_eq!(parse_module_id("005"), "005");
        assert_eq!(parse_module_id("5"), "005");
        assert_eq!(parse_module_id("005_dev-tooling"), "005");
        assert_eq!(parse_module_id("5_dev-tooling"), "005");
    }

    #[test]
    fn test_extract_module_id() {
        assert_eq!(
            extract_module_id("005-01_my-change"),
            Some("005".to_string())
        );
        assert_eq!(extract_module_id("013-18_cleanup"), Some("013".to_string()));
        assert_eq!(extract_module_id("5-1_foo"), Some("005".to_string()));
        assert_eq!(extract_module_id("invalid"), None);
    }

    #[test]
    fn test_change_status_display() {
        assert_eq!(ChangeStatus::NoTasks.to_string(), "no-tasks");
        assert_eq!(ChangeStatus::InProgress.to_string(), "in-progress");
        assert_eq!(ChangeStatus::Complete.to_string(), "complete");
    }

    #[test]
    fn test_change_summary_status() {
        let mut summary = ChangeSummary {
            id: "test".to_string(),
            module_id: None,
            completed_tasks: 0,
            shelved_tasks: 0,
            in_progress_tasks: 0,
            pending_tasks: 0,
            total_tasks: 0,
            last_modified: Utc::now(),
            has_proposal: false,
            has_design: false,
            has_specs: false,
            has_tasks: false,
        };

        assert_eq!(summary.status(), ChangeStatus::NoTasks);

        summary.total_tasks = 5;
        summary.completed_tasks = 3;
        assert_eq!(summary.status(), ChangeStatus::InProgress);

        summary.completed_tasks = 5;
        assert_eq!(summary.status(), ChangeStatus::Complete);
    }

    #[test]
    fn test_change_work_status() {
        let mut summary = ChangeSummary {
            id: "test".to_string(),
            module_id: None,
            completed_tasks: 0,
            shelved_tasks: 0,
            in_progress_tasks: 0,
            pending_tasks: 0,
            total_tasks: 0,
            last_modified: Utc::now(),
            has_proposal: false,
            has_design: false,
            has_specs: false,
            has_tasks: false,
        };

        assert_eq!(summary.work_status(), ChangeWorkStatus::Draft);

        summary.has_proposal = true;
        summary.has_specs = true;
        summary.has_tasks = true;
        summary.total_tasks = 3;
        summary.pending_tasks = 3;

        assert_eq!(summary.work_status(), ChangeWorkStatus::Ready);

        summary.in_progress_tasks = 1;
        summary.pending_tasks = 2;
        assert_eq!(summary.work_status(), ChangeWorkStatus::InProgress);

        summary.in_progress_tasks = 0;
        summary.pending_tasks = 0;
        summary.shelved_tasks = 1;
        summary.completed_tasks = 2;
        assert_eq!(summary.work_status(), ChangeWorkStatus::Paused);

        summary.shelved_tasks = 0;
        summary.completed_tasks = 3;
        assert_eq!(summary.work_status(), ChangeWorkStatus::Complete);
    }
}

//! Task domain models, parsing, update helpers, and repository ports.
//!
//! This module implements the core logic for Ito's task tracking system, which supports
//! two formats:
//! - **Enhanced**: Structured markdown with waves, dependencies, and metadata.
//! - **Checkbox**: Legacy simple checklist format.
//!
//! Key components:
//! - [`TaskRepository`]: Port for accessing task data (implemented by core).
//! - [`parse_tasks_tracking_file`]: Normalizes markdown into [`TasksParseResult`].
//! - [`compute_ready_and_blocked`]: Determines which tasks are actionable based on waves/deps.
//! - [`update_enhanced_task_status`]: Modifies markdown content to reflect status changes.

mod checkbox;
#[cfg(test)]
mod checkbox_tests;
mod compute;
mod cycle;
mod parse;
mod relational;
mod repository;
mod update;

/// Compute ready vs blocked tasks for a parsed tracking file.
pub use compute::compute_ready_and_blocked;
/// Detect whether a `tasks.md` file is enhanced or checkbox format.
pub use parse::detect_tasks_format;
/// Generate the enhanced `tasks.md` template for a change.
pub use parse::enhanced_tasks_template;
/// Parse task tracking markdown into a normalized representation.
pub use parse::parse_tasks_tracking_file;
/// Build a tasks path with fallback behavior for invalid ids.
pub use parse::tasks_path;
/// Build a tasks path only when the change id is a safe path segment.
pub use parse::tasks_path_checked;
/// Repository port for loading and querying task data.
pub use repository::TaskRepository;
/// Update checkbox-format task status markers.
pub use update::update_checkbox_task_status;
/// Update enhanced-format task status and metadata.
pub use update::update_enhanced_task_status;

/// Parsed task tracking result.
pub use parse::TasksParseResult;
/// Parsed wave metadata from enhanced format.
pub use parse::WaveInfo;
/// Common task domain types.
pub use parse::{
    DiagnosticLevel, ProgressInfo, TaskDiagnostic, TaskItem, TaskKind, TaskStatus, TasksFormat,
};

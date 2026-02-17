//! Parsing for Ito `tasks.md` tracking files.
//!
//! Ito supports two tasks formats:
//! - a legacy checkbox list (minimal structure)
//! - an enhanced format with waves, explicit dependencies, and status metadata
//!
//! This module parses either format into a single normalized representation
//! ([`TasksParseResult`]) used by the tasks CLI and workflow execution.

use chrono::{DateTime, Local, NaiveDate};
use regex::Regex;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use super::checkbox::split_checkbox_task_label;

static ENHANCED_HEADING_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^###\s+(Task\s+)?[^:]+:\s+.+$").unwrap());

static CHECKBOX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*[-*]\s+\[[ xX~>]\]").unwrap());

static WAVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Accept a strict `## Wave <N>` heading, optionally followed by a human title.
    // Supported separators after the wave number:
    // - `:` (e.g. `## Wave 1: Foundations`)
    // - `-` (e.g. `## Wave 1 - Foundations`)
    Regex::new(r"^##\s+Wave\s+(\d+)(?:\s*[:-]\s*.*)?\s*$").unwrap()
});

static WAVE_DEP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*[-*]\s+\*\*Depends On\*\*:\s*(.+?)\s*$").unwrap());

static TASK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^###\s+(?:Task\s+)?([^:]+):\s+(.+?)\s*$").unwrap());

static DEPS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Dependencies\*\*:\s*(.+?)\s*$").unwrap());

static STATUS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Status\*\*:\s*\[([ xX\-~])\]\s+(pending|in-progress|complete|shelved)\s*$")
        .unwrap()
});

static UPDATED_AT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Updated At\*\*:\s*(\d{4}-\d{2}-\d{2})\s*$").unwrap());

static FILES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Files\*\*:\s*`([^`]+)`\s*$").unwrap());

static VERIFY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Verify\*\*:\s*`([^`]+)`\s*$").unwrap());

static DONE_WHEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Done When\*\*:\s*(.+?)\s*$").unwrap());

static ALL_WAVE_CAPTURE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^all\s+wave\s+(\d+)\s+tasks$").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The detected format of a `tasks.md` file.
pub enum TasksFormat {
    /// Enhanced wave-based format.
    Enhanced,
    /// Legacy checkbox list format.
    Checkbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Status values supported by Ito task tracking.
pub enum TaskStatus {
    /// Not started.
    Pending,
    /// Currently being worked.
    InProgress,
    /// Finished.
    Complete,
    /// Intentionally deferred/paused.
    Shelved,
}

impl TaskStatus {
    /// Status label used by the enhanced tasks format.
    pub fn as_enhanced_label(self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in-progress",
            TaskStatus::Complete => "complete",
            TaskStatus::Shelved => "shelved",
        }
    }

    /// Parse an enhanced-format status label.
    pub fn from_enhanced_label(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(TaskStatus::Pending),
            "in-progress" => Some(TaskStatus::InProgress),
            "complete" => Some(TaskStatus::Complete),
            "shelved" => Some(TaskStatus::Shelved),
            _ => None,
        }
    }

    /// Return true when the status counts as "done" for gating.
    pub fn is_done(self) -> bool {
        match self {
            TaskStatus::Pending => false,
            TaskStatus::InProgress => false,
            TaskStatus::Complete => true,
            TaskStatus::Shelved => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A parse-time diagnostic emitted while reading a tasks file.
pub struct TaskDiagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Human-readable message.
    pub message: String,
    /// Optional task id the diagnostic refers to.
    pub task_id: Option<String>,
    /// Optional 0-based line index.
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Diagnostic severity.
pub enum DiagnosticLevel {
    /// The file is malformed and results may be incomplete.
    Error,
    /// The file is parseable but contains suspicious content.
    Warning,
}

impl DiagnosticLevel {
    /// Render as a stable string label.
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A normalized task entry parsed from a tasks tracking file.
pub struct TaskItem {
    /// Task identifier (e.g. `1.1`).
    pub id: String,
    /// Task title/name.
    pub name: String,
    /// Optional wave number (enhanced format).
    pub wave: Option<u32>,
    /// Current status.
    pub status: TaskStatus,
    /// Optional `YYYY-MM-DD` updated date.
    pub updated_at: Option<String>,
    /// Explicit task dependencies by id.
    pub dependencies: Vec<String>,
    /// File paths mentioned for the task.
    pub files: Vec<String>,
    /// Freeform action description.
    pub action: String,
    /// Optional verification command.
    pub verify: Option<String>,
    /// Optional completion criteria.
    pub done_when: Option<String>,
    /// Task kind (normal vs checkpoint).
    pub kind: TaskKind,
    /// 0-based line index where the task header was found.
    pub header_line_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Task classification.
pub enum TaskKind {
    #[default]
    /// A runnable task.
    Normal,
    /// A checkpoint that requires explicit approval.
    Checkpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Summary counts computed from the parsed tasks.
pub struct ProgressInfo {
    /// Total tasks.
    pub total: usize,
    /// Completed tasks.
    pub complete: usize,
    /// Shelved tasks.
    pub shelved: usize,
    /// In-progress tasks.
    pub in_progress: usize,
    /// Pending tasks.
    pub pending: usize,
    /// Remaining work (`total - complete - shelved`).
    pub remaining: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Wave metadata parsed from enhanced tasks files.
pub struct WaveInfo {
    /// Wave number.
    pub wave: u32,
    /// Other waves that must be complete before this wave is unlocked.
    pub depends_on: Vec<u32>,
    /// 0-based line index for the wave heading.
    pub header_line_index: usize,
    /// 0-based line index for the depends-on line, when present.
    pub depends_on_line_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Output of parsing a `tasks.md` file.
pub struct TasksParseResult {
    /// Detected file format.
    pub format: TasksFormat,
    /// Parsed tasks in source order.
    pub tasks: Vec<TaskItem>,
    /// Parsed wave declarations.
    pub waves: Vec<WaveInfo>,
    /// Parse diagnostics.
    pub diagnostics: Vec<TaskDiagnostic>,
    /// Aggregate progress counts.
    pub progress: ProgressInfo,
}

impl TasksParseResult {
    /// Create an empty result (for when no tasks file exists).
    pub fn empty() -> Self {
        Self {
            format: TasksFormat::Checkbox,
            tasks: Vec::new(),
            waves: Vec::new(),
            diagnostics: Vec::new(),
            progress: ProgressInfo {
                total: 0,
                complete: 0,
                shelved: 0,
                in_progress: 0,
                pending: 0,
                remaining: 0,
            },
        }
    }
}

/// Generate the default template for an enhanced-format `tasks.md`.
///
/// The template includes:
/// - A header with execution notes and CLI hints.
/// - A sample Wave 1 with a placeholder task.
/// - A sample Checkpoint for review.
pub fn enhanced_tasks_template(change_id: &str, now: DateTime<Local>) -> String {
    let date = now.format("%Y-%m-%d").to_string();
    format!(
        "# Tasks for: {change_id}\n\n## Execution Notes\n\n- **Tool**: Any (OpenCode, Codex, Claude Code)\n- **Mode**: Sequential (or parallel if tool supports)\n- **Template**: Enhanced task format with waves, verification, and status tracking\n- **Tracking**: Prefer the tasks CLI to drive status updates and pick work\n\n```bash\nito tasks status {change_id}\nito tasks next {change_id}\nito tasks start {change_id} 1.1\nito tasks complete {change_id} 1.1\nito tasks shelve {change_id} 1.1\nito tasks unshelve {change_id} 1.1\nito tasks show {change_id}\n```\n\n______________________________________________________________________\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: [Task Name]\n\n- **Files**: `path/to/file.rs`\n- **Dependencies**: None\n- **Action**:\n  [Describe what needs to be done]\n- **Verify**: `cargo test --workspace`\n- **Done When**: [Success criteria]\n- **Updated At**: {date}\n- **Status**: [ ] pending\n\n______________________________________________________________________\n\n## Checkpoints\n\n### Checkpoint: Review Implementation\n\n- **Type**: checkpoint (requires human approval)\n- **Dependencies**: All Wave 1 tasks\n- **Action**: Review the implementation before proceeding\n- **Done When**: User confirms implementation is correct\n- **Updated At**: {date}\n- **Status**: [ ] pending\n"
    )
}

/// Detects whether task tracking contents use the enhanced wave-based format or the legacy checkbox format.
///
/// The enhanced format is recognized by an enhanced-style heading plus a **Status** section; otherwise the legacy checkbox format is assumed.
///
/// # Examples
///
/// ```
/// use crate::TasksFormat;
/// let enhanced = "Task: Example\n- **Status**: Pending\n";
/// assert_eq!(detect_tasks_format(enhanced), TasksFormat::Enhanced);
///
/// let checkbox = "- [ ] Task 1";
/// assert_eq!(detect_tasks_format(checkbox), TasksFormat::Checkbox);
/// ```
pub fn detect_tasks_format(contents: &str) -> TasksFormat {
    let enhanced_heading = &*ENHANCED_HEADING_RE;
    let has_status = contents.contains("- **Status**:");
    if enhanced_heading.is_match(contents) && has_status {
        return TasksFormat::Enhanced;
    }
    let checkbox = &*CHECKBOX_RE;
    if checkbox.is_match(contents) {
        return TasksFormat::Checkbox;
    }
    TasksFormat::Checkbox
}

/// Parse a `tasks.md` tracking file into a normalized representation.
pub fn parse_tasks_tracking_file(contents: &str) -> TasksParseResult {
    match detect_tasks_format(contents) {
        TasksFormat::Enhanced => parse_enhanced_tasks(contents),
        TasksFormat::Checkbox => parse_checkbox_tasks(contents),
    }
}

/// Parses a legacy checkbox-style tasks.md into a normalized TasksParseResult.
///
/// This recognizes list items that start with `- ` or `* ` followed by a status checkbox
/// (`[ ]`, `[x]`, `[~]`, `[>]`) and an optional label of the form `ID: Name`. Each matched
/// line produces a TaskItem with a sequential numeric id if no explicit id is present.
/// The returned result uses the Checkbox format, contains no wave metadata, and includes
/// computed progress information.
///
/// # Returns
///
/// A TasksParseResult containing all parsed TaskItem entries, no waves, no diagnostics,
/// and computed ProgressInfo.
///
/// # Examples
///
/// ```
/// let contents = "- [ ] 1: First task\n- [x] Second task\n";
/// let result = parse_checkbox_tasks(contents);
/// assert_eq!(result.format, TasksFormat::Checkbox);
/// assert_eq!(result.tasks.len(), 2);
/// assert_eq!(result.tasks[0].id, "1");
/// assert_eq!(result.tasks[1].id, "2");
/// ```
fn parse_checkbox_tasks(contents: &str) -> TasksParseResult {
    // Minimal compat: tasks are numbered 1..N.
    let mut tasks: Vec<TaskItem> = Vec::new();
    for (line_idx, line) in contents.lines().enumerate() {
        let l = line.trim_start();
        let bytes = l.as_bytes();
        if bytes.len() < 5 {
            continue;
        }
        let bullet = bytes[0] as char;
        if bullet != '-' && bullet != '*' {
            continue;
        }
        if bytes[1] != b' ' || bytes[2] != b'[' || bytes[4] != b']' {
            continue;
        }
        let marker = bytes[3] as char;
        let status = match marker {
            'x' | 'X' => TaskStatus::Complete,
            ' ' => TaskStatus::Pending,
            '~' | '>' => TaskStatus::InProgress,
            _other => continue,
        };

        let rest_start = if let Some(b' ') = bytes.get(5) { 6 } else { 5 };
        let rest = &l[rest_start..];
        let rest = rest.trim();

        let (id, name) = match split_checkbox_task_label(rest) {
            Some((id, name)) => (id.to_string(), name.to_string()),
            None => ((tasks.len() + 1).to_string(), rest.to_string()),
        };
        tasks.push(TaskItem {
            id,
            name,
            wave: None,
            status,
            updated_at: None,
            dependencies: Vec::new(),
            files: Vec::new(),
            action: String::new(),
            verify: None,
            done_when: None,
            kind: TaskKind::Normal,
            header_line_index: line_idx,
        });
    }
    let progress = compute_progress(&tasks);
    TasksParseResult {
        format: TasksFormat::Checkbox,
        tasks,
        waves: Vec::new(),
        diagnostics: Vec::new(),
        progress,
    }
}

/// Parses a tasks.md document written in the enhanced (wave-based) format into a normalized TasksParseResult.
///
/// The parser extracts waves, per-task metadata (id, name, wave, status, updated date, dependencies, files, action, verify, and done-when),
/// validates fields (emitting diagnostics for missing/invalid status or dates, duplicate or missing wave dependency lines, self-dependencies, and missing referenced waves),
/// and computes overall progress information.
///
/// # Returns
///
/// A `TasksParseResult` containing the detected `TasksFormat::Enhanced`, the parsed `tasks`, `waves`, any `diagnostics` produced during parsing, and computed `progress`.
///
/// # Examples
///
/// ```
/// let src = r#"
/// ## Wave 1
/// - Depends On: none
///
/// ### 1: Example task
/// - **Dependencies**: none
/// - **Updated At**: 2025-01-01
/// - **Status**: [ ] pending
/// - **Action**:
///   Do something
/// "#;
///
/// let res = parse_enhanced_tasks(src);
/// assert_eq!(res.format, TasksFormat::Enhanced);
/// assert_eq!(res.tasks.len(), 1);
/// assert_eq!(res.waves.len(), 1);
/// ```
fn parse_enhanced_tasks(contents: &str) -> TasksParseResult {
    let mut diagnostics: Vec<TaskDiagnostic> = Vec::new();
    let mut tasks: Vec<TaskItem> = Vec::new();

    let wave_re = &*WAVE_RE;
    let wave_dep_re = &*WAVE_DEP_RE;
    let task_re = &*TASK_RE;
    let deps_re = &*DEPS_RE;
    let status_re = &*STATUS_RE;
    let updated_at_re = &*UPDATED_AT_RE;
    let files_re = &*FILES_RE;
    let verify_re = &*VERIFY_RE;
    let done_when_re = &*DONE_WHEN_RE;

    let mut current_wave: Option<u32> = None;
    let mut in_checkpoints = false;

    #[derive(Debug, Default, Clone)]
    struct WaveBuilder {
        header_line_index: usize,
        depends_on_raw: Option<String>,
        depends_on_line_index: Option<usize>,
    }

    let mut waves: BTreeMap<u32, WaveBuilder> = BTreeMap::new();

    #[derive(Debug, Default)]
    struct CurrentTask {
        id: Option<String>,
        desc: Option<String>,
        wave: Option<u32>,
        header_line_index: usize,
        kind: TaskKind,
        deps_raw: Option<String>,
        updated_at_raw: Option<String>,
        status_raw: Option<String>,
        status_marker_raw: Option<char>,
        files: Vec<String>,
        action_lines: Vec<String>,
        verify: Option<String>,
        done_when: Option<String>,
    }

    fn flush_current(
        current: &mut CurrentTask,
        tasks: &mut Vec<TaskItem>,
        diagnostics: &mut Vec<TaskDiagnostic>,
    ) {
        let Some(id) = current.id.take() else {
            current.desc = None;
            current.deps_raw = None;
            current.updated_at_raw = None;
            current.status_raw = None;
            current.kind = TaskKind::Normal;
            return;
        };
        let desc = current.desc.take().unwrap_or_default();
        let wave = current.wave.take();
        let header_line_index = current.header_line_index;
        let deps_raw = current.deps_raw.take().unwrap_or_default();
        let updated_at_raw = current.updated_at_raw.take();
        let status_raw = current.status_raw.take();
        let status_marker_raw = current.status_marker_raw.take();
        let files = std::mem::take(&mut current.files);
        let action = std::mem::take(&mut current.action_lines)
            .join("\n")
            .trim()
            .to_string();
        let verify = current.verify.take();
        let done_when = current.done_when.take();

        let status = match status_raw
            .as_deref()
            .and_then(TaskStatus::from_enhanced_label)
        {
            Some(s) => s,
            None => {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: "Invalid or missing status".to_string(),
                    task_id: Some(id.clone()),
                    line: Some(header_line_index + 1),
                });
                TaskStatus::Pending
            }
        };

        // Validate marker conventions to make manual edits harder to corrupt.
        // We treat `[x] complete` as the only marker with semantic meaning and keep the others
        // as formatting conventions.
        if let Some(marker) = status_marker_raw {
            match status {
                TaskStatus::Complete => {
                    if marker != 'x' && marker != 'X' {
                        diagnostics.push(TaskDiagnostic {
                            level: DiagnosticLevel::Warning,
                            message: "Status marker for complete should be [x]".to_string(),
                            task_id: Some(id.clone()),
                            line: Some(header_line_index + 1),
                        });
                    }
                }
                TaskStatus::Shelved => {
                    if marker != '-' && marker != '~' {
                        diagnostics.push(TaskDiagnostic {
                            level: DiagnosticLevel::Warning,
                            message: "Status marker for shelved should be [-]".to_string(),
                            task_id: Some(id.clone()),
                            line: Some(header_line_index + 1),
                        });
                    }
                }
                TaskStatus::Pending | TaskStatus::InProgress => {
                    if marker == 'x' || marker == 'X' {
                        diagnostics.push(TaskDiagnostic {
                            level: DiagnosticLevel::Warning,
                            message: "Only complete tasks should use [x]".to_string(),
                            task_id: Some(id.clone()),
                            line: Some(header_line_index + 1),
                        });
                    }
                }
            }
        }
        let deps = parse_dependencies(&deps_raw);

        let updated_at = match updated_at_raw.as_deref() {
            Some(s) => {
                if NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok() {
                    Some(s.to_string())
                } else {
                    diagnostics.push(TaskDiagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("Invalid Updated At date: {s} (expected YYYY-MM-DD)"),
                        task_id: Some(id.clone()),
                        line: Some(header_line_index + 1),
                    });
                    None
                }
            }
            None => {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: "Missing Updated At field (expected YYYY-MM-DD)".to_string(),
                    task_id: Some(id.clone()),
                    line: Some(header_line_index + 1),
                });
                None
            }
        };

        tasks.push(TaskItem {
            id,
            name: desc,
            wave,
            status,
            updated_at,
            dependencies: deps,
            files,
            action,
            verify,
            done_when,
            kind: current.kind,
            header_line_index,
        });
        current.kind = TaskKind::Normal;
    }

    let mut current_task = CurrentTask {
        id: None,
        desc: None,
        wave: None,
        header_line_index: 0,
        kind: TaskKind::Normal,
        deps_raw: None,
        updated_at_raw: None,
        status_raw: None,
        status_marker_raw: None,
        files: Vec::new(),
        action_lines: Vec::new(),
        verify: None,
        done_when: None,
    };

    let mut in_action = false;

    for (line_idx, line) in contents.lines().enumerate() {
        if in_action && current_task.id.is_some() {
            if line.starts_with("- **") || line.starts_with("### ") || line.starts_with("## ") {
                in_action = false;
                // fall through to process this line normally
            } else {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    current_task.action_lines.push(trimmed.to_string());
                }
                continue;
            }
        }

        if let Some(cap) = wave_re.captures(line) {
            flush_current(&mut current_task, &mut tasks, &mut diagnostics);
            current_wave = cap.get(1).and_then(|m| m.as_str().parse::<u32>().ok());
            in_checkpoints = false;
            if let Some(w) = current_wave {
                waves.entry(w).or_insert_with(|| WaveBuilder {
                    header_line_index: line_idx,
                    depends_on_raw: None,
                    depends_on_line_index: None,
                });
            }
            continue;
        }
        if line.trim() == "## Checkpoints" {
            flush_current(&mut current_task, &mut tasks, &mut diagnostics);
            current_wave = None;
            in_checkpoints = true;
            continue;
        }

        if current_task.id.is_none()
            && let Some(w) = current_wave
            && let Some(cap) = wave_dep_re.captures(line)
        {
            let raw = cap[1].trim().to_string();
            let entry = waves.entry(w).or_insert_with(|| WaveBuilder {
                header_line_index: line_idx,
                depends_on_raw: None,
                depends_on_line_index: None,
            });
            if entry.depends_on_raw.is_some() {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Warning,
                    message: format!("Wave {w}: duplicate Depends On line; using the first one"),
                    task_id: None,
                    line: Some(line_idx + 1),
                });
            } else {
                entry.depends_on_raw = Some(raw);
                entry.depends_on_line_index = Some(line_idx);
            }
            continue;
        }

        if let Some(cap) = task_re.captures(line) {
            flush_current(&mut current_task, &mut tasks, &mut diagnostics);
            let id = cap[1].trim().to_string();
            let desc = cap[2].trim().to_string();
            current_task.id = Some(id.clone());
            current_task.desc = Some(desc);
            current_task.wave = current_wave;
            current_task.header_line_index = line_idx;
            current_task.kind = TaskKind::Normal;
            current_task.deps_raw = None;
            current_task.updated_at_raw = None;
            current_task.status_raw = None;
            current_task.status_marker_raw = None;
            current_task.files.clear();
            current_task.action_lines.clear();
            current_task.verify = None;
            current_task.done_when = None;
            in_action = false;

            if current_wave.is_none() && !in_checkpoints {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Warning,
                    message: format!(
                        "{id}: Task '{id}' appears outside any Wave section; wave gating may not behave as expected"
                    ),
                    task_id: None,
                    line: Some(line_idx + 1),
                });
            }
            continue;
        }

        if current_task.id.is_some() {
            if line.trim() == "- **Action**:" {
                in_action = true;
                current_task.action_lines.clear();
                continue;
            }
            if let Some(cap) = deps_re.captures(line) {
                current_task.deps_raw = Some(cap[1].trim().to_string());
                continue;
            }
            if let Some(cap) = updated_at_re.captures(line) {
                current_task.updated_at_raw = Some(cap[1].trim().to_string());
                continue;
            }
            if let Some(cap) = status_re.captures(line) {
                let marker = cap
                    .get(1)
                    .and_then(|m| m.as_str().chars().next())
                    .unwrap_or(' ');
                current_task.status_marker_raw = Some(marker);
                current_task.status_raw = Some(cap[2].trim().to_string());
                continue;
            }
            if let Some(cap) = files_re.captures(line) {
                let inner = cap[1].trim();
                current_task.files = inner
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                continue;
            }
            if let Some(cap) = verify_re.captures(line) {
                current_task.verify = Some(cap[1].trim().to_string());
                continue;
            }
            if let Some(cap) = done_when_re.captures(line) {
                current_task.done_when = Some(cap[1].trim().to_string());
                continue;
            }
        }
    }

    flush_current(&mut current_task, &mut tasks, &mut diagnostics);

    // Build wave dependency model.
    let mut wave_nums: Vec<u32> = waves.keys().copied().collect();
    wave_nums.sort();
    wave_nums.dedup();
    let wave_set: std::collections::BTreeSet<u32> = wave_nums.iter().copied().collect();

    let mut waves_out: Vec<WaveInfo> = Vec::new();
    for w in &wave_nums {
        let builder = waves.get(w).cloned().unwrap_or_default();

        let mut depends_on: Vec<u32> = Vec::new();
        if let Some(raw) = builder.depends_on_raw.as_deref() {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Wave {w}: Depends On is empty"),
                    task_id: None,
                    line: Some(builder.header_line_index + 1),
                });
            } else if trimmed.eq_ignore_ascii_case("none") {
                // no deps
            } else {
                for part in trimmed.split(',') {
                    let p = part.trim();
                    if p.is_empty() {
                        continue;
                    }
                    let p2 = if p.to_ascii_lowercase().starts_with("wave ") {
                        p[5..].trim()
                    } else {
                        p
                    };
                    match p2.parse::<u32>() {
                        Ok(n) => depends_on.push(n),
                        Err(_) => diagnostics.push(TaskDiagnostic {
                            level: DiagnosticLevel::Error,
                            message: format!("Wave {w}: invalid Depends On entry '{p}'"),
                            task_id: None,
                            line: Some(
                                builder
                                    .depends_on_line_index
                                    .unwrap_or(builder.header_line_index)
                                    + 1,
                            ),
                        }),
                    }
                }
            }
        } else {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Wave {w}: missing Depends On line"),
                task_id: None,
                line: Some(builder.header_line_index + 1),
            });

            // Preserve behavior for readiness calculations, but refuse to operate due to error.
            depends_on = wave_nums.iter().copied().filter(|n| *n < *w).collect();
        }

        depends_on.sort();
        depends_on.dedup();

        for dep_wave in &depends_on {
            if dep_wave == w {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Wave {w}: cannot depend on itself"),
                    task_id: None,
                    line: Some(
                        builder
                            .depends_on_line_index
                            .unwrap_or(builder.header_line_index)
                            + 1,
                    ),
                });
                continue;
            }
            if !wave_set.contains(dep_wave) {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Wave {w}: depends on missing Wave {dep_wave}"),
                    task_id: None,
                    line: Some(
                        builder
                            .depends_on_line_index
                            .unwrap_or(builder.header_line_index)
                            + 1,
                    ),
                });
            }
        }

        waves_out.push(WaveInfo {
            wave: *w,
            depends_on,
            header_line_index: builder.header_line_index,
            depends_on_line_index: builder.depends_on_line_index,
        });
    }

    // Relational invariants (cycles, task deps rules) on the finalized model.
    diagnostics.extend(super::relational::validate_relational(&tasks, &waves_out));

    let progress = compute_progress(&tasks);

    TasksParseResult {
        format: TasksFormat::Enhanced,
        tasks,
        waves: waves_out,
        diagnostics,
        progress,
    }
}

fn parse_dependencies(raw: &str) -> Vec<String> {
    parse_dependencies_with_checkpoint(raw, TaskKind::Normal).0
}

/// Parses a dependency string and returns explicit task IDs and an optional checkpoint wave.
///
/// This accepts comma-separated dependency lists (optionally prefixed with "Task "), trims whitespace,
/// and treats the following special cases:
/// - empty or "none" yields no dependencies and no wave;
/// - "all previous waves" or "all prior tasks" yields no explicit dependencies and no wave;
/// - strings matching "all wave N tasks" capture N and return it as `Some(N)` only when `kind` is `Checkpoint`.
///
/// The returned tuple is (dependencies, checkpoint_wave). `dependencies` contains parsed task identifiers
/// (without the "Task " prefix). `checkpoint_wave` is `Some(wave)` only for the captured "all wave N tasks"
/// case when `kind` is `Checkpoint`.
///
/// # Examples
///
/// ```
/// use crate::TaskKind;
///
/// assert_eq!(
///     parse_dependencies_with_checkpoint("Task 1, Task 2", TaskKind::Normal),
///     (vec!["1".to_string(), "2".to_string()], None)
/// );
///
/// assert_eq!(
///     parse_dependencies_with_checkpoint("none", TaskKind::Normal),
///     (Vec::<String>::new(), None)
/// );
///
/// assert_eq!(
///     parse_dependencies_with_checkpoint("All wave 3 tasks", TaskKind::Checkpoint),
///     (Vec::<String>::new(), Some(3))
/// );
/// ```
fn parse_dependencies_with_checkpoint(raw: &str, kind: TaskKind) -> (Vec<String>, Option<u32>) {
    let r = raw.trim();
    if r.is_empty() {
        return (Vec::new(), None);
    }
    let lower = r.to_ascii_lowercase();
    if lower == "none" {
        return (Vec::new(), None);
    }

    // Special-case strings from the enhanced template.
    let all_wave_capture = &*ALL_WAVE_CAPTURE_RE;
    if let Some(cap) = all_wave_capture.captures(r) {
        let wave = cap.get(1).and_then(|m| m.as_str().parse::<u32>().ok());
        if kind == TaskKind::Checkpoint {
            return (Vec::new(), wave);
        }
        return (Vec::new(), None);
    }
    if lower == "all previous waves" || lower == "all prior tasks" {
        // We don't expand this into explicit deps here.
        return (Vec::new(), None);
    }

    let deps = r
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.strip_prefix("Task ").unwrap_or(s).trim().to_string())
        .collect();
    (deps, None)
}

fn compute_progress(tasks: &[TaskItem]) -> ProgressInfo {
    let total = tasks.len();
    let complete = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Complete)
        .count();
    let shelved = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Shelved)
        .count();
    let in_progress = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::InProgress)
        .count();
    let pending = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Pending)
        .count();
    let done = tasks.iter().filter(|t| t.status.is_done()).count();
    let remaining = total.saturating_sub(done);
    ProgressInfo {
        total,
        complete,
        shelved,
        in_progress,
        pending,
        remaining,
    }
}

/// Path to `{ito_path}/changes/{change_id}/tasks.md`.
pub fn tasks_path(ito_path: &Path, change_id: &str) -> PathBuf {
    let Some(path) = tasks_path_checked(ito_path, change_id) else {
        return ito_path
            .join("changes")
            .join("invalid-change-id")
            .join("tasks.md");
    };
    path
}

/// Path to `{ito_path}/changes/{change_id}/tasks.md` when `change_id` is safe.
///
/// This rejects path traversal tokens, path separators, empty ids, and overlong
/// ids to ensure the resulting path cannot escape the intended `changes/`
/// subtree.
pub fn tasks_path_checked(ito_path: &Path, change_id: &str) -> Option<PathBuf> {
    if !is_safe_change_id_segment(change_id) {
        return None;
    }

    Some(ito_path.join("changes").join(change_id).join("tasks.md"))
}

/// Return `true` when `change_id` is safe as a single path segment.
pub fn is_safe_change_id_segment(change_id: &str) -> bool {
    let change_id = change_id.trim();
    if change_id.is_empty() {
        return false;
    }
    if change_id.len() > 256 {
        return false;
    }
    if change_id.contains('/') || change_id.contains('\\') || change_id.contains("..") {
        return false;
    }
    true
}

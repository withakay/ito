//! Schema-driven change workflow helpers.
//!
//! This module reads a change directory and a workflow schema (`schema.yaml`) and
//! produces JSON-friendly status and instruction payloads.
//!
//! These types are designed for use by the CLI and by any web/API layer that
//! wants to present "what should I do next?" without duplicating the filesystem
//! logic.

use ito_templates::ITO_END_MARKER;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use ito_common::fs::StdFs;
use ito_common::paths;
use ito_config::ConfigContext;

#[derive(Debug, thiserror::Error)]
/// Errors returned by workflow helpers.
pub enum WorkflowError {
    #[error("Invalid change name")]
    /// Change name failed basic sanitization.
    InvalidChangeName,

    #[error("Missing required option --change")]
    /// The caller did not provide a required change id.
    MissingChange,

    #[error("Change '{0}' not found")]
    /// The requested change directory does not exist.
    ChangeNotFound(String),

    #[error("Schema '{0}' not found")]
    /// The requested schema name did not resolve to a schema directory.
    SchemaNotFound(String),

    #[error("Artifact '{0}' not found")]
    /// The requested artifact id does not exist in the resolved schema.
    ArtifactNotFound(String),

    #[error(transparent)]
    /// IO error while reading or writing workflow files.
    Io(#[from] std::io::Error),

    #[error(transparent)]
    /// YAML parsing error.
    Yaml(#[from] serde_yaml::Error),
}

#[derive(Debug, Clone, Serialize)]
/// Status for one schema artifact for a given change directory.
pub struct ArtifactStatus {
    /// Artifact id from the schema.
    pub id: String,
    #[serde(rename = "outputPath")]
    /// Path (relative to the change directory) the artifact should generate.
    pub output_path: String,

    /// Computed state: `done`, `ready`, or `blocked`.
    pub status: String,
    #[serde(rename = "missingDeps", skip_serializing_if = "Vec::is_empty")]
    /// Artifact ids that are required but not yet complete.
    pub missing_deps: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
/// High-level status for a change against a resolved schema.
pub struct ChangeStatus {
    #[serde(rename = "changeName")]
    /// Change directory name.
    pub change_name: String,
    #[serde(rename = "schemaName")]
    /// Resolved schema name.
    pub schema_name: String,
    #[serde(rename = "isComplete")]
    /// Whether all schema artifacts are complete.
    pub is_complete: bool,
    #[serde(rename = "applyRequires")]
    /// Artifacts required before "apply" is allowed.
    pub apply_requires: Vec<String>,

    /// Per-artifact status entries.
    pub artifacts: Vec<ArtifactStatus>,
}

#[derive(Debug, Clone, Serialize)]
/// Information about an artifact template resolved from a schema.
pub struct TemplateInfo {
    /// Template source (`package` or `user`).
    pub source: String,
    /// Full path to the template file.
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
/// One dependency entry shown alongside artifact instructions.
pub struct DependencyInfo {
    /// Dependency artifact id.
    pub id: String,
    /// Whether the dependency is complete.
    pub done: bool,
    /// Dependency output path.
    pub path: String,
    /// Optional schema description for the dependency.
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
/// Instruction payload for building a single artifact.
pub struct InstructionsResponse {
    #[serde(rename = "changeName")]
    /// Change directory name.
    pub change_name: String,
    #[serde(rename = "artifactId")]
    /// Artifact id.
    pub artifact_id: String,
    #[serde(rename = "schemaName")]
    /// Schema name.
    pub schema_name: String,
    #[serde(rename = "changeDir")]
    /// Full path to the change directory.
    pub change_dir: String,
    #[serde(rename = "outputPath")]
    /// Artifact output path (relative to the change directory).
    pub output_path: String,

    /// Human-readable artifact description.
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional per-artifact instruction text.
    pub instruction: Option<String>,

    /// Template contents used to generate the artifact.
    pub template: String,

    /// Dependency details shown to the caller.
    pub dependencies: Vec<DependencyInfo>,

    /// Artifact ids that become unblocked once this artifact is complete.
    pub unlocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
/// One task parsed from a tracking file (e.g. `tasks.md`).
pub struct TaskItem {
    /// Task id.
    pub id: String,
    /// Task description.
    pub description: String,
    /// Whether the task is complete.
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional workflow status string (format-dependent).
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
/// Progress totals derived from parsed tasks.
pub struct ProgressInfo {
    /// Total tasks.
    pub total: usize,
    /// Completed tasks.
    pub complete: usize,
    /// Remaining tasks.
    pub remaining: usize,
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    /// Count of tasks marked in-progress (when known).
    pub in_progress: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Count of tasks pending (when known).
    pub pending: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
/// Diagnostic message associated with a task file.
pub struct TaskDiagnostic {
    /// Severity level.
    pub level: String,
    /// Human-readable message.
    pub message: String,
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    /// Optional task id this diagnostic refers to.
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
/// Instruction payload for applying a change.
pub struct ApplyInstructionsResponse {
    #[serde(rename = "changeName")]
    /// Change directory name.
    pub change_name: String,
    #[serde(rename = "changeDir")]
    /// Full path to the change directory.
    pub change_dir: String,
    #[serde(rename = "schemaName")]
    /// Schema name.
    pub schema_name: String,
    #[serde(rename = "tracksPath")]
    /// Full path to the tracking file if configured.
    pub tracks_path: Option<String>,
    #[serde(rename = "tracksFile")]
    /// Tracking filename relative to the change directory.
    pub tracks_file: Option<String>,
    #[serde(rename = "tracksFormat")]
    /// Detected tracking file format.
    pub tracks_format: Option<String>,
    #[serde(rename = "tracksDiagnostics", skip_serializing_if = "Option::is_none")]
    /// Optional diagnostics produced while parsing the tracking file.
    pub tracks_diagnostics: Option<Vec<TaskDiagnostic>>,

    /// Machine-readable state label.
    pub state: String,
    #[serde(rename = "contextFiles")]
    /// Map of artifact id to full path for context files.
    pub context_files: BTreeMap<String, String>,

    /// Task progress totals.
    pub progress: ProgressInfo,

    /// Parsed tasks.
    pub tasks: Vec<TaskItem>,
    #[serde(rename = "missingArtifacts", skip_serializing_if = "Option::is_none")]
    /// Missing artifacts that block applying the change.
    pub missing_artifacts: Option<Vec<String>>,

    /// Human-readable instruction to display to the user.
    pub instruction: String,
}

#[derive(Debug, Clone, Serialize)]
/// Instruction payload for agent-oriented endpoints.
pub struct AgentInstructionResponse {
    #[serde(rename = "artifactId")]
    /// Artifact id.
    pub artifact_id: String,

    /// Instruction text.
    pub instruction: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Where a schema was resolved from.
pub enum SchemaSource {
    /// Schema provided by the Ito package/repository.
    Package,
    /// Schema provided by the user (XDG data dir).
    User,
}

impl SchemaSource {
    /// Return a stable string identifier for serialization.
    pub fn as_str(self) -> &'static str {
        match self {
            SchemaSource::Package => "package",
            SchemaSource::User => "user",
        }
    }
}

#[derive(Debug, Clone)]
/// A fully-resolved schema (yaml + directory + source).
pub struct ResolvedSchema {
    /// Parsed schema yaml.
    pub schema: SchemaYaml,
    /// Directory containing `schema.yaml`.
    pub schema_dir: PathBuf,
    /// Where the schema was found.
    pub source: SchemaSource,
}

#[derive(Debug, Clone, Deserialize)]
/// Schema file model (`schema.yaml`).
pub struct SchemaYaml {
    /// Schema name.
    pub name: String,
    #[serde(default)]
    /// Optional schema version.
    pub version: Option<u32>,
    #[serde(default)]
    /// Optional schema description.
    pub description: Option<String>,

    /// Artifact definitions.
    pub artifacts: Vec<ArtifactYaml>,
    #[serde(default)]
    /// Optional apply-stage configuration.
    pub apply: Option<ApplyYaml>,
}

#[derive(Debug, Clone, Deserialize)]
/// One artifact definition from a schema.
pub struct ArtifactYaml {
    /// Artifact id.
    pub id: String,
    /// Output path pattern relative to the change dir.
    pub generates: String,
    #[serde(default)]
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Template filename within the schema templates directory.
    pub template: String,
    #[serde(default)]
    /// Optional additional instruction text.
    pub instruction: Option<String>,
    #[serde(default)]
    /// Artifact ids that must be completed first.
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
/// Apply-stage configuration from a schema.
pub struct ApplyYaml {
    #[serde(default)]
    /// Artifacts required to consider the change ready to apply.
    pub requires: Option<Vec<String>>,
    #[serde(default)]
    /// Optional task tracking filename (relative to change dir).
    pub tracks: Option<String>,
    #[serde(default)]
    /// Optional instruction text displayed during apply.
    pub instruction: Option<String>,
}

/// Default schema name used when a change does not specify one.
pub fn default_schema_name() -> &'static str {
    "spec-driven"
}

/// Validate a user-provided change id for safe filesystem access.
pub fn validate_change_name_input(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    if name.starts_with('/') || name.starts_with('\\') {
        return false;
    }
    if name.contains('/') || name.contains('\\') {
        return false;
    }
    if name.contains("..") {
        return false;
    }
    true
}

/// Read a change's configured schema name from its metadata.
pub fn read_change_schema(ito_path: &Path, change: &str) -> String {
    let meta = paths::change_meta_path(ito_path, change);
    if let Ok(Some(s)) = ito_common::io::read_to_string_optional(&meta) {
        for line in s.lines() {
            let l = line.trim();
            if let Some(rest) = l.strip_prefix("schema:") {
                let v = rest.trim();
                if !v.is_empty() {
                    return v.to_string();
                }
            }
        }
    }
    default_schema_name().to_string()
}

/// List available change directory names under `.ito/changes/`.
pub fn list_available_changes(ito_path: &Path) -> Vec<String> {
    let fs = StdFs;
    ito_domain::discovery::list_change_dir_names(&fs, ito_path).unwrap_or_default()
}

/// List available schema names from package and user schema directories.
pub fn list_available_schemas(ctx: &ConfigContext) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    let fs = StdFs;
    for dir in [Some(package_schemas_dir()), user_schemas_dir(ctx)] {
        let Some(dir) = dir else { continue };
        let Ok(names) = ito_domain::discovery::list_dir_names(&fs, &dir) else {
            continue;
        };
        for name in names {
            let schema_dir = dir.join(&name);
            if schema_dir.join("schema.yaml").exists() {
                set.insert(name);
            }
        }
    }
    set.into_iter().collect()
}

/// Resolve a schema name into a [`ResolvedSchema`].
///
/// User schemas take precedence over package schemas.
pub fn resolve_schema(
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<ResolvedSchema, WorkflowError> {
    let name = schema_name.unwrap_or(default_schema_name());
    let user_dir = user_schemas_dir(ctx).map(|d| d.join(name));
    if let Some(d) = user_dir
        && d.join("schema.yaml").exists()
    {
        let schema = load_schema_yaml(&d)?;
        return Ok(ResolvedSchema {
            schema,
            schema_dir: d,
            source: SchemaSource::User,
        });
    }

    let pkg = package_schemas_dir().join(name);
    if pkg.join("schema.yaml").exists() {
        let schema = load_schema_yaml(&pkg)?;
        return Ok(ResolvedSchema {
            schema,
            schema_dir: pkg,
            source: SchemaSource::Package,
        });
    }

    Err(WorkflowError::SchemaNotFound(name.to_string()))
}

/// Compute per-artifact status for a change.
pub fn compute_change_status(
    ito_path: &Path,
    change: &str,
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<ChangeStatus, WorkflowError> {
    if !validate_change_name_input(change) {
        return Err(WorkflowError::InvalidChangeName);
    }
    let schema_name = schema_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| read_change_schema(ito_path, change));
    let resolved = resolve_schema(Some(&schema_name), ctx)?;

    let change_dir = paths::change_dir(ito_path, change);
    if !change_dir.exists() {
        return Err(WorkflowError::ChangeNotFound(change.to_string()));
    }

    let mut artifacts_out: Vec<ArtifactStatus> = Vec::new();
    let mut done_count: usize = 0;
    let done_by_id = compute_done_by_id(&change_dir, &resolved.schema);

    let order = build_order(&resolved.schema);
    for id in order {
        let Some(a) = resolved.schema.artifacts.iter().find(|a| a.id == id) else {
            continue;
        };
        let done = *done_by_id.get(&a.id).unwrap_or(&false);
        let mut missing: Vec<String> = Vec::new();
        if !done {
            for r in &a.requires {
                if !*done_by_id.get(r).unwrap_or(&false) {
                    missing.push(r.clone());
                }
            }
        }

        let status = if done {
            done_count += 1;
            "done".to_string()
        } else if missing.is_empty() {
            "ready".to_string()
        } else {
            "blocked".to_string()
        };
        artifacts_out.push(ArtifactStatus {
            id: a.id.clone(),
            output_path: a.generates.clone(),
            status,
            missing_deps: missing,
        });
    }

    let all_artifact_ids: Vec<String> = resolved
        .schema
        .artifacts
        .iter()
        .map(|a| a.id.clone())
        .collect();
    let apply_requires: Vec<String> = match resolved.schema.apply.as_ref() {
        Some(apply) => apply
            .requires
            .clone()
            .unwrap_or_else(|| all_artifact_ids.clone()),
        None => all_artifact_ids.clone(),
    };

    let is_complete = done_count == resolved.schema.artifacts.len();
    Ok(ChangeStatus {
        change_name: change.to_string(),
        schema_name: resolved.schema.name,
        is_complete,
        apply_requires,
        artifacts: artifacts_out,
    })
}

fn build_order(schema: &SchemaYaml) -> Vec<String> {
    // Match TS ArtifactGraph.getBuildOrder (Kahn's algorithm with deterministic sorting
    // of roots + newlyReady only).
    let mut in_degree: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut dependents: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for a in &schema.artifacts {
        in_degree.insert(a.id.clone(), a.requires.len());
        dependents.insert(a.id.clone(), Vec::new());
    }
    for a in &schema.artifacts {
        for req in &a.requires {
            dependents
                .entry(req.clone())
                .or_default()
                .push(a.id.clone());
        }
    }

    let mut queue: Vec<String> = schema
        .artifacts
        .iter()
        .map(|a| a.id.clone())
        .filter(|id| in_degree.get(id).copied().unwrap_or(0) == 0)
        .collect();
    queue.sort();

    let mut result: Vec<String> = Vec::new();
    while !queue.is_empty() {
        let current = queue.remove(0);
        result.push(current.clone());

        let mut newly_ready: Vec<String> = Vec::new();
        if let Some(deps) = dependents.get(&current) {
            for dep in deps {
                let new_degree = in_degree.get(dep).copied().unwrap_or(0).saturating_sub(1);
                in_degree.insert(dep.clone(), new_degree);
                if new_degree == 0 {
                    newly_ready.push(dep.clone());
                }
            }
        }
        newly_ready.sort();
        queue.extend(newly_ready);
    }

    result
}

/// Resolve template file paths for all artifacts in a schema.
pub fn resolve_templates(
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<(String, BTreeMap<String, TemplateInfo>), WorkflowError> {
    let resolved = resolve_schema(schema_name, ctx)?;
    let templates_dir = resolved.schema_dir.join("templates");

    let mut templates: BTreeMap<String, TemplateInfo> = BTreeMap::new();
    for a in &resolved.schema.artifacts {
        templates.insert(
            a.id.clone(),
            TemplateInfo {
                source: resolved.source.as_str().to_string(),
                path: templates_dir
                    .join(&a.template)
                    .to_string_lossy()
                    .to_string(),
            },
        );
    }
    Ok((resolved.schema.name, templates))
}

/// Resolve build instructions for a single artifact.
pub fn resolve_instructions(
    ito_path: &Path,
    change: &str,
    schema_name: Option<&str>,
    artifact_id: &str,
    ctx: &ConfigContext,
) -> Result<InstructionsResponse, WorkflowError> {
    if !validate_change_name_input(change) {
        return Err(WorkflowError::InvalidChangeName);
    }
    let schema_name = schema_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| read_change_schema(ito_path, change));
    let resolved = resolve_schema(Some(&schema_name), ctx)?;

    let change_dir = paths::change_dir(ito_path, change);
    if !change_dir.exists() {
        return Err(WorkflowError::ChangeNotFound(change.to_string()));
    }

    let a = resolved
        .schema
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .ok_or_else(|| WorkflowError::ArtifactNotFound(artifact_id.to_string()))?;

    let templates_dir = resolved.schema_dir.join("templates");
    let done_by_id = compute_done_by_id(&change_dir, &resolved.schema);

    let deps: Vec<DependencyInfo> = a
        .requires
        .iter()
        .map(|id| {
            let dep = resolved.schema.artifacts.iter().find(|d| d.id == *id);
            DependencyInfo {
                id: id.clone(),
                done: *done_by_id.get(id).unwrap_or(&false),
                path: dep
                    .map(|d| d.generates.clone())
                    .unwrap_or_else(|| id.clone()),
                description: dep.and_then(|d| d.description.clone()).unwrap_or_default(),
            }
        })
        .collect();

    let mut unlocks: Vec<String> = resolved
        .schema
        .artifacts
        .iter()
        .filter(|other| other.requires.iter().any(|r| r == artifact_id))
        .map(|a| a.id.clone())
        .collect();
    unlocks.sort();

    let template = ito_common::io::read_to_string_std(&templates_dir.join(&a.template))?;

    Ok(InstructionsResponse {
        change_name: change.to_string(),
        artifact_id: a.id.clone(),
        schema_name: resolved.schema.name,
        change_dir: change_dir.to_string_lossy().to_string(),
        output_path: a.generates.clone(),
        description: a.description.clone().unwrap_or_default(),
        instruction: a.instruction.clone(),
        template,
        dependencies: deps,
        unlocks,
    })
}

/// Compute apply-stage instructions and progress for a change.
pub fn compute_apply_instructions(
    ito_path: &Path,
    change: &str,
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<ApplyInstructionsResponse, WorkflowError> {
    if !validate_change_name_input(change) {
        return Err(WorkflowError::InvalidChangeName);
    }
    let schema_name = schema_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| read_change_schema(ito_path, change));
    let resolved = resolve_schema(Some(&schema_name), ctx)?;
    let change_dir = paths::change_dir(ito_path, change);
    if !change_dir.exists() {
        return Err(WorkflowError::ChangeNotFound(change.to_string()));
    }

    let schema = &resolved.schema;
    let apply = schema.apply.as_ref();
    let all_artifact_ids: Vec<String> = schema.artifacts.iter().map(|a| a.id.clone()).collect();

    // Determine required artifacts and tracking file from schema.
    // Match TS: apply.requires ?? allArtifacts (nullish coalescing).
    let required_artifact_ids: Vec<String> = apply
        .and_then(|a| a.requires.clone())
        .unwrap_or_else(|| all_artifact_ids.clone());
    let tracks_file: Option<String> = apply.and_then(|a| a.tracks.clone());
    let schema_instruction: Option<String> = apply.and_then(|a| a.instruction.clone());

    // Check which required artifacts are missing.
    let mut missing_artifacts: Vec<String> = Vec::new();
    for artifact_id in &required_artifact_ids {
        let Some(artifact) = schema.artifacts.iter().find(|a| a.id == *artifact_id) else {
            continue;
        };
        if !artifact_done(&change_dir, &artifact.generates) {
            missing_artifacts.push(artifact_id.clone());
        }
    }

    // Build context files from all existing artifacts in schema.
    let mut context_files: BTreeMap<String, String> = BTreeMap::new();
    for artifact in &schema.artifacts {
        if artifact_done(&change_dir, &artifact.generates) {
            context_files.insert(
                artifact.id.clone(),
                change_dir
                    .join(&artifact.generates)
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    // Parse tasks if tracking file exists.
    let mut tasks: Vec<TaskItem> = Vec::new();
    let mut tracks_file_exists = false;
    let mut tracks_path: Option<String> = None;
    let mut tracks_format: Option<String> = None;
    let tracks_diagnostics: Option<Vec<TaskDiagnostic>> = None;

    if let Some(tf) = &tracks_file {
        let p = change_dir.join(tf);
        tracks_path = Some(p.to_string_lossy().to_string());
        tracks_file_exists = p.exists();
        if tracks_file_exists {
            let content = ito_common::io::read_to_string_std(&p)?;
            let checkbox = parse_checkbox_tasks(&content);
            if !checkbox.is_empty() {
                tracks_format = Some("checkbox".to_string());
                tasks = checkbox;
            } else {
                let enhanced = parse_enhanced_tasks(&content);
                if !enhanced.is_empty() {
                    tracks_format = Some("enhanced".to_string());
                    tasks = enhanced;
                } else if looks_like_enhanced_tasks(&content) {
                    tracks_format = Some("enhanced".to_string());
                } else {
                    tracks_format = Some("unknown".to_string());
                }
            }
        }
    }

    // Calculate progress.
    let total = tasks.len();
    let complete = tasks.iter().filter(|t| t.done).count();
    let remaining = total.saturating_sub(complete);
    let mut in_progress: Option<usize> = None;
    let mut pending: Option<usize> = None;
    if tracks_format.as_deref() == Some("enhanced") {
        let mut in_progress_count = 0;
        let mut pending_count = 0;
        for task in &tasks {
            let Some(status) = task.status.as_deref() else {
                continue;
            };
            let status = status.trim();
            match status {
                "in-progress" | "in_progress" | "in progress" => in_progress_count += 1,
                "pending" => pending_count += 1,
                _ => {}
            }
        }
        in_progress = Some(in_progress_count);
        pending = Some(pending_count);
    }
    if tracks_format.as_deref() == Some("checkbox") {
        let mut in_progress_count = 0;
        for task in &tasks {
            let Some(status) = task.status.as_deref() else {
                continue;
            };
            if status.trim() == "in-progress" {
                in_progress_count += 1;
            }
        }
        in_progress = Some(in_progress_count);
        pending = Some(total.saturating_sub(complete + in_progress_count));
    }
    let progress = ProgressInfo {
        total,
        complete,
        remaining,
        in_progress,
        pending,
    };

    // Determine state and instruction.
    let (state, instruction) = if !missing_artifacts.is_empty() {
        (
            "blocked".to_string(),
            format!(
                "Cannot apply this change yet. Missing artifacts: {}.\nUse the ito-continue-change skill to create the missing artifacts first.",
                missing_artifacts.join(", ")
            ),
        )
    } else if tracks_file.is_some() && !tracks_file_exists {
        let tracks_filename = tracks_file
            .as_deref()
            .and_then(|p| Path::new(p).file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "tasks.md".to_string());
        (
            "blocked".to_string(),
            format!(
                "The {tracks_filename} file is missing and must be created.\nUse ito-continue-change to generate the tracking file."
            ),
        )
    } else if tracks_file.is_some() && tracks_file_exists && total == 0 {
        let tracks_filename = tracks_file
            .as_deref()
            .and_then(|p| Path::new(p).file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "tasks.md".to_string());
        (
            "blocked".to_string(),
            format!(
                "The {tracks_filename} file exists but contains no tasks.\nAdd tasks to {tracks_filename} or regenerate it with ito-continue-change."
            ),
        )
    } else if tracks_file.is_some() && remaining == 0 && total > 0 {
        (
            "all_done".to_string(),
            "All tasks are complete! This change is ready to be archived.\nConsider running tests and reviewing the changes before archiving."
                .to_string(),
        )
    } else if tracks_file.is_none() {
        (
            "ready".to_string(),
            schema_instruction
                .as_deref()
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| {
                    "All required artifacts complete. Proceed with implementation.".to_string()
                }),
        )
    } else {
        (
            "ready".to_string(),
            schema_instruction
                .as_deref()
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| {
                    "Read context files, work through pending tasks, mark complete as you go.\nPause if you hit blockers or need clarification.".to_string()
                }),
        )
    };

    Ok(ApplyInstructionsResponse {
        change_name: change.to_string(),
        change_dir: change_dir.to_string_lossy().to_string(),
        schema_name: schema.name.clone(),
        tracks_path,
        tracks_file,
        tracks_format,
        tracks_diagnostics,
        context_files,
        progress,
        tasks,
        state,
        missing_artifacts: if missing_artifacts.is_empty() {
            None
        } else {
            Some(missing_artifacts)
        },
        instruction,
    })
}

fn parse_checkbox_tasks(contents: &str) -> Vec<TaskItem> {
    let mut tasks: Vec<TaskItem> = Vec::new();
    for line in contents.lines() {
        let l = line.trim_start();
        let bytes = l.as_bytes();
        if bytes.len() < 6 {
            continue;
        }
        let bullet = bytes[0] as char;
        if bullet != '-' && bullet != '*' {
            continue;
        }
        if bytes[1] != b' ' || bytes[2] != b'[' || bytes[4] != b']' || bytes[5] != b' ' {
            continue;
        }

        let marker = bytes[3] as char;
        let (done, rest, status) = match marker {
            'x' | 'X' => (true, &l[6..], None),
            ' ' => (false, &l[6..], None),
            '~' | '>' => (false, &l[6..], Some("in-progress".to_string())),
            _ => continue,
        };
        tasks.push(TaskItem {
            id: (tasks.len() + 1).to_string(),
            description: rest.trim().to_string(),
            done,
            status,
        });
    }
    tasks
}

fn looks_like_enhanced_tasks(contents: &str) -> bool {
    for line in contents.lines() {
        let l = line.trim_start();
        if l.starts_with("### Task ") {
            return true;
        }
    }
    false
}

fn parse_enhanced_tasks(contents: &str) -> Vec<TaskItem> {
    let mut tasks: Vec<TaskItem> = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_desc: Option<String> = None;
    let mut current_done = false;
    let mut current_status: Option<String> = None;

    fn push_current(
        tasks: &mut Vec<TaskItem>,
        current_id: &mut Option<String>,
        current_desc: &mut Option<String>,
        current_done: &mut bool,
        current_status: &mut Option<String>,
    ) {
        let Some(desc) = current_desc.take() else {
            current_id.take();
            *current_done = false;
            *current_status = None;
            return;
        };
        let id = current_id
            .take()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| (tasks.len() + 1).to_string());
        tasks.push(TaskItem {
            id,
            description: desc,
            done: *current_done,
            status: current_status.take(),
        });
        *current_done = false;
    }

    for line in contents.lines() {
        let l = line.trim_start();

        if let Some(rest) = l.strip_prefix("### Task ") {
            push_current(
                &mut tasks,
                &mut current_id,
                &mut current_desc,
                &mut current_done,
                &mut current_status,
            );

            let (id, desc) = rest.split_once(':').unwrap_or((rest, ""));
            let id = id.trim();
            let desc = if desc.trim().is_empty() {
                rest.trim()
            } else {
                desc.trim()
            };

            current_id = Some(id.to_string());
            current_desc = Some(desc.to_string());
            current_done = false;
            current_status = Some("pending".to_string());
            continue;
        }

        if let Some(rest) = l.strip_prefix("- **Status**:") {
            let status = rest.trim();
            if let Some(status) = status
                .strip_prefix("[x]")
                .or_else(|| status.strip_prefix("[X]"))
            {
                current_done = true;
                current_status = Some(status.trim().to_string());
                continue;
            }
            if let Some(status) = status.strip_prefix("[ ]") {
                current_done = false;
                current_status = Some(status.trim().to_string());
                continue;
            }
        }
    }

    push_current(
        &mut tasks,
        &mut current_id,
        &mut current_desc,
        &mut current_done,
        &mut current_status,
    );

    tasks
}

/// Load user guidance text from `user-guidance.md`.
///
/// When a file contains an Ito-managed header block, only the content after the
/// `ITO_END_MARKER` is returned.
pub fn load_user_guidance_for_artifact(
    ito_path: &Path,
    artifact_id: &str,
) -> Result<Option<String>, WorkflowError> {
    if artifact_id.contains("..") || artifact_id.contains('/') || artifact_id.contains('\\') {
        return Ok(None);
    }
    let path = ito_path
        .join("user-prompts")
        .join(format!("{artifact_id}.md"));
    load_guidance_file(&path)
}
    }
}

fn load_guidance_file(path: &Path) -> Result<Option<String>, WorkflowError> {
    if !path.exists() {
        return Ok(None);
    }

    let content = ito_common::io::read_to_string_std(path)?;
    let content = content.replace("\r\n", "\n");
    let content = match content.find(ITO_END_MARKER) {
        Some(i) => &content[i + ITO_END_MARKER.len()..],
        None => content.as_str(),
    };
    let content = content.trim();
    if content.is_empty() {
        return Ok(None);
    }

    Ok(Some(content.to_string()))
}

fn package_schemas_dir() -> PathBuf {
    // In this repo, schemas live at the repository root.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .ancestors()
        .nth(3)
        .unwrap_or(manifest_dir.as_path());
    root.join("schemas")
}

fn user_schemas_dir(ctx: &ConfigContext) -> Option<PathBuf> {
    let data_home = match env::var("XDG_DATA_HOME") {
        Ok(v) if !v.trim().is_empty() => Some(PathBuf::from(v)),
        _ => ctx
            .home_dir
            .as_ref()
            .map(|h| h.join(".local").join("share")),
    }?;
    Some(data_home.join("ito").join("schemas"))
}

fn load_schema_yaml(schema_dir: &Path) -> Result<SchemaYaml, WorkflowError> {
    let s = ito_common::io::read_to_string_std(&schema_dir.join("schema.yaml"))?;
    Ok(serde_yaml::from_str(&s)?)
}

fn compute_done_by_id(change_dir: &Path, schema: &SchemaYaml) -> BTreeMap<String, bool> {
    let mut out = BTreeMap::new();
    for a in &schema.artifacts {
        out.insert(a.id.clone(), artifact_done(change_dir, &a.generates));
    }
    out
}

fn artifact_done(change_dir: &Path, generates: &str) -> bool {
    if !generates.contains('*') {
        return change_dir.join(generates).exists();
    }

    // Minimal glob support for patterns used by schemas:
    //   dir/**/*.ext
    //   dir/*.suffix
    //   **/*.ext
    let (base, suffix) = match split_glob_pattern(generates) {
        Some(v) => v,
        None => return false,
    };
    let base_dir = change_dir.join(base);
    dir_contains_filename_suffix(&base_dir, &suffix)
}

fn split_glob_pattern(pattern: &str) -> Option<(String, String)> {
    let pattern = pattern.strip_prefix("./").unwrap_or(pattern);

    let (dir_part, file_pat) = match pattern.rsplit_once('/') {
        Some((d, f)) => (d, f),
        None => ("", pattern),
    };
    if !file_pat.starts_with('*') {
        return None;
    }
    let suffix = file_pat[1..].to_string();

    let base = dir_part
        .strip_suffix("/**")
        .or_else(|| dir_part.strip_suffix("**"))
        .unwrap_or(dir_part);

    // If the directory still contains wildcards (e.g. "**"), search from change_dir.
    let base = if base.contains('*') { "" } else { base };
    Some((base.to_string(), suffix))
}

fn dir_contains_filename_suffix(dir: &Path, suffix: &str) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for e in entries.flatten() {
        let path = e.path();
        if e.file_type().ok().is_some_and(|t| t.is_dir()) {
            if dir_contains_filename_suffix(&path, suffix) {
                return true;
            }
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(suffix) {
            return true;
        }
    }
    false
}

// (intentionally no checkbox counting helpers here; checkbox tasks are parsed into TaskItems)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_user_guidance_returns_trimmed_content_after_marker() {
        let dir = tempfile::tempdir().expect("tempdir should succeed");
        let ito_path = dir.path();

        let content = "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\nPrefer BDD.\n";
        std::fs::write(ito_path.join("user-guidance.md"), content).expect("write should succeed");

        let guidance = load_user_guidance(ito_path)
            .expect("load should succeed")
            .expect("should be present");

        assert_eq!(guidance, "Prefer BDD.");
    }

    #[test]
    fn parse_enhanced_tasks_extracts_ids_status_and_done() {
        let contents = r#"### Task 1.1: First
 - **Status**: [x] complete

 ### Task 1.2: Second
 - **Status**: [ ] in-progress
 "#;

        let tasks = parse_enhanced_tasks(contents);
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, "1.1");
        assert_eq!(tasks[0].description, "First");
        assert!(tasks[0].done);
        assert_eq!(tasks[0].status.as_deref(), Some("complete"));

        assert_eq!(tasks[1].id, "1.2");
        assert_eq!(tasks[1].description, "Second");
        assert!(!tasks[1].done);
        assert_eq!(tasks[1].status.as_deref(), Some("in-progress"));
    }
}

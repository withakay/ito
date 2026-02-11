use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

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

    #[error("Invalid artifact id: '{0}'")]
    /// Artifact id failed sanitization for user-guidance lookup.
    InvalidArtifactId(String),

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
    /// Schema provided by the current project (`.ito/templates/schemas`).
    Project,
    /// Schema provided by the user (XDG data dir).
    User,
    /// Schema provided by embedded assets in `ito-templates`.
    Embedded,
    /// Schema provided by the legacy package/repository filesystem path.
    Package,
}

impl SchemaSource {
    /// Provide the serialization label for a `SchemaSource` variant.
    pub fn as_str(self) -> &'static str {
        match self {
            SchemaSource::Project => "project",
            SchemaSource::User => "user",
            SchemaSource::Embedded => "embedded",
            SchemaSource::Package => "package",
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

#[cfg(test)]
mod tests {
    use super::SchemaSource;

    #[test]
    fn schema_source_as_str_returns_expected_labels() {
        assert_eq!(SchemaSource::Project.as_str(), "project");
        assert_eq!(SchemaSource::User.as_str(), "user");
        assert_eq!(SchemaSource::Embedded.as_str(), "embedded");
        assert_eq!(SchemaSource::Package.as_str(), "package");
    }
}

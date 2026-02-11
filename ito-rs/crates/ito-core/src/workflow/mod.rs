//! Schema-driven change workflow helpers.
//!
//! This module reads a change directory and a workflow schema (`schema.yaml`) and
//! produces JSON-friendly status and instruction payloads.
//!
//! These types are designed for use by the CLI and by any web/API layer that
//! wants to present "what should I do next?" without duplicating the filesystem
//! logic.

use ito_templates::ITO_END_MARKER;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

mod schema_assets;
mod types;
pub use schema_assets::{ExportSchemasResult, export_embedded_schemas};
use schema_assets::{
    embedded_schema_names, is_safe_relative_path, is_safe_schema_name, load_embedded_schema_yaml,
    package_schemas_dir, project_schemas_dir, read_schema_template, user_schemas_dir,
};
pub use types::{
    AgentInstructionResponse, ApplyInstructionsResponse, ApplyYaml, ArtifactStatus, ArtifactYaml,
    ChangeStatus, DependencyInfo, InstructionsResponse, ProgressInfo, ResolvedSchema, SchemaSource,
    SchemaYaml, TaskDiagnostic, TaskItem, TemplateInfo, WorkflowError,
};

use ito_common::fs::StdFs;
use ito_common::paths;
use ito_config::ConfigContext;

/// Default schema name used when a change does not specify one.
pub fn default_schema_name() -> &'static str {
    "spec-driven"
}

/// Validates a user-provided change name to ensure it is safe to use as a filesystem path segment.
///
/// The name must be non-empty, must not start with `/` or `\`, must not contain `/` or `\` anywhere, and must not contain the substring `..`.
///
/// # Examples
///
/// ```
/// assert!(validate_change_name_input("feature-123"));
/// assert!(!validate_change_name_input("")); // empty
/// assert!(!validate_change_name_input("../escape"));
/// assert!(!validate_change_name_input("dir/name"));
/// assert!(!validate_change_name_input("\\absolute"));
/// ```
///
/// # Returns
///
/// `true` if the name meets the safety constraints described above, `false` otherwise.
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

/// Determines the schema name configured for a change by reading its metadata.
///
/// Returns the schema name configured for the change, or the default schema name (`spec-driven`) if none is set.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let name = read_change_schema(Path::new("/nonexistent/path"), "nope");
/// assert_eq!(name, "spec-driven");
/// ```
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

/// List change directory names under the `.ito/changes` directory.
///
/// Each element is the change directory name (not a full path).
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let names = ito_core::workflow::list_available_changes(Path::new("."));
/// // `names` is a `Vec<String>` of change directory names
/// ```
pub fn list_available_changes(ito_path: &Path) -> Vec<String> {
    let fs = StdFs;
    ito_domain::discovery::list_change_dir_names(&fs, ito_path).unwrap_or_default()
}

/// Lists available schema names discovered from the project, user, embedded, and package schema locations.
///
/// The result contains unique schema names and is deterministically sorted.
///
/// # Returns
///
/// A sorted, de-duplicated `Vec<String>` of available schema names.
///
/// # Examples
///
/// ```rust
/// // `ctx` should be a prepared `ConfigContext` for the current project.
/// let names = list_available_schemas(&ctx);
/// assert!(names.iter().all(|s| !s.is_empty()));
/// ```
pub fn list_available_schemas(ctx: &ConfigContext) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    let fs = StdFs;
    for dir in [
        project_schemas_dir(ctx),
        user_schemas_dir(ctx),
        Some(package_schemas_dir()),
    ] {
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

    for name in embedded_schema_names() {
        set.insert(name);
    }

    set.into_iter().collect()
}

/// Resolves a schema name into a [`ResolvedSchema`].
///
/// If `schema_name` is `None`, the default schema name is used. Resolution
/// precedence is project-local -> user -> embedded -> package; the returned
/// `ResolvedSchema` contains the loaded `SchemaYaml`, the directory or embedded
/// path that contained `schema.yaml`, and a `SchemaSource` indicating where it
/// was found.
///
/// # Parameters
///
/// - `schema_name`: Optional schema name to resolve; uses the module default when
///   `None`.
/// - `ctx`: Configuration context used to locate project and user schema paths.
///
/// # Errors
///
/// Returns `WorkflowError::SchemaNotFound(name)` when the schema cannot be
/// located. Other `WorkflowError` variants may be returned for IO or YAML
/// parsing failures encountered while loading `schema.yaml`.
///
/// # Examples
///
/// ```
/// // Resolves the default schema using `ctx`.
/// let resolved = resolve_schema(None, &ctx).expect("schema not found");
/// println!("Resolved {} from {}", resolved.schema.name, resolved.schema_dir.display());
/// ```
pub fn resolve_schema(
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<ResolvedSchema, WorkflowError> {
    let name = schema_name.unwrap_or(default_schema_name());
    if !is_safe_schema_name(name) {
        return Err(WorkflowError::SchemaNotFound(name.to_string()));
    }

    let project_dir = project_schemas_dir(ctx).map(|d| d.join(name));
    if let Some(d) = project_dir
        && d.join("schema.yaml").exists()
    {
        let schema = load_schema_yaml(&d)?;
        return Ok(ResolvedSchema {
            schema,
            schema_dir: d,
            source: SchemaSource::Project,
        });
    }

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

    if let Some(schema) = load_embedded_schema_yaml(name)? {
        return Ok(ResolvedSchema {
            schema,
            schema_dir: PathBuf::from(format!("embedded://schemas/{name}")),
            source: SchemaSource::Embedded,
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

/// Compute the workflow status for every artifact in a change.
///
/// Validates the change name, resolves the effective schema (explicit or from the change metadata),
/// verifies the change directory exists, and produces per-artifact statuses plus the list of
/// artifacts required before an apply operation.
///
/// # Parameters
///
/// - `ito_path`: base repository path containing the `.ito` state directories.
/// - `change`: change directory name to inspect (must be a validated change name).
/// - `schema_name`: optional explicit schema name; when `None`, the change's metadata is consulted.
/// - `ctx`: configuration/context used to locate and load schemas.
///
/// # Returns
///
/// `ChangeStatus` describing the change name, resolved schema, overall completion flag,
/// the set of artifact ids required for apply, and a list of `ArtifactStatus` entries where each
/// artifact is labeled `done`, `ready`, or `blocked` and includes any missing dependency ids.
///
/// # Errors
///
/// Returns a `WorkflowError` when the change name is invalid, the change directory is missing,
/// or the schema cannot be resolved or loaded.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use ito_core::workflow::{compute_change_status, ChangeStatus};
/// # use ito_core::config::ConfigContext;
/// let ctx = ConfigContext::default();
/// let status = compute_change_status(Path::new("."), "my-change", None, &ctx).unwrap();
/// assert_eq!(status.change_name, "my-change");
/// ```
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

/// Computes a deterministic topological build order of artifact ids for the given schema.
///
/// The returned vector lists artifact ids in an order where each artifact appears after all of
/// its declared `requires`. When multiple artifacts become ready at the same time, their ids
/// are emitted in sorted order to ensure deterministic output.
///
/// # Examples
///
/// ```
/// // Construct a minimal schema with three artifacts:
/// // - "a" has no requirements
/// // - "b" requires "a"
/// // - "c" requires "a"
/// let schema = SchemaYaml {
///     name: "example".to_string(),
///     version: None,
///     description: None,
///     artifacts: vec![
///         ArtifactYaml {
///             id: "a".to_string(),
///             generates: "a.out".to_string(),
///             description: None,
///             template: "a.tpl".to_string(),
///             instruction: None,
///             requires: vec![],
///         },
///         ArtifactYaml {
///             id: "b".to_string(),
///             generates: "b.out".to_string(),
///             description: None,
///             template: "b.tpl".to_string(),
///             instruction: None,
///             requires: vec!["a".to_string()],
///         },
///         ArtifactYaml {
///             id: "c".to_string(),
///             generates: "c.out".to_string(),
///             description: None,
///             template: "c.tpl".to_string(),
///             instruction: None,
///             requires: vec!["a".to_string()],
///         },
///     ],
///     apply: None,
/// };
///
/// let order = build_order(&schema);
/// // "a" must come before both "b" and "c"; "b" and "c" are sorted deterministically
/// assert_eq!(order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
/// ```
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

/// Resolve template paths for every artifact in a schema.
///
/// If `schema_name` is `None`, the schema is resolved using project -> user -> embedded -> package
/// precedence. For embedded schemas each template path is returned as an `embedded://schemas/{name}/templates/{file}`
/// URI; for filesystem-backed schemas each template path is an absolute filesystem string.
///
/// Returns the resolved schema name and a map from artifact id to `TemplateInfo` (contains `source` and `path`).
///
/// # Examples
///
/// ```rust,no_run
/// // Obtain a ConfigContext from your application environment.
/// let ctx = /* obtain ConfigContext */ unimplemented!();
/// let (schema_name, templates) = resolve_templates(None, &ctx).unwrap();
/// // `templates` maps artifact ids to TemplateInfo with `source` and `path`.
/// ```
pub fn resolve_templates(
    schema_name: Option<&str>,
    ctx: &ConfigContext,
) -> Result<(String, BTreeMap<String, TemplateInfo>), WorkflowError> {
    let resolved = resolve_schema(schema_name, ctx)?;

    let mut templates: BTreeMap<String, TemplateInfo> = BTreeMap::new();
    for a in &resolved.schema.artifacts {
        if !is_safe_relative_path(&a.template) {
            return Err(WorkflowError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid template path: {}", a.template),
            )));
        }

        let path = if resolved.source == SchemaSource::Embedded {
            format!(
                "embedded://schemas/{}/templates/{}",
                resolved.schema.name, a.template
            )
        } else {
            resolved
                .schema_dir
                .join("templates")
                .join(&a.template)
                .to_string_lossy()
                .to_string()
        };
        templates.insert(
            a.id.clone(),
            TemplateInfo {
                source: resolved.source.as_str().to_string(),
                path,
            },
        );
    }
    Ok((resolved.schema.name, templates))
}

/// Produce user-facing instructions and metadata for performing a single artifact in a change.
///
/// Resolves the effective schema for the change, verifies the change directory and artifact exist,
/// computes the artifact's declared dependencies and which artifacts it will unlock, loads the
/// artifact's template and instruction text, and returns an InstructionsResponse containing the
/// fields required by CLI/API layers.
///
/// # Errors
///
/// Returns a `WorkflowError` when the change name is invalid, the change directory or schema cannot be found,
/// the requested artifact is not defined in the schema, or when underlying I/O/YAML/template reads fail
/// (for example: `InvalidChangeName`, `ChangeNotFound`, `SchemaNotFound`, `ArtifactNotFound`, `Io`, `Yaml`).
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // `config_ctx` should be a prepared ConfigContext in real usage.
/// let resp = resolve_instructions(
///     Path::new("/project/ito"),
///     "0001-add-feature",
///     Some("spec-driven"),
///     "service-config",
///     &config_ctx,
/// ).unwrap();
/// assert_eq!(resp.artifact_id, "service-config");
/// ```
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

    let template = read_schema_template(&resolved, &a.template)?;

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

/// Detects whether the given text uses the enhanced task format.
///
/// Scans lines for headings of the form `### Task ` and returns `true` if any are found.
///
/// # Examples
///
/// ```
/// let contents = "Some header\n### Task 1: Do thing\n- **Status**: [ ] pending";
/// assert!(looks_like_enhanced_tasks(contents));
///
/// let plain = "- [ ] item one\n- [x] item two";
/// assert!(!looks_like_enhanced_tasks(plain));
/// ```
///
/// # Returns
///
/// `true` if the contents contain at least one line beginning with `### Task `, `false` otherwise.
fn looks_like_enhanced_tasks(contents: &str) -> bool {
    for line in contents.lines() {
        let l = line.trim_start();
        if l.starts_with("### Task ") {
            return true;
        }
    }
    false
}

/// Parses an "enhanced" task list format into a vector of TaskItem.
///
/// The parser recognizes sections starting with `### Task {id}: {description}` and
/// subsequent `- **Status**: [ ]|[x] {status}` lines to set `done` and `status`.
/// If an id is missing or empty, a numeric id is assigned (1-based by parse order).
///
/// # Examples
///
/// ```
/// let src = r#"
/// ### Task alpha: First task
/// - **Status**: [ ] needs-review
///
/// ### Task : Second task without id
/// - **Status**: [x] completed
/// "#;
///
/// let tasks = parse_enhanced_tasks(src);
/// assert_eq!(tasks.len(), 2);
/// assert_eq!(tasks[0].id, "alpha");
/// assert_eq!(tasks[0].description, "First task");
/// assert_eq!(tasks[0].done, false);
/// assert_eq!(tasks[0].status.as_deref(), Some("needs-review"));
///
/// assert_eq!(tasks[1].id, "2"); // auto-assigned numeric id
/// assert_eq!(tasks[1].description, "Second task without id");
/// assert_eq!(tasks[1].done, true);
/// assert_eq!(tasks[1].status.as_deref(), Some("completed"));
/// ```
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

/// Extracts user guidance text from a repository directory's `user-guidance.md`.
///
/// If the file contains an Ito-managed header block, the returned content is the
/// portion after the ITO end marker. Carriage-return/newline pairs (`\r\n`)
/// are normalized to `\n` and the result is trimmed; a missing file or empty
/// result yields `None`.
///
/// # Examples
///
/// ```no_run
/// use std::fs;
/// use std::path::Path;
///
/// // missing file -> None
/// let tmp = Path::new("/tmp/ito-example-missing");
/// let _ = fs::remove_dir_all(tmp);
/// assert!(crate::workflow::load_user_guidance(tmp).unwrap().is_none());
///
/// // present file -> Some(trimmed content)
/// let dir = Path::new("/tmp/ito-example");
/// fs::create_dir_all(dir).unwrap();
/// fs::write(dir.join("user-guidance.md"), "User guidance text\n").unwrap();
/// let guidance = crate::workflow::load_user_guidance(dir).unwrap();
/// assert_eq!(guidance.as_deref(), Some("User guidance text"));
/// ```
pub fn load_user_guidance(ito_path: &Path) -> Result<Option<String>, WorkflowError> {
    let path = ito_path.join("user-guidance.md");
    if !path.exists() {
        return Ok(None);
    }

    let content = ito_common::io::read_to_string_std(&path)?;
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

/// Load and parse `schema.yaml` from the given schema directory.
///
/// Reads `schema.yaml` located in `schema_dir` and deserializes it into a `SchemaYaml`.
///
/// # Errors
///
/// Returns `WorkflowError::Io` if the file cannot be read, or `WorkflowError::Yaml` if parsing fails.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::fs;
///
/// let dir = std::env::temp_dir().join("ito_example_schema");
/// let _ = fs::create_dir_all(&dir);
/// let yaml = r#"
/// name: example
/// artifacts: []
/// "#;
/// fs::write(dir.join("schema.yaml"), yaml).unwrap();
///
/// let schema = crate::workflow::load_schema_yaml(&Path::new(&dir)).unwrap();
/// assert_eq!(schema.name, "example");
/// ```
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

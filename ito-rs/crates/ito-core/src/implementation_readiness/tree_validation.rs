//! Authority-tree loading and content-based strict validation.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use glob::{MatchOptions, Pattern};
use ito_domain::tasks::{TasksParseResult, parse_tasks_tracking_file};

use crate::change_meta::parse_change_meta;
use crate::show::{DeltaSpecFile, parse_change_show_json};
use crate::templates::{
    ArtifactYaml, SchemaYaml, ValidationTrackingSourceYaml, ValidationYaml, ValidatorId,
};

use super::git::{GitTreeEntry, ReadinessGit};
use super::{AuthoritySnapshot, ReadinessRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PrepareFailureKind {
    Artifacts,
    Validation,
}

#[derive(Debug)]
pub(super) struct PrepareFailure {
    pub kind: PrepareFailureKind,
    pub path: String,
    pub validator_code: Option<String>,
    pub message: String,
}

impl PrepareFailure {
    fn artifact(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: PrepareFailureKind::Artifacts,
            path: path.into(),
            validator_code: None,
            message: message.into(),
        }
    }

    fn validation(
        path: impl Into<String>,
        validator_code: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: PrepareFailureKind::Validation,
            path: path.into(),
            validator_code: validator_code.map(ToOwned::to_owned),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub(super) struct PrepareProof {
    pub schema_name: String,
    pub artifact_paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct LoadedFile {
    path: String,
    contents: String,
}

struct ReadinessSchema {
    schema: SchemaYaml,
    validation: Option<ValidationYaml>,
    schema_path: String,
    schema_contents: String,
    validation_path: String,
    validation_contents: Option<String>,
}

pub(super) fn validate_authoritative_change(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    snapshot: &AuthoritySnapshot,
) -> Result<PrepareProof, PrepareFailure> {
    if !crate::templates::validate_change_name_input(&request.change_id) {
        return Err(PrepareFailure::artifact(
            request.change_id.clone(),
            "Change ID is not a safe Git-tree path segment.",
        ));
    }

    let change_prefix = format!(".ito/changes/{}", request.change_id);
    let marker_path = format!("{change_prefix}/.ito.yaml");
    let entries = git
        .list_tree(&request.repository_root, &snapshot.oid, &change_prefix)
        .map_err(|error| PrepareFailure::artifact(&marker_path, error.to_string()))?;
    let marker = read_exact_regular(git, request, &entries, &marker_path)?;
    let metadata = parse_change_meta(&marker.contents).map_err(|error| {
        PrepareFailure::validation(
            &marker_path,
            Some("ito.change-metadata.v1"),
            error.to_string(),
        )
    })?;
    let schema_name = metadata
        .schema
        .filter(|name| !name.trim().is_empty())
        .ok_or_else(|| {
            PrepareFailure::validation(
                &marker_path,
                Some("ito.change-metadata.v1"),
                "Authoritative .ito.yaml must declare a schema.",
            )
        })?;
    if !safe_schema_name(&schema_name) {
        return Err(PrepareFailure::validation(
            &marker_path,
            Some("ito.change-metadata.v1"),
            format!("Authoritative .ito.yaml declares unsafe schema name '{schema_name}'."),
        ));
    }

    let readiness_schema = load_schema(git, request, snapshot, &schema_name)?;
    let required_ids = required_artifact_ids(&readiness_schema)?;
    let mut loaded: BTreeMap<String, Vec<LoadedFile>> = BTreeMap::new();
    let mut artifact_paths = Vec::new();
    for artifact_id in required_ids {
        let artifact = readiness_schema
            .schema
            .artifacts
            .iter()
            .find(|artifact| artifact.id == artifact_id)
            .expect("required artifact IDs were validated against the schema");
        let files = load_artifact_files(git, request, &entries, &change_prefix, artifact)?;
        artifact_paths.extend(files.iter().map(|file| file.path.clone()));
        loaded.insert(artifact_id, files);
    }

    let tracking = load_tracking_file(git, request, &entries, &change_prefix, &readiness_schema)?;
    if let Some(tracking) = &tracking
        && !artifact_paths.contains(&tracking.path)
    {
        artifact_paths.push(tracking.path.clone());
    }

    validate_contents(&readiness_schema, &loaded, tracking.as_ref())?;
    validate_configured_rules_from_authority(git, request, snapshot, &readiness_schema, &entries)?;
    artifact_paths.sort();
    artifact_paths.dedup();
    Ok(PrepareProof {
        schema_name,
        artifact_paths,
    })
}

fn load_schema(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    snapshot: &AuthoritySnapshot,
    schema_name: &str,
) -> Result<ReadinessSchema, PrepareFailure> {
    let schema_path = format!(".ito/templates/schemas/{schema_name}/schema.yaml");
    let entries = git
        .list_tree(&request.repository_root, &snapshot.oid, &schema_path)
        .map_err(|error| PrepareFailure::validation(&schema_path, None, error.to_string()))?;
    let (schema_contents, project_schema) = match entries.iter().find(|e| e.path == schema_path) {
        Some(entry) if entry.is_regular_blob() => (
            git.read_blob(&request.repository_root, &entry.oid)
                .map_err(|error| {
                    PrepareFailure::validation(&schema_path, None, error.to_string())
                })?,
            true,
        ),
        Some(_) => {
            return Err(PrepareFailure::validation(
                &schema_path,
                None,
                "Authoritative project schema must be a regular Git blob.",
            ));
        }
        None => {
            let bytes = ito_templates::get_schema_file(&format!("{schema_name}/schema.yaml"))
                .ok_or_else(|| {
                    PrepareFailure::validation(
                        &schema_path,
                        None,
                        format!("Declared schema '{schema_name}' is not available in authority or embedded assets."),
                    )
                })?;
            (
                std::str::from_utf8(bytes)
                    .map_err(|error| {
                        PrepareFailure::validation(&schema_path, None, error.to_string())
                    })?
                    .to_string(),
                false,
            )
        }
    };
    let schema: SchemaYaml = serde_yaml::from_str(&schema_contents).map_err(|error| {
        PrepareFailure::validation(&schema_path, Some("ito.schema.v1"), error.to_string())
    })?;
    if schema.name != schema_name {
        return Err(PrepareFailure::validation(
            &schema_path,
            Some("ito.schema.v1"),
            format!(
                "Declared schema name '{schema_name}' does not match schema document name '{}'.",
                schema.name
            ),
        ));
    }

    let validation_path = format!(".ito/templates/schemas/{schema_name}/validation.yaml");
    let validation_contents = if project_schema {
        let validation_entries = git
            .list_tree(&request.repository_root, &snapshot.oid, &validation_path)
            .map_err(|error| {
                PrepareFailure::validation(&validation_path, None, error.to_string())
            })?;
        match validation_entries
            .iter()
            .find(|entry| entry.path == validation_path)
        {
            Some(entry) if entry.is_regular_blob() => Some(
                git.read_blob(&request.repository_root, &entry.oid)
                    .map_err(|error| {
                        PrepareFailure::validation(&validation_path, None, error.to_string())
                    })?,
            ),
            Some(_) => {
                return Err(PrepareFailure::validation(
                    &validation_path,
                    None,
                    "Authoritative project validation config must be a regular Git blob.",
                ));
            }
            None => None,
        }
    } else {
        ito_templates::get_schema_file(&format!("{schema_name}/validation.yaml"))
            .map(|bytes| {
                std::str::from_utf8(bytes)
                    .map(ToOwned::to_owned)
                    .map_err(|error| {
                        PrepareFailure::validation(&validation_path, None, error.to_string())
                    })
            })
            .transpose()?
    };
    let validation = validation_contents
        .as_deref()
        .map(|contents| {
            serde_yaml::from_str(contents).map_err(|error| {
                PrepareFailure::validation(
                    &validation_path,
                    Some("ito.schema-validation.v1"),
                    error.to_string(),
                )
            })
        })
        .transpose()?;
    Ok(ReadinessSchema {
        schema,
        validation,
        schema_path,
        schema_contents,
        validation_path,
        validation_contents,
    })
}

fn required_artifact_ids(schema: &ReadinessSchema) -> Result<Vec<String>, PrepareFailure> {
    let mut roots = schema
        .schema
        .apply
        .as_ref()
        .and_then(|apply| apply.requires.clone())
        .unwrap_or_else(|| {
            schema
                .schema
                .artifacts
                .iter()
                .filter(|artifact| !artifact.optional)
                .map(|artifact| artifact.id.clone())
                .collect()
        });
    if let Some(validation) = &schema.validation {
        roots.extend(
            validation
                .artifacts
                .iter()
                .filter(|(_, config)| config.required)
                .map(|(id, _)| id.clone()),
        );
        if validation
            .proposal
            .as_ref()
            .is_some_and(|config| config.required)
        {
            roots.push("proposal".to_string());
        }
    }

    let by_id: BTreeMap<_, _> = schema
        .schema
        .artifacts
        .iter()
        .map(|artifact| (artifact.id.as_str(), artifact))
        .collect();
    let mut required = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    for root in roots {
        visit_artifact(&root, &by_id, &mut visiting, &mut required)?;
    }
    Ok(required.into_iter().collect())
}

fn visit_artifact(
    id: &str,
    by_id: &BTreeMap<&str, &ArtifactYaml>,
    visiting: &mut BTreeSet<String>,
    required: &mut BTreeSet<String>,
) -> Result<(), PrepareFailure> {
    if required.contains(id) {
        return Ok(());
    }
    let artifact = by_id.get(id).ok_or_else(|| {
        PrepareFailure::validation(
            "schema.yaml",
            Some("ito.schema.v1"),
            format!("Schema apply prerequisites reference unknown artifact '{id}'."),
        )
    })?;
    if !visiting.insert(id.to_string()) {
        return Err(PrepareFailure::validation(
            "schema.yaml",
            Some("ito.schema.v1"),
            format!("Schema artifact dependency cycle includes '{id}'."),
        ));
    }
    for dependency in &artifact.requires {
        visit_artifact(dependency, by_id, visiting, required)?;
    }
    visiting.remove(id);
    required.insert(id.to_string());
    Ok(())
}

fn load_artifact_files(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    entries: &[GitTreeEntry],
    change_prefix: &str,
    artifact: &ArtifactYaml,
) -> Result<Vec<LoadedFile>, PrepareFailure> {
    let pattern = Pattern::new(&artifact.generates).map_err(|error| {
        PrepareFailure::validation(
            "schema.yaml",
            Some("ito.schema.v1"),
            format!(
                "Artifact '{}' has invalid output pattern: {error}",
                artifact.id
            ),
        )
    })?;
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: false,
    };
    let prefix = format!("{change_prefix}/");
    let matching: Vec<_> = entries
        .iter()
        .filter_map(|entry| {
            let relative = entry.path.strip_prefix(&prefix)?;
            pattern
                .matches_with(relative, options)
                .then_some((entry, relative))
        })
        .collect();
    if matching.is_empty() {
        return Err(PrepareFailure::artifact(
            format!("{change_prefix}/{}", artifact.generates),
            format!(
                "Apply-required artifact '{}' is absent from the authoritative Git tree.",
                artifact.id
            ),
        ));
    }

    let mut files = Vec::new();
    for (entry, _) in matching {
        if !entry.is_regular_blob() {
            return Err(PrepareFailure::artifact(
                entry.path.clone(),
                format!(
                    "Apply-required artifact '{}' must be a regular Git blob; mode '{}' is not accepted.",
                    artifact.id, entry.mode
                ),
            ));
        }
        let contents = git
            .read_blob(&request.repository_root, &entry.oid)
            .map_err(|error| PrepareFailure::artifact(&entry.path, error.to_string()))?;
        if contents.trim().is_empty() {
            return Err(PrepareFailure::validation(
                &entry.path,
                None,
                format!("Apply-required artifact '{}' is empty.", artifact.id),
            ));
        }
        files.push(LoadedFile {
            path: entry.path.clone(),
            contents,
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn read_exact_regular(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    entries: &[GitTreeEntry],
    path: &str,
) -> Result<LoadedFile, PrepareFailure> {
    let entry = entries
        .iter()
        .find(|entry| entry.path == path)
        .ok_or_else(|| {
            PrepareFailure::artifact(
                path,
                format!("Required authoritative file '{path}' is missing."),
            )
        })?;
    if !entry.is_regular_blob() {
        return Err(PrepareFailure::artifact(
            path,
            format!(
                "Required authoritative file '{path}' must be a regular Git blob; mode '{}' is not accepted.",
                entry.mode
            ),
        ));
    }
    let contents = git
        .read_blob(&request.repository_root, &entry.oid)
        .map_err(|error| PrepareFailure::artifact(path, error.to_string()))?;
    Ok(LoadedFile {
        path: path.to_string(),
        contents,
    })
}

fn load_tracking_file(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    entries: &[GitTreeEntry],
    change_prefix: &str,
    schema: &ReadinessSchema,
) -> Result<Option<LoadedFile>, PrepareFailure> {
    let required = schema
        .validation
        .as_ref()
        .and_then(|validation| validation.tracking.as_ref())
        .is_some_and(|tracking| tracking.required);
    let Some(tracks) = schema
        .schema
        .apply
        .as_ref()
        .and_then(|apply| apply.tracks.as_deref())
    else {
        if required {
            return Err(PrepareFailure::validation(
                "schema.yaml",
                Some("ito.schema-validation.v1"),
                "Schema validation requires tracking but apply.tracks is not configured.",
            ));
        }
        return Ok(None);
    };
    let path = format!("{change_prefix}/{tracks}");
    match read_exact_regular(git, request, entries, &path) {
        Ok(file) => Ok(Some(file)),
        Err(_) if !required => Ok(None),
        Err(error) => Err(error),
    }
}

fn validate_contents(
    schema: &ReadinessSchema,
    loaded: &BTreeMap<String, Vec<LoadedFile>>,
    tracking: Option<&LoadedFile>,
) -> Result<(), PrepareFailure> {
    let mut parsed_tasks = TasksParseResult::empty();
    if let Some(tracking_config) = schema
        .validation
        .as_ref()
        .and_then(|validation| validation.tracking.as_ref())
    {
        if tracking_config.source != ValidationTrackingSourceYaml::ApplyTracks {
            return Err(PrepareFailure::validation(
                "validation.yaml",
                Some("ito.schema-validation.v1"),
                "Readiness supports only apply_tracks validation sources.",
            ));
        }
        if let Some(file) = tracking {
            parsed_tasks = validate_file(file, tracking_config.validate_as)?;
        }
    }

    if let Some(validation) = &schema.validation {
        for (artifact_id, config) in &validation.artifacts {
            let Some(validator) = config.validate_as else {
                continue;
            };
            let Some(files) = loaded.get(artifact_id) else {
                if config.required {
                    return Err(PrepareFailure::validation(
                        artifact_id,
                        Some(validator.as_str()),
                        format!("Validation-required artifact '{artifact_id}' was not loaded."),
                    ));
                }
                continue;
            };
            match validator {
                ValidatorId::DeltaSpecsV1 => validate_delta_files(files, &parsed_tasks)?,
                ValidatorId::TasksTrackingV1 => {
                    for file in files {
                        parsed_tasks = validate_file(file, validator)?;
                    }
                }
            }
        }

        if let Some(proposal) = validation.proposal.as_ref()
            && let Some(validator) = proposal.validate_as
        {
            match validator {
                ValidatorId::DeltaSpecsV1 => {}
                ValidatorId::TasksTrackingV1 => {
                    let path = loaded
                        .get("proposal")
                        .and_then(|files| files.first())
                        .map(|file| file.path.as_str())
                        .unwrap_or("proposal.md");
                    return Err(PrepareFailure::validation(
                        path,
                        Some(validator.as_str()),
                        "Tasks tracking validation is not valid for proposal.md.",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_file(
    file: &LoadedFile,
    validator: ValidatorId,
) -> Result<TasksParseResult, PrepareFailure> {
    match validator {
        ValidatorId::TasksTrackingV1 => validate_tasks(file),
        ValidatorId::DeltaSpecsV1 => {
            validate_delta_files(std::slice::from_ref(file), &TasksParseResult::empty())?;
            Ok(TasksParseResult::empty())
        }
    }
}

fn validate_tasks(file: &LoadedFile) -> Result<TasksParseResult, PrepareFailure> {
    let parsed = parse_tasks_tracking_file(&file.contents);
    if parsed.tasks.is_empty() {
        return Err(PrepareFailure::validation(
            &file.path,
            Some(ValidatorId::TasksTrackingV1.as_str()),
            "Tracking file contains no recognizable tasks.",
        ));
    }
    if let Some(diagnostic) = parsed.diagnostics.first() {
        return Err(PrepareFailure::validation(
            &file.path,
            Some(ValidatorId::TasksTrackingV1.as_str()),
            format!("Strict tracking validation failed: {}", diagnostic.message),
        ));
    }
    Ok(parsed)
}

fn validate_delta_files(
    files: &[LoadedFile],
    tasks: &TasksParseResult,
) -> Result<(), PrepareFailure> {
    let delta_files: Vec<_> = files
        .iter()
        .map(|file| DeltaSpecFile {
            spec: spec_id_from_path(&file.path),
            markdown: file.contents.clone(),
        })
        .collect();
    let show = parse_change_show_json("authoritative", &delta_files);
    if show.deltas.is_empty() {
        return Err(PrepareFailure::validation(
            files
                .first()
                .map(|file| file.path.as_str())
                .unwrap_or("specs"),
            Some(ValidatorId::DeltaSpecsV1.as_str()),
            "Change must have at least one parseable delta requirement.",
        ));
    }

    let mut requirements = Vec::new();
    for delta in &show.deltas {
        if delta.description.trim().len() < 20 {
            return Err(delta_failure(files, "Delta description is too brief."));
        }
        if delta.requirements.is_empty() {
            return Err(delta_failure(files, "Delta must include requirements."));
        }
        for requirement in &delta.requirements {
            let upper = requirement.text.to_ascii_uppercase();
            if requirement.text.trim().is_empty()
                || (!upper.contains("SHALL") && !upper.contains("MUST"))
            {
                return Err(delta_failure(
                    files,
                    "Requirement text must be non-empty and contain SHALL or MUST.",
                ));
            }
            if requirement.scenarios.is_empty()
                || requirement
                    .scenarios
                    .iter()
                    .any(|scenario| scenario.raw_text.trim().is_empty())
            {
                return Err(delta_failure(
                    files,
                    "Every requirement must include a non-empty scenario.",
                ));
            }
            requirements.push((requirement.text.clone(), requirement.requirement_id.clone()));
        }
    }

    if requirements.iter().any(|(_, id)| id.is_some()) {
        let traceability = ito_domain::traceability::compute_traceability(&requirements, tasks);
        match traceability.status {
            ito_domain::traceability::TraceStatus::Invalid { missing_ids } => {
                return Err(delta_failure(
                    files,
                    format!("Requirements are missing IDs: {}", missing_ids.join(", ")),
                ));
            }
            ito_domain::traceability::TraceStatus::Unavailable { reason } => {
                return Err(delta_failure(
                    files,
                    format!("Traceability is unavailable: {reason}"),
                ));
            }
            ito_domain::traceability::TraceStatus::Ready => {}
        }
        if let Some(diagnostic) = traceability.diagnostics.first() {
            return Err(delta_failure(files, diagnostic));
        }
        if let Some(unresolved) = traceability.unresolved_references.first() {
            return Err(delta_failure(
                files,
                format!(
                    "Task '{}' references unknown requirement ID '{}'.",
                    unresolved.task_id, unresolved.requirement_id
                ),
            ));
        }
        if let Some(uncovered) = traceability.uncovered_requirements.first() {
            return Err(delta_failure(
                files,
                format!("Requirement '{uncovered}' is not covered by an active task."),
            ));
        }
    }
    Ok(())
}

fn delta_failure(files: &[LoadedFile], message: impl Into<String>) -> PrepareFailure {
    PrepareFailure::validation(
        files
            .first()
            .map(|file| file.path.as_str())
            .unwrap_or("specs"),
        Some(ValidatorId::DeltaSpecsV1.as_str()),
        message,
    )
}

fn validate_configured_rules_from_authority(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    snapshot: &AuthoritySnapshot,
    schema: &ReadinessSchema,
    change_entries: &[GitTreeEntry],
) -> Result<(), PrepareFailure> {
    let Some(validation) = schema.validation.as_ref() else {
        return Ok(());
    };
    if !has_configured_rules(validation) {
        return Ok(());
    }

    let materialized = tempfile::tempdir().map_err(|error| {
        PrepareFailure::validation(
            "authority-validation",
            None,
            format!("Failed to create authority validation workspace: {error}"),
        )
    })?;
    materialize_entries(git, request, change_entries, materialized.path())?;

    let baseline_entries = git
        .list_tree(&request.repository_root, &snapshot.oid, ".ito/specs")
        .map_err(|error| PrepareFailure::validation(".ito/specs", None, error.to_string()))?;
    materialize_entries(git, request, &baseline_entries, materialized.path())?;
    write_materialized_file(
        materialized.path(),
        &schema.schema_path,
        &schema.schema_contents,
    )?;
    if let Some(contents) = schema.validation_contents.as_deref() {
        write_materialized_file(materialized.path(), &schema.validation_path, contents)?;
    }

    let ito_path = materialized.path().join(".ito");
    let report = crate::validate::validate_configured_schema_rules(
        &ito_path,
        &request.change_id,
        &schema.schema,
        validation,
        true,
    )
    .map_err(|error| {
        PrepareFailure::validation(
            "authority-validation",
            None,
            format!("Configured authority validation failed: {error}"),
        )
    })?;
    if report.valid {
        return Ok(());
    }

    let issue = report
        .issues
        .iter()
        .find(|issue| issue.level != crate::validate::LEVEL_INFO)
        .expect("an invalid strict validation report contains an error or warning");
    let validator_code = issue
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.get("validator_id"))
        .and_then(serde_json::Value::as_str);
    Err(PrepareFailure::validation(
        issue.path.clone(),
        validator_code,
        issue.message.clone(),
    ))
}

fn has_configured_rules(validation: &ValidationYaml) -> bool {
    validation
        .artifacts
        .values()
        .any(|config| config.rules.as_ref().is_some_and(|rules| !rules.is_empty()))
        || validation
            .proposal
            .as_ref()
            .is_some_and(|config| config.rules.as_ref().is_some_and(|rules| !rules.is_empty()))
        || validation
            .tracking
            .as_ref()
            .is_some_and(|config| config.rules.as_ref().is_some_and(|rules| !rules.is_empty()))
}

fn materialize_entries(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    entries: &[GitTreeEntry],
    root: &Path,
) -> Result<(), PrepareFailure> {
    for entry in entries {
        if !entry.is_regular_blob() {
            return Err(PrepareFailure::validation(
                &entry.path,
                None,
                format!(
                    "Authority validation input must be a regular Git blob; mode '{}' is not accepted.",
                    entry.mode
                ),
            ));
        }
        let contents = git
            .read_blob(&request.repository_root, &entry.oid)
            .map_err(|error| PrepareFailure::validation(&entry.path, None, error.to_string()))?;
        write_materialized_file(root, &entry.path, &contents)?;
    }
    Ok(())
}

fn write_materialized_file(
    root: &Path,
    authority_path: &str,
    contents: &str,
) -> Result<(), PrepareFailure> {
    let authority_path = Path::new(authority_path);
    let mut components = authority_path.components();
    let Some(Component::Normal(root_component)) = components.next() else {
        return Err(PrepareFailure::validation(
            authority_path.display().to_string(),
            None,
            "Authority validation path must be a safe repository-relative .ito path.",
        ));
    };
    if root_component != ".ito"
        || components.any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(PrepareFailure::validation(
            authority_path.display().to_string(),
            None,
            "Authority validation path must be a safe repository-relative .ito path.",
        ));
    }
    let path = root.join(authority_path);
    let Some(parent) = path.parent() else {
        return Err(PrepareFailure::validation(
            authority_path.display().to_string(),
            None,
            "Authority validation path has no parent directory.",
        ));
    };
    std::fs::create_dir_all(parent).map_err(|error| {
        PrepareFailure::validation(
            authority_path.display().to_string(),
            None,
            format!("Failed to create authority validation directory: {error}"),
        )
    })?;
    std::fs::write(&path, contents).map_err(|error| {
        PrepareFailure::validation(
            authority_path.display().to_string(),
            None,
            format!("Failed to materialize authority validation file: {error}"),
        )
    })
}

fn spec_id_from_path(path: &str) -> String {
    let components: Vec<_> = path.split('/').collect();
    components
        .windows(3)
        .find(|window| window[0] == "specs")
        .map(|window| window[1].to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn safe_schema_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

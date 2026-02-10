use super::{ResolvedSchema, SchemaSource, SchemaYaml, WorkflowError};
use ito_config::ConfigContext;
use ito_templates::{get_schema_file, schema_files};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn package_schemas_dir() -> PathBuf {
    // In this repo, schemas live at the repository root.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .ancestors()
        .nth(3)
        .unwrap_or(manifest_dir.as_path());
    root.join("schemas")
}

pub(super) fn project_schemas_dir(ctx: &ConfigContext) -> Option<PathBuf> {
    Some(
        ctx.project_dir
            .as_ref()?
            .join(".ito")
            .join("templates")
            .join("schemas"),
    )
}

pub(super) fn user_schemas_dir(ctx: &ConfigContext) -> Option<PathBuf> {
    let data_home = match env::var("XDG_DATA_HOME") {
        Ok(v) if !v.trim().is_empty() => Some(PathBuf::from(v)),
        _ => ctx
            .home_dir
            .as_ref()
            .map(|h| h.join(".local").join("share")),
    }?;
    Some(data_home.join("ito").join("schemas"))
}

pub(super) fn embedded_schema_names() -> Vec<String> {
    let mut names: BTreeSet<String> = BTreeSet::new();
    for file in schema_files() {
        let mut parts = file.relative_path.split('/');
        let Some(name) = parts.next() else {
            continue;
        };
        if !name.is_empty() {
            names.insert(name.to_string());
        }
    }
    names.into_iter().collect()
}

pub(super) fn load_embedded_schema_yaml(name: &str) -> Result<Option<SchemaYaml>, WorkflowError> {
    let path = format!("{name}/schema.yaml");
    let Some(bytes) = get_schema_file(&path) else {
        return Ok(None);
    };

    let s = std::str::from_utf8(bytes).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("embedded schema is not utf-8 ({path}): {e}"),
        )
    })?;
    let schema = serde_yaml::from_str(s)?;
    Ok(Some(schema))
}

pub(super) fn read_schema_template(
    resolved: &ResolvedSchema,
    template: &str,
) -> Result<String, WorkflowError> {
    if resolved.source == SchemaSource::Embedded {
        let path = format!("{}/templates/{template}", resolved.schema.name);
        let bytes = get_schema_file(&path).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("embedded template not found: {path}"),
            )
        })?;
        let text = std::str::from_utf8(bytes).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("embedded template is not utf-8 ({path}): {e}"),
            )
        })?;
        return Ok(text.to_string());
    }

    let path = resolved.schema_dir.join("templates").join(template);
    ito_common::io::read_to_string_std(&path).map_err(WorkflowError::from)
}

#[derive(Debug, Clone)]
/// Summary of exported schema files.
pub struct ExportSchemasResult {
    /// Number of files written.
    pub written: usize,
    /// Number of existing files skipped because force was false.
    pub skipped: usize,
}

/// Export embedded schemas to a filesystem directory.
pub fn export_embedded_schemas(
    to_dir: &Path,
    force: bool,
) -> Result<ExportSchemasResult, WorkflowError> {
    let mut written = 0usize;
    let mut skipped = 0usize;

    for file in schema_files() {
        let dest = to_dir.join(file.relative_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        if dest.exists() && !force {
            skipped += 1;
            continue;
        }

        fs::write(dest, file.contents)?;
        written += 1;
    }

    Ok(ExportSchemasResult { written, skipped })
}

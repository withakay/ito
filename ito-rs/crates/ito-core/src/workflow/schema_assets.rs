use super::{ResolvedSchema, SchemaSource, SchemaYaml, WorkflowError};
use ito_config::ConfigContext;
use ito_templates::{get_schema_file, schema_files};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Repository root's `schemas` directory path.
///
/// Returns a `PathBuf` pointing to the repository root's `schemas` subdirectory.
/// The repository root is derived from `CARGO_MANIFEST_DIR` by walking up three parent
/// directories; if that ancestor is not present, the manifest directory itself is used.
///
/// # Examples
///
/// ```
/// let dir = package_schemas_dir();
/// assert!(dir.ends_with("schemas"));
/// ```
pub(super) fn package_schemas_dir() -> PathBuf {
    // In this repo, schemas live at the repository root.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .ancestors()
        .nth(3)
        .unwrap_or(manifest_dir.as_path());
    root.join("schemas")
}

/// Compute the project-specific schemas directory when a project directory is configured.
///
/// If `ctx.project_dir` is present, returns `Some(path)` where `path` is `project_dir/.ito/templates/schemas`;
/// returns `None` when no project directory is set.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use crate::workflow::schema_assets::project_schemas_dir;
/// # use crate::ConfigContext;
/// // Construct a ConfigContext with a project_dir for the example.
/// let ctx = ConfigContext { project_dir: Some(PathBuf::from("/repo")), ..Default::default() };
/// let dir = project_schemas_dir(&ctx);
/// assert_eq!(dir.unwrap(), PathBuf::from("/repo/.ito/templates/schemas"));
/// ```
pub(super) fn project_schemas_dir(ctx: &ConfigContext) -> Option<PathBuf> {
    Some(
        ctx.project_dir
            .as_ref()?
            .join(".ito")
            .join("templates")
            .join("schemas"),
    )
}

/// Resolves the per-user schemas directory using XDG conventions.
///
/// If the `XDG_DATA_HOME` environment variable is set and not empty, its value is used;
/// otherwise the function falls back to `ctx.home_dir` joined with `.local/share`.
/// When a data home can be determined, the function returns that path with `ito/schemas` appended.
/// Returns `None` if neither `XDG_DATA_HOME` nor `ctx.home_dir` are available.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// // Construct a minimal ConfigContext with a home_dir for the example.
/// let ctx = ConfigContext { home_dir: Some(PathBuf::from("/home/alice")), ..Default::default() };
/// let dir = user_schemas_dir(&ctx).unwrap();
/// assert!(dir.ends_with("ito/schemas"));
/// ```
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

/// Lists top-level embedded schema names included in the binary.
///
/// Each entry is the first path segment of an embedded file's relative path (the directory containing a schema's files).
///
/// # Returns
///
/// `Vec<String>` of unique, non-empty top-level schema names sorted in ascending order.
///
/// # Examples
///
/// ```
/// let names = embedded_schema_names();
/// assert!(names.iter().all(|n| !n.is_empty()));
/// for w in names.windows(2) {
///     assert!(w[0] <= w[1]);
/// }
/// ```
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

/// Load an embedded schema's `schema.yaml` by schema name.
///
/// Attempts to read `{name}/schema.yaml` from the embedded assets and deserialize it into
/// `SchemaYaml`.
///
/// Returns `Ok(Some(schema))` when the file exists and parses successfully, `Ok(None)` when the
/// embedded file is not present, and `Err(WorkflowError)` if the embedded bytes are not valid UTF-8
/// or if YAML deserialization (or other I/O) fails.
///
/// # Examples
///
/// ```no_run
/// let res = load_embedded_schema_yaml("example-schema").unwrap();
/// if let Some(schema) = res {
///     // use `schema`
/// }
/// ```
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

/// Load a schema template string for a resolved schema.
///
/// Loads the template from the embedded asset bundle at `{schema}/templates/{template}` when
/// the resolved schema's source is `SchemaSource::Embedded`; otherwise reads
/// `<schema_dir>/templates/<template>` from the filesystem.
///
/// Returns the template contents as a `String`. Returns a `WorkflowError` if the embedded
/// template is missing, the embedded bytes are not valid UTF-8, or a filesystem I/O error
/// occurs when reading a non-embedded template.
///
/// # Examples
///
/// ```no_run
/// // `resolved` is a ResolvedSchema obtained from your configuration or discovery logic.
/// // let content = read_schema_template(&resolved, "main.tpl")?;
/// ```
pub(super) fn read_schema_template(
    resolved: &ResolvedSchema,
    template: &str,
) -> Result<String, WorkflowError> {
    if !is_safe_relative_path(template) {
        return Err(WorkflowError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("invalid template path: {template}"),
        )));
    }

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

pub(super) fn is_safe_relative_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    if path.contains('\\') {
        return false;
    }

    let p = Path::new(path);
    if p.is_absolute() {
        return false;
    }

    for component in p.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return false;
            }
        }
    }

    true
}

pub(super) fn is_safe_schema_name(name: &str) -> bool {
    is_safe_relative_path(name) && !name.contains('.')
}

#[derive(Debug, Clone)]
/// Summary of exported schema files.
pub struct ExportSchemasResult {
    /// Number of files written.
    pub written: usize,
    /// Number of existing files skipped because force was false.
    pub skipped: usize,
}

/// Export all embedded schema files into a target directory.
///
/// Existing destination files are not overwritten unless `force` is `true`.
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

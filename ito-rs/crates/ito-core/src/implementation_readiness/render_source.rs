//! Immutable apply-input materialization from an authority commit.

use std::path::{Component, Path, PathBuf};

use tempfile::TempDir;

use crate::change_meta::parse_change_meta;

use super::ReadinessReport;
use super::git::{GitTreeEntry, ReadinessGit};

/// A temporary `.ito` tree populated exclusively from one captured authority
/// commit.
pub struct AuthoritativeChangeSource {
    root: TempDir,
    ito_path: PathBuf,
    change_id: String,
}

impl AuthoritativeChangeSource {
    /// Project root containing the materialized `.ito` directory.
    #[must_use]
    pub fn project_root(&self) -> &Path {
        self.root.path()
    }

    /// Materialized authoritative `.ito` path.
    #[must_use]
    pub fn ito_path(&self) -> &Path {
        &self.ito_path
    }

    /// Canonical change ID represented by this source.
    #[must_use]
    pub fn change_id(&self) -> &str {
        &self.change_id
    }
}

/// Failure to construct an immutable authoritative rendering source.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct AuthoritativeSourceError {
    path: Option<String>,
    message: String,
}

impl AuthoritativeSourceError {
    fn new(path: Option<String>, message: impl Into<String>) -> Self {
        Self {
            path,
            message: message.into(),
        }
    }

    /// Authority path associated with the failure, when known.
    #[must_use]
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}

pub(super) fn materialize_authoritative_change(
    prepare: &ReadinessReport,
    repository_root: &Path,
    guidance_artifacts: &[&str],
    git: &dyn ReadinessGit,
) -> Result<AuthoritativeChangeSource, AuthoritativeSourceError> {
    if !prepare.ready {
        return Err(AuthoritativeSourceError::new(
            None,
            "authoritative rendering requires a successful prepare report",
        ));
    }
    let Some(snapshot) = prepare.authority_snapshot() else {
        return Err(AuthoritativeSourceError::new(
            None,
            "authoritative rendering requires a captured authority commit",
        ));
    };
    let change_id = prepare.change_id.as_str();
    if !crate::templates::validate_change_name_input(change_id) {
        return Err(AuthoritativeSourceError::new(
            Some(change_id.to_string()),
            "authoritative rendering requires a canonical safe change ID",
        ));
    }

    let root = tempfile::tempdir().map_err(|error| {
        AuthoritativeSourceError::new(None, format!("cannot create authority workspace: {error}"))
    })?;
    let ito_path = root.path().join(".ito");
    let change_prefix = format!(".ito/changes/{change_id}");
    let entries = materialize_prefix(
        git,
        repository_root,
        &snapshot.oid,
        &change_prefix,
        root.path(),
    )?;
    let marker_path = format!("{change_prefix}/.ito.yaml");
    let marker = entries
        .iter()
        .find(|entry| entry.path == marker_path && entry.is_regular_blob())
        .ok_or_else(|| {
            AuthoritativeSourceError::new(
                Some(marker_path.clone()),
                "authority commit does not contain a regular change metadata file",
            )
        })?;
    let marker = git
        .read_blob(repository_root, &marker.oid)
        .map_err(|error| {
            AuthoritativeSourceError::new(Some(marker_path.clone()), error.to_string())
        })?;
    let metadata = parse_change_meta(&marker)
        .map_err(|error| AuthoritativeSourceError::new(Some(marker_path), error.to_string()))?;
    let schema_name = metadata
        .schema
        .filter(|schema| !schema.trim().is_empty())
        .ok_or_else(|| {
            AuthoritativeSourceError::new(
                Some(change_prefix.clone()),
                "authoritative change metadata does not declare a schema",
            )
        })?;
    if schema_name.contains('/') || schema_name.contains('\\') || schema_name.contains("..") {
        return Err(AuthoritativeSourceError::new(
            Some(change_prefix.clone()),
            format!("authoritative change metadata declares unsafe schema '{schema_name}'"),
        ));
    }
    let schema_prefix = format!(".ito/templates/schemas/{schema_name}");
    materialize_prefix(
        git,
        repository_root,
        &snapshot.oid,
        &schema_prefix,
        root.path(),
    )?;

    for artifact in guidance_artifacts {
        if !safe_artifact_id(artifact) {
            return Err(AuthoritativeSourceError::new(
                None,
                format!("unsafe guidance artifact ID '{artifact}'"),
            ));
        }
        materialize_optional_file(
            git,
            repository_root,
            &snapshot.oid,
            &format!(".ito/user-prompts/{artifact}.md"),
            root.path(),
        )?;
    }
    materialize_optional_file(
        git,
        repository_root,
        &snapshot.oid,
        ".ito/user-prompts/guidance.md",
        root.path(),
    )?;
    materialize_optional_file(
        git,
        repository_root,
        &snapshot.oid,
        ".ito/user-guidance.md",
        root.path(),
    )?;

    Ok(AuthoritativeChangeSource {
        root,
        ito_path,
        change_id: change_id.to_string(),
    })
}

fn materialize_prefix(
    git: &dyn ReadinessGit,
    repository_root: &Path,
    authority_oid: &str,
    prefix: &str,
    destination_root: &Path,
) -> Result<Vec<GitTreeEntry>, AuthoritativeSourceError> {
    let entries = git
        .list_tree(repository_root, authority_oid, prefix)
        .map_err(|error| {
            AuthoritativeSourceError::new(Some(prefix.to_string()), error.to_string())
        })?;
    for entry in &entries {
        materialize_entry(git, repository_root, entry, destination_root)?;
    }
    Ok(entries)
}

fn materialize_optional_file(
    git: &dyn ReadinessGit,
    repository_root: &Path,
    authority_oid: &str,
    path: &str,
    destination_root: &Path,
) -> Result<(), AuthoritativeSourceError> {
    let entries = git
        .list_tree(repository_root, authority_oid, path)
        .map_err(|error| {
            AuthoritativeSourceError::new(Some(path.to_string()), error.to_string())
        })?;
    for entry in &entries {
        materialize_entry(git, repository_root, entry, destination_root)?;
    }
    Ok(())
}

fn materialize_entry(
    git: &dyn ReadinessGit,
    repository_root: &Path,
    entry: &GitTreeEntry,
    destination_root: &Path,
) -> Result<(), AuthoritativeSourceError> {
    if !entry.is_regular_blob() {
        return Err(AuthoritativeSourceError::new(
            Some(entry.path.clone()),
            "authoritative rendering accepts regular Git blobs only",
        ));
    }
    let relative = Path::new(&entry.path);
    if relative.components().any(|component| match component {
        Component::Normal(_) => false,
        Component::Prefix(_) | Component::RootDir | Component::CurDir | Component::ParentDir => {
            true
        }
    }) {
        return Err(AuthoritativeSourceError::new(
            Some(entry.path.clone()),
            "authority tree contains an unsafe path",
        ));
    }
    let destination = destination_root.join(relative);
    let Some(parent) = destination.parent() else {
        return Err(AuthoritativeSourceError::new(
            Some(entry.path.clone()),
            "authority path has no parent directory",
        ));
    };
    std::fs::create_dir_all(parent).map_err(|error| {
        AuthoritativeSourceError::new(
            Some(entry.path.clone()),
            format!("cannot create authority workspace path: {error}"),
        )
    })?;
    let contents = git
        .read_blob(repository_root, &entry.oid)
        .map_err(|error| {
            AuthoritativeSourceError::new(Some(entry.path.clone()), error.to_string())
        })?;
    std::fs::write(&destination, contents).map_err(|error| {
        AuthoritativeSourceError::new(
            Some(entry.path.clone()),
            format!("cannot write authority workspace file: {error}"),
        )
    })
}

fn safe_artifact_id(artifact: &str) -> bool {
    !artifact.is_empty()
        && artifact.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

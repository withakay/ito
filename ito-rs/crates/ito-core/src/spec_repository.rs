//! Filesystem-backed promoted spec repository.

use std::path::Path;

use chrono::{DateTime, Utc};

use ito_common::paths;
use ito_domain::errors::DomainError;
use ito_domain::specs::{SpecDocument, SpecRepository, SpecSummary};

/// Filesystem-backed promoted spec repository.
pub struct FsSpecRepository<'a> {
    ito_path: &'a Path,
}

impl<'a> FsSpecRepository<'a> {
    /// Create a filesystem-backed promoted spec repository.
    pub fn new(ito_path: &'a Path) -> Self {
        Self { ito_path }
    }

    fn spec_ids(&self) -> Result<Vec<String>, DomainError> {
        let fs = ito_common::fs::StdFs;
        let mut ids = ito_domain::discovery::list_spec_dir_names(&fs, self.ito_path)?;
        ids.sort();
        Ok(ids)
    }

    fn last_modified(&self, path: &Path) -> DateTime<Utc> {
        let Ok(metadata) = std::fs::metadata(path) else {
            return Utc::now();
        };
        let Ok(modified) = metadata.modified() else {
            return Utc::now();
        };
        DateTime::<Utc>::from(modified)
    }
}

impl SpecRepository for FsSpecRepository<'_> {
    fn list(&self) -> Result<Vec<SpecSummary>, DomainError> {
        let ids = self.spec_ids()?;
        let mut specs = Vec::with_capacity(ids.len());
        for id in ids {
            let path = paths::spec_markdown_path(self.ito_path, &id);
            if !path.is_file() {
                continue;
            }
            specs.push(SpecSummary {
                id,
                path: path.clone(),
                last_modified: self.last_modified(&path),
            });
        }
        Ok(specs)
    }

    fn get(&self, id: &str) -> Result<SpecDocument, DomainError> {
        let path = paths::spec_markdown_path(self.ito_path, id);
        let markdown = ito_common::io::read_to_string(&path).map_err(|err| {
            DomainError::io(
                "reading promoted spec",
                std::io::Error::other(err.to_string()),
            )
        })?;
        Ok(SpecDocument {
            id: id.to_string(),
            path: path.clone(),
            markdown,
            last_modified: self.last_modified(&path),
        })
    }
}

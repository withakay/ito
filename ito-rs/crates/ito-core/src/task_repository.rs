//! Filesystem-backed task repository implementation.

use std::path::Path;

use ito_common::fs::{FileSystem, StdFs};
use ito_config::ConfigContext;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::{
    TaskRepository as DomainTaskRepository, TasksParseResult, is_safe_tracking_filename,
    parse_tasks_tracking_file, tasks_path_checked, tracking_path_checked,
};

use crate::templates::{default_schema_name, read_change_schema, resolve_schema};

/// Filesystem-backed implementation of the domain `TaskRepository` port.
pub struct FsTaskRepository<'a, F: FileSystem = StdFs> {
    ito_path: &'a Path,
    fs: F,
}

impl<'a> FsTaskRepository<'a, StdFs> {
    /// Create a filesystem-backed task repository using the standard filesystem.
    pub fn new(ito_path: &'a Path) -> Self {
        Self::with_fs(ito_path, StdFs)
    }
}

impl<'a, F: FileSystem> FsTaskRepository<'a, F> {
    /// Create a filesystem-backed task repository with a custom filesystem.
    pub fn with_fs(ito_path: &'a Path, fs: F) -> Self {
        Self { ito_path, fs }
    }

    /// Load tasks from an explicit change directory.
    pub(crate) fn load_tasks_from_dir(&self, change_dir: &Path) -> DomainResult<TasksParseResult> {
        let schema_name = read_schema_from_dir(&self.fs, change_dir);
        let mut ctx = ConfigContext::from_process_env();
        ctx.project_dir = self.ito_path.parent().map(|p| p.to_path_buf());

        let mut tracking_file = "tasks.md".to_string();
        if let Ok(resolved) = resolve_schema(Some(&schema_name), &ctx)
            && let Some(apply) = resolved.schema.apply.as_ref()
            && let Some(tracks) = apply.tracks.as_deref()
            && is_safe_tracking_filename(tracks)
        {
            tracking_file = tracks.to_string();
        }

        if !is_safe_tracking_filename(&tracking_file) {
            return Ok(TasksParseResult::empty());
        }

        let path = change_dir.join(&tracking_file);
        if !self.fs.is_file(&path) {
            return Ok(TasksParseResult::empty());
        }

        let contents = self
            .fs
            .read_to_string(&path)
            .map_err(|source| DomainError::io("reading tasks file", source))?;
        Ok(parse_tasks_tracking_file(&contents))
    }
}

fn read_schema_from_dir<F: FileSystem>(fs: &F, change_dir: &Path) -> String {
    let meta = crate::change_meta::read_change_meta_from_dir(fs, change_dir);
    meta.schema
        .unwrap_or_else(|| default_schema_name().to_string())
}

impl<F: FileSystem> DomainTaskRepository for FsTaskRepository<'_, F> {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        // `read_change_schema` uses `change_id` as a path segment; reject traversal.
        if tasks_path_checked(self.ito_path, change_id).is_none() {
            return Ok(TasksParseResult::empty());
        }

        let schema_name = read_change_schema(self.ito_path, change_id);
        let mut ctx = ConfigContext::from_process_env();
        ctx.project_dir = self.ito_path.parent().map(|p| p.to_path_buf());

        let mut tracking_file = "tasks.md".to_string();
        if let Ok(resolved) = resolve_schema(Some(&schema_name), &ctx)
            && let Some(apply) = resolved.schema.apply.as_ref()
            && let Some(tracks) = apply.tracks.as_deref()
        {
            tracking_file = tracks.to_string();
        }

        let Some(path) = tracking_path_checked(self.ito_path, change_id, &tracking_file) else {
            return Ok(TasksParseResult::empty());
        };
        if !self.fs.is_file(&path) {
            return Ok(TasksParseResult::empty());
        }
        let contents = self
            .fs
            .read_to_string(&path)
            .map_err(|source| DomainError::io("reading tasks file", source))?;
        Ok(parse_tasks_tracking_file(&contents))
    }
}

/// Backward-compatible alias for the default filesystem-backed task repository.
pub type TaskRepository<'a> = FsTaskRepository<'a, StdFs>;

#[cfg(test)]
#[path = "task_repository_tests.rs"]
mod task_repository_tests;

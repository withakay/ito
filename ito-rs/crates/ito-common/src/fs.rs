//! File-system abstractions.
//!
//! This crate uses a narrow `FileSystem` trait to allow unit tests to inject
//! fake implementations without touching the real disk.

use std::io;
use std::path::{Path, PathBuf};

/// A minimal file-system interface.
///
/// Prefer accepting a `&dyn FileSystem` in code that performs I/O so it can be
/// tested without relying on `std::fs`.
pub trait FileSystem: Send + Sync {
    /// Read the entire file at `path` into a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Write `contents` to `path`, creating or truncating the file.
    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()>;

    /// Return `true` if `path` exists.
    fn exists(&self, path: &Path) -> bool;

    /// Create all directories needed for `path`.
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Return the immediate children of `path`.
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;

    /// Remove a file.
    fn remove_file(&self, path: &Path) -> io::Result<()>;

    /// Remove a directory and all of its contents.
    fn remove_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Return `true` if `path` is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Return `true` if `path` is a file.
    fn is_file(&self, path: &Path) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
/// A `FileSystem` backed by the standard library's `std::fs`.
pub struct StdFs;

impl FileSystem for StdFs {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        std::fs::write(path, contents)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let entries = std::fs::read_dir(path)?;
        let mut out = Vec::new();
        for entry in entries {
            let entry = entry?;
            out.push(entry.path());
        }
        Ok(out)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_dir_all(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
}

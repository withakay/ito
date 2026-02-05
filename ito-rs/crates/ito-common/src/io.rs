//! Convenience I/O helpers.
//!
//! These functions wrap common `std::fs` operations and attach contextual error
//! messages (including the failing path) where appropriate.

use std::io::ErrorKind;
use std::path::Path;

use miette::{Result, miette};

/// Read a file into a UTF-8 string.
///
/// On error, the returned diagnostic includes the failing path.
pub fn read_to_string(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| miette!("I/O error reading {p}: {e}", p = path.display()))
}

/// Read a file into a UTF-8 string.
///
/// Missing files map to `Ok(None)`; other I/O errors are surfaced.
pub fn read_to_string_optional(path: &Path) -> Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(miette!("I/O error reading {p}: {e}", p = path.display())),
    }
}

/// Read a file into a string, returning an empty string on error.
///
/// This is intentionally lossy and should be used for best-effort output only.
pub fn read_to_string_or_default(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Write bytes to a file.
///
/// On error, the returned diagnostic includes the failing path.
pub fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
    std::fs::write(path, contents)
        .map_err(|e| miette!("I/O error writing {p}: {e}", p = path.display()))
}

/// Create a directory and any missing parent directories.
///
/// On error, the returned diagnostic includes the failing path.
pub fn create_dir_all(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .map_err(|e| miette!("I/O error creating {p}: {e}", p = path.display()))
}

/// `std::io` variant of [`read_to_string()`].
pub fn read_to_string_std(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// `std::io` variant of [`write()`].
pub fn write_std(path: &Path, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

/// `std::io` variant of [`create_dir_all`].
pub fn create_dir_all_std(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Write bytes to `path` atomically, best-effort.
///
/// The write happens via a temporary file in the same directory followed by a
/// rename.
pub fn write_atomic_std(path: &Path, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
    let Some(parent) = path.parent() else {
        return std::fs::write(path, contents);
    };
    std::fs::create_dir_all(parent)?;

    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "config".to_string());
    let tmp_name = format!(".{file_name}.tmp.{}", std::process::id());
    let tmp_path = parent.join(tmp_name);

    std::fs::write(&tmp_path, contents)?;

    #[cfg(windows)]
    {
        let _ = std::fs::remove_file(path);
    }

    let r = std::fs::rename(&tmp_path, path);
    if r.is_err() {
        let _ = std::fs::remove_file(&tmp_path);
    }
    r
}

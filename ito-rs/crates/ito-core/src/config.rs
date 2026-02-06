//! JSON configuration file CRUD operations.
//!
//! This module provides low-level functions for reading, writing, and
//! manipulating JSON configuration files with dot-delimited path navigation.

use std::path::Path;

use crate::errors::{CoreError, CoreResult};

/// Read a JSON config file, returning an empty object if the file doesn't exist.
///
/// # Errors
///
/// Returns [`CoreError::Serde`] if the file contains invalid JSON or is not a JSON object.
pub fn read_json_config(path: &Path) -> CoreResult<serde_json::Value> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    };
    let v: serde_json::Value = serde_json::from_str(&contents).map_err(|e| {
        CoreError::serde(format!("Invalid JSON in {}", path.display()), e.to_string())
    })?;
    match v {
        serde_json::Value::Object(_) => Ok(v),
        _ => Err(CoreError::serde(
            format!("Expected JSON object in {}", path.display()),
            "root value is not an object",
        )),
    }
}

/// Write a JSON value to a config file (pretty-printed with trailing newline).
///
/// # Errors
///
/// Returns [`CoreError::Serde`] if serialization fails, or [`CoreError::Io`] if writing fails.
pub fn write_json_config(path: &Path, value: &serde_json::Value) -> CoreResult<()> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|e| CoreError::serde("Failed to serialize JSON config", e.to_string()))?;
    bytes.push(b'\n');
    ito_common::io::write_atomic_std(path, bytes)
        .map_err(|e| CoreError::io(format!("Failed to write config to {}", path.display()), e))?;
    Ok(())
}

/// Parse a CLI argument as a JSON value, falling back to a string if parsing fails.
///
/// If `force_string` is true, always returns a JSON string without attempting to parse.
pub fn parse_json_value_arg(raw: &str, force_string: bool) -> serde_json::Value {
    if force_string {
        return serde_json::Value::String(raw.to_string());
    }
    match serde_json::from_str::<serde_json::Value>(raw) {
        Ok(v) => v,
        Err(_) => serde_json::Value::String(raw.to_string()),
    }
}

/// Split a dot-delimited config key path into parts, trimming and filtering empty segments.
pub fn json_split_path(key: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    for part in key.split('.') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        out.push(part);
    }
    out
}

/// Navigate a JSON object by a slice of path parts, returning the value if found.
pub fn json_get_path<'a>(
    root: &'a serde_json::Value,
    parts: &[&str],
) -> Option<&'a serde_json::Value> {
    let mut cur = root;
    for p in parts {
        let serde_json::Value::Object(map) = cur else {
            return None;
        };
        let next = map.get(*p)?;
        cur = next;
    }
    Some(cur)
}

/// Set a value at a dot-delimited path in a JSON object, creating intermediate objects as needed.
///
/// # Errors
///
/// Returns [`CoreError::Validation`] if the path is empty or if setting the path fails.
pub fn json_set_path(
    root: &mut serde_json::Value,
    parts: &[&str],
    value: serde_json::Value,
) -> CoreResult<()> {
    if parts.is_empty() {
        return Err(CoreError::validation("Invalid empty path"));
    }

    let mut cur = root;
    for (i, key) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();

        let is_object = matches!(cur, serde_json::Value::Object(_));
        if !is_object {
            *cur = serde_json::Value::Object(serde_json::Map::new());
        }

        let serde_json::Value::Object(map) = cur else {
            return Err(CoreError::validation("Failed to set path"));
        };

        if is_last {
            map.insert((*key).to_string(), value);
            return Ok(());
        }

        let needs_object = match map.get(*key) {
            Some(serde_json::Value::Object(_)) => false,
            Some(_) => true,
            None => true,
        };
        if needs_object {
            map.insert(
                (*key).to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        let Some(next) = map.get_mut(*key) else {
            return Err(CoreError::validation("Failed to set path"));
        };
        cur = next;
    }

    Ok(())
}

/// Remove a key at a dot-delimited path in a JSON object.
///
/// Returns `true` if a key was removed, `false` if the path didn't exist.
///
/// # Errors
///
/// Returns [`CoreError::Validation`] if the path is empty.
pub fn json_unset_path(root: &mut serde_json::Value, parts: &[&str]) -> CoreResult<bool> {
    if parts.is_empty() {
        return Err(CoreError::validation("Invalid empty path"));
    }

    let mut cur = root;
    for (i, p) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();
        let serde_json::Value::Object(map) = cur else {
            return Ok(false);
        };

        if is_last {
            return Ok(map.remove(*p).is_some());
        }

        let Some(next) = map.get_mut(*p) else {
            return Ok(false);
        };
        cur = next;
    }

    Ok(false)
}

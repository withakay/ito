//! Module ID parsing and normalization.

use std::fmt;

use super::IdParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A module identifier.
///
/// Modules are grouped epics (e.g. `001_project-setup`).
pub struct ModuleId(String);

impl ModuleId {
    pub(crate) fn new(inner: String) -> Self {
        Self(inner)
    }

    /// Borrow the canonical `NNN` module id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed representation of a module identifier.
pub struct ParsedModuleId {
    /// Canonical numeric id, padded to 3 digits.
    pub module_id: ModuleId,

    /// Optional module name suffix.
    pub module_name: Option<String>,
}

/// Parse a module identifier.
///
/// Accepts either `NNN` or `NNN_name` (flexible padding); the returned id is
/// always canonicalized to a 3-digit `NNN` string.
pub fn parse_module_id(input: &str) -> Result<ParsedModuleId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Module ID cannot be empty",
            Some("Provide a module ID like \"1\", \"001\", or \"001_my-module\""),
        ));
    }

    if trimmed.len() > 256 {
        return Err(IdParseError::new(
            format!("Module ID is too long: {} bytes (max 256)", trimmed.len()),
            Some("Provide a shorter module ID in the form \"NNN\" or \"NNN_name\""),
        ));
    }

    // TS: const FLEXIBLE_MODULE_PATTERN = /^(\d+)(?:_([a-z][a-z0-9-]*))?$/i;
    let (num_part, name_part) = match trimmed.split_once('_') {
        Some((left, right)) => (left, Some(right)),
        None => (trimmed, None),
    };

    let mut num_all_digits = true;
    for b in num_part.as_bytes() {
        if !b.is_ascii_digit() {
            num_all_digits = false;
            break;
        }
    }

    if num_part.is_empty() || !num_all_digits {
        return Err(IdParseError::new(
            format!("Invalid module ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN\" or \"NNN_name\" (e.g., \"1\", \"001\", \"001_my-module\")",
            ),
        ));
    }

    let num: u32 = num_part.parse().map_err(|_| {
        IdParseError::new(
            "Module ID is required",
            Some("Provide a module ID like \"1\", \"001\", or \"001_my-module\""),
        )
    })?;

    if num > 999 {
        return Err(IdParseError::new(
            format!("Module ID {num} exceeds maximum (999)"),
            Some("Module IDs must be between 0 and 999"),
        ));
    }

    let module_id = ModuleId::new(format!("{num:03}"));

    let module_name = if let Some(name) = name_part {
        if name.is_empty() {
            return Err(IdParseError::new(
                format!("Invalid module ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN\" or \"NNN_name\" (e.g., \"1\", \"001\", \"001_my-module\")",
                ),
            ));
        }

        let mut chars = name.chars();
        let first = chars.next().unwrap_or('\0');
        if !first.is_ascii_alphabetic() {
            return Err(IdParseError::new(
                format!("Invalid module ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN\" or \"NNN_name\" (e.g., \"1\", \"001\", \"001_my-module\")",
                ),
            ));
        }
        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '-') {
                return Err(IdParseError::new(
                    format!("Invalid module ID format: \"{input}\""),
                    Some(
                        "Expected format: \"NNN\" or \"NNN_name\" (e.g., \"1\", \"001\", \"001_my-module\")",
                    ),
                ));
            }
        }
        Some(name.to_ascii_lowercase())
    } else {
        None
    };

    Ok(ParsedModuleId {
        module_id,
        module_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_module_id_pads_and_lowercases_name() {
        let parsed = parse_module_id("1_Foo-Bar").unwrap();
        assert_eq!(parsed.module_id.as_str(), "001");
        assert_eq!(parsed.module_name.as_deref(), Some("foo-bar"));
    }

    #[test]
    fn parse_module_id_rejects_overflow() {
        let err = parse_module_id("1000").unwrap_err();
        assert_eq!(err.error, "Module ID 1000 exceeds maximum (999)");
    }

    #[test]
    fn parse_module_id_rejects_overlong_input() {
        let input = format!("001_{}", "a".repeat(300));
        let err = parse_module_id(&input).expect_err("overlong module id should fail");
        assert!(err.error.contains("too long"));
    }
}

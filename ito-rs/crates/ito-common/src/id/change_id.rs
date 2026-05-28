//! Change ID parsing and normalization.

use std::fmt;

use super::IdParseError;
use super::ModuleId;
use super::is_all_ascii_digits;
use super::sub_module_id::SubModuleId;

/// A change identifier in canonical form.
///
/// Supports both the legacy module format (`NNN-NN_name`, e.g.
/// `014-01_add-rust-crate-documentation`) and the sub-module format
/// (`NNN.SS-NN_name`, e.g. `014.01-03_add-jwt`).
///
/// Canonical strings are produced by [`parse_change_id`] and are always
/// zero-padded to the minimum widths (`NNN`, `SS`, `NN`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChangeId(String);

impl ChangeId {
    pub(crate) fn new(inner: String) -> Self {
        Self(inner)
    }

    /// Borrow the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed representation of a change identifier.
pub struct ParsedChangeId {
    /// Canonical module id.
    pub module_id: ModuleId,

    /// Sub-module id in canonical `NNN.SS` form, present only for sub-module changes.
    ///
    /// `None` for legacy `NNN-NN_name` format changes without a sub-module component.
    pub sub_module_id: Option<SubModuleId>,

    /// Canonical change number (at least 2 digits).
    pub change_num: String,

    /// Canonicalized change name (lowercase).
    pub name: String,

    /// Canonical `NNN-NN_name` or `NNN.SS-NN_name` string.
    pub canonical: ChangeId,
}

/// Parse a change identifier.
///
/// Accepts both the legacy `NNN-NN_name` format and the sub-module
/// `NNN.SS-NN_name` format with flexible zero-padding; always returns a
/// canonical representation.
///
/// When the input contains a sub-module component (`NNN.SS-NN_name`), the
/// returned [`ParsedChangeId`] has `sub_module_id` set to `Some(...)`.
/// Legacy `NNN-NN_name` inputs produce `sub_module_id = None`.
pub fn parse_change_id(input: &str) -> Result<ParsedChangeId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Change ID cannot be empty",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        ));
    }

    if trimmed.len() > 256 {
        return Err(IdParseError::new(
            format!("Change ID is too long: {} bytes (max 256)", trimmed.len()),
            Some("Provide a shorter change ID in the form \"NNN-NN_name\""),
        ));
    }

    // Match TS hint for the common mistake: using '_' between module and change number.
    // Example: "001_02_name" (should be "001-02_name").
    if trimmed.contains('_') && !trimmed.contains('-') {
        let mut parts = trimmed.split('_');
        let a = parts.next().unwrap_or("");
        let b = parts.next().unwrap_or("");
        let c = parts.next().unwrap_or("");

        if is_all_ascii_digits(a) && is_all_ascii_digits(b) && !c.is_empty() {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Change IDs use \"-\" between module and change number (e.g., \"001-02_name\" not \"001_02_name\")",
                ),
            ));
        }
    }

    // Split off the name suffix (everything after the first `_`).
    let Some((left, name_part)) = trimmed.split_once('_') else {
        if let Some((a, b)) = trimmed.split_once('-')
            && is_all_ascii_digits(a)
            && is_all_ascii_digits(b)
        {
            return Err(IdParseError::new(
                format!("Change ID missing name: \"{input}\""),
                Some("Change IDs require a name suffix (e.g., \"001-02_my-change\")"),
            ));
        }
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    };

    // Determine whether this is a sub-module change (`NNN.SS-NN_name`) or a
    // legacy change (`NNN-NN_name`) by checking for a `.` in the left part.
    let (module_num, sub_module_id, change_part) = if left.contains('.') {
        // Sub-module format: NNN.SS-NN
        let Some((module_sub_part, change_str)) = left.split_once('-') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some("Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")"),
            ));
        };

        let Some((module_str, sub_str)) = module_sub_part.split_once('.') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some("Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")"),
            ));
        };

        // Validate all three numeric parts.
        if !is_all_ascii_digits(module_str)
            || !is_all_ascii_digits(sub_str)
            || !is_all_ascii_digits(change_str)
        {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some("Expected format: \"NNN.SS-NN_name\" (e.g., \"005.01-02_my-change\")"),
            ));
        }

        let module_num: u32 = module_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"005.01-02_my-change\""),
            )
        })?;

        let sub_num: u32 = sub_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"005.01-02_my-change\""),
            )
        })?;

        if module_num > 999 {
            return Err(IdParseError::new(
                format!("Module number {module_num} exceeds maximum (999)"),
                Some("Module numbers must be between 0 and 999"),
            ));
        }

        if sub_num > 99 {
            return Err(IdParseError::new(
                format!("Sub-module number {sub_num} exceeds maximum (99)"),
                Some("Sub-module numbers must be between 0 and 99"),
            ));
        }

        let sub_id = SubModuleId::new(format!("{module_num:03}.{sub_num:02}"));
        (module_num, Some(sub_id), change_str)
    } else {
        // Legacy format: NNN-NN
        let Some((module_str, change_str)) = left.split_once('-') else {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        };

        if !is_all_ascii_digits(module_str) || !is_all_ascii_digits(change_str) {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        }

        let module_num: u32 = module_str.parse().map_err(|_| {
            IdParseError::new(
                "Change ID is required",
                Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
            )
        })?;

        if module_num > 999 {
            return Err(IdParseError::new(
                format!("Module number {module_num} exceeds maximum (999)"),
                Some("Module numbers must be between 0 and 999"),
            ));
        }

        (module_num, None, change_str)
    };

    let change_num: u32 = change_part.parse().map_err(|_| {
        IdParseError::new(
            "Change ID is required",
            Some("Provide a change ID like \"1-2_my-change\" or \"001-02_my-change\""),
        )
    })?;

    // NOTE: Do not enforce an upper bound for change numbers.
    // Padding is for readability/sorting only; functionality is more important.

    // Validate name
    let mut chars = name_part.chars();
    let first = chars.next().unwrap_or('\0');
    if !first.is_ascii_alphabetic() {
        return Err(IdParseError::new(
            format!("Invalid change ID format: \"{input}\""),
            Some(
                "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
            ),
        ));
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '-') {
            return Err(IdParseError::new(
                format!("Invalid change ID format: \"{input}\""),
                Some(
                    "Expected format: \"NNN-NN_name\" (e.g., \"1-2_my-change\", \"001-02_my-change\")",
                ),
            ));
        }
    }

    let module_id = ModuleId::new(format!("{module_num:03}"));
    let change_num_str = format!("{change_num:02}");
    let name = name_part.to_ascii_lowercase();

    let canonical = match &sub_module_id {
        Some(sub_id) => ChangeId::new(format!("{sub_id}-{change_num_str}_{name}")),
        None => ChangeId::new(format!("{module_id}-{change_num_str}_{name}")),
    };

    Ok(ParsedChangeId {
        module_id,
        sub_module_id,
        change_num: change_num_str,
        name,
        canonical,
    })
}

#[cfg(test)]
#[path = "change_id_tests.rs"]
mod change_id_tests;

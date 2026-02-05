use std::fmt;

use super::IdParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A spec identifier (directory name under `.ito/specs/`).
pub struct SpecId(String);

impl SpecId {
    /// Borrow the underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SpecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed representation of a spec identifier.
pub struct ParsedSpecId {
    /// The parsed spec id.
    pub spec_id: SpecId,
}

/// Parse a spec identifier.
///
/// This is intentionally permissive: any non-empty directory name is accepted
/// as a spec id.
pub fn parse_spec_id(input: &str) -> Result<ParsedSpecId, IdParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(IdParseError::new(
            "Spec ID cannot be empty",
            Some("Provide a spec ID like \"cli-init\""),
        ));
    }

    // TS accepts any directory name with a spec.md inside it. We treat the ID
    // as the directory name and do not normalize it.
    Ok(ParsedSpecId {
        spec_id: SpecId(trimmed.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spec_id_preserves_value() {
        let parsed = parse_spec_id("cli-init").unwrap();
        assert_eq!(parsed.spec_id.as_str(), "cli-init");
    }
}

use ito_templates::ITO_END_MARKER;
use std::path::Path;

use super::WorkflowError;

const ITO_INTERNAL_COMMENT_START: &str = "<!-- ITO:INTERNAL:START -->";
const ITO_INTERNAL_COMMENT_END: &str = "<!-- ITO:INTERNAL:END -->";

/// Load shared user guidance text.
///
/// Prefers `.ito/user-prompts/guidance.md`, with fallback to `.ito/user-guidance.md`.
pub fn load_user_guidance(ito_path: &Path) -> Result<Option<String>, WorkflowError> {
    let path = ito_path.join("user-prompts").join("guidance.md");
    if path.exists() {
        return load_guidance_file(&path);
    }
    let path = ito_path.join("user-guidance.md");
    load_guidance_file(&path)
}

/// Load artifact-scoped user guidance text from `.ito/user-prompts/<artifact-id>.md`.
pub fn load_user_guidance_for_artifact(
    ito_path: &Path,
    artifact_id: &str,
) -> Result<Option<String>, WorkflowError> {
    if !is_safe_artifact_id(artifact_id) {
        return Err(WorkflowError::InvalidArtifactId(artifact_id.to_string()));
    }
    let path = ito_path
        .join("user-prompts")
        .join(format!("{artifact_id}.md"));
    load_guidance_file(&path)
}

/// Compose scoped and shared user guidance into one guidance string.
///
/// When both scoped and shared guidance are present, the returned text contains:
/// - `## Scoped Guidance (<artifact_id>)`
/// - `## Shared Guidance`
///
/// If only one source exists, that content is returned. If neither exists, returns `None`.
pub fn load_composed_user_guidance(
    ito_path: &Path,
    artifact_id: &str,
) -> Result<Option<String>, WorkflowError> {
    let scoped = load_user_guidance_for_artifact(ito_path, artifact_id)?;
    let shared = load_user_guidance(ito_path)?;

    match (scoped, shared) {
        (None, None) => Ok(None),
        (Some(scoped), None) => Ok(Some(scoped)),
        (None, Some(shared)) => Ok(Some(shared)),
        (Some(scoped), Some(shared)) => Ok(Some(format!(
            "## Scoped Guidance ({artifact_id})\n\n{scoped}\n\n## Shared Guidance\n\n{shared}"
        ))),
    }
}

/// Load and normalize a guidance file from disk.
///
/// Behavior:
/// - returns `Ok(None)` if the file does not exist,
/// - normalizes CRLF to LF,
/// - removes managed content before `ITO_END_MARKER`,
/// - removes internal placeholder blocks between
///   `<!-- ITO:INTERNAL:START -->` and `<!-- ITO:INTERNAL:END -->`,
/// - trims whitespace and returns `Ok(None)` when empty.
fn load_guidance_file(path: &Path) -> Result<Option<String>, WorkflowError> {
    if !path.exists() {
        return Ok(None);
    }

    let content = ito_common::io::read_to_string_std(path)?;
    let content = content.replace("\r\n", "\n");
    let content = match content.find(ITO_END_MARKER) {
        Some(i) => &content[i + ITO_END_MARKER.len()..],
        None => content.as_str(),
    };
    let content = strip_ito_internal_comment_blocks(content);
    let content = content.trim();
    if content.is_empty() {
        return Ok(None);
    }

    Ok(Some(content.to_string()))
}

fn strip_ito_internal_comment_blocks(content: &str) -> String {
    let mut out = String::new();
    let mut in_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if in_block {
            if trimmed == ITO_INTERNAL_COMMENT_END {
                in_block = false;
            }
            continue;
        }

        if trimmed == ITO_INTERNAL_COMMENT_START {
            in_block = true;
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }

    out
}

fn is_safe_artifact_id(artifact_id: &str) -> bool {
    if artifact_id.is_empty() || artifact_id.contains("..") {
        return false;
    }

    for c in artifact_id.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::strip_ito_internal_comment_blocks;

    #[test]
    fn strip_ito_internal_comment_blocks_removes_internal_template_guidance() {
        let contents = r#"
Keep this.
<!-- ITO:INTERNAL:START -->
## Your Guidance
(placeholder)
<!-- ITO:INTERNAL:END -->
Keep this too.
"#;

        let stripped = strip_ito_internal_comment_blocks(contents);
        assert!(stripped.contains("Keep this."));
        assert!(stripped.contains("Keep this too."));
        assert!(!stripped.contains("## Your Guidance"));
        assert!(!stripped.contains("(placeholder)"));
    }
}

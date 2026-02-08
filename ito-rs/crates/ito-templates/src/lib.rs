//! Embedded templates and assets installed by `ito init` / `ito update`.
//!
//! `ito-templates` packages the default project and home templates (plus shared
//! skills/adapters/commands) as embedded assets.
//!
//! The Rust CLI writes these files to disk, optionally rewriting `.ito/` path
//! prefixes when users configure a custom Ito directory name.

#![warn(missing_docs)]

use std::borrow::Cow;

use include_dir::{Dir, include_dir};

/// Embedded agent definitions.
pub mod agents;

/// Embedded instruction artifacts.
pub mod instructions;

/// Jinja2 rendering for project templates (AGENTS.md, skills).
pub mod project_templates;

static DEFAULT_PROJECT_DIR: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/default/project");
static DEFAULT_HOME_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/default/home");
static SKILLS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/skills");
static ADAPTERS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/adapters");
static COMMANDS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/commands");
static AGENTS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/agents");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A file embedded in the `ito-templates` assets.
pub struct EmbeddedFile {
    /// Path relative to the template root directory.
    pub relative_path: &'static str,
    /// Raw file contents.
    pub contents: &'static [u8],
}

/// Return all embedded files for the default project template.
pub fn default_project_files() -> Vec<EmbeddedFile> {
    dir_files(&DEFAULT_PROJECT_DIR)
}

/// Return all embedded files for the default home template.
pub fn default_home_files() -> Vec<EmbeddedFile> {
    dir_files(&DEFAULT_HOME_DIR)
}

/// Return all embedded shared skill files.
pub fn skills_files() -> Vec<EmbeddedFile> {
    dir_files(&SKILLS_DIR)
}

/// Return all embedded harness adapter files.
pub fn adapters_files() -> Vec<EmbeddedFile> {
    dir_files(&ADAPTERS_DIR)
}

/// Get a specific skill file by path (e.g., "brainstorming/SKILL.md")
pub fn get_skill_file(path: &str) -> Option<&'static [u8]> {
    SKILLS_DIR.get_file(path).map(|f| f.contents())
}

/// Get a specific adapter file by path (e.g., "claude/session-start.sh")
pub fn get_adapter_file(path: &str) -> Option<&'static [u8]> {
    ADAPTERS_DIR.get_file(path).map(|f| f.contents())
}

/// Return all embedded shared command files.
pub fn commands_files() -> Vec<EmbeddedFile> {
    dir_files(&COMMANDS_DIR)
}

/// Get a specific command file by path (e.g., "ito-apply.md")
pub fn get_command_file(path: &str) -> Option<&'static [u8]> {
    COMMANDS_DIR.get_file(path).map(|f| f.contents())
}

fn dir_files(dir: &'static Dir<'static>) -> Vec<EmbeddedFile> {
    let mut out = Vec::new();
    collect_dir_files(dir, &mut out);
    out
}

fn collect_dir_files(dir: &'static Dir<'static>, out: &mut Vec<EmbeddedFile>) {
    for f in dir.files() {
        out.push(EmbeddedFile {
            relative_path: f.path().to_str().unwrap_or_default(),
            contents: f.contents(),
        });
    }

    for d in dir.dirs() {
        collect_dir_files(d, out);
    }
}

/// Normalize an Ito directory name to the dotted form (e.g. `.ito`).
///
/// Empty inputs default to `.ito`. Non-dotted names are prefixed with `.`.
pub fn normalize_ito_dir(ito_dir: &str) -> String {
    if ito_dir.is_empty() {
        return ".ito".to_string();
    }
    if ito_dir.starts_with('.') {
        ito_dir.to_string()
    } else {
        format!(".{ito_dir}")
    }
}

/// Rewrite a relative template path for a custom Ito directory.
///
/// When `ito_dir` is `.ito`, this returns `rel` unchanged.
pub fn render_rel_path<'a>(rel: &'a str, ito_dir: &str) -> Cow<'a, str> {
    if ito_dir == ".ito" {
        return Cow::Borrowed(rel);
    }
    if let Some(rest) = rel.strip_prefix(".ito/") {
        return Cow::Owned(format!("{ito_dir}/{rest}"));
    }
    Cow::Borrowed(rel)
}

/// Rewrite file bytes for a custom Ito directory.
///
/// This performs a best-effort UTF-8 rewrite of `.ito/` path occurrences.
pub fn render_bytes<'a>(bytes: &'a [u8], ito_dir: &str) -> Cow<'a, [u8]> {
    if ito_dir == ".ito" {
        return Cow::Borrowed(bytes);
    }

    let Ok(s) = std::str::from_utf8(bytes) else {
        return Cow::Borrowed(bytes);
    };

    // Match TS replaceHardcodedDotItoPaths: replace `.ito/` occurrences.
    let out = s.replace(".ito/", &format!("{ito_dir}/"));
    Cow::Owned(out.into_bytes())
}

/// Start marker for Ito-managed file blocks.
pub const ITO_START_MARKER: &str = "<!-- ITO:START -->";

/// End marker for Ito-managed file blocks.
pub const ITO_END_MARKER: &str = "<!-- ITO:END -->";

/// Extract the substring between [`ITO_START_MARKER`] and [`ITO_END_MARKER`].
///
/// Returns `None` if the markers are not present *on their own lines*.
pub fn extract_managed_block(text: &str) -> Option<&str> {
    let start = find_marker_index(text, ITO_START_MARKER, 0)?;
    let end = find_marker_index(text, ITO_END_MARKER, start + ITO_START_MARKER.len())?;
    let after_start = line_end(text, start + ITO_START_MARKER.len());
    let before_end = line_start(text, end);
    if before_end < after_start {
        return Some("");
    }

    // TS `updateFileWithMarkers` writes:
    //   start + "\n" + content + "\n" + end
    // The substring between markers therefore always ends with the *separator* newline
    // immediately before the end marker line. We want to recover the original `content`
    // argument, so we drop exactly one trailing line break.
    let mut inner = &text[after_start..before_end];
    if inner.ends_with('\n') {
        inner = &inner[..inner.len() - 1];
        if inner.ends_with('\r') {
            inner = &inner[..inner.len() - 1];
        }
    }
    Some(inner)
}

fn line_start(text: &str, idx: usize) -> usize {
    let bytes = text.as_bytes();
    let mut i = idx;
    while i > 0 {
        if bytes[i - 1] == b'\n' {
            break;
        }
        i -= 1;
    }
    i
}

fn line_end(text: &str, idx: usize) -> usize {
    let bytes = text.as_bytes();
    let mut i = idx;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            return i + 1;
        }
        i += 1;
    }
    i
}

fn is_marker_on_own_line(content: &str, marker_index: usize, marker_len: usize) -> bool {
    let bytes = content.as_bytes();

    let mut i = marker_index;
    while i > 0 {
        let c = bytes[i - 1];
        if c == b'\n' {
            break;
        }
        if c != b' ' && c != b'\t' && c != b'\r' {
            return false;
        }
        i -= 1;
    }

    let mut j = marker_index + marker_len;
    while j < bytes.len() {
        let c = bytes[j];
        if c == b'\n' {
            break;
        }
        if c != b' ' && c != b'\t' && c != b'\r' {
            return false;
        }
        j += 1;
    }

    true
}

fn find_marker_index(content: &str, marker: &str, from_index: usize) -> Option<usize> {
    let mut search_from = from_index;
    while let Some(rel) = content.get(search_from..)?.find(marker) {
        let idx = search_from + rel;
        if is_marker_on_own_line(content, idx, marker.len()) {
            return Some(idx);
        }
        search_from = idx + marker.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_ito_dir_prefixes_dot() {
        assert_eq!(normalize_ito_dir(".ito"), ".ito");
        assert_eq!(normalize_ito_dir("ito"), ".ito");
        assert_eq!(normalize_ito_dir(".x"), ".x");
    }

    #[test]
    fn render_rel_path_rewrites_ito_prefix() {
        assert_eq!(render_rel_path(".ito/AGENTS.md", ".ito"), ".ito/AGENTS.md");
        assert_eq!(render_rel_path(".ito/AGENTS.md", ".x"), ".x/AGENTS.md");
        assert_eq!(render_rel_path("AGENTS.md", ".x"), "AGENTS.md");
    }

    #[test]
    fn render_bytes_rewrites_dot_ito_paths() {
        let b = render_bytes(b"see .ito/AGENTS.md", ".x");
        assert_eq!(std::str::from_utf8(&b).unwrap(), "see .x/AGENTS.md");
    }

    #[test]
    fn extract_managed_block_returns_inner_content() {
        let s = "pre\n<!-- ITO:START -->\nhello\nworld\n<!-- ITO:END -->\npost\n";
        assert_eq!(extract_managed_block(s), Some("hello\nworld"));
    }

    #[test]
    fn extract_managed_block_preserves_trailing_newline_from_content() {
        // Content ends with a newline, plus the TS separator newline before the end marker.
        let s = "pre\n<!-- ITO:START -->\nhello\nworld\n\n<!-- ITO:END -->\npost\n";
        assert_eq!(extract_managed_block(s), Some("hello\nworld\n"));
    }

    #[test]
    fn default_project_files_contains_expected_files() {
        let files = default_project_files();
        assert!(!files.is_empty());

        let mut has_user_guidance = false;
        for EmbeddedFile {
            relative_path,
            contents,
        } in files
        {
            if relative_path == ".ito/user-guidance.md" {
                has_user_guidance = true;
                let contents = std::str::from_utf8(contents).expect("template should be UTF-8");
                assert!(contents.contains(ITO_START_MARKER));
                assert!(contents.contains(ITO_END_MARKER));
            }
        }

        assert!(
            has_user_guidance,
            "expected .ito/user-guidance.md in templates"
        );
    }

    #[test]
    fn default_home_files_returns_a_vec() {
        // The default home templates may be empty, but should still be loadable.
        let _ = default_home_files();
    }

    #[test]
    fn normalize_ito_dir_empty_defaults_to_dot_ito() {
        assert_eq!(normalize_ito_dir(""), ".ito");
    }

    #[test]
    fn render_bytes_returns_borrowed_when_no_rewrite_needed() {
        let b = b"see .ito/AGENTS.md";
        let out = render_bytes(b, ".ito");
        assert_eq!(out.as_ref(), b);

        let b = b"no ito path";
        let out = render_bytes(b, ".x");
        assert_eq!(out.as_ref(), b);
    }

    #[test]
    fn render_bytes_preserves_non_utf8() {
        let b = [0xff, 0x00, 0x41];
        let out = render_bytes(&b, ".x");
        assert_eq!(out.as_ref(), &b);
    }

    #[test]
    fn extract_managed_block_rejects_inline_markers() {
        let s = "pre <!-- ITO:START -->\nhello\n<!-- ITO:END -->\n";
        assert_eq!(extract_managed_block(s), None);
    }

    #[test]
    fn extract_managed_block_returns_empty_for_empty_inner() {
        let s = "<!-- ITO:START -->\n<!-- ITO:END -->\n";
        assert_eq!(extract_managed_block(s), Some(""));
    }
}

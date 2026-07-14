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

/// Legacy Ito-managed paths from previous releases.
pub mod legacy;

/// Expected Ito-managed install manifest generation.
pub mod manifest;

/// Jinja2 rendering for project templates (AGENTS.md, skills).
pub mod project_templates;

#[cfg(test)]
mod wiki_tests;

static DEFAULT_PROJECT_DIR: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/default/project");
static DEFAULT_HOME_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/default/home");
static SKILLS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/skills");
static ADAPTERS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/adapters");
static COMMANDS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/commands");

/// Shared command asset that invokes the legacy-coordination recovery instruction.
pub const MIGRATE_TO_MAIN_COMMAND_PATH: &str = "ito-migrate-to-main.md";
static AGENTS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/agents");
static SCHEMAS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/schemas");
static PRESETS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/presets");

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

/// Retrieves an embedded skill file by its path within the skills assets.
///
/// The `path` should be the file's path relative to the skills root (for example
/// "brainstorming/SKILL.md").
///
/// # Returns
///
/// `Some(&[u8])` with the file contents if a file exists at `path`, `None` otherwise.
///
/// # Examples
///
/// ```
/// use ito_templates::get_skill_file;
/// let contents = get_skill_file("brainstorming/SKILL.md");
/// if let Some(bytes) = contents {
///     assert!(!bytes.is_empty());
/// }
/// ```
pub fn get_skill_file(path: &str) -> Option<&'static [u8]> {
    SKILLS_DIR.get_file(path).map(|f| f.contents())
}

/// Retrieves an embedded adapter file by its relative path within the adapters assets.
///
/// Returns `Some(&[u8])` with the file contents if the path exists, `None` otherwise.
///
/// # Examples
///
/// ```
/// use ito_templates::get_adapter_file;
/// let bytes = get_adapter_file("claude/session-start.sh").expect("adapter exists");
/// assert!(!bytes.is_empty());
/// ```
pub fn get_adapter_file(path: &str) -> Option<&'static [u8]> {
    ADAPTERS_DIR.get_file(path).map(|f| f.contents())
}

/// Lists embedded shared command files.
///
/// Returns a vector of `EmbeddedFile` entries for every file embedded under the commands asset directory,
/// each with a `relative_path` (path relative to the commands root) and `contents`.
///
/// # Examples
///
/// ```
/// use ito_templates::commands_files;
/// let files = commands_files();
/// // every entry has a non-empty relative path and contents
/// assert!(files.iter().all(|f| !f.relative_path.is_empty() && !f.contents.is_empty()));
/// ```
pub fn commands_files() -> Vec<EmbeddedFile> {
    dir_files(&COMMANDS_DIR)
}

/// Lists embedded workflow schema files.
///
/// Each entry contains the file's path relative to the schema root and its raw contents.
///
/// # Examples
///
/// ```
/// use ito_templates::schema_files;
/// let files = schema_files();
/// assert!(files.iter().all(|f| !f.relative_path.is_empty() && !f.contents.is_empty()));
/// ```
pub fn schema_files() -> Vec<EmbeddedFile> {
    dir_files(&SCHEMAS_DIR)
}

/// Lists embedded workflow preset files.
///
/// Each entry contains the file's path relative to the presets root and its raw contents.
pub fn presets_files() -> Vec<EmbeddedFile> {
    dir_files(&PRESETS_DIR)
}

/// Returns the contents of an embedded preset file identified by its path relative to the presets root.
///
/// The `path` is relative to the embedded presets directory, for example `"orchestrate/rust.yaml"`.
pub fn get_preset_file(path: &str) -> Option<&'static [u8]> {
    PRESETS_DIR.get_file(path).map(|f| f.contents())
}

/// Returns the contents of an embedded schema file identified by its path relative to the schemas root.
///
/// The `path` is relative to the embedded schemas directory, for example `"spec-driven/schema.yaml"`.
///
/// # Returns
///
/// `Some(&[u8])` with the file contents if a matching embedded schema exists, `None` otherwise.
///
/// # Examples
///
/// ```
/// use ito_templates::get_schema_file;
/// let bytes = get_schema_file("spec-driven/schema.yaml").expect("schema should exist");
/// assert!(!bytes.is_empty());
/// ```
pub fn get_schema_file(path: &str) -> Option<&'static [u8]> {
    SCHEMAS_DIR.get_file(path).map(|f| f.contents())
}

/// Fetches the contents of an embedded command file by its path relative to the commands asset root.
///
/// # Returns
///
/// `Some(&[u8])` with the file contents if a file at `path` exists, `None` otherwise.
///
/// # Examples
///
/// ```rust
/// use ito_templates::get_command_file;
/// let contents = get_command_file("ito-apply.md");
/// if let Some(bytes) = contents {
///     assert!(!bytes.is_empty());
/// }
/// ```
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
    let ito_dir = ito_dir.trim();
    if ito_dir.is_empty() {
        return ".ito".to_string();
    }

    if !is_safe_ito_dir_name(ito_dir) {
        return ".ito".to_string();
    }

    if ito_dir.starts_with('.') {
        ito_dir.to_string()
    } else {
        format!(".{ito_dir}")
    }
}

fn is_safe_ito_dir_name(ito_dir: &str) -> bool {
    if ito_dir.len() > 128 {
        return false;
    }
    if ito_dir.contains('/') || ito_dir.contains('\\') || ito_dir.contains("..") {
        return false;
    }

    for c in ito_dir.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
            continue;
        }
        return false;
    }

    true
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

/// Canonical writer prefix for the version stamp (no internal whitespace).
///
/// Combined with the semver and [`ITO_VERSION_MARKER_SUFFIX`], this produces
/// a stamp line of the form `<!--ITO:VERSION:1.2.3-->`. Readers tolerate
/// whitespace variants via [`stamp_version`]'s recognition logic; the writer
/// always emits the tight canonical form.
pub const ITO_VERSION_MARKER_PREFIX: &str = "<!--ITO:VERSION:";

/// Canonical writer suffix for the version stamp.
pub const ITO_VERSION_MARKER_SUFFIX: &str = "-->";

/// Inject (or refresh) the Ito version stamp inside a managed-block file.
///
/// Behaviour:
///
/// - If `content` does not contain [`ITO_START_MARKER`] on its own line, the
///   input is returned unchanged.
/// - If the line immediately following `<!-- ITO:START -->` already contains
///   the canonical stamp `<!--ITO:VERSION:<version>-->`, the input is returned
///   byte-identical (idempotent).
/// - If that line contains a stamp in any recognised form (tight or with
///   surrounding whitespace, e.g. `<!-- ITO:VERSION: 1.2.3 -->`) but with a
///   different version OR a non-canonical shape, the line is rewritten in the
///   canonical writer form for `version`.
/// - If that line is anything else (no stamp), a new canonical stamp line is
///   inserted between `<!-- ITO:START -->` and the existing first line of the
///   managed block.
///
/// The rest of the file is preserved byte-for-byte.
pub fn stamp_version(content: &str, version: &str) -> String {
    let Some(start) = find_marker_index(content, ITO_START_MARKER, 0) else {
        return content.to_string();
    };
    let after_start_line = line_end(content, start + ITO_START_MARKER.len());

    let next_line_end = next_line_break(content, after_start_line);
    let next_line = &content[after_start_line..next_line_end];
    let canonical = format!("{ITO_VERSION_MARKER_PREFIX}{version}{ITO_VERSION_MARKER_SUFFIX}");

    if is_stamp_line(next_line) {
        // A stamp is already there (any whitespace shape). The canonical
        // writer form has no leading whitespace, so we only consider the line
        // already-canonical when its leading bytes are bare and trailing
        // whitespace (carriage return, spaces, tabs) is absent. Anything else
        // — leading indent, internal spaces, an old version — gets rewritten
        // to the canonical form and the file then stabilises on the next run.
        if next_line.trim_end_matches(['\r', ' ', '\t']) == canonical {
            return content.to_string();
        }
        let line_break_end = line_end(content, next_line_end);
        let mut out = String::with_capacity(content.len());
        out.push_str(&content[..after_start_line]);
        out.push_str(&canonical);
        out.push_str(&content[next_line_end..line_break_end]);
        if line_break_end == next_line_end {
            out.push('\n');
        }
        out.push_str(&content[line_break_end..]);
        return out;
    }

    let mut out = String::with_capacity(content.len() + canonical.len() + 1);
    out.push_str(&content[..after_start_line]);
    out.push_str(&canonical);
    out.push('\n');
    out.push_str(&content[after_start_line..]);
    out
}

/// Recognise an Ito version stamp line in any whitespace shape.
///
/// Accepts `<!--ITO:VERSION:1.2.3-->`, `<!-- ITO:VERSION: 1.2.3 -->`, and any
/// surrounding leading/trailing whitespace on the line. Rejects unrelated HTML
/// comments and rejects empty version tokens. Tooling that wants the captured
/// version should match the line against the regex
/// `<!--\s*ITO:VERSION:\s*([^>\s]+)\s*-->` separately.
fn is_stamp_line(line: &str) -> bool {
    let trimmed = line.trim();
    let Some(body) = trimmed
        .strip_prefix("<!--")
        .and_then(|s| s.strip_suffix("-->"))
    else {
        return false;
    };
    let body = body.trim();
    let Some(rest) = body.strip_prefix("ITO") else {
        return false;
    };
    let Some(rest) = rest.trim_start().strip_prefix(':') else {
        return false;
    };
    let Some(rest) = rest.trim_start().strip_prefix("VERSION") else {
        return false;
    };
    let Some(version) = rest.trim_start().strip_prefix(':').map(str::trim) else {
        return false;
    };
    !version.is_empty()
}

/// Return the byte index of the next `\n`, or `text.len()` if none.
fn next_line_break(text: &str, from: usize) -> usize {
    let bytes = text.as_bytes();
    let mut i = from;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            return i;
        }
        i += 1;
    }
    i
}

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
mod agent_surface_tests;

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;

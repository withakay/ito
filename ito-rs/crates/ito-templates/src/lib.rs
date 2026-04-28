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
    fn schema_files_contains_builtins() {
        let files = schema_files();
        assert!(!files.is_empty());
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == "spec-driven/schema.yaml")
        );
        assert!(files.iter().any(|f| f.relative_path == "tdd/schema.yaml"));
    }

    #[test]
    fn get_schema_file_returns_contents() {
        let file = get_schema_file("spec-driven/schema.yaml").expect("schema should exist");
        let text = std::str::from_utf8(file).expect("schema should be utf8");
        assert!(text.contains("name: spec-driven"));
    }

    #[test]
    fn presets_files_contains_orchestrate_builtins() {
        let files = presets_files();
        assert!(!files.is_empty());
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == "orchestrate/generic.yaml"),
            "expected orchestrate/generic.yaml in presets"
        );
    }

    #[test]
    fn get_preset_file_returns_contents() {
        let file = get_preset_file("orchestrate/generic.yaml").expect("preset should exist");
        let text = std::str::from_utf8(file).expect("preset should be utf8");
        assert!(text.contains("name:"));
    }

    #[test]
    fn loop_skill_template_includes_yaml_frontmatter() {
        let file = get_skill_file("ito-loop/SKILL.md").expect("loop skill should exist");
        let text = std::str::from_utf8(file).expect("skill should be utf8");
        assert!(text.starts_with("---\nname: ito-loop\n"));
        assert!(text.contains(
            "description: Run an ito ralph loop for a change, module, or repo-ready sequence, with safe defaults and automatic restart context on early exits."
        ));
        assert!(text.contains("restart at most **2** times"));
        assert!(text.contains("ito ralph --no-interactive --change <change-id> --status"));
        assert!(text.contains("\n---\n\n<!-- ITO:START -->"));
    }

    #[test]
    fn loop_command_template_uses_ito_loop_command_name() {
        let file = commands_files()
            .into_iter()
            .find(|f| f.relative_path == "ito-loop.md")
            .expect("loop command should exist");
        let text = std::str::from_utf8(file.contents).expect("command should be utf8");
        assert!(text.starts_with("---\nname: ito-loop\n"));
        assert!(text.contains("/ito-loop"));
        assert!(text.contains("continue ready work across the repo"));
    }

    #[test]
    fn tmux_skill_and_scripts_are_embedded() {
        let skill = get_skill_file("ito-tmux/SKILL.md").expect("ito-tmux skill should exist");
        let skill_text = std::str::from_utf8(skill).expect("skill should be utf8");
        assert!(skill_text.starts_with("---\nname: ito-tmux\n"));
        assert!(skill_text.contains("tmux -S \"$SOCKET\" send-keys"));
        assert!(skill_text.contains("wait-for-text.sh -S \"$SOCKET\""));

        let wait_for_text =
            get_skill_file("ito-tmux/scripts/wait-for-text.sh").expect("wait-for-text script");
        let wait_for_text = std::str::from_utf8(wait_for_text).expect("script should be utf8");
        assert!(wait_for_text.contains("-S|--socket-path"));
        assert!(wait_for_text.contains("tmux_cmd+=(-S \"$socket_path\")"));

        let files = skills_files();
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == "ito-tmux/scripts/wait-for-text.sh"),
            "expected wait-for-text helper script to be embedded"
        );
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == "ito-tmux/scripts/find-sessions.sh"),
            "expected find-sessions helper script to be embedded"
        );
    }

    #[test]
    fn fix_and_feature_commands_are_embedded() {
        let files = commands_files();
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == "ito-proposal-intake.md")
        );
        assert!(files.iter().any(|f| f.relative_path == "ito-fix.md"));
        assert!(files.iter().any(|f| f.relative_path == "ito-feature.md"));
    }

    #[test]
    fn orchestrate_skills_and_command_are_embedded() {
        let orchestrate =
            get_skill_file("ito-orchestrate/SKILL.md").expect("ito-orchestrate skill");
        let orchestrate = std::str::from_utf8(orchestrate).expect("utf8");
        assert!(orchestrate.starts_with("---\nname: ito-orchestrate\n"));

        let setup =
            get_skill_file("ito-orchestrate-setup/SKILL.md").expect("ito-orchestrate-setup skill");
        let setup = std::str::from_utf8(setup).expect("utf8");
        assert!(setup.starts_with("---\nname: ito-orchestrate-setup\n"));

        let workflow = get_skill_file("ito-orchestrator-workflow/SKILL.md")
            .expect("ito-orchestrator-workflow skill");
        let workflow = std::str::from_utf8(workflow).expect("utf8");
        assert!(workflow.starts_with("---\nname: ito-orchestrator-workflow\n"));

        let commands = commands_files();
        assert!(
            commands
                .iter()
                .any(|f| f.relative_path == "ito-orchestrate.md"),
            "expected ito-orchestrate command to be embedded"
        );
    }

    #[test]
    fn default_project_includes_orchestrate_user_prompt() {
        let files = default_project_files();
        assert!(
            files
                .iter()
                .any(|f| f.relative_path == ".ito/user-prompts/orchestrate.md"),
            "expected default project orchestrate.md prompt stub"
        );
    }

    #[test]
    fn orchestrator_agent_templates_are_embedded_for_all_harnesses() {
        use crate::agents::{Harness, get_agent_files};

        for harness in Harness::all() {
            let files = get_agent_files(*harness);
            if *harness == Harness::OpenCode {
                assert!(
                    files.iter().any(|(name, _)| *name == "ito-orchestrator.md"),
                    "expected ito-orchestrator.md in OpenCode agent templates"
                );
            }

            let expected = match harness {
                Harness::Codex => [
                    "ito-orchestrator/SKILL.md",
                    "ito-orchestrator-planner/SKILL.md",
                    "ito-orchestrator-researcher/SKILL.md",
                    "ito-orchestrator-worker/SKILL.md",
                    "ito-orchestrator-reviewer/SKILL.md",
                ],
                Harness::OpenCode | Harness::ClaudeCode | Harness::GitHubCopilot | Harness::Pi => [
                    "ito-orchestrator.md",
                    "ito-orchestrator-planner.md",
                    "ito-orchestrator-researcher.md",
                    "ito-orchestrator-worker.md",
                    "ito-orchestrator-reviewer.md",
                ],
            };

            for expected in expected {
                assert!(
                    files.iter().any(|(name, _)| *name == expected),
                    "expected {expected} in {harness:?} agent templates"
                );
            }
        }
    }

    #[test]
    fn proposal_intake_and_routing_skills_are_embedded() {
        let intake = get_skill_file("ito-proposal-intake/SKILL.md")
            .expect("proposal intake skill should exist");
        let intake_text = std::str::from_utf8(intake).expect("skill should be utf8");
        assert!(intake_text.starts_with("---\nname: ito-proposal-intake\n"));

        let fix = get_skill_file("ito-fix/SKILL.md").expect("fix skill should exist");
        let fix_text = std::str::from_utf8(fix).expect("skill should be utf8");
        assert!(fix_text.starts_with("---\nname: ito-fix\n"));

        let feature = get_skill_file("ito-feature/SKILL.md").expect("feature skill should exist");
        let feature_text = std::str::from_utf8(feature).expect("skill should be utf8");
        assert!(feature_text.starts_with("---\nname: ito-feature\n"));
    }

    #[test]
    fn memory_skill_is_embedded() {
        let skill = get_skill_file("ito-memory/SKILL.md").expect("ito-memory skill should exist");
        let text = std::str::from_utf8(skill).expect("skill should be utf8");
        assert!(text.starts_with("---\nname: ito-memory\n"));
        assert!(text.contains("ito agent instruction memory-capture"));
        assert!(text.contains("ito agent instruction memory-search"));
        assert!(text.contains("ito agent instruction memory-query"));
    }

    #[test]
    fn default_project_agents_mentions_fix_and_feature_entrypoints() {
        let agents = default_project_files()
            .into_iter()
            .find(|f| f.relative_path == ".ito/AGENTS.md")
            .expect("expected .ito/AGENTS.md in templates");
        let text = std::str::from_utf8(agents.contents).expect("template should be UTF-8");

        assert!(text.contains("`ito-fix`"));
        assert!(text.contains("`ito-feature`"));
        assert!(text.contains("`ito-brainstorming`"));
        assert!(text.contains("ito patch change <id> proposal"));
        assert!(text.contains("ito write change <id> design"));
    }

    #[test]
    fn agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts() {
        use crate::agents::{Harness, get_agent_files};

        let expected = [
            "ito-general",
            "ito-quick",
            "ito-thinking",
            "ito-orchestrator-worker",
        ];

        for harness in Harness::all() {
            let files = get_agent_files(*harness);
            for name in expected {
                let path = if *harness == Harness::Codex {
                    format!("{name}/SKILL.md")
                } else {
                    format!("{name}.md")
                };
                let contents = files
                    .iter()
                    .find(|(file_name, _)| *file_name == path)
                    .unwrap_or_else(|| {
                        panic!("missing agent template {path} for harness {:?}", harness)
                    })
                    .1;
                let text = std::str::from_utf8(contents).expect("template should be utf8");
                assert!(
                    text.contains("ito patch"),
                    "missing ito patch guidance in {path}"
                );
                assert!(
                    text.contains("ito write"),
                    "missing ito write guidance in {path}"
                );
            }
        }
    }

    #[test]
    fn normalize_ito_dir_empty_defaults_to_dot_ito() {
        assert_eq!(normalize_ito_dir(""), ".ito");
    }

    #[test]
    fn normalize_ito_dir_rejects_traversal_and_path_separators() {
        assert_eq!(normalize_ito_dir("../escape"), ".ito");
        assert_eq!(normalize_ito_dir("a/b"), ".ito");
        assert_eq!(normalize_ito_dir("a\\b"), ".ito");
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

    // -------- stamp_version --------

    #[test]
    fn stamp_version_noop_without_marker() {
        let s = "no markers here\n";
        assert_eq!(stamp_version(s, "1.2.3"), s);
    }

    #[test]
    fn stamp_version_inserts_when_missing() {
        let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3");
        assert_eq!(
            out,
            "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
        );
    }

    #[test]
    fn stamp_version_idempotent_on_canonical_match() {
        let s = "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n";
        assert_eq!(stamp_version(s, "1.2.3"), s);
    }

    #[test]
    fn stamp_version_rewrites_spaced_form_to_canonical() {
        let s = "<!-- ITO:START -->\n<!-- ITO:VERSION: 1.2.3 -->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3");
        assert_eq!(
            out,
            "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
        );
    }

    #[test]
    fn stamp_version_rewrites_older_version() {
        let s = "<!-- ITO:START -->\n<!--ITO:VERSION:0.9.0-->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3");
        assert_eq!(
            out,
            "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
        );
    }

    #[test]
    fn stamp_version_preserves_frontmatter() {
        let s = "---\nname: foo\n---\n\n<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3");
        assert_eq!(
            out,
            "---\nname: foo\n---\n\n<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
        );
    }

    #[test]
    fn stamp_version_preserves_trailing_content() {
        let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\nepilogue line\n";
        let out = stamp_version(s, "9.9.9");
        assert!(out.ends_with("<!-- ITO:END -->\nepilogue line\n"));
        assert!(out.contains("<!--ITO:VERSION:9.9.9-->"));
    }

    #[test]
    fn stamp_version_handles_prerelease_semver() {
        let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3-asd");
        assert!(out.contains("<!--ITO:VERSION:1.2.3-asd-->"));
    }

    #[test]
    fn stamp_version_idempotent_on_canonical_with_trailing_whitespace() {
        // Trailing CR / spaces / tabs after the canonical stamp should still
        // be treated as canonical so the file is not rewritten.
        let s = "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->  \nbody\n<!-- ITO:END -->\n";
        assert_eq!(stamp_version(s, "1.2.3"), s);
    }

    #[test]
    fn stamp_version_canonical_with_leading_whitespace_is_rewritten() {
        // Leading whitespace makes the line non-canonical even if the version
        // matches; the writer normalises to the tight form on next run.
        let s = "<!-- ITO:START -->\n  <!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n";
        let out = stamp_version(s, "1.2.3");
        assert_eq!(
            out,
            "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
        );
        // And then it's stable.
        assert_eq!(stamp_version(&out, "1.2.3"), out);
    }

    #[test]
    fn stamp_version_handles_crlf_line_endings() {
        let s = "<!-- ITO:START -->\r\nbody\r\n<!-- ITO:END -->\r\n";
        let out = stamp_version(s, "1.2.3");
        assert!(out.contains("<!--ITO:VERSION:1.2.3-->"));
        // Re-stamp must be a no-op even though the surrounding line endings
        // are CRLF.
        assert_eq!(stamp_version(&out, "1.2.3"), out);
    }

    #[test]
    fn stamp_version_round_trip_on_real_skill() {
        let bytes = get_skill_file("ito-feature/SKILL.md").expect("ito-feature skill exists");
        let text = std::str::from_utf8(bytes).expect("skill is utf8");
        let stamped = stamp_version(text, "1.2.3");
        let restamped = stamp_version(&stamped, "1.2.3");
        assert_eq!(stamped, restamped, "stamping must be idempotent");
        assert!(stamped.contains("<!--ITO:VERSION:1.2.3-->"));
        assert_eq!(
            stamped.matches("<!--ITO:VERSION:").count(),
            1,
            "exactly one stamp must be present"
        );
    }

    // -------- bundle invariants --------

    #[test]
    fn every_shipped_skill_has_ito_prefix() {
        let mut violations: Vec<&'static str> = Vec::new();
        for f in skills_files() {
            let Some(top) = f.relative_path.split('/').next() else {
                continue;
            };
            if top == "ito" || top.starts_with("ito-") {
                continue;
            }
            violations.push(f.relative_path);
        }
        assert!(
            violations.is_empty(),
            "skills missing `ito-` prefix: {violations:?}"
        );
    }

    #[test]
    fn every_shipped_command_has_ito_prefix() {
        let mut violations: Vec<&'static str> = Vec::new();
        for f in commands_files() {
            let Some(name) = f.relative_path.split('/').next_back() else {
                continue;
            };
            let stem = name.strip_suffix(".md").unwrap_or(name);
            if stem == "ito" || stem.starts_with("ito-") {
                continue;
            }
            violations.push(f.relative_path);
        }
        assert!(
            violations.is_empty(),
            "commands missing `ito-` prefix: {violations:?}"
        );
    }

    #[test]
    fn every_shipped_agent_has_ito_prefix() {
        let mut violations: Vec<String> = Vec::new();
        let agent_dirs = AGENTS_DIR.dirs();
        for harness_dir in agent_dirs {
            for entry_file in harness_dir.files() {
                let Some(name) = entry_file.path().file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                let stem = name
                    .strip_suffix(".md")
                    .or_else(|| name.strip_suffix(".md.j2"))
                    .unwrap_or(name);
                if stem.starts_with("ito-") {
                    continue;
                }
                violations.push(entry_file.path().display().to_string());
            }
            for nested in harness_dir.dirs() {
                let Some(name) = nested.path().file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                if name.starts_with("ito-") {
                    continue;
                }
                violations.push(nested.path().display().to_string());
            }
        }
        assert!(
            violations.is_empty(),
            "agents missing `ito-` prefix: {violations:?}"
        );
    }

    fn count_marker_on_own_line(text: &str, marker: &str) -> usize {
        let mut from = 0;
        let mut count = 0;
        while let Some(idx) = find_marker_index(text, marker, from) {
            count += 1;
            from = idx + marker.len();
        }
        count
    }

    #[test]
    fn every_shipped_markdown_has_exactly_one_marker_pair() {
        let mut violations: Vec<String> = Vec::new();
        let mut all = Vec::new();
        all.extend(default_project_files());
        all.extend(default_home_files());
        all.extend(skills_files());
        all.extend(adapters_files());
        all.extend(commands_files());
        all.extend(schema_files());
        all.extend(presets_files());
        all.extend(dir_files(&AGENTS_DIR));
        for f in all {
            if !f.relative_path.ends_with(".md") {
                continue;
            }
            let Ok(text) = std::str::from_utf8(f.contents) else {
                continue;
            };
            let starts = count_marker_on_own_line(text, ITO_START_MARKER);
            let ends = count_marker_on_own_line(text, ITO_END_MARKER);
            if starts != 1 || ends != 1 {
                violations.push(format!("{}: starts={starts} ends={ends}", f.relative_path));
            }
        }
        assert!(
            violations.is_empty(),
            "expected exactly one ITO:START and one ITO:END per shipped markdown:\n  {}",
            violations.join("\n  ")
        );
    }

    #[test]
    fn every_shipped_markdown_has_managed_markers() {
        let mut missing_start: Vec<&'static str> = Vec::new();
        let mut missing_end: Vec<&'static str> = Vec::new();

        let bundles: [&[EmbeddedFile]; 0] = [];
        let _ = bundles;

        let collect = || -> Vec<EmbeddedFile> {
            let mut all = Vec::new();
            all.extend(default_project_files());
            all.extend(default_home_files());
            all.extend(skills_files());
            all.extend(adapters_files());
            all.extend(commands_files());
            all.extend(schema_files());
            all.extend(presets_files());
            all.extend(dir_files(&AGENTS_DIR));
            all
        };

        for f in collect() {
            if !f.relative_path.ends_with(".md") {
                continue;
            }
            let Ok(text) = std::str::from_utf8(f.contents) else {
                continue;
            };
            if find_marker_index(text, ITO_START_MARKER, 0).is_none() {
                missing_start.push(f.relative_path);
            }
            if find_marker_index(text, ITO_END_MARKER, 0).is_none() {
                missing_end.push(f.relative_path);
            }
        }

        assert!(
            missing_start.is_empty() && missing_end.is_empty(),
            "markdown assets missing managed markers — start: {missing_start:?}, end: {missing_end:?}"
        );
    }
}

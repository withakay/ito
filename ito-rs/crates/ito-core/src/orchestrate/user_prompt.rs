use crate::errors::CoreError;
use crate::orchestrate::types::{OrchestrateUserPrompt, OrchestrateUserPromptFrontMatter};
use std::path::Path;

const FRONT_MATTER_DELIM: &str = "---";

/// Load `.ito/user-prompts/orchestrate.md` and parse its front matter + sections.
///
/// Returns [`CoreError::NotFound`] if the file is absent, or a wrapped I/O
/// error if the file exists but cannot be read.
pub fn load_orchestrate_user_prompt(ito_path: &Path) -> Result<OrchestrateUserPrompt, CoreError> {
    let path = ito_path.join("user-prompts").join("orchestrate.md");

    let raw = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(CoreError::NotFound(format!(
                "missing orchestrate user prompt: {}",
                path.display()
            )));
        }
        Err(err) => {
            return Err(CoreError::io(
                format!("reading orchestrate user prompt: {}", path.display()),
                err,
            ));
        }
    };

    let (front_matter, body) = parse_front_matter(&raw)?;
    let (must, prefer, notes) = parse_sections(&body);

    Ok(OrchestrateUserPrompt {
        path,
        raw,
        front_matter,
        must,
        prefer,
        notes,
    })
}

fn parse_front_matter(raw: &str) -> Result<(OrchestrateUserPromptFrontMatter, String), CoreError> {
    let Some(rest) = raw.strip_prefix(FRONT_MATTER_DELIM) else {
        return Ok(default_front_matter_result(raw));
    };

    let Some(rest) = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))
    else {
        return Ok(default_front_matter_result(raw));
    };

    let Some(end_pos) = find_closing_delimiter(rest) else {
        return Ok(default_front_matter_result(raw));
    };

    let yaml_block = &rest[..end_pos];
    // Skip past the closing `---` line, including any trailing whitespace
    // and the line terminator. Using the next newline (or EOF) as the body
    // anchor avoids drifting into the body when the delimiter line has
    // trailing spaces (e.g. "---   \n").
    let body_start = rest[end_pos..]
        .find('\n')
        .map(|offset| end_pos + offset + 1)
        .unwrap_or(rest.len());
    let body = rest.get(body_start..).unwrap_or("").to_string();

    let front_matter: OrchestrateUserPromptFrontMatter = serde_yaml::from_str(yaml_block)
        .map_err(|e| CoreError::Parse(format!("invalid orchestrate front matter: {e}")))?;

    Ok((front_matter, body))
}

fn default_front_matter_result(raw: &str) -> (OrchestrateUserPromptFrontMatter, String) {
    (OrchestrateUserPromptFrontMatter::default(), raw.to_string())
}

fn find_closing_delimiter(text: &str) -> Option<usize> {
    let mut pos = 0;
    for line in text.lines() {
        if line.trim() == FRONT_MATTER_DELIM {
            return Some(pos);
        }
        pos += line.len();
        if text[pos..].starts_with("\r\n") {
            pos += 2;
        } else if text[pos..].starts_with('\n') {
            pos += 1;
        }
    }
    None
}

fn parse_sections(body: &str) -> (String, String, String) {
    let mut must = String::new();
    let mut prefer = String::new();
    let mut notes = String::new();

    enum Section {
        None,
        Must,
        Prefer,
        Notes,
    }

    let mut section = Section::None;
    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            section = match heading.trim() {
                "MUST" => Section::Must,
                "PREFER" => Section::Prefer,
                "Notes" => Section::Notes,
                _ => Section::None,
            };
            continue;
        }

        match section {
            Section::Must => push_section_line(&mut must, line),
            Section::Prefer => push_section_line(&mut prefer, line),
            Section::Notes => push_section_line(&mut notes, line),
            Section::None => {}
        }
    }

    (
        must.trim().to_string(),
        prefer.trim().to_string(),
        notes.trim().to_string(),
    )
}

fn push_section_line(section: &mut String, line: &str) {
    section.push_str(line);
    section.push('\n');
}

use std::borrow::Cow;

use crate::errors::{CoreError, CoreResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeltaKind {
    Added,
    Modified,
    Removed,
    Renamed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RequirementBlock {
    name: String,
    markdown: String,
}

pub(super) fn reconcile_spec(base: Option<&str>, delta: &str) -> CoreResult<Option<String>> {
    let normalized_base = base.map(normalize_newlines);
    let normalized_delta = normalize_newlines(delta);
    let base = normalized_base.as_deref();
    let delta = normalized_delta.as_ref();
    validate_requirement_headings(delta, "delta")?;
    if let Some(base) = base {
        validate_requirement_headings(base, "current spec")?;
    }
    let mut requirements = base.map(requirement_blocks).unwrap_or_default();
    if let Some(base) = base {
        validate_current_spec(base, &requirements)?;
    }
    let delta_blocks = delta_requirement_blocks(delta)?;
    let renames = renamed_requirements(delta)?;
    if delta_blocks.is_empty() && renames.is_empty() {
        return Err(CoreError::validation(
            "delta contains no valid requirement operations".to_string(),
        ));
    }
    let initial_requirement_count = requirements.len();
    let mut removed_requirements = 0usize;
    apply_renames(&mut requirements, &renames)?;

    for (kind, block) in delta_blocks {
        let existing = requirements
            .iter()
            .position(|candidate| candidate.name == block.name);
        match kind {
            DeltaKind::Added => {
                if existing.is_some() {
                    return Err(CoreError::validation(format!(
                        "cannot add duplicate requirement '{}'",
                        block.name
                    )));
                }
                requirements.push(block);
            }
            DeltaKind::Modified => {
                let Some(index) = existing else {
                    return Err(CoreError::validation(format!(
                        "cannot modify missing requirement '{}'",
                        block.name
                    )));
                };
                requirements[index] = block;
            }
            DeltaKind::Removed => {
                let Some(index) = existing else {
                    return Err(CoreError::validation(format!(
                        "cannot remove missing requirement '{}'",
                        block.name
                    )));
                };
                requirements.remove(index);
                removed_requirements += 1;
            }
            DeltaKind::Renamed => unreachable!("renames are parsed separately"),
        }
    }

    if requirements.is_empty() {
        if initial_requirement_count > 0 && removed_requirements > 0 {
            return Ok(None);
        }
        return Err(CoreError::validation(
            "delta produced an empty spec without removing existing requirements".to_string(),
        ));
    }

    let source = base.unwrap_or(delta);
    let prefix = document_prefix(source);
    let managed = source.contains(ito_templates::ITO_START_MARKER)
        || delta.contains(ito_templates::ITO_START_MARKER);
    let prefix = prefix
        .strip_prefix(ito_templates::ITO_START_MARKER)
        .unwrap_or(prefix)
        .trim_start();
    let mut output = String::new();
    if managed {
        output.push_str(ito_templates::ITO_START_MARKER);
        output.push_str("\n\n");
    }
    if !prefix.is_empty() {
        output.push_str(prefix);
        output.push_str("\n\n");
    }
    output.push_str("## Requirements\n\n");
    output.push_str(
        &requirements
            .iter()
            .map(|requirement| requirement.markdown.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n\n"),
    );
    output.push('\n');
    if managed {
        output.push_str(ito_templates::ITO_END_MARKER);
        output.push('\n');
    }
    Ok(Some(output))
}

fn normalize_newlines(document: &str) -> Cow<'_, str> {
    if document.contains('\r') {
        Cow::Owned(document.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        Cow::Borrowed(document)
    }
}

fn validate_requirement_headings(document: &str, source: &str) -> CoreResult<()> {
    for line in document.lines() {
        let line = line.trim();
        if line.starts_with("### Requirement") && requirement_name(line).is_none() {
            return Err(CoreError::validation(format!(
                "{source} contains malformed requirement heading '{line}'"
            )));
        }
    }
    Ok(())
}

fn apply_renames(
    requirements: &mut [RequirementBlock],
    renames: &[(String, String)],
) -> CoreResult<()> {
    for (from, to) in renames {
        let Some(index) = requirements
            .iter()
            .position(|requirement| requirement.name.as_str() == from.as_str())
        else {
            return Err(CoreError::validation(format!(
                "cannot rename missing requirement '{from}'"
            )));
        };
        if requirements
            .iter()
            .enumerate()
            .any(|(candidate, requirement)| {
                candidate != index && requirement.name.as_str() == to.as_str()
            })
        {
            return Err(CoreError::validation(format!(
                "cannot rename requirement '{from}' to duplicate name '{to}'"
            )));
        }
        let old_header = requirements[index]
            .markdown
            .lines()
            .next()
            .expect("requirement block has a header");
        requirements[index].markdown =
            requirements[index]
                .markdown
                .replacen(old_header, &format!("### Requirement: {to}"), 1);
        requirements[index].name = to.clone();
    }
    Ok(())
}

fn validate_current_spec(base: &str, requirements: &[RequirementBlock]) -> CoreResult<()> {
    if base.lines().any(|line| section_kind(line).is_some()) {
        return Err(CoreError::validation(
            "current spec contains delta section headings; reconcile it before archiving another change"
                .to_string(),
        ));
    }
    if !requirements.is_empty() && !base.lines().any(|line| line.trim() == "## Requirements") {
        return Err(CoreError::validation(
            "current spec requirements are missing the '## Requirements' section".to_string(),
        ));
    }
    let unique = requirements
        .iter()
        .map(|requirement| requirement.name.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if unique.len() != requirements.len() {
        return Err(CoreError::validation(
            "current spec contains duplicate requirement headings".to_string(),
        ));
    }
    Ok(())
}

fn requirement_blocks(document: &str) -> Vec<RequirementBlock> {
    let lines = document.lines().collect::<Vec<_>>();
    let starts = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| requirement_name(line).map(|_| index))
        .collect::<Vec<_>>();
    starts
        .iter()
        .enumerate()
        .filter_map(|(position, start)| {
            let end = starts.get(position + 1).copied().unwrap_or(lines.len());
            let mut end = end;
            while end > *start
                && (lines[end - 1].trim().is_empty()
                    || lines[end - 1].trim() == ito_templates::ITO_END_MARKER
                    || section_kind(lines[end - 1]).is_some())
            {
                end -= 1;
            }
            let name = requirement_name(lines[*start])?;
            Some(RequirementBlock {
                name,
                markdown: lines[*start..end].join("\n").trim().to_string(),
            })
        })
        .collect()
}

fn delta_requirement_blocks(delta: &str) -> CoreResult<Vec<(DeltaKind, RequirementBlock)>> {
    let mut kind = None;
    let mut section_items = 0usize;
    let lines = delta.lines().collect::<Vec<_>>();
    let mut output = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if is_level_two_heading(lines[index]) {
            ensure_non_rename_section_has_payload(kind, section_items)?;
            let heading = lines[index].trim();
            kind = section_kind(heading);
            if kind.is_none() && looks_like_requirement_operation_heading(heading) {
                return Err(CoreError::validation(format!(
                    "unrecognized requirement operation heading '{heading}'"
                )));
            }
            section_items = 0;
            index += 1;
            continue;
        }
        let Some(name) = requirement_name(lines[index]) else {
            index += 1;
            continue;
        };
        let Some(block_kind) = kind else {
            return Err(CoreError::validation(format!(
                "requirement '{name}' is outside a delta section"
            )));
        };
        if block_kind == DeltaKind::Renamed {
            return Err(CoreError::validation(
                "RENAMED Requirements must use FROM:/TO: pairs".to_string(),
            ));
        }
        let start = index;
        index += 1;
        while index < lines.len()
            && requirement_name(lines[index]).is_none()
            && !is_level_two_heading(lines[index])
            && lines[index].trim() != ito_templates::ITO_END_MARKER
        {
            index += 1;
        }
        output.push((
            block_kind,
            RequirementBlock {
                name,
                markdown: lines[start..index].join("\n").trim().to_string(),
            },
        ));
        section_items += 1;
    }
    ensure_non_rename_section_has_payload(kind, section_items)?;
    Ok(output)
}

fn ensure_non_rename_section_has_payload(
    kind: Option<DeltaKind>,
    section_items: usize,
) -> CoreResult<()> {
    if matches!(
        kind,
        Some(DeltaKind::Added | DeltaKind::Modified | DeltaKind::Removed)
    ) && section_items == 0
    {
        return Err(CoreError::validation(format!(
            "{} section contains no valid requirement payload",
            delta_kind_name(kind.expect("matched kind"))
        )));
    }
    Ok(())
}

fn delta_kind_name(kind: DeltaKind) -> &'static str {
    match kind {
        DeltaKind::Added => "ADDED Requirements",
        DeltaKind::Modified => "MODIFIED Requirements",
        DeltaKind::Removed => "REMOVED Requirements",
        DeltaKind::Renamed => "RENAMED Requirements",
    }
}

fn renamed_requirements(delta: &str) -> CoreResult<Vec<(String, String)>> {
    let mut in_renamed = false;
    let mut from = None;
    let mut section_pairs = 0usize;
    let mut output = Vec::new();
    for line in delta.lines() {
        if is_level_two_heading(line) {
            ensure_rename_section_complete(in_renamed, section_pairs, from.as_deref())?;
            in_renamed = section_kind(line) == Some(DeltaKind::Renamed);
            from = None;
            section_pairs = 0;
            continue;
        }
        if !in_renamed {
            continue;
        }
        let normalized = line.trim().trim_start_matches('-').trim().trim_matches('`');
        if let Some(value) = normalized.strip_prefix("FROM:") {
            let value = clean_rename_name(value);
            if value.is_empty() {
                return Err(CoreError::validation(
                    "RENAMED Requirements contains an empty FROM:".to_string(),
                ));
            }
            if from.replace(value).is_some() {
                return Err(CoreError::validation(
                    "RENAMED Requirements contains consecutive FROM: entries".to_string(),
                ));
            }
        } else if let Some(value) = normalized.strip_prefix("TO:") {
            let value = clean_rename_name(value);
            if value.is_empty() {
                return Err(CoreError::validation(
                    "RENAMED Requirements contains an empty TO:".to_string(),
                ));
            }
            let Some(from) = from.take() else {
                return Err(CoreError::validation(
                    "RENAMED Requirements contains TO: without FROM:".to_string(),
                ));
            };
            output.push((from, value));
            section_pairs += 1;
        } else if !normalized.is_empty()
            && normalized != ito_templates::ITO_START_MARKER
            && normalized != ito_templates::ITO_END_MARKER
        {
            return Err(CoreError::validation(format!(
                "RENAMED Requirements contains malformed entry '{normalized}'"
            )));
        }
    }
    ensure_rename_section_complete(in_renamed, section_pairs, from.as_deref())?;
    Ok(output)
}

fn ensure_rename_section_complete(
    in_renamed: bool,
    section_pairs: usize,
    pending_from: Option<&str>,
) -> CoreResult<()> {
    if !in_renamed {
        return Ok(());
    }
    if pending_from.is_some() {
        return Err(CoreError::validation(
            "RENAMED Requirements contains FROM: without TO:".to_string(),
        ));
    }
    if section_pairs == 0 {
        return Err(CoreError::validation(
            "RENAMED Requirements section contains no complete FROM:/TO: pair".to_string(),
        ));
    }
    Ok(())
}

fn clean_rename_name(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_start_matches("### Requirement:")
        .trim()
        .to_string()
}

fn requirement_name(line: &str) -> Option<String> {
    let line = line.trim();
    line.strip_prefix("### Requirement:")
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
}

fn section_kind(line: &str) -> Option<DeltaKind> {
    match line.trim() {
        "## ADDED Requirements" => Some(DeltaKind::Added),
        "## MODIFIED Requirements" => Some(DeltaKind::Modified),
        "## REMOVED Requirements" => Some(DeltaKind::Removed),
        "## RENAMED Requirements" => Some(DeltaKind::Renamed),
        _ => None,
    }
}

fn is_level_two_heading(line: &str) -> bool {
    line.trim().starts_with("## ")
}

fn looks_like_requirement_operation_heading(line: &str) -> bool {
    let heading = line
        .trim()
        .strip_prefix("## ")
        .unwrap_or_default()
        .to_ascii_uppercase();
    heading.ends_with("REQUIREMENTS")
        || ["ADDED", "MODIFIED", "REMOVED", "RENAMED"]
            .iter()
            .any(|operation| heading.contains(operation))
}

fn document_prefix(document: &str) -> &str {
    let end = document
        .lines()
        .scan(0usize, |offset, line| {
            let start = *offset;
            *offset += line.len() + 1;
            Some((start, line))
        })
        .find_map(|(offset, line)| {
            (section_kind(line).is_some() || line.trim() == "## Requirements").then_some(offset)
        });
    let Some(end) = end else {
        return document
            .trim()
            .trim_end_matches(ito_templates::ITO_END_MARKER)
            .trim_end();
    };
    document[..end]
        .trim()
        .trim_end_matches(ito_templates::ITO_END_MARKER)
        .trim_end()
}

#[cfg(test)]
#[path = "archive_specs_tests.rs"]
mod archive_specs_tests;

use std::path::Path;

use crate::errors::{CoreError, CoreResult};

pub(super) fn update_agent_model_field(path: &Path, model: &str) -> CoreResult<()> {
    update_agent_yaml_field(path, "model", &format!("\"{model}\""))
}

pub(super) fn update_agent_activation_field_from_rendered(
    path: &Path,
    rendered: &[u8],
) -> CoreResult<()> {
    let Ok(rendered) = std::str::from_utf8(rendered) else {
        return Ok(());
    };
    let Some(activation) = frontmatter_field(rendered, "activation") else {
        return Ok(());
    };

    update_agent_yaml_field(path, "activation", activation)
}

pub(super) fn remove_agent_mode_field_for_direct_activation(
    path: &Path,
    rendered: &[u8],
) -> CoreResult<()> {
    let Ok(rendered) = std::str::from_utf8(rendered) else {
        return Ok(());
    };
    let Some(activation) = frontmatter_field(rendered, "activation") else {
        return Ok(());
    };
    if activation != "direct" {
        return Ok(());
    }

    remove_agent_yaml_field(path, "mode")
}

fn update_agent_yaml_field(path: &Path, key: &str, value: &str) -> CoreResult<()> {
    let content = ito_common::io::read_to_string_or_default(path);
    let Some((frontmatter, body)) = split_frontmatter(&content) else {
        return Ok(());
    };

    let frontmatter = update_yaml_field(frontmatter, key, value);
    let updated = format!("---{frontmatter}\n---{body}");
    ito_common::io::write_std(path, updated)
        .map_err(|e| CoreError::io(format!("writing {}", path.display()), e))
}

fn remove_agent_yaml_field(path: &Path, key: &str) -> CoreResult<()> {
    let content = ito_common::io::read_to_string_or_default(path);
    let Some((frontmatter, body)) = split_frontmatter(&content) else {
        return Ok(());
    };

    let frontmatter = remove_yaml_field(frontmatter, key);
    let updated = format!("---{frontmatter}\n---{body}");
    ito_common::io::write_std(path, updated)
        .map_err(|e| CoreError::io(format!("writing {}", path.display()), e))
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let rest = content.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    Some((&rest[..end], &rest[end + 4..]))
}

fn frontmatter_field<'a>(content: &'a str, key: &str) -> Option<&'a str> {
    let (frontmatter, _) = split_frontmatter(content)?;
    frontmatter.lines().find_map(|line| {
        let line = line.trim_start();
        let value = line.strip_prefix(key)?.trim_start();
        value.strip_prefix(':').map(str::trim)
    })
}

fn update_yaml_field(yaml: &str, key: &str, value: &str) -> String {
    let mut lines: Vec<String> = yaml.lines().map(str::to_string).collect();
    let prefix = format!("{key}:");

    for line in &mut lines {
        if line.trim_start().starts_with(&prefix) {
            *line = format!("{key}: {value}");
            return lines.join("\n");
        }
    }

    lines.push(format!("{key}: {value}"));
    lines.join("\n")
}

fn remove_yaml_field(yaml: &str, key: &str) -> String {
    let had_trailing_newline = yaml.ends_with('\n');
    let mut lines = Vec::new();
    let mut removing = false;

    for line in yaml.lines() {
        if removing {
            if line.starts_with(' ') || line.starts_with('\t') || line.is_empty() {
                continue;
            }
            removing = false;
        }

        if is_top_level_yaml_key(line, key) {
            removing = true;
            continue;
        }

        lines.push(line);
    }

    let mut yaml = lines.join("\n");
    if had_trailing_newline && !yaml.is_empty() {
        yaml.push('\n');
    }
    yaml
}

fn is_top_level_yaml_key(line: &str, key: &str) -> bool {
    if line.starts_with(' ') || line.starts_with('\t') {
        return false;
    }

    let Some((candidate, _)) = line.split_once(':') else {
        return false;
    };
    candidate == key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_agent_model_field_updates_frontmatter_when_present() {
        let td = tempfile::tempdir().unwrap();
        let path = td.path().join("agent.md");
        std::fs::write(&path, "---\nname: test\nmodel: \"old\"\n---\nbody\n").unwrap();
        update_agent_model_field(&path, "new").unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert!(s.contains("model: \"new\""));

        let path = td.path().join("no-frontmatter.md");
        std::fs::write(&path, "no frontmatter\n").unwrap();
        update_agent_model_field(&path, "newer").unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert_eq!(s, "no frontmatter\n");
    }

    #[test]
    fn activation_field_is_copied_from_rendered_template() {
        let td = tempfile::tempdir().unwrap();
        let path = td.path().join("agent.md");
        std::fs::write(&path, "---\nname: test\n---\nbody\n").unwrap();
        update_agent_activation_field_from_rendered(
            &path,
            b"---\nname: test\nactivation: delegated\n---\nrendered\n",
        )
        .unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert!(s.contains("activation: delegated"));
    }

    #[test]
    fn mode_field_is_removed_for_direct_activation() {
        let td = tempfile::tempdir().unwrap();
        let path = td.path().join("agent.md");
        std::fs::write(
            &path,
            "---\nname: test\nmode: subagent\nmode_extra: keep\nnested:\n  mode: keep\nmodel: old\n---\nbody\n",
        )
        .unwrap();
        remove_agent_mode_field_for_direct_activation(
            &path,
            b"---\nname: test\nactivation: direct\n---\nrendered\n",
        )
        .unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert!(!s.contains("mode: subagent"));
        assert!(s.contains("mode_extra: keep"));
        assert!(s.contains("  mode: keep"));
        assert!(s.contains("model: old"));
    }

    #[test]
    fn mode_field_removal_drops_continuation_lines() {
        let yaml = "name: test\nmode: |\n  stale\n  stale again\nmodel: old";
        let updated = remove_yaml_field(yaml, "mode");
        assert_eq!(updated, "name: test\nmodel: old");
    }

    #[test]
    fn update_yaml_field_replaces_or_inserts() {
        let yaml = "name: test\nmodel: \"old\"\n";
        let updated = update_yaml_field(yaml, "model", "\"new\"");
        assert!(updated.contains("model: \"new\""));

        let yaml = "name: test\n";
        let updated = update_yaml_field(yaml, "model", "\"new\"");
        assert!(updated.contains("model: \"new\""));
    }
}

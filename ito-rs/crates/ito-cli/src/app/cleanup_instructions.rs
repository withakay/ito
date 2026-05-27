use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_templates::manifest::HarnessTool;
use std::path::Path;

pub(super) fn generate_cleanup_instruction(rt: &Runtime) -> CliResult<String> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let loaded_from: Vec<String> = rt
        .resolved_config()
        .loaded_from
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();
    let tools = detect_cleanup_manifest_tools(project_root);
    let manifest_entries = ito_templates::manifest::generate_manifest(&tools)
        .into_iter()
        .map(CleanupManifestEntry::from)
        .collect::<Vec<_>>();
    let legacy_entries = ito_templates::legacy::LEGACY_ENTRIES
        .iter()
        .copied()
        .map(CleanupLegacyEntry::from)
        .collect::<Vec<_>>();

    #[derive(serde::Serialize)]
    struct Ctx {
        configured_tools: Vec<&'static str>,
        loaded_from: Vec<String>,
        manifest_entries: Vec<CleanupManifestEntry>,
        legacy_entries: Vec<CleanupLegacyEntry>,
    }

    let ctx = Ctx {
        configured_tools: tools.iter().map(harness_tool_id).collect(),
        loaded_from,
        manifest_entries,
        legacy_entries,
    };
    ito_templates::instructions::render_instruction_template("agent/cleanup.md.j2", &ctx)
        .map_err(|e| to_cli_error(format!("rendering cleanup instruction: {e}")))
}

fn detect_cleanup_manifest_tools(project_root: &Path) -> Vec<HarnessTool> {
    let candidates = [
        (".claude", HarnessTool::ClaudeCode),
        (".opencode", HarnessTool::OpenCode),
        (".codex", HarnessTool::Codex),
        (".github", HarnessTool::GitHubCopilot),
        (".pi", HarnessTool::Pi),
    ];
    candidates
        .into_iter()
        .filter_map(|(dir, tool)| project_root.join(dir).exists().then_some(tool))
        .collect()
}

fn harness_tool_id(tool: &HarnessTool) -> &'static str {
    match tool {
        HarnessTool::ClaudeCode => "claude",
        HarnessTool::OpenCode => "opencode",
        HarnessTool::Codex => "codex",
        HarnessTool::GitHubCopilot => "github-copilot",
        HarnessTool::Pi => "pi",
    }
}

#[derive(serde::Serialize)]
struct CleanupManifestEntry {
    relative_path: String,
    source_path: String,
    source: &'static str,
    harness: Option<&'static str>,
}

impl From<ito_templates::manifest::ManifestEntry> for CleanupManifestEntry {
    fn from(entry: ito_templates::manifest::ManifestEntry) -> Self {
        Self {
            relative_path: entry.relative_path,
            source_path: entry.source_path,
            source: match entry.source {
                ito_templates::manifest::ManifestSource::Project => "project",
                ito_templates::manifest::ManifestSource::Skill => "skill",
                ito_templates::manifest::ManifestSource::Command => "command",
                ito_templates::manifest::ManifestSource::Adapter => "adapter",
            },
            harness: entry.harness.as_ref().map(harness_tool_id),
        }
    }
}

#[derive(serde::Serialize)]
struct CleanupLegacyEntry {
    old_path: &'static str,
    new_path: Option<&'static str>,
    entry_type: &'static str,
    description: &'static str,
}

impl From<ito_templates::legacy::LegacyEntry> for CleanupLegacyEntry {
    fn from(entry: ito_templates::legacy::LegacyEntry) -> Self {
        Self {
            old_path: entry.old_path,
            new_path: entry.new_path,
            entry_type: match entry.entry_type {
                ito_templates::legacy::LegacyEntryType::Renamed => "renamed",
                ito_templates::legacy::LegacyEntryType::Removed => "removed",
                ito_templates::legacy::LegacyEntryType::Relocated => "relocated",
            },
            description: entry.description,
        }
    }
}

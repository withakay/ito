use crate::errors::CoreError;
use crate::orchestrate::types::OrchestratePreset;
use std::collections::BTreeSet;

/// Load a built-in orchestrator preset (embedded YAML).
pub fn load_orchestrate_preset(name: &str) -> Result<OrchestratePreset, CoreError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CoreError::Validation("preset name is required".to_string()));
    }

    let bytes = ito_templates::get_preset_file(&format!("orchestrate/{name}.yaml"))
        .ok_or_else(|| CoreError::NotFound(format!("orchestrate preset '{name}' not found")))?;

    serde_yaml::from_slice(bytes)
        .map_err(|e| CoreError::Parse(format!("invalid orchestrate preset '{name}': {e}")))
}

/// List the names of all embedded orchestrator presets.
pub fn list_orchestrate_presets() -> Vec<String> {
    let mut names: BTreeSet<String> = BTreeSet::new();
    for file in ito_templates::presets_files() {
        let rel = file.relative_path;
        let Some(rest) = rel.strip_prefix("orchestrate/") else {
            continue;
        };
        let Some(name) = rest.strip_suffix(".yaml") else {
            continue;
        };
        if !name.trim().is_empty() {
            names.insert(name.to_string());
        }
    }
    names.into_iter().collect()
}

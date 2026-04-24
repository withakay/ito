//! Structured parsing for per-change `.ito.yaml` metadata.

use crate::errors::CoreError;
use ito_common::fs::FileSystem;
use ito_domain::changes::ChangeOrchestrateMetadata;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
struct ChangeOrchestrateYaml {
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    preferred_gates: Vec<String>,
    #[serde(flatten, default)]
    _extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ChangeMetaYaml {
    #[serde(default)]
    schema: Option<String>,
    #[serde(default)]
    orchestrate: Option<ChangeOrchestrateYaml>,
    #[serde(flatten, default)]
    _extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ChangeMeta {
    pub(crate) schema: Option<String>,
    pub(crate) orchestrate: ChangeOrchestrateMetadata,
}

pub(crate) fn parse_change_meta(contents: &str) -> Result<ChangeMeta, CoreError> {
    let contents = contents.trim();
    if contents.is_empty() {
        return Ok(ChangeMeta::default());
    }

    let yaml: ChangeMetaYaml = serde_yaml::from_str(contents)
        .map_err(|e| CoreError::Parse(format!("invalid .ito.yaml metadata: {e}")))?;

    Ok(ChangeMeta {
        schema: yaml.schema,
        orchestrate: parse_orchestrate_metadata(yaml.orchestrate),
    })
}

pub(crate) fn parse_change_meta_best_effort(contents: &str) -> ChangeMeta {
    parse_change_meta(contents).unwrap_or_default()
}

pub(crate) fn read_change_meta_from_dir<F: FileSystem>(fs: &F, change_dir: &Path) -> ChangeMeta {
    let path = change_dir.join(".ito.yaml");
    if !fs.is_file(&path) {
        return ChangeMeta::default();
    }

    let Ok(contents) = fs.read_to_string(&path) else {
        return ChangeMeta::default();
    };

    parse_change_meta_best_effort(&contents)
}

fn parse_orchestrate_metadata(
    orchestrate: Option<ChangeOrchestrateYaml>,
) -> ChangeOrchestrateMetadata {
    let Some(ChangeOrchestrateYaml {
        depends_on,
        preferred_gates,
        _extra: _,
    }) = orchestrate
    else {
        return ChangeOrchestrateMetadata::default();
    };

    ChangeOrchestrateMetadata {
        depends_on,
        preferred_gates,
    }
}

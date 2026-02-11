//! Default configuration constructors.

use serde_json::Value;

use super::types::ItoConfig;

/// Centralized default configuration values.
///
/// This is the single source of truth for defaults used by config loading and
/// JSON schema generation.
pub fn default_config() -> ItoConfig {
    ItoConfig::default()
}

/// Default configuration serialized as JSON.
pub fn default_config_json() -> Value {
    serde_json::to_value(default_config()).expect("default config should serialize")
}

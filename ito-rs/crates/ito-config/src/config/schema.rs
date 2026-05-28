//! JSON schema generation for Ito config.

use schemars::schema_for;
use serde_json::Value;

use super::types::ItoConfig;

/// Generate the JSON schema (as JSON) for [`ItoConfig`].
pub fn config_schema_json() -> Value {
    let schema = schema_for!(ItoConfig);
    serde_json::to_value(&schema).expect("config schema should serialize to json")
}

/// Generate the JSON schema (pretty-printed) for [`ItoConfig`].
pub fn config_schema_pretty_json() -> String {
    serde_json::to_string_pretty(&config_schema_json()).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
#[path = "schema_tests.rs"]
mod schema_tests;

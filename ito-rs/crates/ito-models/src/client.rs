//! HTTP client for fetching models from models.dev

use crate::types::Model;
use std::time::Duration;

/// Client for the models.dev API
pub struct ModelsDevClient {
    agent: ureq::Agent,
    url: String,
}

impl Default for ModelsDevClient {
    fn default() -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build()
            .new_agent();

        Self {
            agent,
            url: "https://models.dev/api.json".to_string(),
        }
    }
}

impl ModelsDevClient {
    /// Create a new client with a custom URL (for testing)
    pub fn with_url(url: String) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build()
            .new_agent();

        Self { agent, url }
    }

    /// Fetch all models from models.dev
    pub fn fetch_models(&self) -> Result<Vec<Model>, ClientError> {
        let response = self
            .agent
            .get(&self.url)
            .call()
            .map_err(|e| ClientError::Request(e.to_string()))?;

        let json: serde_json::Value = response
            .into_body()
            .read_json()
            .map_err(|e| ClientError::Parse(e.to_string()))?;

        // The API returns an object with provider keys containing model arrays
        let mut models = Vec::new();

        if let serde_json::Value::Object(providers) = json {
            for (provider_id, provider_data) in providers {
                if let serde_json::Value::Object(provider_obj) = provider_data {
                    let provider_name = provider_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&provider_id)
                        .to_string();

                    if let Some(serde_json::Value::Array(model_list)) = provider_obj.get("models") {
                        for model_value in model_list {
                            if let Ok(mut model) =
                                serde_json::from_value::<Model>(model_value.clone())
                            {
                                // Ensure provider info is set
                                model.provider = crate::types::Provider {
                                    id: provider_id.clone(),
                                    name: provider_name.clone(),
                                };
                                // Build full model ID if not present
                                if !model.id.contains('/') {
                                    model.id = format!("{}/{}", provider_id, model.id);
                                }
                                models.push(model);
                            }
                        }
                    }
                }
            }
        }

        Ok(models)
    }
}

/// Errors from the API client
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Request failed: {0}")]
    Request(String),

    #[error("Failed to parse response: {0}")]
    Parse(String),
}

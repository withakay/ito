//! Model registry for querying and filtering models

use crate::cache::ModelCache;
use crate::client::ModelsDevClient;
use crate::types::{LoadOptions, Model, ModelCapability, ModelSource, RegistryLoad};

/// Model registry that provides access to models from models.dev
pub struct ModelRegistry {
    client: ModelsDevClient,
    cache: ModelCache,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self {
            client: ModelsDevClient::default(),
            cache: ModelCache::default_location(),
        }
    }
}

impl ModelRegistry {
    /// Create a registry with custom client and cache
    pub fn new(client: ModelsDevClient, cache: ModelCache) -> Self {
        Self { client, cache }
    }

    /// Load models from cache or API
    pub fn load(&self, options: LoadOptions) -> Result<RegistryLoad, RegistryError> {
        let ttl_hours = if options.ttl_hours == 0 {
            24
        } else {
            options.ttl_hours
        };

        // Try cache first (unless force refresh)
        if !options.force_refresh
            && let Some(cached) = self.cache.load()
        {
            let stale = self.cache.is_stale(ttl_hours);

            // If cache is fresh, use it
            if !stale {
                return Ok(RegistryLoad {
                    updated_at: cached.updated_at,
                    source: ModelSource::Cache,
                    stale: false,
                    models: cached.models,
                });
            }

            // Cache is stale, try to refresh
            match self.client.fetch_models() {
                Ok(models) => {
                    // Save to cache
                    let _ = self.cache.save(&models);
                    return Ok(RegistryLoad {
                        updated_at: chrono::Utc::now(),
                        source: ModelSource::Api,
                        stale: false,
                        models,
                    });
                }
                Err(_) => {
                    // API failed, use stale cache
                    return Ok(RegistryLoad {
                        updated_at: cached.updated_at,
                        source: ModelSource::Cache,
                        stale: true,
                        models: cached.models,
                    });
                }
            }
        }

        // No cache or force refresh - fetch from API
        let models = self
            .client
            .fetch_models()
            .map_err(|e| RegistryError::Fetch(e.to_string()))?;

        // Save to cache
        let _ = self.cache.save(&models);

        Ok(RegistryLoad {
            updated_at: chrono::Utc::now(),
            source: ModelSource::Api,
            stale: false,
            models,
        })
    }

    /// Find a model by ID
    pub fn find<'a>(&self, load: &'a RegistryLoad, id: &str) -> Option<&'a Model> {
        load.models.iter().find(|m| m.id == id)
    }

    /// Filter models by provider name
    pub fn by_provider<'a>(models: &'a [Model], provider: &str) -> Vec<&'a Model> {
        models
            .iter()
            .filter(|m| m.provider.id == provider)
            .collect()
    }

    /// Filter models by minimum context window size
    pub fn by_min_context(models: &[Model], min_tokens: u64) -> Vec<&Model> {
        models
            .iter()
            .filter(|m| m.max_input_tokens.unwrap_or(0) >= min_tokens)
            .collect()
    }

    /// Filter models by capability
    pub fn by_capability<'a>(models: &'a [Model], capability: &ModelCapability) -> Vec<&'a Model> {
        models
            .iter()
            .filter(|m| m.capabilities.contains(capability))
            .collect()
    }

    /// Sort models by cost tier (cheapest first based on combined input+output cost)
    pub fn by_cost_tier(models: &[Model], tier: CostTier) -> Vec<&Model> {
        let mut sorted: Vec<&Model> = models.iter().collect();
        sorted.sort_by(|a, b| {
            let cost_a = a.total_cost_per_million();
            let cost_b = b.total_cost_per_million();
            cost_a
                .partial_cmp(&cost_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        match tier {
            CostTier::Cheapest => sorted.into_iter().take(10).collect(),
            CostTier::Balanced => {
                let len = sorted.len();
                let start = len / 3;
                let end = (2 * len) / 3;
                sorted.into_iter().skip(start).take(end - start).collect()
            }
            CostTier::Premium => sorted.into_iter().rev().take(10).collect(),
        }
    }
}

/// Cost tier for filtering models
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostTier {
    /// Cheapest models (bottom 10)
    Cheapest,
    /// Mid-range models (middle third)
    Balanced,
    /// Premium/expensive models (top 10)
    Premium,
}

/// Registry errors
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Failed to fetch models: {0}")]
    /// Remote fetch failed.
    Fetch(String),

    #[error("No models available (cache empty and API unreachable)")]
    /// No models are available (cache empty and API unreachable).
    NoModels,
}

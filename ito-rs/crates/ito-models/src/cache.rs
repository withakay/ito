//! Local cache for model data

use crate::types::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Cached model data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheFile {
    /// Cache format version
    pub version: u32,
    /// When the cache was last updated
    pub updated_at: DateTime<Utc>,
    /// Cached models
    pub models: Vec<Model>,
}

impl Default for CacheFile {
    fn default() -> Self {
        Self {
            version: 1,
            updated_at: Utc::now(),
            models: Vec::new(),
        }
    }
}

/// Model cache manager
pub struct ModelCache {
    path: PathBuf,
}

impl ModelCache {
    /// Create a cache at the default location (~/.config/ito/models-cache.json)
    pub fn default_location() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".config"));
        let cache_path = config_dir.join("ito").join("models-cache.json");
        Self { path: cache_path }
    }

    /// Create a cache at a custom path
    pub fn with_path(path: PathBuf) -> Self {
        Self { path }
    }

    /// Load cached models
    pub fn load(&self) -> Option<CacheFile> {
        let content = fs::read_to_string(&self.path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Save models to cache
    pub fn save(&self, models: &[Model]) -> Result<(), CacheError> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| CacheError::Io(e.to_string()))?;
        }

        let cache = CacheFile {
            version: 1,
            updated_at: Utc::now(),
            models: models.to_vec(),
        };

        let content = serde_json::to_string_pretty(&cache)
            .map_err(|e| CacheError::Serialize(e.to_string()))?;

        fs::write(&self.path, content).map_err(|e| CacheError::Io(e.to_string()))?;

        Ok(())
    }

    /// Check if the cache is stale (older than TTL hours)
    pub fn is_stale(&self, ttl_hours: u64) -> bool {
        match self.load() {
            Some(cache) => {
                let age = Utc::now().signed_duration_since(cache.updated_at);
                age.num_hours() >= ttl_hours as i64
            }
            None => true, // No cache = stale
        }
    }

    /// Get the cache path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Cache errors
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Serialization error: {0}")]
    Serialize(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn cache_save_and_load() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("cache.json");
        let cache = ModelCache::with_path(cache_path);

        let models = vec![Model {
            id: "test/model".to_string(),
            provider: crate::types::Provider {
                id: "test".to_string(),
                name: "Test".to_string(),
            },
            name: "Test Model".to_string(),
            family: None,
            input_cost_per_million: Some(1.0),
            output_cost_per_million: Some(2.0),
            cache_read_cost_per_million: None,
            cache_write_cost_per_million: None,
            max_input_tokens: Some(100000),
            max_output_tokens: Some(4096),
            modalities: Default::default(),
            capabilities: vec![],
        }];

        cache.save(&models).unwrap();

        let loaded = cache.load().unwrap();
        assert_eq!(loaded.models.len(), 1);
        assert_eq!(loaded.models[0].id, "test/model");
    }

    #[test]
    fn cache_is_stale_when_missing() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("nonexistent.json");
        let cache = ModelCache::with_path(cache_path);

        assert!(cache.is_stale(24));
    }
}

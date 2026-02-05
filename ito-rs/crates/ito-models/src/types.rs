//! Model and provider type definitions

use serde::{Deserialize, Serialize};
use std::fmt;

/// A model from models.dev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Model ID in provider/model format (e.g., "anthropic/claude-sonnet-4-5")
    pub id: String,
    /// Provider information
    pub provider: Provider,
    /// Human-readable model name
    pub name: String,
    /// Model family (e.g., "claude", "gpt")
    #[serde(default)]
    pub family: Option<String>,
    /// Input cost per million tokens
    #[serde(default)]
    pub input_cost_per_million: Option<f64>,
    /// Output cost per million tokens
    #[serde(default)]
    pub output_cost_per_million: Option<f64>,
    /// Cache read cost per million tokens
    #[serde(default)]
    pub cache_read_cost_per_million: Option<f64>,
    /// Cache write cost per million tokens
    #[serde(default)]
    pub cache_write_cost_per_million: Option<f64>,
    /// Maximum input tokens
    #[serde(default)]
    pub max_input_tokens: Option<u64>,
    /// Maximum output tokens
    #[serde(default)]
    pub max_output_tokens: Option<u64>,
    /// Supported modalities
    #[serde(default)]
    pub modalities: ModelModalities,
    /// Model capabilities
    #[serde(default)]
    pub capabilities: Vec<ModelCapability>,
}

impl Model {
    /// Calculate total cost per million tokens (input + output)
    pub fn total_cost_per_million(&self) -> f64 {
        self.input_cost_per_million.unwrap_or(0.0) + self.output_cost_per_million.unwrap_or(0.0)
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Provider ID (e.g., "anthropic", "openai")
    pub id: String,
    /// Human-readable provider name
    pub name: String,
}

/// Supported input/output modalities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelModalities {
    /// Supported input modalities.
    #[serde(default)]
    pub input: Vec<Modality>,
    /// Supported output modalities.
    #[serde(default)]
    pub output: Vec<Modality>,
}

/// A modality type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    /// Plain text.
    Text,
    /// Images.
    Image,
    /// Audio.
    Audio,
    /// Video.
    Video,
    #[serde(other)]
    /// Unknown / unsupported modality.
    Unknown,
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Image => write!(f, "image"),
            Self::Audio => write!(f, "audio"),
            Self::Video => write!(f, "video"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Model capability
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelCapability {
    /// Vision / image understanding.
    Vision,
    /// Tool / function calling.
    FunctionCalling,
    /// Native JSON-mode output.
    JsonMode,
    /// Server-sent streaming.
    Streaming,
    /// Enhanced reasoning mode.
    Reasoning,
    #[serde(other)]
    /// Unknown / unsupported capability.
    Unknown,
}

impl fmt::Display for ModelCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vision => write!(f, "vision"),
            Self::FunctionCalling => write!(f, "function-calling"),
            Self::JsonMode => write!(f, "json-mode"),
            Self::Streaming => write!(f, "streaming"),
            Self::Reasoning => write!(f, "reasoning"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Options for loading the model registry
#[derive(Debug, Clone, Default)]
pub struct LoadOptions {
    /// Force refresh from API even if cache is fresh
    pub force_refresh: bool,
    /// Cache TTL in hours (default: 24)
    pub ttl_hours: u64,
}

/// Result of loading the model registry
#[derive(Debug, Clone)]
pub struct RegistryLoad {
    /// When the data was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Where the data came from
    pub source: ModelSource,
    /// Whether the data is stale (older than TTL)
    pub stale: bool,
    /// The loaded models
    pub models: Vec<Model>,
}

/// Source of model data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSource {
    /// Fetched fresh from API
    Api,
    /// Loaded from local cache
    Cache,
}

impl fmt::Display for ModelSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Cache => write!(f, "cache"),
        }
    }
}

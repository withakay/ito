//! Configuration types for proposal review and integration.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::types::{ArchiveConfig, CoordinationBranchConfig};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Change workflow settings")]
/// Configuration for change proposal, coordination, and archive behavior.
pub struct ChangesConfig {
    #[serde(default)]
    #[schemars(default, description = "Proposal integration settings")]
    /// Proposal review and integration settings.
    pub proposal: ProposalConfig,

    #[serde(default)]
    #[schemars(default, description = "Coordination branch settings")]
    /// Coordination branch settings.
    pub coordination_branch: CoordinationBranchConfig,

    #[serde(default)]
    #[schemars(default, description = "Archive integration settings")]
    /// Archive follow-up settings.
    pub archive: ArchiveConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Proposal integration settings")]
/// Configuration for integrating reviewed proposals before implementation.
pub struct ProposalConfig {
    #[serde(default = "ProposalConfig::default_integration_mode")]
    #[schemars(
        default = "ProposalConfig::default_integration_mode",
        description = "Mode for integrating reviewed proposals into the authoritative target branch"
    )]
    /// How reviewed proposals are integrated before implementation begins.
    pub integration_mode: ProposalIntegrationMode,
}

impl ProposalConfig {
    fn default_integration_mode() -> ProposalIntegrationMode {
        ProposalIntegrationMode::PullRequest
    }
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            integration_mode: Self::default_integration_mode(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Integration mode for reviewed proposals")]
/// Integration mode for establishing the authoritative proposal history.
pub enum ProposalIntegrationMode {
    /// Integrate through the target branch's tracked upstream ref.
    #[default]
    PullRequest,
    /// Integrate directly into the local target branch.
    DirectMerge,
}

impl ProposalIntegrationMode {
    /// All supported proposal integration mode values.
    pub const ALL: &'static [&'static str] = &["pull_request", "direct_merge"];

    /// Return a stable string identifier for display and diagnostics.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PullRequest => "pull_request",
            Self::DirectMerge => "direct_merge",
        }
    }

    /// Parse a string into a proposal integration mode.
    pub fn parse_value(value: &str) -> Option<Self> {
        match value {
            "pull_request" => Some(Self::PullRequest),
            "direct_merge" => Some(Self::DirectMerge),
            _ => None,
        }
    }
}

impl std::fmt::Display for ProposalIntegrationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

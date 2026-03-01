//! Backend server configuration types for the multi-tenant Ito API.
//!
//! These types define the configuration schema for hosting a multi-tenant
//! backend server, including storage, auth, allowlist, CORS, and HTTP
//! transport settings.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Project namespace configuration for multi-tenant backend routing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Project namespace for backend API routing")]
pub struct BackendProjectConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Organization namespace for backend routes")]
    /// Organization namespace used in `/api/v1/projects/{org}/{repo}/...`.
    pub org: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Repository namespace for backend routes")]
    /// Repository namespace used in `/api/v1/projects/{org}/{repo}/...`.
    pub repo: Option<String>,
}

/// Backend server configuration for hosting a multi-tenant Ito API.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Backend server configuration for multi-tenant API hosting")]
pub struct BackendServerConfig {
    #[serde(default)]
    #[schemars(default, description = "Enable backend server features")]
    /// Whether backend server mode is enabled.
    pub enabled: bool,

    #[serde(default = "BackendServerConfig::default_bind")]
    #[schemars(
        default = "BackendServerConfig::default_bind",
        description = "Bind address for the server"
    )]
    /// Bind address for the server.
    pub bind: String,

    #[serde(default = "BackendServerConfig::default_port")]
    #[schemars(
        default = "BackendServerConfig::default_port",
        description = "Port for the server"
    )]
    /// Port for the server.
    pub port: u16,

    #[serde(default, rename = "dataDir", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Storage root directory for backend-managed project data")]
    /// Storage root directory. Defaults to `$XDG_DATA_HOME/ito/backend`
    /// or `$HOME/.local/share/ito/backend`.
    pub data_dir: Option<String>,

    #[serde(default)]
    #[schemars(default, description = "Storage backend configuration")]
    /// Storage backend selection and configuration.
    pub storage: BackendStorageConfig,

    #[serde(default)]
    #[schemars(default, description = "HTTP transport settings")]
    /// HTTP transport settings.
    pub http: BackendHttpConfig,

    #[serde(default)]
    #[schemars(default, description = "CORS configuration")]
    /// CORS configuration.
    pub cors: BackendCorsConfig,

    #[serde(default)]
    #[schemars(default, description = "Allowlist for organizations and repositories")]
    /// Allowlist for organizations and repositories.
    pub allowed: BackendAllowlistConfig,

    #[serde(default)]
    #[schemars(default, description = "Authentication configuration")]
    /// Authentication configuration.
    pub auth: BackendAuthConfig,
}

impl BackendServerConfig {
    fn default_bind() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> u16 {
        9010
    }
}

impl Default for BackendServerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bind: Self::default_bind(),
            port: Self::default_port(),
            data_dir: None,
            storage: BackendStorageConfig::default(),
            http: BackendHttpConfig::default(),
            cors: BackendCorsConfig::default(),
            allowed: BackendAllowlistConfig::default(),
            auth: BackendAuthConfig::default(),
        }
    }
}

/// Storage backend selection for the backend server.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Storage backend configuration")]
pub struct BackendStorageConfig {
    #[serde(default = "BackendStorageConfig::default_kind")]
    #[schemars(
        default = "BackendStorageConfig::default_kind",
        description = "Storage backend kind: filesystem or sqlite"
    )]
    /// Storage backend kind.
    pub kind: BackendStorageKind,

    #[serde(default)]
    #[schemars(default, description = "SQLite-specific configuration")]
    /// SQLite-specific configuration.
    pub sqlite: BackendSqliteConfig,
}

impl BackendStorageConfig {
    fn default_kind() -> BackendStorageKind {
        BackendStorageKind::Filesystem
    }
}

impl Default for BackendStorageConfig {
    fn default() -> Self {
        Self {
            kind: Self::default_kind(),
            sqlite: BackendSqliteConfig::default(),
        }
    }
}

/// Supported storage backend kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(description = "Storage backend kind")]
pub enum BackendStorageKind {
    /// Filesystem-backed markdown storage (default).
    Filesystem,
    /// SQLite-backed storage (proof-of-concept).
    Sqlite,
}

/// SQLite-specific backend configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "SQLite backend configuration")]
pub struct BackendSqliteConfig {
    #[serde(default, rename = "dbPath", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Path to the SQLite database file")]
    /// Path to the SQLite database file.
    pub db_path: Option<String>,
}

/// HTTP transport settings for the backend server.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "HTTP transport configuration")]
pub struct BackendHttpConfig {
    #[serde(
        default = "BackendHttpConfig::default_max_body_bytes",
        rename = "maxBodyBytes"
    )]
    #[schemars(
        default = "BackendHttpConfig::default_max_body_bytes",
        description = "Maximum HTTP request body size in bytes"
    )]
    /// Maximum HTTP request body size in bytes.
    pub max_body_bytes: usize,
}

impl BackendHttpConfig {
    fn default_max_body_bytes() -> usize {
        10 * 1024 * 1024 // 10 MiB
    }
}

impl Default for BackendHttpConfig {
    fn default() -> Self {
        Self {
            max_body_bytes: Self::default_max_body_bytes(),
        }
    }
}

/// CORS configuration for the backend server.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "CORS configuration")]
pub struct BackendCorsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Allowed CORS origins (None = permissive)")]
    /// Allowed CORS origins. `None` means permissive.
    pub origins: Option<Vec<String>>,
}

/// Allowlist for organizations and repositories.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Organization and repository allowlist")]
pub struct BackendAllowlistConfig {
    #[serde(default)]
    #[schemars(default, description = "Allowed organizations")]
    /// Allowed organization names.
    pub orgs: Vec<String>,

    #[serde(default)]
    #[schemars(
        default,
        description = "Per-org repo policies: org -> '*' or list of repos"
    )]
    /// Per-org repo policies. Map from org name to either `"*"` (all repos)
    /// or a list of allowed repository names.
    pub repos: std::collections::BTreeMap<String, BackendRepoPolicy>,
}

/// Repository policy for an organization.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(description = "Repo policy: '*' for all or list of repo names")]
pub enum BackendRepoPolicy {
    /// Allow all repositories for this organization.
    All(String),
    /// Allow only the listed repositories.
    List(Vec<String>),
}

impl BackendAllowlistConfig {
    /// Check if a given `{org}/{repo}` pair is allowed.
    pub fn is_allowed(&self, org: &str, repo: &str) -> bool {
        if !self.orgs.contains(&org.to_string()) {
            return false;
        }

        let Some(policy) = self.repos.get(org) else {
            // Org is in the allowed list but no repo policy -> deny
            return false;
        };

        match policy {
            BackendRepoPolicy::All(s) if s == "*" => true,
            BackendRepoPolicy::All(_) => false,
            BackendRepoPolicy::List(repos) => repos.contains(&repo.to_string()),
        }
    }
}

/// Authentication configuration for the backend server.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Backend authentication configuration")]
pub struct BackendAuthConfig {
    #[serde(default, rename = "adminTokens")]
    #[schemars(default, description = "Admin bearer tokens with full access")]
    /// Admin bearer tokens that authorize access to any project.
    pub admin_tokens: Vec<String>,

    #[serde(default, rename = "tokenSeed", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Secret seed for deriving per-project tokens via HMAC-SHA256")]
    /// Secret seed for deriving per-project tokens.
    ///
    /// Per-project tokens are computed as `HMAC-SHA256(seed, "{org}/{repo}")`.
    pub token_seed: Option<String>,
}

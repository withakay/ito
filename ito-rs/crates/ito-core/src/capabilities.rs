//! Compile-time capability reporting and configuration preflight.

use std::fmt;

use ito_config::types::{CoordinationStorage, ItoConfig};

use crate::errors::{CoreError, CoreResult};

/// Experimental implementation that may be absent from a compiled Ito binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompiledFeature {
    /// Remote backend repositories and backend service integration.
    Backend,
    /// Coordination-branch synchronization and worktree storage.
    CoordinationBranch,
}

impl CompiledFeature {
    /// Stable Cargo feature identifier used in diagnostics and machine output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Backend => "backend",
            Self::CoordinationBranch => "coordination-branch",
        }
    }

    /// Whether this implementation is present in the current `ito-core` build.
    ///
    /// This observes dependency-crate features. Executables should report
    /// their local feature boundary with [`CompiledCapabilities`] instead.
    pub const fn is_compiled(self) -> bool {
        match self {
            Self::Backend => cfg!(feature = "backend"),
            Self::CoordinationBranch => cfg!(feature = "coordination-branch"),
        }
    }
}

impl fmt::Display for CompiledFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A caller-reported set of compiled capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompiledCapabilities {
    /// Whether backend integration is compiled in.
    pub backend: bool,
    /// Whether coordination-branch integration is compiled in.
    pub coordination_branch: bool,
}

impl CompiledCapabilities {
    /// Report the features compiled into the current `ito-core` build.
    pub const fn current() -> Self {
        Self {
            backend: cfg!(feature = "backend"),
            coordination_branch: cfg!(feature = "coordination-branch"),
        }
    }

    /// Return whether a particular feature is compiled in.
    pub const fn contains(self, feature: CompiledFeature) -> bool {
        match feature {
            CompiledFeature::Backend => self.backend,
            CompiledFeature::CoordinationBranch => self.coordination_branch,
        }
    }
}

/// How the caller intends to use the resolved configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityPreflight {
    /// Ordinary command execution that may initialize mutable subsystems.
    Stateful,
    /// Read-only diagnostics or recovery that must remain available for migration.
    Recovery,
}

/// Reject configuration that requests an implementation absent from this binary.
///
/// Call this after configuration resolution and before repository, audit, or
/// coordination initialization. Recovery callers deliberately bypass the gate so
/// they can explain or migrate legacy configuration without mutating through it.
pub fn preflight_config(config: &ItoConfig, mode: CapabilityPreflight) -> CoreResult<()> {
    preflight_config_with_capabilities(config, mode, CompiledCapabilities::current())
}

/// Reject configuration against the capabilities reported by the executable.
///
/// Cargo unifies dependency features across a workspace build. Executable
/// crates should pass their own feature set here so another workspace member
/// cannot accidentally make an experimental dependency appear available in a
/// shipping binary.
pub fn preflight_config_with_capabilities(
    config: &ItoConfig,
    mode: CapabilityPreflight,
    capabilities: CompiledCapabilities,
) -> CoreResult<()> {
    if mode == CapabilityPreflight::Recovery {
        return Ok(());
    }

    if config.backend.enabled && !capabilities.contains(CompiledFeature::Backend) {
        return Err(CoreError::feature_unavailable(
            CompiledFeature::Backend,
            "backend.enabled",
            "disable backend.enabled, or install an experimental build with the backend feature",
        ));
    }

    let coordination = &config.changes.coordination_branch;
    if (coordination.enabled.0 || coordination.storage == CoordinationStorage::Worktree)
        && !capabilities.contains(CompiledFeature::CoordinationBranch)
    {
        let requested_by = if coordination.enabled.0 {
            "changes.coordination_branch.enabled"
        } else {
            "changes.coordination_branch.storage=worktree"
        };
        return Err(CoreError::feature_unavailable(
            CompiledFeature::CoordinationBranch,
            requested_by,
            "run `ito agent instruction migrate-to-main`, then disable coordination-branch synchronization",
        ));
    }

    Ok(())
}

#[cfg(test)]
#[path = "capabilities_tests.rs"]
mod capabilities_tests;

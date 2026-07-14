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

    /// Whether this implementation is present in the current binary.
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

/// Capabilities compiled into the current `ito-core` build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompiledCapabilities {
    /// Whether backend integration is compiled in.
    pub backend: bool,
    /// Whether coordination-branch integration is compiled in.
    pub coordination_branch: bool,
}

impl CompiledCapabilities {
    /// Report the features compiled into this binary.
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
    if mode == CapabilityPreflight::Recovery {
        return Ok(());
    }

    if config.backend.enabled && !CompiledFeature::Backend.is_compiled() {
        return Err(CoreError::feature_unavailable(
            CompiledFeature::Backend,
            "backend.enabled",
            "disable backend.enabled, or install an experimental build with the backend feature",
        ));
    }

    let coordination = &config.changes.coordination_branch;
    if (coordination.enabled.0 || coordination.storage == CoordinationStorage::Worktree)
        && !CompiledFeature::CoordinationBranch.is_compiled()
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

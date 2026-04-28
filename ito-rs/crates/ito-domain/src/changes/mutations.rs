//! Change artifact mutation port definitions.

use std::io;

use thiserror::Error;

/// Result alias for change artifact mutation operations.
pub type ChangeArtifactMutationServiceResult<T> = Result<T, ChangeArtifactMutationError>;

/// A mutable artifact within an active change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeArtifactKind {
    /// `proposal.md`
    Proposal,
    /// `design.md`
    Design,
    /// The change's tracking artifact (usually `tasks.md`).
    Tasks,
    /// A change-local spec delta identified by capability.
    SpecDelta {
        /// Capability directory name under the change's `specs/` directory.
        capability: String,
    },
}

impl ChangeArtifactKind {
    /// Render a stable label for diagnostics and CLI output.
    pub fn label(&self) -> String {
        match self {
            ChangeArtifactKind::Proposal => "proposal".to_string(),
            ChangeArtifactKind::Design => "design".to_string(),
            ChangeArtifactKind::Tasks => "tasks".to_string(),
            ChangeArtifactKind::SpecDelta { capability } => format!("spec:{capability}"),
        }
    }
}

/// Reference to a mutable artifact within an active change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeArtifactRef {
    /// Canonical change identifier.
    pub change_id: String,
    /// Artifact kind to mutate.
    pub artifact: ChangeArtifactKind,
}

impl ChangeArtifactRef {
    /// Render a stable label for diagnostics and CLI output.
    pub fn label(&self) -> String {
        format!("{}:{}", self.change_id, self.artifact.label())
    }
}

/// Result of mutating a change artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeArtifactMutationResult {
    /// Artifact target that was mutated.
    pub target: ChangeArtifactRef,
    /// Whether the artifact existed before the mutation.
    pub existed: bool,
    /// Backend or store revision after the mutation, when applicable.
    pub revision: Option<String>,
}

/// Error type for change artifact mutation ports.
#[derive(Debug, Error)]
pub enum ChangeArtifactMutationError {
    /// Filesystem or transport failure.
    #[error("I/O failure while {context}: {source}")]
    Io {
        /// Short operation context.
        context: String,
        /// Source error.
        #[source]
        source: io::Error,
    },

    /// Validation or precondition failure.
    #[error("{0}")]
    Validation(String),

    /// Requested artifact was not found.
    #[error("{0}")]
    NotFound(String),

    /// Unexpected transport or backend failure.
    #[error("{0}")]
    Other(String),
}

impl ChangeArtifactMutationError {
    /// Build an I/O flavored error.
    pub fn io(context: impl Into<String>, source: io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }

    /// Build a validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Build a not-found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    /// Build a catch-all error.
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

/// Port for active-change artifact mutations.
pub trait ChangeArtifactMutationService: Send + Sync {
    /// Load the current artifact contents, if available.
    fn load_artifact(
        &self,
        target: &ChangeArtifactRef,
    ) -> ChangeArtifactMutationServiceResult<Option<String>>;

    /// Replace the artifact contents completely.
    fn write_artifact(
        &self,
        target: &ChangeArtifactRef,
        content: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult>;

    /// Apply a targeted patch to the current artifact contents.
    fn patch_artifact(
        &self,
        target: &ChangeArtifactRef,
        patch: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult>;
}

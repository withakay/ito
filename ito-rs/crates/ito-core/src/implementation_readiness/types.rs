//! Stable request and report types for implementation readiness.

use std::path::PathBuf;

use ito_config::types::ProposalIntegrationMode;
use serde::{Deserialize, Serialize};

/// Readiness phase selected by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessPhase {
    /// Prove that authoritative main contains a valid accepted proposal.
    Prepare,
    /// Also prove implementation ancestry and checkout identity.
    Execute,
}

/// Authority evidence captured while one readiness evaluation is in progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityEvidence {
    /// Configured integration mode used to select authority.
    pub integration_mode: ProposalIntegrationMode,
    /// Fully qualified authoritative Git ref, when selection succeeded.
    pub target_ref: Option<String>,
    /// Commit OID resolved from `target_ref`, when resolution succeeded.
    pub oid: Option<String>,
}

impl AuthorityEvidence {
    /// Return an immutable snapshot only when both the ref and commit were resolved.
    pub fn snapshot(&self) -> Option<AuthoritySnapshot> {
        Some(AuthoritySnapshot {
            integration_mode: self.integration_mode,
            target_ref: self.target_ref.clone()?,
            oid: self.oid.clone()?,
        })
    }
}

/// Immutable authority snapshot consumed by tree, history, and worktree checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySnapshot {
    /// Configured integration mode used to select authority.
    pub integration_mode: ProposalIntegrationMode,
    /// Fully qualified authoritative Git ref.
    pub target_ref: String,
    /// Commit OID resolved exactly once from `target_ref`.
    pub oid: String,
}

/// One inspectable condition contributing to a readiness decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessCondition {
    /// Stable machine-readable condition identifier.
    pub code: String,
    /// Whether this condition passed.
    pub passed: bool,
    /// Human-readable evidence or failure explanation.
    pub message: String,
    /// Action that can remediate a failed condition.
    pub remediation: Option<String>,
    /// Authoritative artifact path associated with this condition.
    pub path: Option<String>,
    /// Validator or rule identifier associated with this condition.
    pub validator_code: Option<String>,
}

impl ReadinessCondition {
    pub(super) fn passed(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            passed: true,
            message: message.into(),
            remediation: None,
            path: None,
            validator_code: None,
        }
    }

    pub(super) fn failed(
        code: impl Into<String>,
        message: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            passed: false,
            message: message.into(),
            remediation: Some(remediation.into()),
            path: None,
            validator_code: None,
        }
    }

    pub(super) fn failed_validation(
        message: impl Into<String>,
        remediation: impl Into<String>,
        path: impl Into<String>,
        validator_code: Option<String>,
    ) -> Self {
        Self {
            code: "authoritative_validation".to_string(),
            passed: false,
            message: message.into(),
            remediation: Some(remediation.into()),
            path: Some(path.into()),
            validator_code,
        }
    }

    pub(super) fn failed_artifact(
        message: impl Into<String>,
        remediation: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            code: "authoritative_artifacts".to_string(),
            passed: false,
            message: message.into(),
            remediation: Some(remediation.into()),
            path: Some(path.into()),
            validator_code: None,
        }
    }
}

/// Structured readiness result shared by CLI and lifecycle consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessReport {
    /// Full Ito change ID evaluated.
    pub change_id: String,
    /// Requested readiness phase.
    pub phase: ReadinessPhase,
    /// Overall phase result.
    pub ready: bool,
    /// Authority ref and immutable OID evidence.
    pub authority: AuthorityEvidence,
    /// Commit that introduced the proposal marker into authority history.
    pub proposal_integration_oid: Option<String>,
    /// Ordered evaluation conditions.
    pub conditions: Vec<ReadinessCondition>,
}

impl ReadinessReport {
    pub(super) fn new(
        request: &ReadinessRequest,
        integration_mode: ProposalIntegrationMode,
    ) -> Self {
        Self {
            change_id: request.change_id.clone(),
            phase: request.phase,
            ready: false,
            authority: AuthorityEvidence {
                integration_mode,
                target_ref: None,
                oid: None,
            },
            proposal_integration_oid: None,
            conditions: Vec::new(),
        }
    }

    /// Return the immutable authority snapshot when resolution succeeded.
    pub fn authority_snapshot(&self) -> Option<AuthoritySnapshot> {
        self.authority.snapshot()
    }
}

/// Inputs for one readiness evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessRequest {
    /// Ito change ID or unique authoritative prefix to evaluate.
    pub change_id: String,
    /// Required readiness phase.
    pub phase: ReadinessPhase,
    /// Fetch the configured upstream target before authority resolution.
    pub refresh_authority: bool,
    /// Repository in which Git authority is evaluated.
    pub repository_root: PathBuf,
    /// Current implementation checkout, when execute readiness is requested.
    pub current_checkout: Option<PathBuf>,
}

impl ReadinessRequest {
    /// Construct a readiness request without refreshing authority.
    pub fn new(
        change_id: impl Into<String>,
        phase: ReadinessPhase,
        repository_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            change_id: change_id.into(),
            phase,
            refresh_authority: false,
            repository_root: repository_root.into(),
            current_checkout: None,
        }
    }

    /// Select whether pull-request authority should be refreshed first.
    pub fn with_refresh_authority(mut self, refresh_authority: bool) -> Self {
        self.refresh_authority = refresh_authority;
        self
    }

    /// Attach the current checkout for later execute-phase checks.
    pub fn with_current_checkout(mut self, current_checkout: impl Into<PathBuf>) -> Self {
        self.current_checkout = Some(current_checkout.into());
        self
    }
}

//! Backend coordination use-cases for CLI commands.
//!
//! Provides the business logic for claim, release, allocate, and sync
//! operations that the CLI adapter calls. Each function accepts trait
//! objects for the backend clients so the CLI can inject the concrete
//! implementation.

use ito_domain::backend::{
    AllocateResult, ArtifactBundle, BackendLeaseClient, BackendSyncClient, ClaimResult, PushResult,
    ReleaseResult,
};

use crate::backend_sync::map_backend_error;
use crate::errors::{CoreError, CoreResult};

/// Claim a change lease through the backend.
pub fn claim_change(
    lease_client: &dyn BackendLeaseClient,
    change_id: &str,
) -> CoreResult<ClaimResult> {
    lease_client
        .claim(change_id)
        .map_err(|e| map_backend_error(e, "claim"))
}

/// Release a change lease through the backend.
pub fn release_change(
    lease_client: &dyn BackendLeaseClient,
    change_id: &str,
) -> CoreResult<ReleaseResult> {
    lease_client
        .release(change_id)
        .map_err(|e| map_backend_error(e, "release"))
}

/// Allocate the next available change through the backend.
pub fn allocate_change(lease_client: &dyn BackendLeaseClient) -> CoreResult<AllocateResult> {
    lease_client
        .allocate()
        .map_err(|e| map_backend_error(e, "allocate"))
}

/// Pull artifacts from the backend for a change.
pub fn sync_pull(
    sync_client: &dyn BackendSyncClient,
    ito_path: &std::path::Path,
    change_id: &str,
    backup_dir: &std::path::Path,
) -> CoreResult<ArtifactBundle> {
    crate::backend_sync::pull_artifacts(sync_client, ito_path, change_id, backup_dir)
}

/// Push local artifacts to the backend for a change.
pub fn sync_push(
    sync_client: &dyn BackendSyncClient,
    ito_path: &std::path::Path,
    change_id: &str,
    backup_dir: &std::path::Path,
) -> CoreResult<PushResult> {
    crate::backend_sync::push_artifacts(sync_client, ito_path, change_id, backup_dir)
}

/// Check whether the given `CoreError` represents a backend availability failure.
///
/// The CLI can use this to suggest fallback to filesystem mode.
pub fn is_backend_unavailable(err: &CoreError) -> bool {
    match err {
        CoreError::Process(msg) => msg.contains("Backend unavailable"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_domain::backend::{BackendError, ClaimResult, LeaseConflict, ReleaseResult};

    struct FakeLeaseClient {
        claim_result: Result<ClaimResult, BackendError>,
        release_result: Result<ReleaseResult, BackendError>,
        allocate_result: Result<AllocateResult, BackendError>,
    }

    impl FakeLeaseClient {
        fn success() -> Self {
            Self {
                claim_result: Ok(ClaimResult {
                    change_id: "test".to_string(),
                    holder: "me".to_string(),
                    expires_at: None,
                }),
                release_result: Ok(ReleaseResult {
                    change_id: "test".to_string(),
                }),
                allocate_result: Ok(AllocateResult {
                    claim: Some(ClaimResult {
                        change_id: "test".to_string(),
                        holder: "me".to_string(),
                        expires_at: None,
                    }),
                }),
            }
        }

        fn conflict() -> Self {
            Self {
                claim_result: Err(BackendError::LeaseConflict(LeaseConflict {
                    change_id: "test".to_string(),
                    holder: "other".to_string(),
                    expires_at: None,
                })),
                release_result: Ok(ReleaseResult {
                    change_id: "test".to_string(),
                }),
                allocate_result: Ok(AllocateResult { claim: None }),
            }
        }
    }

    impl BackendLeaseClient for FakeLeaseClient {
        fn claim(&self, _change_id: &str) -> Result<ClaimResult, BackendError> {
            self.claim_result.clone()
        }

        fn release(&self, _change_id: &str) -> Result<ReleaseResult, BackendError> {
            self.release_result.clone()
        }

        fn allocate(&self) -> Result<AllocateResult, BackendError> {
            self.allocate_result.clone()
        }
    }

    #[test]
    fn claim_success() {
        let client = FakeLeaseClient::success();
        let result = claim_change(&client, "test").unwrap();
        assert_eq!(result.change_id, "test");
        assert_eq!(result.holder, "me");
    }

    #[test]
    fn claim_conflict() {
        let client = FakeLeaseClient::conflict();
        let err = claim_change(&client, "test").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Lease conflict"), "msg: {msg}");
        assert!(msg.contains("other"), "msg: {msg}");
    }

    #[test]
    fn release_success() {
        let client = FakeLeaseClient::success();
        let result = release_change(&client, "test").unwrap();
        assert_eq!(result.change_id, "test");
    }

    #[test]
    fn allocate_with_work() {
        let client = FakeLeaseClient::success();
        let result = allocate_change(&client).unwrap();
        assert!(result.claim.is_some());
        assert_eq!(result.claim.unwrap().change_id, "test");
    }

    #[test]
    fn allocate_no_work() {
        let client = FakeLeaseClient::conflict();
        let result = allocate_change(&client).unwrap();
        assert!(result.claim.is_none());
    }

    #[test]
    fn is_backend_unavailable_detects_process_error() {
        let err = CoreError::process("Backend unavailable during pull: timeout");
        assert!(is_backend_unavailable(&err));

        let err = CoreError::validation("some other error");
        assert!(!is_backend_unavailable(&err));
    }
}

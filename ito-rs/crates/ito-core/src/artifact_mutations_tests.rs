use super::*;
use ito_domain::backend::{LeaseConflict, PushResult, RevisionConflict};
use std::sync::{Arc, Mutex};
use tempfile::tempdir;

#[test]
fn fs_service_writes_and_patches_proposal() {
    let tmp = tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

    let service = FsChangeArtifactMutationService::new(&ito_path);
    let target = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::Proposal,
    };

    let result = service
        .write_artifact(&target, "# Proposal\n")
        .expect("write proposal");
    assert!(!result.existed);
    assert_eq!(
        std::fs::read_to_string(
            ito_path
                .join("changes")
                .join("025-11_demo")
                .join("proposal.md")
        )
        .expect("read proposal"),
        "# Proposal\n"
    );

    let patch = "--- proposal\n+++ proposal\n@@ -1 +1 @@\n-# Proposal\n+# Updated Proposal\n";
    let result = service
        .patch_artifact(&target, patch)
        .expect("patch proposal");
    assert!(result.existed);
    assert_eq!(
        std::fs::read_to_string(
            ito_path
                .join("changes")
                .join("025-11_demo")
                .join("proposal.md")
        )
        .expect("read patched proposal"),
        "# Updated Proposal\n"
    );
}

#[test]
fn fs_service_creates_spec_delta_directory_on_write() {
    let tmp = tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

    let service = FsChangeArtifactMutationService::new(&ito_path);
    let target = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::SpecDelta {
            capability: "backend-agent-instructions".to_string(),
        },
    };

    service
        .write_artifact(&target, "## ADDED Requirements\n")
        .expect("write spec delta");

    let spec_path = ito_path
        .join("changes")
        .join("025-11_demo")
        .join("specs")
        .join("backend-agent-instructions")
        .join("spec.md");
    assert_eq!(
        std::fs::read_to_string(spec_path).expect("read spec delta"),
        "## ADDED Requirements\n"
    );
}

#[test]
fn fs_service_loads_missing_and_rejects_invalid_or_unknown_targets() {
    let tmp = tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

    let service = FsChangeArtifactMutationService::new(&ito_path);
    let missing = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::Design,
    };
    assert_eq!(service.load_artifact(&missing).expect("load missing"), None);

    let invalid = ChangeArtifactRef {
        change_id: "bad".to_string(),
        artifact: ChangeArtifactKind::Proposal,
    };
    let err = service
        .load_artifact(&invalid)
        .expect_err("invalid change id should fail");
    assert!(err.to_string().contains("Invalid change id 'bad'"));

    let unknown = ChangeArtifactRef {
        change_id: "025-12_missing".to_string(),
        artifact: ChangeArtifactKind::Proposal,
    };
    let err = service
        .write_artifact(&unknown, "# Missing\n")
        .expect_err("unknown change should fail");
    assert!(
        err.to_string()
            .contains("Change '025-12_missing' not found")
    );
}

#[test]
fn fs_service_rejects_unsafe_spec_capability_and_patch_errors() {
    let tmp = tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

    let service = FsChangeArtifactMutationService::new(&ito_path);
    let unsafe_spec = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::SpecDelta {
            capability: "../escape".to_string(),
        },
    };
    let err = service
        .write_artifact(&unsafe_spec, "bad")
        .expect_err("unsafe capability should fail");
    assert!(
        err.to_string()
            .contains("capability contains unsafe path characters")
    );

    let proposal = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::Proposal,
    };
    service
        .write_artifact(&proposal, "# Proposal\n")
        .expect("write proposal");
    let invalid = service
        .patch_artifact(&proposal, "--- proposal\n+++ proposal\n@@ invalid\n")
        .expect_err("invalid patch should fail");
    assert!(invalid.to_string().contains("Invalid patch"));

    let clean_miss = service
        .patch_artifact(
            &proposal,
            "--- proposal\n+++ proposal\n@@ -1 +1 @@\n-# Other\n+# Updated\n",
        )
        .expect_err("non-matching patch should fail");
    assert!(
        clean_miss
            .to_string()
            .contains("Patch did not apply cleanly")
    );
}

#[derive(Debug, Clone)]
struct FakeBundleClient {
    bundle: Arc<Mutex<ArtifactBundle>>,
    revision: Arc<Mutex<u32>>,
}

impl ChangeArtifactBundleClient for FakeBundleClient {
    fn pull_bundle(&self, _change_id: &str) -> ChangeArtifactMutationServiceResult<ArtifactBundle> {
        Ok(self.bundle.lock().expect("bundle lock").clone())
    }

    fn push_bundle(
        &self,
        _change_id: &str,
        bundle: &ArtifactBundle,
    ) -> ChangeArtifactMutationServiceResult<String> {
        *self.bundle.lock().expect("bundle lock") = bundle.clone();
        let mut revision = self.revision.lock().expect("revision lock");
        *revision += 1;
        Ok(format!("rev-{}", *revision))
    }
}

#[test]
fn bundle_service_patches_design_and_returns_revision() {
    let client = FakeBundleClient {
        bundle: Arc::new(Mutex::new(ArtifactBundle {
            change_id: "025-11_demo".to_string(),
            proposal: Some("# Proposal\n".to_string()),
            design: Some("# Design\n".to_string()),
            tasks: None,
            specs: vec![],
            revision: "rev-1".to_string(),
        })),
        revision: Arc::new(Mutex::new(1)),
    };

    let service = BundleBackedChangeArtifactMutationService::new(client.clone());
    let target = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::Design,
    };
    let patch = "--- design\n+++ design\n@@ -1 +1 @@\n-# Design\n+# Updated Design\n";

    let result = service
        .patch_artifact(&target, patch)
        .expect("patch design");
    assert_eq!(result.revision.as_deref(), Some("rev-2"));
    assert_eq!(
        client.bundle.lock().expect("bundle lock").design.as_deref(),
        Some("# Updated Design\n")
    );
}

#[test]
fn bundle_service_loads_writes_and_sorts_spec_deltas() {
    let client = FakeBundleClient {
        bundle: Arc::new(Mutex::new(ArtifactBundle {
            change_id: "025-11_demo".to_string(),
            proposal: Some("# Proposal\n".to_string()),
            design: None,
            tasks: Some("- [ ] task\n".to_string()),
            specs: vec![("zeta".to_string(), "old".to_string())],
            revision: "rev-1".to_string(),
        })),
        revision: Arc::new(Mutex::new(1)),
    };
    let service = BundleBackedChangeArtifactMutationService::new(client.clone());

    let tasks = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::Tasks,
    };
    assert_eq!(
        service
            .load_artifact(&tasks)
            .expect("load tasks")
            .as_deref(),
        Some("- [ ] task\n")
    );

    let alpha = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::SpecDelta {
            capability: "alpha".to_string(),
        },
    };
    let result = service
        .write_artifact(&alpha, "alpha spec")
        .expect("write new spec");
    assert!(!result.existed);
    assert_eq!(result.revision.as_deref(), Some("rev-2"));

    let zeta = ChangeArtifactRef {
        change_id: "025-11_demo".to_string(),
        artifact: ChangeArtifactKind::SpecDelta {
            capability: "zeta".to_string(),
        },
    };
    let result = service
        .write_artifact(&zeta, "new zeta")
        .expect("replace existing spec");
    assert!(result.existed);

    let bundle = client.bundle.lock().expect("bundle lock");
    assert_eq!(
        bundle.specs,
        vec![
            ("alpha".to_string(), "alpha spec".to_string()),
            ("zeta".to_string(), "new zeta".to_string()),
        ]
    );
}

#[test]
fn backend_errors_map_to_actionable_mutation_errors() {
    let lease = change_artifact_error_from_backend(BackendError::LeaseConflict(LeaseConflict {
        change_id: "025-11_demo".to_string(),
        holder: "worker-a".to_string(),
        expires_at: None,
    }));
    assert!(lease.to_string().contains("claimed by 'worker-a'"));

    let revision =
        change_artifact_error_from_backend(BackendError::RevisionConflict(RevisionConflict {
            change_id: "025-11_demo".to_string(),
            local_revision: "old".to_string(),
            server_revision: "new".to_string(),
        }));
    assert!(revision.to_string().contains("Retry after re-reading"));

    let unavailable =
        change_artifact_error_from_backend(BackendError::Unavailable("down".to_string()));
    assert_eq!(unavailable.to_string(), "backend unavailable: down");

    let unauthorized =
        change_artifact_error_from_backend(BackendError::Unauthorized("bad token".to_string()));
    assert_eq!(unauthorized.to_string(), "backend auth failed: bad token");

    let not_found = change_artifact_error_from_backend(BackendError::NotFound("gone".to_string()));
    assert_eq!(not_found.to_string(), "backend resource not found: gone");

    let other = change_artifact_error_from_backend(BackendError::Other("boom".to_string()));
    assert_eq!(other.to_string(), "boom");
}

#[derive(Debug, Clone)]
struct FakeSyncClient {
    bundle: ArtifactBundle,
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, _change_id: &str) -> Result<ArtifactBundle, BackendError> {
        Ok(self.bundle.clone())
    }

    fn push(&self, change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        Ok(PushResult {
            change_id: change_id.to_string(),
            new_revision: "remote-rev".to_string(),
        })
    }
}

#[test]
fn remote_bundle_client_delegates_pull_and_push() {
    let bundle = ArtifactBundle {
        change_id: "025-11_demo".to_string(),
        proposal: Some("# Proposal\n".to_string()),
        design: None,
        tasks: None,
        specs: vec![],
        revision: "rev-1".to_string(),
    };
    let client = RemoteChangeArtifactBundleClient::new(FakeSyncClient {
        bundle: bundle.clone(),
    });

    assert_eq!(
        client
            .pull_bundle("025-11_demo")
            .expect("pull bundle")
            .proposal,
        bundle.proposal
    );
    assert_eq!(
        client
            .push_bundle("025-11_demo", &bundle)
            .expect("push bundle"),
        "remote-rev"
    );
}

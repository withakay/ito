use ito_config::types::{CoordinationStorage, ItoConfig};

use super::{CapabilityPreflight, CompiledCapabilities, CompiledFeature, preflight_config};
use crate::errors::CoreError;

#[test]
fn reports_the_compiled_feature_set() {
    let capabilities = CompiledCapabilities::current();
    assert_eq!(capabilities.backend, cfg!(feature = "backend"));
    assert_eq!(
        capabilities.coordination_branch,
        cfg!(feature = "coordination-branch")
    );
    assert_eq!(
        capabilities.contains(CompiledFeature::Backend),
        cfg!(feature = "backend")
    );
}

#[test]
fn backend_config_is_rejected_or_accepted_by_compiled_capability() {
    let mut config = ItoConfig::default();
    config.backend.enabled = true;

    let result = preflight_config(&config, CapabilityPreflight::Stateful);
    if cfg!(feature = "backend") {
        result.expect("backend-enabled builds should accept backend config");
    } else {
        assert_feature_error(
            result.unwrap_err(),
            CompiledFeature::Backend,
            "backend.enabled",
        );
    }
}

#[test]
fn worktree_coordination_is_rejected_or_accepted_by_compiled_capability() {
    let mut config = ItoConfig::default();
    config.backend.enabled = false;
    config.changes.coordination_branch.enabled.0 = true;
    config.changes.coordination_branch.storage = CoordinationStorage::Worktree;

    let result = preflight_config(&config, CapabilityPreflight::Stateful);
    if cfg!(feature = "coordination-branch") {
        result.expect("coordination-enabled builds should accept worktree storage");
    } else {
        assert_feature_error(
            result.unwrap_err(),
            CompiledFeature::CoordinationBranch,
            "changes.coordination_branch.enabled",
        );
    }
}

#[test]
fn either_legacy_coordination_signal_requests_the_feature() {
    for (enabled, storage, requested_by) in [
        (
            true,
            CoordinationStorage::Embedded,
            "changes.coordination_branch.enabled",
        ),
        (
            false,
            CoordinationStorage::Worktree,
            "changes.coordination_branch.storage=worktree",
        ),
    ] {
        let mut config = ItoConfig::default();
        config.backend.enabled = false;
        config.changes.coordination_branch.enabled.0 = enabled;
        config.changes.coordination_branch.storage = storage;

        let result = preflight_config(&config, CapabilityPreflight::Stateful);
        if cfg!(feature = "coordination-branch") {
            result.expect("coordination-enabled builds should accept legacy config");
        } else {
            assert_feature_error(
                result.unwrap_err(),
                CompiledFeature::CoordinationBranch,
                requested_by,
            );
        }
    }
}

#[test]
fn recovery_preflight_remains_available_for_legacy_config() {
    let mut config = ItoConfig::default();
    config.backend.enabled = true;
    config.changes.coordination_branch.enabled.0 = true;
    config.changes.coordination_branch.storage = CoordinationStorage::Worktree;

    preflight_config(&config, CapabilityPreflight::Recovery)
        .expect("recovery commands must remain available");
}

fn assert_feature_error(error: CoreError, feature: CompiledFeature, requested_by: &str) {
    let CoreError::FeatureUnavailable {
        feature: actual,
        requested_by: actual_request,
        recovery,
    } = error
    else {
        panic!("expected FeatureUnavailable, got {error:?}");
    };
    assert_eq!(actual, feature);
    assert_eq!(actual_request, requested_by);
    assert!(!recovery.is_empty());
}

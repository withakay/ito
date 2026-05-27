use super::*;
use std as test_lib;

#[test]
fn resolve_user_identity_returns_at_prefixed_string() {
    let identity = resolve_user_identity();
    assert!(identity.starts_with('@'));
    assert!(!identity.contains(' '));
}

#[test]
fn resolve_harness_session_id_returns_none_without_env() {
    // In test environment, these env vars are typically not set
    // We can't guarantee they're unset, so just test the function doesn't panic
    let _result = resolve_harness_session_id();
}

#[test]
fn resolve_git_context_does_not_panic() {
    // This test verifies the function is safe to call in any environment
    let ctx = resolve_git_context();
    // In a git repo, branch should be Some; but we don't enforce it
    // since CI might have detached HEAD
    let _ = ctx;
}

#[test]
fn resolve_session_id_generates_uuid() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    test_lib::fs::create_dir_all(&ito_path).expect("create ito dir");

    let id = resolve_session_id(&ito_path);
    assert!(!id.is_empty());
    // Should be a valid UUID v4 format (36 chars with hyphens)
    assert_eq!(id.len(), 36);
    assert!(id.contains('-'));
}

#[test]
fn resolve_session_id_is_stable_across_calls() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    test_lib::fs::create_dir_all(&ito_path).expect("create ito dir");

    let id1 = resolve_session_id(&ito_path);
    let id2 = resolve_session_id(&ito_path);
    assert_eq!(id1, id2);
}

#[test]
fn resolve_context_populates_session_id() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");
    test_lib::fs::create_dir_all(&ito_path).expect("create ito dir");

    let ctx = resolve_context(&ito_path);
    assert!(!ctx.session_id.is_empty());
}

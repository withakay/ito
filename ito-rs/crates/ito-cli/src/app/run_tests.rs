use super::{is_recovery_safe_invocation, removed_serve_api_replacement};
use crate::cli::RemovedServeApiArgs;

#[test]
fn removed_serve_api_replacement_preserves_flags_and_args() {
    let replacement = removed_serve_api_replacement(&RemovedServeApiArgs {
        args: vec![
            "--service".to_string(),
            "--bind".to_string(),
            "127.0.0.1".to_string(),
        ],
    });

    assert_eq!(
        replacement,
        "ito backend serve --service --bind 127.0.0.1".to_string()
    );
}

#[test]
fn backend_serve_is_independent_of_project_coordination_config() {
    let args = [
        "backend".to_string(),
        "serve".to_string(),
        "--port".to_string(),
        "9010".to_string(),
    ];

    assert!(is_recovery_safe_invocation(&args));
    assert!(!is_recovery_safe_invocation(&[
        "backend".to_string(),
        "status".to_string(),
    ]));
}

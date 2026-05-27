use super::removed_serve_api_replacement;
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

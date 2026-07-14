use crate::cli::SyncAction;
use crate::cli_error::{CliError, CliResult};
use crate::runtime::Runtime;

const RECOVERY: &str =
    "install an experimental build with the backend feature, or disable backend.enabled";

pub(super) fn sync_after_mutation(_rt: &Runtime, _change_id: &str) {}

pub(super) fn handle_backend_claim(
    _rt: &Runtime,
    _change_id: &str,
    want_json: bool,
) -> CliResult<()> {
    unavailable("ito tasks claim", want_json)
}

pub(super) fn handle_backend_release(
    _rt: &Runtime,
    _change_id: &str,
    want_json: bool,
) -> CliResult<()> {
    unavailable("ito tasks release", want_json)
}

pub(super) fn handle_backend_allocate(_rt: &Runtime, want_json: bool) -> CliResult<()> {
    unavailable("ito tasks allocate", want_json)
}

pub(super) fn handle_backend_sync(
    _rt: &Runtime,
    _action: &SyncAction,
    want_json: bool,
) -> CliResult<()> {
    unavailable("ito tasks sync", want_json)
}

fn unavailable(requested_by: &str, want_json: bool) -> CliResult<()> {
    let error = CliError::feature_unavailable("backend", requested_by, RECOVERY);
    if want_json {
        let value = error
            .feature_unavailable_json()
            .expect("feature-unavailable errors have JSON details");
        println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("JSON value serializes")
        );
        return Err(CliError::silent());
    }
    Err(error)
}

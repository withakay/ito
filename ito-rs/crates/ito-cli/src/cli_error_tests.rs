use ito_core::capabilities::CompiledFeature;
use ito_core::errors::CoreError;

use super::CliError;

#[test]
fn core_feature_error_retains_stable_json_fields() {
    let error = CliError::from_core(CoreError::feature_unavailable(
        CompiledFeature::CoordinationBranch,
        "changes.coordination_branch.enabled",
        "migrate-to-main",
    ));
    let json = error.feature_unavailable_json().expect("typed JSON");

    assert_eq!(json["error"]["kind"], "feature_unavailable");
    assert_eq!(json["error"]["feature"], "coordination-branch");
    assert_eq!(
        json["error"]["requested_by"],
        "changes.coordination_branch.enabled"
    );
    assert_eq!(json["error"]["recovery"], "migrate-to-main");
}

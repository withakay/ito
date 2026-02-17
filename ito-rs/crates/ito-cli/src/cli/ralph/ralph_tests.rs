use super::*;

#[test]
fn harness_arg_converts_to_core_harness_name() {
    assert_eq!(
        ito_core::harness::HarnessName::from(HarnessArg::Opencode),
        ito_core::harness::HarnessName::Opencode
    );
    assert_eq!(
        ito_core::harness::HarnessName::from(HarnessArg::Claude),
        ito_core::harness::HarnessName::Claude
    );
    assert_eq!(
        ito_core::harness::HarnessName::from(HarnessArg::Codex),
        ito_core::harness::HarnessName::Codex
    );
    assert_eq!(
        ito_core::harness::HarnessName::from(HarnessArg::Copilot),
        ito_core::harness::HarnessName::GithubCopilot
    );
    assert_eq!(
        ito_core::harness::HarnessName::from(HarnessArg::Stub),
        ito_core::harness::HarnessName::Stub
    );
}

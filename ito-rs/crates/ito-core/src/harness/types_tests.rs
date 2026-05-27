use super::*;

#[test]
fn harness_help_matches_user_facing() {
    let mut names = Vec::new();
    for name in HarnessName::user_facing() {
        names.push(name.as_str());
    }
    assert_eq!(names, vec!["opencode", "claude", "codex", "copilot"]);
}

#[test]
fn from_str_valid_variants() {
    assert_eq!(
        "opencode".parse::<HarnessName>().unwrap(),
        HarnessName::Opencode
    );
    assert_eq!(
        "claude".parse::<HarnessName>().unwrap(),
        HarnessName::Claude
    );
    assert_eq!("codex".parse::<HarnessName>().unwrap(), HarnessName::Codex);
    assert_eq!(
        "copilot".parse::<HarnessName>().unwrap(),
        HarnessName::GithubCopilot
    );
    assert_eq!(
        "github-copilot".parse::<HarnessName>().unwrap(),
        HarnessName::GithubCopilot
    );
    assert_eq!("stub".parse::<HarnessName>().unwrap(), HarnessName::Stub);
}

#[test]
fn from_str_invalid_returns_error() {
    let invalid_inputs = vec!["invalid", "", "OPENCODE"];
    for input in invalid_inputs {
        let result = input.parse::<HarnessName>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.input, input);
    }
}

#[test]
fn as_str_all_variants() {
    assert_eq!(HarnessName::Opencode.as_str(), "opencode");
    assert_eq!(HarnessName::Claude.as_str(), "claude");
    assert_eq!(HarnessName::Codex.as_str(), "codex");
    assert_eq!(HarnessName::GithubCopilot.as_str(), "copilot");
    assert_eq!(HarnessName::Stub.as_str(), "stub");
}

#[test]
fn display_matches_as_str() {
    let variants = vec![
        HarnessName::Opencode,
        HarnessName::Claude,
        HarnessName::Codex,
        HarnessName::GithubCopilot,
        HarnessName::Stub,
    ];
    for variant in variants {
        assert_eq!(format!("{}", variant), variant.as_str());
    }
}

#[test]
fn parse_error_display() {
    let err = HarnessNameParseError {
        input: "foo".to_string(),
    };
    assert_eq!(format!("{}", err), "Unknown harness name: foo");
}

fn make_result(exit_code: i32) -> HarnessRunResult {
    HarnessRunResult {
        stdout: String::new(),
        stderr: String::new(),
        exit_code,
        duration: Duration::from_secs(1),
        timed_out: false,
    }
}

#[test]
fn is_retriable_for_all_retriable_codes() {
    let retriable_codes = vec![128, 129, 130, 131, 132, 134, 135, 136, 137, 139, 141, 143];
    for code in retriable_codes {
        let result = make_result(code);
        assert!(
            result.is_retriable(),
            "Exit code {} should be retriable",
            code
        );
    }
}

#[test]
fn is_not_retriable_for_normal_codes() {
    let normal_codes = vec![0, 1, 2, 127, 133, 144, 255, -1];
    for code in normal_codes {
        let result = make_result(code);
        assert!(
            !result.is_retriable(),
            "Exit code {} should not be retriable",
            code
        );
    }
}

use super::*;

#[test]
fn build_prompt_preamble_includes_iteration() {
    let result = build_prompt_preamble(3, Some(10), 1, "DONE_TOKEN", None, None, "Test task");
    assert!(result.contains("3"));
    assert!(result.contains("10"));
}

#[test]
fn build_prompt_preamble_includes_completion_promise() {
    let result = build_prompt_preamble(1, Some(5), 1, "DONE_TOKEN", None, None, "Test task");
    assert!(result.contains("DONE_TOKEN"));
}

#[test]
fn build_prompt_preamble_includes_context() {
    let result = build_prompt_preamble(
        1,
        Some(5),
        1,
        "DONE_TOKEN",
        Some("extra context"),
        None,
        "Test task",
    );
    assert!(result.contains("extra context"));
}

#[test]
fn build_prompt_preamble_includes_validation_failure() {
    let result = build_prompt_preamble(
        1,
        Some(5),
        1,
        "DONE_TOKEN",
        None,
        Some("task X not done"),
        "Test task",
    );
    assert!(result.contains("task X not done"));
}

#[test]
fn build_prompt_preamble_omits_context_when_none() {
    let result = build_prompt_preamble(1, Some(5), 1, "DONE_TOKEN", None, None, "Test task");
    assert!(!result.contains("Additional Context"));
}

#[test]
fn build_prompt_preamble_omits_validation_when_none() {
    let result = build_prompt_preamble(1, Some(5), 1, "DONE_TOKEN", None, None, "Test task");
    assert!(!result.contains("Validation Failure"));
}

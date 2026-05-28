use super::*;

#[test]
fn html_viewer_reports_expected_name() {
    assert_eq!(HtmlViewer.name(), "html");
}

#[test]
fn html_viewer_reports_expected_description() {
    let desc = HtmlViewer.description();
    assert!(
        desc.contains("HTML"),
        "description should mention HTML: {desc}"
    );
    assert!(
        desc.contains("pandoc"),
        "description should mention pandoc: {desc}"
    );
}

#[test]
fn html_viewer_availability_depends_on_pandoc() {
    // This test validates the code path rather than the environment.
    // If pandoc is not on PATH, is_available() returns false.
    let viewer = HtmlViewer;
    let pandoc_present = command_on_path("pandoc");
    let opener_present = command_on_path(browser_opener());
    assert_eq!(viewer.is_available(), pandoc_present && opener_present);
}

#[test]
fn html_viewer_open_errors_when_pandoc_missing() {
    // Only run when pandoc is genuinely absent.
    if command_on_path("pandoc") {
        return;
    }
    let result = HtmlViewer.open("# Test");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("pandoc"), "error should mention pandoc: {err}");
    assert!(
        err.contains("https://pandoc.org"),
        "error should include install hint: {err}"
    );
}

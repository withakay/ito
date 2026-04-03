<!-- ITO:START -->
## ADDED Requirements

### Requirement: HTML browser viewer backend

The system SHALL provide an `html` viewer backend for `ito view proposal` that converts the collected change artifacts to HTML using `pandoc` and opens the result in the system default browser.

#### Scenario: Document rendered as HTML and opened in browser

- **WHEN** user selects or specifies `--viewer html`
- **THEN** the system writes the collected document to a temporary Markdown file
- **AND** invokes `pandoc` to convert the Markdown to a standalone HTML file with embedded styles
- **AND** opens the resulting HTML file in the system default browser (`open` on macOS, `xdg-open` on Linux)

#### Scenario: Pandoc not installed

- **WHEN** `pandoc` is not found on PATH
- **THEN** the system displays an error: "✗ 'pandoc' is required for the HTML viewer. Install it from https://pandoc.org/installing.html"
- **AND** exits with a non-zero status code

#### Scenario: Browser opener not available

- **WHEN** neither `open` (macOS) nor `xdg-open` (Linux) is found on PATH
- **THEN** the system displays an error indicating the HTML file path so the user can open it manually
- **AND** exits with a non-zero status code

#### Scenario: Temporary file cleaned up

- **WHEN** the HTML viewer has finished opening the browser
- **THEN** the temporary Markdown and HTML files are scheduled for cleanup (e.g., after a short delay or on next Ito invocation)
<!-- ITO:END -->

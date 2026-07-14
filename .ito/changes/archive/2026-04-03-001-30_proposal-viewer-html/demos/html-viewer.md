# HTML Viewer Backend Implementation

*2026-04-02T01:03:38Z by Showboat 0.6.1*
<!-- showboat-id: 04a8ee00-645a-4785-ac39-29a8db6b6834 -->

Implemented HtmlViewer backend: converts markdown to HTML via pandoc, opens in system browser. Registered in default ViewerRegistry. Added unit tests + integration tests.

```bash
cd ito-rs && rtk cargo test -p ito-core viewer::html 2>&1 | tail -5
```

```output
cargo test: 4 passed, 766 filtered out (48 suites, 0.00s)
```

```bash
cd ito-rs && rtk cargo test -p ito-cli view_proposal 2>&1 | tail -5
```

```output
cargo test: 8 passed, 316 filtered out (48 suites, 0.75s)
```

```bash
cd ito-rs && rtk cargo clippy -p ito-core -p ito-cli -- -D warnings 2>&1 | tail -3
```

```output
cargo clippy: No issues found
```

```bash
ito validate 001-30 --strict 2>&1
```

```output
Change '001-30' is valid
```

```bash
head -50 ito-rs/crates/ito-core/src/viewer/html.rs
```

```output
//! HTML browser viewer backend.
//!
//! Converts a markdown document to standalone HTML via `pandoc` and opens it in
//! the system default browser (`open` on macOS, `xdg-open` on Linux).

use std::process::Command;

use crate::errors::{CoreError, CoreResult};

use super::ViewerBackend;
use super::bat::command_on_path;

/// Render markdown as HTML in the system browser via `pandoc`.
pub struct HtmlViewer;

/// Return the platform-specific command for opening a file in the default application.
///
/// Returns `open` on macOS and `xdg-open` on Linux/other Unix systems.
/// Windows is not currently supported.
fn browser_opener() -> &'static str {
    if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
}

impl ViewerBackend for HtmlViewer {
    fn name(&self) -> &str {
        "html"
    }

    fn description(&self) -> &str {
        "Open the proposal as HTML in the system browser (requires pandoc)"
    }

    fn is_available(&self) -> bool {
        command_on_path("pandoc") && command_on_path(browser_opener())
    }

    fn open(&self, content: &str) -> CoreResult<()> {
        if !command_on_path("pandoc") {
            return Err(CoreError::not_found(
                "pandoc is required for the HTML viewer. \
                 Install it from https://pandoc.org/installing.html",
            ));
        }

        let opener = browser_opener();
        if !command_on_path(opener) {
```

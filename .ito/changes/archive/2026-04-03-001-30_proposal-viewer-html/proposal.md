<!-- ITO:START -->
## Why

The proposal viewer introduced in 001-29 supports terminal-based viewers (tmux/neovim, bat, glow). For richer review sessions — particularly when sharing proposals with stakeholders or reviewing complex spec documents — a rendered HTML view opened in the system browser is significantly more readable. This change adds the HTML viewer backend as a natural extension of the `proposal-viewer` dispatch architecture established in 001-29.

## What Changes

- Add a `pandoc` → browser HTML viewer backend to `ito view proposal`:
  - Converts the collected artifact document to HTML using `pandoc`
  - Writes the output to a temporary file
  - Opens the temporary file in the system default browser (`open` on macOS, `xdg-open` on Linux)
- The new backend registers itself in the viewer dispatch layer introduced by 001-29; no changes to core command logic required.
- `--viewer html` flag selects this backend non-interactively.
- The interactive prompt (from 001-29) gains a `html (browser)` option.
- Graceful error if `pandoc` is not installed, with an installation hint.

## Capabilities

### New Capabilities

- `proposal-viewer-html`: The HTML viewer backend for `ito view proposal`. Converts markdown artifacts to HTML via `pandoc` and opens the result in the system browser. Plugs into the `ViewerBackend` trait from `proposal-viewer` (001-29).

### Modified Capabilities

- `proposal-viewer`: The viewer prompt gains the `html (browser)` option. The `--viewer` flag gains `html` as a valid value. No other behavioral changes.

## Impact

- `ito-rs/crates/ito-core/` — new `HtmlViewerBackend` implementing the `ViewerBackend` trait; registration in the viewer dispatch
- External tool dependency: `pandoc` (optional; graceful error with install hint if absent)
- Platform: `open` (macOS) / `xdg-open` (Linux) for launching the browser
- Depends on 001-29 (`proposal-viewer-command`) — the `ViewerBackend` trait and dispatch layer must exist before this backend can be added
<!-- ITO:END -->

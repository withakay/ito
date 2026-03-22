<!-- ITO:START -->
# Tasks for: 001-30_proposal-viewer-html

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-30_proposal-viewer-html
ito tasks next 001-30_proposal-viewer-html
ito tasks start 001-30_proposal-viewer-html 1.1
ito tasks complete 001-30_proposal-viewer-html 1.1
```

**Note**: This change depends on `001-29_proposal-viewer-command` being complete. The `ViewerBackend` trait and registry must exist before this backend can be added.

______________________________________________________________________

## Wave 1: Implement HtmlViewerBackend

### Task 1.1: Implement HtmlViewer

- **Files**: `ito-rs/crates/ito-core/src/viewer/html.rs`
- **Dependencies**: None
- **Action**: Implement `ViewerBackend` for `HtmlViewer`:
  - `is_available()`: checks `pandoc` on PATH
  - `open(content)`: writes content to a tempfile `.md`; invokes `pandoc --standalone --from=markdown --to=html5 -o <tmpfile>.html <tmpfile>.md`; then opens `<tmpfile>.html` with `open` (macOS) or `xdg-open` (Linux) via `std::process::Command`
  - On pandoc missing: error with install hint URL
  - On opener missing: error displaying the file path for manual opening
- **Verify**: Unit tests: `is_available()` false when pandoc not on PATH; `open()` errors with expected message when pandoc missing
- **Done When**: Unit tests pass; `cargo test -p ito-core viewer::html` green
- **Status**: [ ] pending

### Task 1.2: Register HtmlViewer in registry

- **Files**: `ito-rs/crates/ito-core/src/viewer/registry.rs`
- **Dependencies**: Task 1.1
- **Action**: Add `HtmlViewer` to the default `ViewerRegistry` instance; ensure it appears in `available_viewers()` only when `pandoc` is detected
- **Verify**: Registry unit test confirms `html` viewer appears when pandoc present; absent otherwise
- **Done When**: Registry test updated and passing
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2: CLI integration

- **Depends On**: Wave 1

### Task 2.1: Add `html` as valid --viewer value

- **Files**: `ito-rs/crates/ito-cli/src/commands/view_proposal.rs`
- **Dependencies**: None
- **Action**: Confirm `--viewer html` is accepted by the existing flag (no code change needed if the registry drives valid values); verify the interactive prompt shows `html (browser)` when pandoc is detected; add any display name mapping if needed
- **Verify**: `ito view proposal <id> --viewer html` (with pandoc installed) runs without parse error; `--viewer html` without pandoc shows a clear error with install hint
- **Done When**: Both cases verified manually or via integration test
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3: Integration test and validation

- **Depends On**: Wave 2

### Task 3.1: Write integration test for HTML viewer

- **Files**: `ito-rs/crates/ito-cli/tests/` or integration test module
- **Dependencies**: None
- **Action**: Write an integration test that mocks or stubs the `pandoc` and `open`/`xdg-open` system calls and verifies: correct tempfile creation, pandoc invoked with expected args, opener invoked with html file path, and graceful error when pandoc absent
- **Verify**: `cargo test -p ito-cli view_proposal_html` passes
- **Done When**: Integration test green; error case (pandoc missing) covered
- **Status**: [ ] pending

### Task 3.2: Validate with ito validate

- **Files**: N/A
- **Dependencies**: Task 3.1
- **Action**: Run `ito validate 001-30_proposal-viewer-html --strict`
- **Verify**: Exits 0 with no errors
- **Done When**: Validation passes
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
<!-- ITO:END -->

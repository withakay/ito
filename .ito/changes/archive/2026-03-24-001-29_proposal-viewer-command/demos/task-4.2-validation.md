# Task 4.2: Strict validation

*2026-03-22T13:56:35Z by Showboat 0.6.1*
<!-- showboat-id: 0e751891-0fda-4579-b520-be432bfb6329 -->

Verified the proposal viewer change package passes strict Ito validation after the viewer, registry, CLI, and snapshot updates.

```bash
ito validate 001-29_proposal-viewer-command --strict
```

```output
Change '001-29_proposal-viewer-command' is valid
```

```bash
make check
```

```output
check for added large files..............................................Passed
check for merge conflicts................................................Passed
check toml...............................................................Passed
check yaml...............................................................Passed
check json...............................................................Passed
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
pretty format json.......................................................Passed
yamllint.................................................................Passed
markdownlint-cli2........................................................Passed
cargo fmt (ito-rs).......................................................Passed
forbid local version metadata in Cargo.toml..............................Passed
cargo clippy (ito-rs)....................................................Passed
cargo doc warnings as errors (ito-rs)....................................Passed
cargo test with coverage (ito-rs)........................................Passed
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
```

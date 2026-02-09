## Why

Release-plz is broken in two ways:

1. **CI config path causes repo-not-found error** — Every workflow run fails with `could not find repository at '/tmp/.../ito-rs'`. The GitHub Action passes `config: ito-rs/release-plz.toml`, which makes release-plz resolve the repo root as `ito-rs/` (a subdirectory with no `.git`). Passing `--config ito-rs/release-plz.toml` locally reproduces the same failure.

2. **"Up-to-date" false positive without config** — Running from the repo root with just `--manifest-path ito-rs/Cargo.toml` (no `--config`) works but incorrectly reports "the repository is already up-to-date" despite 71 commits and 207 changed files in `ito-rs/crates/` since the `v0.1.0` tag. This likely stems from release-plz's version/tag resolution logic interacting with the bare-repo worktree layout (`gitdir: ../.bare/worktrees/main`), or from the absence of per-package tags (`ito-common-v0.1.0`, etc.) when `git_tag_name` is configured as unified `v{{ version }}`.

Both issues must be resolved for release-plz to function. The fix is to move `release-plz.toml` to the git repo root with a `manifest_path` setting, then debug the "up-to-date" false positive (likely by ensuring `git_tag_name` is configured correctly at the repo root level). Additionally, crates.io publishing needs to be enabled (currently explicitly disabled with `publish = false` / `git_only = true`) and all CI/CD workflows should be migrated from GitHub-hosted runners to the `withakay-selfhost` runner group.

## What Changes

- **Fix release-plz repo detection** — Move `release-plz.toml` from `ito-rs/` to the git repo root and add `manifest_path = "ito-rs/Cargo.toml"` to the `[workspace]` section so release-plz can find both the `.git` directory and the Cargo workspace
- **Update release-plz GitHub Action** — Remove the `manifest_path` and `config` inputs from the action (release-plz auto-discovers `release-plz.toml` at repo root) or adjust paths accordingly
- **Enable crates.io publishing** — Remove `publish = false` and `git_only = true` from the workspace config; add `publish = false` to crates that should NOT be published (`ito-test-support`, `ito-web`)
- **Resolve `ito-cli` crate name conflict** — An unrelated `ito-cli` package exists on crates.io; determine strategy (rename package, secure name, or skip publishing the CLI crate)
- **Configure per-package publish/release settings** — Ensure release ordering respects the workspace dependency graph
- **Migrate all CI/CD workflows to self-hosted runners** — Replace `ubuntu-latest` with `group: withakay-selfhost` in `ci.yml`, `release-plz.yml`, `release.yml`, `update-homebrew.yml`, `polish-release-notes.yml`, and `claude-code-review.yml`
- **Clean up vestigial Release Please references** — Remove `workflow_run` trigger for "Release Please" from `release.yml`, update Makefile `release` target

## Capabilities

### New Capabilities

- `crates-io-publishing`: Requirements for publishing Ito workspace crates to the crates.io registry, including package metadata, publish ordering, registry token configuration, and release-plz config placement

### Modified Capabilities

- `release-artifacts`: Add requirements for crates.io publish step in the release pipeline, self-hosted runner usage, and release-plz configuration that works with the subdirectory workspace layout
- `distribution`: Add self-hosted runner group requirement for all CI/CD workflows and removal of vestigial Release Please references

## Impact

- **Configuration**: `ito-rs/release-plz.toml` moves to repo root as `release-plz.toml`; significant restructuring of settings
- **CI workflows**: All 6 files in `.github/workflows/` — runner group migration + release-plz action path updates
- **Cargo manifests**: `ito-rs/crates/ito-test-support/Cargo.toml`, `ito-rs/crates/ito-web/Cargo.toml` — add `publish = false`
- **Crate naming**: The `ito-cli` package name conflicts with an existing crate on crates.io; resolution needed
- **Secrets**: `CARGO_REGISTRY_TOKEN` must be configured with a valid crates.io API token
- **Dependency ordering**: release-plz must publish crates in dependency order (ito-common → ito-config → ito-domain → ito-templates → ito-logging → ito-core → ito-cli)
- **Makefile**: Remove/update targets referencing non-existent Release Please workflow

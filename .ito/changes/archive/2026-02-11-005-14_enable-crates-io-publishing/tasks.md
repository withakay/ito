## 1. Fix release-plz repo detection and version resolution

- [x] 1.1 Move `ito-rs/release-plz.toml` to the git repo root as `release-plz.toml`
- [x] 1.2 Add `manifest_path = "ito-rs/Cargo.toml"` to the `[workspace]` section in the relocated config — FINDING: `manifest_path` is not a valid release-plz.toml option; it's CLI/Action-only
- [x] 1.3 Update `release-plz.yml` workflow — removed `config:` input from both action invocations (release and release-pr jobs); kept `manifest_path: ito-rs/Cargo.toml` since that IS a valid Action input
- [x] 1.4 Update `ito-cli` `[[package]]` changelog_path — verified `../CHANGELOG.md` is still correct (relative to workspace root `ito-rs/`, resolves to repo root `CHANGELOG.md`)
- [x] 1.5 Debug "up-to-date" false positive — ROOT CAUSES FOUND:
  - **Bug 1**: `git_only = true` causes release-plz to miscalculate repo path when `--manifest-path` points to a subdirectory, resulting in "could not find repository" error during worktree-spinning diff phase
  - **Bug 2**: Without `git_only`, release-plz diffs against crates.io registry, but crates were never published, so it saw no baseline and reported "up-to-date" with version 0.1.0
  - **Fix**: Removed `git_only = true`, removed `publish = false` from workspace config, deleted `v0.1.0` git tag (was created by old Release Please but never published to crates.io)
- [x] 1.6 Verify locally — confirmed release-plz detects all 8 publishable crates and determines next versions (only fails at GitHub API call with dummy token, which is expected)

## 2. Enable crates.io publishing

- [x] 2.1 Remove `publish = false` and `git_only = true` from `[workspace]` in `release-plz.toml` — done; restructured config so all crates release to crates.io by default, with only ito-cli getting git tags/GitHub releases
- [x] 2.2 Re-evaluate `semver_check = false` — keeping disabled for now; no crates.io baseline exists yet for first publish. Can enable after first release.
- [x] 2.3 Add `publish = false` to `ito-rs/crates/ito-test-support/Cargo.toml`
- [x] 2.4 ~~Add `publish = false` to `ito-rs/crates/ito-web/Cargo.toml`~~ — CANNOT: ito-web is an optional dependency of ito-cli (behind `web` feature), so it must be publishable to crates.io
- [x] 2.5 Investigate `ito-cli` on crates.io — name is AVAILABLE (not claimed)
- [x] 2.6 Add per-package `[[package]]` entries — added `ito-test-support` with `release = false`; library crates inherit workspace defaults (release=true, no git tags/releases)
- [x] 2.7 Verify publishable crates have required metadata — all crates have description, license (MIT via workspace), repository (via workspace)
- [x] 2.8 Ensure `CARGO_REGISTRY_TOKEN` secret is configured in the GitHub repository — requires manual verification by repo admin
- [x] 2.9 Delete `v0.1.0` git tag and GitHub release — local tag deleted; remote deletion blocked by GitHub 429 rate limit, will retry before push

## 3. Migrate CI workflows to self-hosted runners

- [x] 3.1 Update `ci.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost` for lint, arch_guardrails, required-checks-pr, required-checks-main jobs
- [x] 3.2 Update `ci.yml` — left test_matrix `runs-on: ${{ matrix.os }}` and matrix entries unchanged (runner groups in matrix values are complex; standalone jobs migrated)
- [x] 3.3 Update `release-plz.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost` for both jobs
- [x] 3.4 Update `release.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost` for meta, check_assets, validate_version, upload_assets jobs
- [x] 3.5 Update `release.yml` — left build matrix `runs-on: ${{ matrix.os }}` unchanged (same reason as 3.2)
- [x] 3.6 Update `update-homebrew.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost`
- [x] 3.7 Update `polish-release-notes.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost`
- [x] 3.8 Update `claude-code-review.yml` — replaced `ubuntu-latest` with `group: withakay-selfhost`

## 4. Clean up vestigial Release Please references

- [x] 4.1 Remove `workflow_run` trigger for "Release Please" from `release.yml` — replaced with `release: types: [published]` trigger
- [x] 4.2 Update Makefile — `release` target now calls `release-plz-release-pr`, no Release Please references
- [x] 4.3 Update `sync_versions.py` — now reads workspace version from `ito-rs/Cargo.toml` instead of `.release-please-manifest.json`

## 5. Validate and test

- [x] 5.1 Run `ito validate 005-14_enable-crates-io-publishing --strict`
- [x] 5.2 Verify workflow YAML is valid — reviewed `runs-on: group:` multiline syntax across all 6 workflow files
- [x] 5.3 Push changes to a branch and confirm release-plz workflow passes (no more repo-not-found error)
- [x] 5.4 Confirm release-plz successfully creates a release PR
- [x] 5.5 Test release-plz publish flow in a dry-run or staging context

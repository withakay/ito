---
children_hash: f8af132bb9f6bd71cc82daaea93d553a683d7a26ca94f25deb2fda2e18d5cbc9
compression_ratio: 0.3047009598818607
condensation_order: 1
covers: [build_and_coverage_guardrails.md, context.md, installer_release_assets.md, manifesto_instruction_implementation_notes.md, release_plz_guardrails.md, release_workflow.md]
covers_token_total: 4063
summary_level: d1
token_count: 1238
type: summary
---
# release_workflow

This topic covers Ito’s release and installer pipeline, spanning release-plz, cargo-dist, installer assets, and release guardrails. The child entries below form the main drill-down points for configuration, automation, and platform-specific behavior.

## Core release pipeline — `release_workflow.md`
- Defines the end-to-end release flow:
  - merge release PR
  - `release-plz` publishes crates and tags `vX.Y.Z`
  - `cargo-dist` builds GitHub Releases from version tags
  - Homebrew formula updates are pushed to `withakay/homebrew-ito`
  - release notes are polished
- Key files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Important rule: do not set `git_only = true` in `release-plz.toml` because it can miscalculate repository paths during diff/worktree operations.
- Supports GitHub Releases, cross-platform installer artifacts, Homebrew formula updates, and local Homebrew installation via the `withakay/ito` tap.

## Installer distribution — `installer_release_assets.md`
- Documents installer scripts for Unix and Windows:
  - prefer `cargo-dist` release assets for current releases
  - fall back to legacy version-pinned archives when needed
  - verify SHA-256 checksums before extraction
  - copy the built `ito` binary into the install directory
- Key files:
  - `scripts/install.sh`
  - `scripts/install.ps1`
- Flow:
  - detect platform
  - determine target triple
  - resolve tag/version
  - download primary archive and checksum
  - fall back to legacy archive if needed
  - verify checksum
  - extract
  - locate binary
  - copy to install dir
- Notable rules:
  - Unix prefers `ito-cli-${TARGET}.tar.xz`
  - Windows prefers `ito-cli-$target.zip`
  - legacy fallback uses `ito-v<tag>-<target>.*`
  - Windows supports optional `AddToPath`
- Facts preserved:
  - Unix supports macOS and Linux only
  - Windows target maps AMD64 to `x86_64-pc-windows-msvc`

## Build and verification guardrails — `build_and_coverage_guardrails.md`
- Fixes build/verification issues around coverage and static guardrails.
- Key changes:
  - `make check` / test-coverage now derives `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset
  - introduced `ito-rs/tools/max_lines_baseline.txt` to track existing oversized Rust files and fail only on regressions/new violations
  - allowed `wit-bindgen@0.51` as a `wasip3` transitive duplicate in `cargo-deny`
- Key files:
  - `Makefile`
  - `ito-rs/tools/max_lines_baseline.txt`
- Main relationship:
  - coverage runs must tolerate mixed Homebrew + rustup environments
  - max-lines enforcement is baseline-driven
  - `cargo-deny` exception is narrowly scoped to `wit-bindgen@0.51`

## Manifesto instruction behavior — `manifesto_instruction_implementation_notes.md`
- Captures rendering and sync-status rules for manifesto generation.
- Main behaviors:
  - `synced_at_generation` is only populated when coordination sync returns `Synchronized`
  - `RateLimited` is not fresh success and should not be reported as such
  - full `--operation` requires `--change`
  - unconfigured operations render as `null`
- Structural relationship:
  - manifesto instruction visibility depends on resolved change state
  - sync reporting is tied directly to coordination sync outcomes
- Key facts:
  - `synced_at_generation` only on `Synchronized`
  - `RateLimited` means no observed sync during generation
  - embedded operation instructions are scoped to the resolved change state

## Release-plz guardrails — `release_plz_guardrails.md`
- Documents release-plz configuration after tracked-main authority cutover.
- Key files:
  - `.gitignore`
  - `release-plz.toml`
  - `.github/workflows/release-plz.yml`
- Main rules:
  - keep the five `.ito` authority roots tracked on `main`
  - never hide release dirtiness by ignoring or untracking canonical Ito state
  - do not generate or publish `docs/ito`
  - do not set `git_only = true`
- Configuration facts:
  - `allow_dirty = false`
  - `publish_allow_dirty = false`
  - workspace changelog and dependency updates are enabled
  - `cliff.toml` is used for changelog config
  - only `ito-cli` has git tags enabled
- Authority paths that must remain tracked:
  - `.ito/changes`
  - `.ito/specs`
  - `.ito/modules`
  - `.ito/workflows`
  - `.ito/audit`

## Shared topic context — `context.md`
- Provides the umbrella overview for `release_workflow`.
- Covers:
  - installer release asset naming
  - download fallback behavior
  - checksum validation
  - platform-specific installation steps
  - PATH handling on Windows
- Key concepts:
  - cargo-dist release artifacts
  - legacy archive fallback
  - SHA-256 checksum verification
  - platform target triples
  - optional PATH updates
- Related drill-downs:
  - `release_plz_guardrails`

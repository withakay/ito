## Context

Ito currently runs Rust build/test in CI across macOS, Linux, and Windows, but it does not ship standalone binaries or a one-liner installer. Users who do not have a Rust toolchain (or who want a faster install path) need a reliable, versioned release artifact.

## Goals / Non-Goals

**Goals:**

- Publish GitHub Releases that include `ito` binaries for macOS, Linux, and Windows.
- Cover x86_64 and ARM targets where practical.
- Include artifact integrity metadata (checksums) and fail the release if verification fails.
- Provide a macOS/Linux installer script suitable for `curl | sh`.

**Non-Goals:**

- NPM-based distribution (tracked as a separate change proposal).
- A Windows installer script (PowerShell) in the initial iteration.
- Package-manager integration (Homebrew, APT, etc.) in the initial iteration.

## Decisions

### Decision: Tag-driven releases

Use annotated tags (e.g. `vX.Y.Z`) as the release trigger.

Alternatives considered:

- Release-on-merge to `main`: simpler automation, but makes versioning ambiguous.
- Manual GitHub UI releases: higher risk of drift and unrepeatable artifacts.

### Decision: Artifact matrix and target set

Initial targets:

- macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
- Linux: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
- Windows: `x86_64-pc-windows-msvc`

Alternatives considered:

- Add Windows ARM and musl targets immediately: broader coverage, but higher complexity.

### Decision: Release build tooling

Prefer adopting a Rust-native release pipeline generator (e.g. `cargo-dist`) to:

- keep the workflow declarative,
- standardize artifact packaging,
- generate install scripts, and
- ensure checksums are produced consistently.

Alternatives considered:

- Hand-rolled GitHub Actions + custom scripts: lower dependency footprint, but higher long-term maintenance and more edge cases (especially for multi-arch Linux).

## Risks / Trade-offs

- Cross-compiling Linux ARM is often the highest-friction target → mitigate by using a proven toolchain approach and keeping the initial target list small.
- Installer security (piping from curl) → mitigate by publishing checksums, supporting checksum verification, and documenting best practices.
- macOS runner architecture drift on GitHub Actions → mitigate by pinning runner versions where needed.

## Migration Plan

1. Add the release workflow(s) behind a tag trigger and `workflow_dispatch`.
1. Validate the produced artifacts on each target.
1. Add/install script and document the recommended installation flow.
1. Cut the first release using the new flow.

## Open Questions

- Should we also publish musl-linked Linux artifacts for simpler distribution?
- Should releases be signed (e.g. minisign/cosign) in addition to checksums?

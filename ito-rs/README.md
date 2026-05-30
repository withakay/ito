# ito-rs

Rust implementation of Ito.

## Development

```bash
cd ito-rs
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
```

## Packaging + Transition Plan

Ito is distributed as a Rust binary.

Distribution approach:

- Publish platform-specific cargo-dist release archives via GitHub Releases (with SHA-256 checksums)
- Support local developer installs via `make rust-install`

Initial release target matrix:

- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Artifact naming:

- `ito-cli-<target>.tar.xz` (Windows: `.zip`)
- `ito-cli-<target>.tar.xz.sha256` (Windows: `.zip.sha256`)

Build commands (release automation):

```bash
cargo dist plan
cargo dist build
```

Optional developer install:

```bash
make rust-install
ito --version
```

## Coverage

Targets:

- Long-term: >= 85% workspace line coverage once parity tests are in place.
- Long-term: >= 85% workspace line coverage once core functionality is in place.
- Near-term: >= 80% line coverage for `ito-core` create/status logic.
- Near-term: >= 80% line coverage for `ito-core` ralph runner/state logic.
- Additional: >= 85% line coverage for `ito-core` foundation modules.

Current (from `cargo llvm-cov --workspace`):

- `ito-core/src/create/mod.rs`: 62.33% lines
- `ito-core/src/ralph/prompt.rs`: 61.60% lines
- `ito-core/src/ralph/runner.rs`: 50.85% lines
- `ito-core/src/ralph/state.rs`: 37.18% lines
- `ito-core/src/workflow/mod.rs`: 70.87% lines

```bash
cd ito-rs
cargo install cargo-llvm-cov --locked
rustup component add llvm-tools-preview
cargo llvm-cov --workspace

# Fallback deterministic local coverage without cargo plugins.
./scripts/coverage.sh
```

## Release Verification Checklist

Binary (per platform):

```bash
./ito --version
./ito --help
./ito validate --help
```

Checksum:

```bash
shasum -a 256 -c ito-cli-<target>.tar.xz.sha256
```

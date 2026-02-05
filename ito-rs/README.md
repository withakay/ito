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

- Publish platform-specific Rust release archives via GitHub Releases (with SHA-256 checksums)
- Support local developer installs via `make rust-install`

Initial release target matrix:

- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Artifact naming:

- `ito-vX.Y.Z-<target>.tar.gz` (Windows: `.zip`)
- `ito-vX.Y.Z-<target>.sha256`

Build commands (per platform job):

```bash
cd ito-rs
cargo build -p ito-cli --release

# linux/macos
tar -C target/release -czf "ito-v${VERSION}-${TARGET}.tar.gz" ito
shasum -a 256 "ito-v${VERSION}-${TARGET}.tar.gz" > "ito-v${VERSION}-${TARGET}.sha256"

# windows
powershell -Command "Compress-Archive -Path target/release/ito.exe -DestinationPath ito-v${env:VERSION}-${env:TARGET}.zip"
powershell -Command "Get-FileHash ito-v${env:VERSION}-${env:TARGET}.zip -Algorithm SHA256 | Format-List" > ito-v${env:VERSION}-${env:TARGET}.sha256
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
shasum -a 256 -c ito-vX.Y.Z-<target>.sha256
```

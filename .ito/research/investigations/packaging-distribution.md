# Packaging + Distribution Plan

## Goals

- Replace or wrap npm distribution without breaking users.
- Keep behavior stable across platforms.

Non-goals for this document:

- Implement release automation (this doc is the plan).
- Change CLI behavior (Rust remains parity-driven).

## Recommended Transition Strategy

1. Publish Rust binaries per platform (GitHub Releases) with checksums.
1. Update the npm package (`@withakay/ito`) to become a thin installer/wrapper:
   - On install, download the correct platform binary (or use a locally supplied binary)
   - Expose a `ito` shim that executes the downloaded binary
   - Pass through argv verbatim; `--help`/`--version` output comes from the Rust binary
   - If a platform is unsupported, fail with a clear error and optional TS fallback (during transition)

This mirrors common patterns used by tools like ripgrep and biome.

## Avoiding Behavior Drift

- The npm wrapper must not implement business logic.
- All output is produced by the Rust binary.
- The wrapper only handles:
  - locating/downloading the binary
  - executing it with the same argv

During transition, the wrapper MAY implement a narrow fallback:

- If a platform binary is not available, the wrapper MAY delegate to the TS CLI (if present) while preserving output shape.
- The fallback is opt-in or clearly messaged, and removed once the Rust platform matrix is complete.

## Platform Matrix

Initial supported targets (release artifacts):

- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

Note: additional targets (musl, armv7) can be added later; the npm wrapper should surface an actionable error for unsupported targets.

## Artifact Names + Contents

Release artifacts are published under GitHub Releases for the corresponding tag `vX.Y.Z`.

Per target:

- Archive name: `ito-vX.Y.Z-<target>.tar.gz` (Windows: `.zip`)
- Archive contents:
  - `ito` (or `ito.exe` on Windows)
  - `LICENSE` (optional)

Checksums:

- `ito-vX.Y.Z-<target>.sha256` (text file containing `sha256  <archive>`)
- Optional aggregated `sha256sums.txt`

## Versioning + Pinning

- Git tag `vX.Y.Z` is the source of truth.
- Rust binary version (via `--version`) matches `X.Y.Z`.
- npm wrapper version matches `X.Y.Z` and pins the exact GitHub Release artifact.
- The npm wrapper verifies:
  - archive checksum (SHA-256)
  - extracted binary name and executability

## npm Wrapper Mechanics

Shape:

- Keep publishing to `@withakay/ito`.
- `package.json` exposes `bin: { "ito": "./bin/ito.js" }`.

Install-time behavior:

- `postinstall` determines platform target mapping:
  - Node: `process.platform` + `process.arch`
  - Map to Rust target triple (see Platform Matrix)
- Compute download URLs:
  - `https://github.com/withakay/ito/releases/download/vX.Y.Z/ito-vX.Y.Z-<target>.tar.gz`
  - checksum: same prefix with `.sha256`
- Download to a cache directory inside the package, e.g. `node_modules/@withakay/ito/dist/`.
- Verify checksum, extract, mark executable, and write a small marker file with installed version/target.

Runtime shim behavior (`bin/ito.js`):

- Resolve the installed binary path and `spawn` it.
- Pass through argv verbatim; exit code is the binary exit code.
- All stdout/stderr are streamed from the binary.

Opt-in local override for development:

- `ITO_RS_BIN=/abs/path/to/ito` causes the shim to run that binary instead of the downloaded one.

## CI Build + Release Plan (Documented)

Build job matrix (GitHub Actions):

- `runs-on: macos-14` for `aarch64-apple-darwin`
- `runs-on: macos-13` for `x86_64-apple-darwin`
- `runs-on: ubuntu-latest` for `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` (native or cross)
- `runs-on: windows-latest` for `x86_64-pc-windows-msvc`

Commands (per job):

```bash
cd ito-rs
cargo build -p ito-cli --release

# Package
# linux/macos
tar -C target/release -czf "ito-v${VERSION}-${TARGET}.tar.gz" ito
shasum -a 256 "ito-v${VERSION}-${TARGET}.tar.gz" > "ito-v${VERSION}-${TARGET}.sha256"

# windows
powershell -Command "Compress-Archive -Path target/release/ito.exe -DestinationPath ito-v${env:VERSION}-${env:TARGET}.zip"
powershell -Command "Get-FileHash ito-v${env:VERSION}-${env:TARGET}.zip -Algorithm SHA256 | Format-List" > ito-v${env:VERSION}-${env:TARGET}.sha256
```

Release job:

- Collect artifacts from matrix jobs
- Attach artifacts + checksums to GitHub Release `vX.Y.Z`

npm publish job:

- Build/publish the wrapper package at version `X.Y.Z`
- Wrapper downloads artifacts from the GitHub Release for `vX.Y.Z`

## Verification Checklist

Binary verification (per platform):

```bash
./ito --version
./ito --help
./ito validate --help
```

Checksum verification:

```bash
shasum -a 256 -c ito-vX.Y.Z-<target>.sha256
```

npm wrapper verification:

```bash
npm i -g @withakay/ito@X.Y.Z
ito --version
ito --help
```

Completion verification:

- Run the install command for completions (once the Rust port includes it) and verify a shell session can tab-complete `ito`.

## Local Development

- Allow opting into local Rust builds via env var (e.g. `ITO_RS_BIN=/path/to/ito`).
- Keep parity tests able to run against both local builds and published artifacts.

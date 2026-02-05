## 1. Proposal Validation

- \[x\] 1.1 Add spec deltas for removing Bun/Node/TypeScript requirements
- \[x\] 1.2 Add spec deltas for Rust-only packaging/testing expectations
- \[x\] 1.3 Run `ito validate 006-17_remove-js-ts --strict`

## 2. Implementation

- \[x\] 2.1 Remove legacy TypeScript implementation directory `ito-bun/`
- \[x\] 2.2 Remove Node/Bun toolchain files (package.json/tsconfig/vitest/biome/build scripts)
- \[x\] 2.3 Remove TypeScript tests under `test/` and any TS-only QA tooling
- \[x\] 2.4 Update `Makefile` to be Rust-only (remove bun targets; keep optional watch/coverage via cargo tools)
- \[x\] 2.5 Update CI workflows to run Rust-only checks
- \[x\] 2.6 Update docs (`README.md`, `AGENTS.md`, templates) to remove Node/Bun references

## 3. Verification

- \[x\] 3.1 Run `make build`
- \[x\] 3.2 Run `make test`
- \[x\] 3.3 Run `make lint`

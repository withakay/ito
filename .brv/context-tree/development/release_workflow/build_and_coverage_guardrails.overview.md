## Key points
- The document describes build and verification workflow updates for `make check`, especially coverage execution and dependency validation.
- `Makefile` test coverage was fixed so `LLVM_COV` and `LLVM_PROFDATA` are derived from the active `rustup` toolchain when they are not explicitly set.
- A new baseline file, `ito-rs/tools/max_lines_baseline.txt`, tracks existing oversized Rust files so the max-lines guardrail only fails on regressions or newly introduced violations.
- `cargo-deny` now permits `wit-bindgen@0.51` as a transitive duplicate for `wasip3`, narrowing the exception to a specific version pattern.
- The changes aim to make mixed Homebrew/rustup environments more reliable, especially where cargo/rustc tool discovery previously caused coverage failures.

## Structure / sections summary
- **Header metadata**: Title, summary, tags, related items, keywords, and timestamps.
- **Reason**: Brief statement of the purpose: fixes for `cargo-llvm-cov`, max-lines guardrail, and a `cargo-deny` exception.
- **Raw Concept**:
  - **Task**: Documentation of build and verification workflow updates.
  - **Changes**: Enumerates the three concrete changes made.
  - **Files**: Lists affected files (`Makefile`, `ito-rs/tools/max_lines_baseline.txt`).
  - **Flow**: Shows the sequence from `make check` through coverage, max-lines checking, and dependency denial handling.
  - **Patterns**: Captures the exact regex-like exception for `wit-bindgen@0.51`.
- **Narrative**:
  - **Structure**: Describes the workflow as resilient to mixed installation environments and backed by a baseline-driven line-limit policy.
  - **Dependencies**: Notes reliance on rustup LLVM tools and the baseline file.
  - **Highlights**: Summarizes the practical benefits: fewer coverage failures, actionable line-limit enforcement, and a narrower dependency exception.
  - **Examples**: Illustrates expected behavior in mixed Homebrew/rustup setups.

## Notable entities, patterns, or decisions
- **Entities/files**: `Makefile`, `ito-rs/tools/max_lines_baseline.txt`, `cargo-llvm-cov`, `cargo-deny`, `wit-bindgen@0.51`.
- **Decision**: Prefer rustup-provided LLVM tools when coverage environment variables are unset.
- **Policy pattern**: Use a baseline file to distinguish legacy oversized files from new violations.
- **Exception pattern**: `^wit-bindgen@0.51$` is explicitly allowed as a duplicate version for a transitive `wasip3` dependency.
- **Operational flow**: `make check` → coverage target resolves LLVM toolchain vars → `cargo-llvm-cov` runs → max-lines guardrail checks baseline → `cargo-deny` accepts the allowed duplicate.
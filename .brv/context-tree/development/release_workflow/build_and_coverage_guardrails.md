---
title: Build and Coverage Guardrails
summary: Build workflow fixes mixed Homebrew/rustup coverage execution, tracks oversized Rust files via a baseline, and permits wit-bindgen@0.51 as a wasip3 transitive duplicate.
tags: []
related: []
keywords: []
createdAt: '2026-04-27T08:33:03.473Z'
updatedAt: '2026-04-27T08:33:03.473Z'
---
## Reason
Document fixes for cargo-llvm-cov, max-lines guardrail, and cargo-deny exception

## Raw Concept
**Task:**
Document build and verification workflow updates for make check and coverage runs

**Changes:**
- Fixed Makefile test-coverage target to derive LLVM_COV and LLVM_PROFDATA from the active rustup toolchain when unset
- Introduced ito-rs/tools/max_lines_baseline.txt to track existing oversized Rust files and fail only on regressions or new violations
- Allowed wit-bindgen@0.51 as a wasip3 transitive duplicate in cargo-deny

**Files:**
- Makefile
- ito-rs/tools/max_lines_baseline.txt

**Flow:**
make check -> coverage target resolves LLVM toolchain vars -> cargo-llvm-cov runs -> max-lines guardrail compares against baseline -> cargo-deny accepts wit-bindgen@0.51 duplicate

**Timestamp:** 2026-04-27

**Patterns:**
- `^wit-bindgen@0.51$` - Permitted duplicate version in cargo-deny for wasip3 transitive dependency

## Narrative
### Structure
The release workflow now includes a coverage path that is resilient to mixed Homebrew and rustup installations, plus a max-lines policy that distinguishes pre-existing oversized files from new growth.

### Dependencies
Relies on rustup toolchain LLVM tools when LLVM_COV and LLVM_PROFDATA are not explicitly set, and on the max_lines_baseline.txt file for guardrail enforcement.

### Highlights
Prevents coverage failures caused by Homebrew cargo/rustc discovery issues, keeps the line-limit policy actionable, and narrows the cargo-deny exception to a specific transitive duplicate.

### Examples
When running make check in a mixed environment, the test-coverage target should infer LLVM_COV and LLVM_PROFDATA from rustup instead of failing.

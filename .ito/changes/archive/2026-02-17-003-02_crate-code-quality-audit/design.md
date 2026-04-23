# Design: Crate Code Quality Audit

## Approach

Each crate will be processed through a 3-phase workflow:

### Phase 1: Code Simplification
- Use @code-simplifier agent to review each source file
- Focus on clarity, consistency, maintainability
- Apply rust-style skill guidelines
- Preserve all existing functionality

### Phase 2: Test Coverage Analysis
- Run `cargo llvm-cov` per crate to identify uncovered code
- Prioritize tests for:
  - Public APIs
  - Error handling paths
  - Edge cases in core logic
- Target: 80%+ line coverage per crate

### Phase 3: Test Deduplication
- Identify tests covering the same code paths
- Keep the most comprehensive/clear test
- Remove redundant tests that add no value

## Crate Processing Order

Process crates from leaf dependencies to root:
1. `ito-fs` (standalone, already at 94.6%)
2. `ito-logging` (standalone, already at 80.3%)
3. `ito-test-support` (test utility, already at 90.5%)
4. `ito-schemas` (low deps)
5. `ito-templates` (depends on fs)
6. `ito-harness` (needs tests, 0% coverage)
7. `ito-workflow` (complex, needs coverage boost)
8. `ito-core` (largest, most deps)
9. `ito-cli` (top-level, integration-heavy)

## Verification

After each crate:
```bash
cargo fmt --check -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo test -p <crate>
cargo llvm-cov -p <crate> --summary-only
```

## Out of Scope

- Feature changes
- API modifications
- Dependency updates (unless required for testing)

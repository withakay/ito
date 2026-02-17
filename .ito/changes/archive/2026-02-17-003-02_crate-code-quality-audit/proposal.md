## Why

The Ito Rust codebase has grown to 9 crates with overall test coverage at 37.26%. Several crates have minimal coverage (0-15%) while others have adequate coverage (80%+). Code complexity has accumulated without systematic simplification review. This change audits all crates to simplify code, achieve 80%+ test coverage, and remove duplicate tests.

## What Changes

- **Code Simplification**: Each crate's source files reviewed by @code-simplifier for clarity, consistency, and maintainability
- **Test Coverage**: Add tests to bring each crate to 80%+ line coverage
- **Test Deduplication**: Remove redundant tests that duplicate coverage without adding value
- **Documentation**: Add or improve doc comments for public APIs with low coverage

### Crates in Scope

| Crate | Current Coverage | Target |
|-------|-----------------|--------|
| ito-cli | ~35% | 80%+ |
| ito-core | ~25% | 80%+ |
| ito-fs | 94.6% | Maintain |
| ito-harness | 0% | 80%+ |
| ito-logging | 80.3% | Maintain |
| ito-schemas | ~40% | 80%+ |
| ito-templates | 72.9% | 80%+ |
| ito-test-support | 90.5% | Maintain |
| ito-workflow | ~65% | 80%+ |

## Capabilities

### New Capabilities

None - this is a quality improvement change, not a feature change.

### Modified Capabilities

None - no spec-level behavior changes, only implementation quality improvements.

## Impact

- **Code**: All 9 crates under `ito-rs/crates/`
- **Tests**: New tests added, duplicate tests removed
- **CI**: Test coverage threshold enforcement may be added
- **Risk**: Low - no behavior changes, only quality improvements
- **Dependencies**: None

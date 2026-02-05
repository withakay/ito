# Archive Command Integration Tests

Comprehensive integration tests for the `ito archive` command.

## Overview

This test suite validates the `ito archive` command functionality with 10 different scenarios covering edge cases, error handling, and expected behavior.

## Running the Tests

```bash
cd qa/archive
./test-archive.sh
```

## Test Coverage

The test suite includes:

1. **Complete Change Archive** - Archives a change with all tasks complete and specs
2. **Incomplete Tasks** - Verifies warning and --yes flag behavior with incomplete tasks
3. **No Specs** - Archives a change without any specs
4. **Skip Specs Flag** - Tests --skip-specs flag to bypass spec updates
5. **Non-existent Change** - Verifies proper error handling for missing changes
6. **Duplicate Archive** - Prevents overwriting existing archives
7. **No Tasks File** - Archives changes without tasks.md
8. **Directory Structure** - Ensures all files and subdirectories are preserved
9. **Help Command** - Validates help output completeness
10. **Directory Creation** - Verifies automatic archive directory creation

## Test Features

- ✅ Isolated test workspace (no impact on main project)
- ✅ Automatic cleanup after tests complete
- ✅ Colored output for easy reading
- ✅ Detailed assertion messages
- ✅ Test summary with pass/fail counts
- ✅ Exit code 0 on success, 1 on failure

## Requirements

- Ito binary built: `ito-rs/target/release/ito`
  - Run `cd ito-rs && cargo build --release` if needed
- Bash shell
- Standard Unix utilities (find, grep, mkdir, etc.)

## Test Workspace

Tests run in a temporary workspace at `qa/archive/.test-workspace/` which is automatically:
- Created before tests run
- Populated with test changes
- Cleaned up after completion

## Assertions

Each test uses helper assertions:
- `assert_file_exists` - Verify file exists
- `assert_dir_exists` - Verify directory exists
- `assert_file_not_exists` - Verify file does not exist
- `assert_dir_not_exists` - Verify directory does not exist
- `assert_contains` - Verify file contains text

## Example Output

```
════════════════════════════════════════
  Ito Archive Integration Tests
════════════════════════════════════════

✔ Using ito binary: /path/to/ito
ℹ Setting up test workspace

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ℹ Test 1: Archive a complete change with all tasks done and specs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✔ PASS: Directory does not exist (as expected)
✔ PASS: Archive directory created
✔ PASS: File exists: .ito/specs/test-spec/spec.md

...

════════════════════════════════════════
  Test Summary
════════════════════════════════════════

Total tests run: 10
Passed: 25
Failed: 0

✔ All tests passed!
```

## Adding New Tests

To add a new test:

1. Create a function named `test_archive_*`
2. Call `test_start "Test description"`
3. Use assertion helpers for validation
4. Add function call to `main()` before cleanup

Example:

```bash
test_archive_my_scenario() {
    test_start "My new test scenario"

    create_test_change "009-my-test" true true true

    "$ITO_BIN" archive 009-my-test --yes

    assert_dir_not_exists "$TEST_ITO/changes/009-my-test"
    # Add more assertions...
}
```

Then add to main():
```bash
main() {
    # ... existing tests ...
    test_archive_my_scenario
    # ... cleanup ...
}
```

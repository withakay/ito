#!/usr/bin/env bash
# Integration test for ito archive command
# Tests various archive scenarios to ensure the implementation works correctly

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Get the project root (two levels up from qa/archive)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
ITO_BIN="$PROJECT_ROOT/ito-rs/target/release/ito"

# Test workspace (temporary)
TEST_WORKSPACE="$SCRIPT_DIR/.test-workspace"
TEST_ITO="$TEST_WORKSPACE/.ito"

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

log_success() {
    echo -e "${GREEN}✔${NC} $1"
}

log_error() {
    echo -e "${RED}✖${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

test_start() {
    TESTS_RUN=$((TESTS_RUN + 1))
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    log_info "Test $TESTS_RUN: $1"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

test_pass() {
    TESTS_PASSED=$((TESTS_PASSED + 1))
    log_success "PASS: $1"
}

test_fail() {
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "FAIL: $1"
}

assert_file_exists() {
    if [ -f "$1" ]; then
        test_pass "File exists: $1"
    else
        test_fail "File does not exist: $1"
        return 1
    fi
}

assert_dir_exists() {
    if [ -d "$1" ]; then
        test_pass "Directory exists: $1"
    else
        test_fail "Directory does not exist: $1"
        return 1
    fi
}

assert_file_not_exists() {
    if [ ! -f "$1" ]; then
        test_pass "File does not exist (as expected): $1"
    else
        test_fail "File exists (should not): $1"
        return 1
    fi
}

assert_dir_not_exists() {
    if [ ! -d "$1" ]; then
        test_pass "Directory does not exist (as expected): $1"
    else
        test_fail "Directory exists (should not): $1"
        return 1
    fi
}

assert_contains() {
    if grep -q "$2" "$1"; then
        test_pass "File $1 contains '$2'"
    else
        test_fail "File $1 does not contain '$2'"
        return 1
    fi
}

# Setup test workspace
setup_workspace() {
    log_info "Setting up test workspace at $TEST_WORKSPACE"
    rm -rf "$TEST_WORKSPACE"
    mkdir -p "$TEST_WORKSPACE"
    cd "$TEST_WORKSPACE"

    # Initialize ito
    mkdir -p "$TEST_ITO/changes"
    mkdir -p "$TEST_ITO/specs"
}

# Cleanup test workspace
cleanup_workspace() {
    log_info "Cleaning up test workspace"
    cd "$PROJECT_ROOT"
    rm -rf "$TEST_WORKSPACE"
}

# Build ito binary if needed
ensure_binary() {
    if [ ! -f "$ITO_BIN" ]; then
        log_warning "Ito binary not found. Building..."
        cd "$PROJECT_ROOT/ito-rs"
        cargo build --release
        cd "$TEST_WORKSPACE"
    fi
    log_success "Using ito binary: $ITO_BIN"
}

# Create a test change with given parameters
create_test_change() {
    local change_name="$1"
    local has_tasks="${2:-true}"
    local tasks_complete="${3:-true}"
    local has_specs="${4:-true}"

    local change_dir="$TEST_ITO/changes/$change_name"
    mkdir -p "$change_dir"

    # Create .ito.yaml
    cat > "$change_dir/.ito.yaml" << EOF
schema: spec-driven
created: 2026-01-31
EOF

    # Create proposal.md
    cat > "$change_dir/proposal.md" << EOF
# Test Change: $change_name

This is a test change for QA testing.
EOF

    # Create tasks.md if requested
    if [ "$has_tasks" = true ]; then
        if [ "$tasks_complete" = true ]; then
            cat > "$change_dir/tasks.md" << EOF
# Tasks

- [x] Task 1
- [x] Task 2
- [x] Task 3
EOF
        else
            cat > "$change_dir/tasks.md" << EOF
# Tasks

- [x] Task 1
- [ ] Task 2
- [ ] Task 3
EOF
        fi
    fi

    # Create specs if requested
    if [ "$has_specs" = true ]; then
        mkdir -p "$change_dir/specs/test-spec-$change_name"
        cat > "$change_dir/specs/test-spec-$change_name/spec.md" << EOF
# Test Specification for $change_name

## Requirements

### Requirement: Test requirement
This is a test requirement.

#### Scenario: Test scenario
- **WHEN** testing archive
- **THEN** it should work
EOF
    fi
}

# Test 1: Archive a complete change with specs
test_archive_complete_change() {
    test_start "Archive a complete change with all tasks done and specs"

    create_test_change "001-complete-change" true true true

    # Run archive command
    "$ITO_BIN" archive 001-complete-change --yes

    # Verify change was moved to archive
    assert_dir_not_exists "$TEST_ITO/changes/001-complete-change"

    # Verify archive directory exists with date prefix
    local archive_dir=$(find "$TEST_ITO/changes/archive" -maxdepth 1 -name "*001-complete-change" -type d | head -1)
    if [ -n "$archive_dir" ]; then
        test_pass "Archive directory created: $archive_dir"
    else
        test_fail "Archive directory not found"
        return 1
    fi

    # Verify spec was copied to main specs
    assert_file_exists "$TEST_ITO/specs/test-spec-001-complete-change/spec.md"

    # Verify archived change contains all original files
    if [ -f "$archive_dir/proposal.md" ]; then
        test_pass "Archived change contains proposal.md"
    else
        test_fail "Archived change missing proposal.md"
    fi

    if [ -f "$archive_dir/tasks.md" ]; then
        test_pass "Archived change contains tasks.md"
    else
        test_fail "Archived change missing tasks.md"
    fi

    if [ -f "$archive_dir/.ito.yaml" ]; then
        test_pass "Archived change contains .ito.yaml"
    else
        test_fail "Archived change missing .ito.yaml"
    fi
}

# Test 2: Archive with incomplete tasks (with --yes to skip prompt)
test_archive_incomplete_tasks() {
    test_start "Archive a change with incomplete tasks using --yes flag"

    create_test_change "002-incomplete-tasks" true false true

    # Run archive command with --yes to skip confirmation
    "$ITO_BIN" archive 002-incomplete-tasks --yes

    # Should still archive even with incomplete tasks when --yes is used
    assert_dir_not_exists "$TEST_ITO/changes/002-incomplete-tasks"

    local archive_dir=$(find "$TEST_ITO/changes/archive" -maxdepth 1 -name "*002-incomplete-tasks" -type d | head -1)
    if [ -n "$archive_dir" ]; then
        test_pass "Archive succeeded despite incomplete tasks with --yes flag"
    else
        test_fail "Archive directory not found"
        return 1
    fi
}

# Test 3: Archive without specs
test_archive_no_specs() {
    test_start "Archive a change without specs"

    create_test_change "003-no-specs" true true false

    "$ITO_BIN" archive 003-no-specs --yes

    # Verify change was archived
    assert_dir_not_exists "$TEST_ITO/changes/003-no-specs"

    local archive_dir=$(find "$TEST_ITO/changes/archive" -maxdepth 1 -name "*003-no-specs" -type d | head -1)
    if [ -n "$archive_dir" ]; then
        test_pass "Archive succeeded without specs"
    else
        test_fail "Archive directory not found"
        return 1
    fi
}

# Test 4: Archive with --skip-specs flag
test_archive_skip_specs() {
    test_start "Archive a change with --skip-specs flag"

    create_test_change "004-skip-specs" true true true

    "$ITO_BIN" archive 004-skip-specs --yes --skip-specs

    # Verify change was archived
    assert_dir_not_exists "$TEST_ITO/changes/004-skip-specs"

    # Verify spec was NOT copied to main specs
    assert_file_not_exists "$TEST_ITO/specs/test-spec-004-skip-specs/spec.md"
}

# Test 5: Archive non-existent change
test_archive_nonexistent() {
    test_start "Attempt to archive non-existent change"

    # This should fail
    if "$ITO_BIN" archive 999-nonexistent --yes 2>/dev/null; then
        test_fail "Should have failed when archiving non-existent change"
    else
        test_pass "Correctly failed when archiving non-existent change"
    fi
}

# Test 6: Archive with existing archive (should fail)
test_archive_duplicate() {
    test_start "Attempt to archive when archive already exists"

    create_test_change "005-duplicate" true true true

    # Archive once
    "$ITO_BIN" archive 005-duplicate --yes

    # Recreate the change
    create_test_change "005-duplicate" true true true

    # Try to archive again (should fail because archive exists)
    if "$ITO_BIN" archive 005-duplicate --yes 2>/dev/null; then
        test_fail "Should have failed when archive already exists"
    else
        test_pass "Correctly failed when archive already exists"
    fi
}

# Test 7: Archive without tasks.md
test_archive_no_tasks() {
    test_start "Archive a change without tasks.md file"

    create_test_change "006-no-tasks" false false true

    "$ITO_BIN" archive 006-no-tasks --yes

    # Should succeed without tasks
    assert_dir_not_exists "$TEST_ITO/changes/006-no-tasks"

    local archive_dir=$(find "$TEST_ITO/changes/archive" -maxdepth 1 -name "*006-no-tasks" -type d | head -1)
    if [ -n "$archive_dir" ]; then
        test_pass "Archive succeeded without tasks.md"
    else
        test_fail "Archive directory not found"
        return 1
    fi
}

# Test 8: Verify archive directory structure
test_archive_directory_structure() {
    test_start "Verify archived change maintains directory structure"

    create_test_change "007-structure" true true true

    # Add some additional files
    local change_dir="$TEST_ITO/changes/007-structure"
    mkdir -p "$change_dir/docs"
    echo "Documentation" > "$change_dir/docs/README.md"
    echo "Notes" > "$change_dir/NOTES.md"

    "$ITO_BIN" archive 007-structure --yes

    local archive_dir=$(find "$TEST_ITO/changes/archive" -maxdepth 1 -name "*007-structure" -type d | head -1)

    # Verify all files were preserved
    if [ -f "$archive_dir/proposal.md" ]; then
        test_pass "Archived change contains proposal.md"
    else
        test_fail "Archived change missing proposal.md"
    fi

    if [ -f "$archive_dir/tasks.md" ]; then
        test_pass "Archived change contains tasks.md"
    else
        test_fail "Archived change missing tasks.md"
    fi

    if [ -f "$archive_dir/docs/README.md" ]; then
        test_pass "Archived change contains docs/README.md"
    else
        test_fail "Archived change missing docs/README.md"
    fi

    if [ -f "$archive_dir/NOTES.md" ]; then
        test_pass "Archived change contains NOTES.md"
    else
        test_fail "Archived change missing NOTES.md"
    fi

    if [ -f "$archive_dir/specs/test-spec-007-structure/spec.md" ]; then
        test_pass "Archived change contains specs/test-spec-007-structure/spec.md"
    else
        test_fail "Archived change missing specs/test-spec-007-structure/spec.md"
    fi
}

# Test 9: Help command
test_archive_help() {
    test_start "Verify archive --help displays usage information"

    local help_output=$("$ITO_BIN" archive --help)

    if echo "$help_output" | grep -q "Archive a completed change"; then
        test_pass "Help output contains description"
    else
        test_fail "Help output missing description"
    fi

    if echo "$help_output" | grep -q "\-\-yes"; then
        test_pass "Help output contains --yes flag"
    else
        test_fail "Help output missing --yes flag"
    fi

    if echo "$help_output" | grep -q "\-\-skip-specs"; then
        test_pass "Help output contains --skip-specs flag"
    else
        test_fail "Help output missing --skip-specs flag"
    fi
}

# Test 10: Archive creates archive directory if it doesn't exist
test_archive_creates_directory() {
    test_start "Verify archive creates archive/ directory if missing"

    # Remove archive directory if it exists
    rm -rf "$TEST_ITO/changes/archive"

    create_test_change "008-create-dir" true true false

    "$ITO_BIN" archive 008-create-dir --yes

    # Verify archive directory was created
    assert_dir_exists "$TEST_ITO/changes/archive"
}

# Main test runner
main() {
    echo ""
    echo -e "${BLUE}════════════════════════════════════════${NC}"
    echo -e "${BLUE}  Ito Archive Integration Tests${NC}"
    echo -e "${BLUE}════════════════════════════════════════${NC}"
    echo ""

    # Ensure binary exists
    ensure_binary

    # Setup workspace
    setup_workspace

    # Run tests
    test_archive_complete_change
    test_archive_incomplete_tasks
    test_archive_no_specs
    test_archive_skip_specs
    test_archive_nonexistent
    test_archive_duplicate
    test_archive_no_tasks
    test_archive_directory_structure
    test_archive_help
    test_archive_creates_directory

    # Cleanup
    cleanup_workspace

    # Summary
    echo ""
    echo -e "${BLUE}════════════════════════════════════════${NC}"
    echo -e "${BLUE}  Test Summary${NC}"
    echo -e "${BLUE}════════════════════════════════════════${NC}"
    echo ""
    echo -e "Total tests run: ${BLUE}$TESTS_RUN${NC}"
    echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
    echo ""

    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "All tests passed!"
        exit 0
    else
        log_error "Some tests failed"
        exit 1
    fi
}

# Run main
main "$@"

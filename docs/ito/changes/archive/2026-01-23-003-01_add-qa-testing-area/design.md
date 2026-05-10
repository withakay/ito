# Design: Add QA Testing Area

## Technical Approach

### Directory Structure

```
qa/
├── README.md              # QA testing area overview
├── ralph/
│   └── test-ralph-loop.sh # Integration test for Ito Ralph
└── demo/                 # Temporary demo directories (created/removed at runtime)
    └── ralph-<random>/
```

### Test Script Design (`qa/ralph/test-ralph-loop.sh`)

**Purpose**: Full integration test simulating real Ito Ralph workflow

**Key Steps**:

1. **Pre-flight Check**: Verify ito version is current
1. **Demo Environment Setup**:
   - Generate short random name (8 chars)
   - Create `qa/demo/ralph-<random>/`
   - Initialize ito project with `ito init`
1. **Create Simple Change**:
   - Create a new ungrouped change via `ito new change`
   - Write a simple `proposal.md` requesting a `hello-world.sh` script
   - Write minimal `tasks.md`
1. **Run Ralph Loop**:
   - Execute `ito ralph "<prompt>" --change <id> --allow-all --max-iterations 1`
   - Capture exit code and output
1. **Verification**:
   - Check that `hello-world.sh` was created
   - Verify script contains "hello world"
   - Validate file is executable or can be made executable
1. **Cleanup**:
   - Remove temporary demo directory
   - Report success/failure

**Exit Codes**:

- `0`: Test passed
- `1`: Test failed
- `2`: Pre-flight check failed (wrong ito version)

### Implementation Details

**Random Name Generation**: Use `openssl rand -hex 4` or `/dev/urandom` for cross-platform

**Version Check**: Parse `ito --version` and compare against expected

**Error Handling**: All key steps should have error handling with cleanup on failure

### Future Expansion

Once Ralph test is working, we can add:

- Test scripts for other harnesses (claude-code, codex)
- Workflow testing scripts
- Proposal-driven change lifecycle tests

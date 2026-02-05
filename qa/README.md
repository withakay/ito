# QA Testing Area

This directory contains integration tests that should be run manually or by LLMs, complementing CI/unit tests.

## Purpose

These are extended integration tests that simulate real-world usage of Ito tools. They are not designed to run in automated CI pipelines but rather for:

- Manual QA workflows
- LLM-driven testing before releases
- Regression testing of complex workflows

## Structure

```
qa/
├── README.md                    # This file
├── archive/                     # Archive command tests
│   └── test-archive.sh          # Integration tests for ito archive
└── ralph/                       # Ralph loop tests
    └── test-ralph-loop.sh       # Full integration test for Ito Ralph
```

## Running Tests

Each test script should:

- Be executable (`chmod +x test-*.sh`)
- Document dependencies (ito version, etc.)
- Clean up temporary directories after completion
- Exit with meaningful codes (0=success, non-zero=failure)

Example:

```bash
cd qa/ralph
./test-ralph-loop.sh
```

---
name: test-with-subagent
description: Use when tests need to be run with minimal output noise and delegated execution; routes test runs through the dedicated test-runner subagent and returns curated pass/fail evidence.
---

# Test With Subagent

## Overview

Always run tests through the `test-runner` subagent to keep the main thread clean and high signal.

**Core principle:** Delegate test execution; keep only actionable results.

## Policy

- ALWAYS use this skill for running tests.
- ALWAYS dispatch the `test-runner` subagent before any direct test command.
- Do not bypass this flow unless the calling agent explicitly requires full raw logs for deep harness debugging.

## Required Pattern

1. Dispatch the `test-runner` subagent (never run tests directly first).
2. Give scope (`full suite` or specific target like file/package/crate).
3. Ask for curated output only: command source, command, PASS/FAIL, duration, relevant failures, short actionable excerpt.
4. Use the returned signal to decide next step.

## Prompt Template

```markdown
Run tests using the test-runner workflow.

Scope: <full suite | specific target>
Context: <optional reason, e.g. pre-commit check or regression verification>

Return only:
- Test command source (AGENTS.md or inferred)
- Command executed
- PASS/FAIL
- Duration if available
- If failing: failing tests and a 5-15 line actionable excerpt
```

## Failure Handling

- If failure is clear, fix code/tests and re-run via `test-runner`.
- If failure is ambiguous, request one additional run scoped to the failing target.
- Escalate to direct/manual execution only when curated output is insufficient.

## Red Flags

- Running raw `make test` in the main thread before delegation
- Posting full unfiltered test logs to the calling agent
- Ignoring AGENTS.md command guidance
- Switching away from Makefile-first inference without a clear reason

## Context

`ito ralph` currently runs an iterative harness loop against a Ito change proposal by constructing a prompt from:

- `.ito/changes/<change-id>/proposal.md`
- `.ito/modules/<module-id>/module.md`
- the user-provided CLI prompt

Compared to the standalone `opencode-ralph-wiggum/ralph.ts` loop, Ito's implementation has several gaps:

- the per-iteration prompt lacks a strong preamble and explicit autonomy/instructions
- context added mid-loop is not structured and is only loaded once per run
- completion detection is a fragile exact string match
- non-zero harness exits stop the loop (brittle)
- iteration history lacks the telemetry needed to debug failures and confirm progress

## Goals / Non-Goals

**Goals:**

- Improve iteration prompt structure (preamble + labeled context) to better steer agents
- Reload context each iteration so mid-loop updates take effect without restarting
- Make completion detection whitespace-tolerant
- Continue iterating on harness failures by default, while still allowing fail-fast behavior
- Improve iteration history and status reporting with execution telemetry
- Add a small set of CLI parity flags that improve usability

**Non-Goals:**

- Implement a full multi-phase "planner/reviewer/tester" orchestration system
- Build new harnesses (claude-code/codex/copilot) as part of this change
- Introduce new external dependencies

## Decisions

### Decision: Add a prompt preamble builder

Add a dedicated preamble builder function that takes iteration state (iteration number, min/max, completion promise, current context) and returns a structured prompt preamble.

**Rationale:**

- Keeps prompt assembly logic focused and testable
- Allows consistent formatting across harnesses

### Decision: Reload context at the start of each iteration

Move context loading into the per-iteration path in `runner.ts` (instead of loading once before the loop).

**Rationale:**

- Enables true mid-loop context injection
- Matches the behavior of the standalone loop

### Decision: Make completion detection whitespace-tolerant

Replace the exact string match with a tolerant matcher that can detect `<promise>...` blocks even with whitespace/newlines.

**Rationale:**

- Reduces false negatives when agents format output differently

### Decision: Treat non-zero harness exits as iteration failures, not fatal errors

Adjust harness execution and runner control flow so that non-zero exit codes are captured and recorded in history, and the loop continues unless fail-fast is enabled.

**Rationale:**

- Loops are expected to fail sometimes; resilience improves success rate
- Recording failures improves debuggability

### Decision: Expand iteration history schema

Extend the stored iteration history records to include:

- harness exit code
- completion detection boolean
- duration
- git change summary (at minimum: changed file count; ideally: changed file list and diffstat)
- optional commit hash (when auto-commit is enabled)

**Rationale:**

- Provides enough signal to debug and understand progress without reading full logs

## Risks / Trade-offs

- Increased prompt size -> higher token usage
- Continuing on error could mask problems if the operator expects fail-fast behavior -> mitigated by explicit fail-fast flag and better status reporting

---
name: ito-subagent-driven-development
description: Use when executing implementation plans with independent tasks in the current session using subagents
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Subagent-Driven Development

Execute one plan in the current session by using a fresh implementer subagent per task, followed by spec review and code-quality review before moving on.

## Use When

- Have an implementation plan (ito change with tasks.md)
- Tasks are mostly independent
- Want to stay in this session (vs. parallel session with `ito-apply`)

Compared with `ito-apply`: same session, fresh context per task, mandatory two-stage review, and faster iteration.

## Workflow

1. Setup: load change context, read the full task list, prepare tracking.
2. Per task: start task → dispatch implementer → run spec review → run quality review → complete task.
3. Finish: run one final review, then use `ito-finish`.

## Setup

```bash
# Get change context
ito agent instruction apply --change <change-id>

# Read tasks.md
   ITO_ROOT="$(ito path ito-root)"
   cat "$ITO_ROOT/changes/<change-id>/tasks.md"

# Extract full task text and context upfront
```

## Per Task Workflow

### 1. Mark task started

```bash
ito tasks start <change-id> <task-id>
```

### 2. Dispatch implementer

Provide:
- Full task text (not just reference)
- Context: what came before, what comes after
- Relevant file paths
- Expected outcome

Choose agent tier by task complexity:

- `ito-quick`: small, localized changes
- `ito-general`: most tasks (default)
- `ito-thinking`: complex refactors, tricky bugs, high-risk edits

Require TDD when appropriate: failing test → confirm failure → implement → confirm pass → self-review/report.

### 3. Spec compliance review

Dispatch spec reviewer subagent with:
- The task specification
- Git diff of changes

Reviewer verifies: all requirements met, no unrequested behavior added, correct files touched. If issues exist, send the task back to the implementer and re-review until approved.

### 4. Code quality review

Dispatch code quality reviewer subagent with:
- Git SHAs for review
- Code review template

Reviewer checks quality, tests, and conventions. If issues exist, send the task back to the implementer and re-review until approved.

### 5. Mark task complete

```bash
ito tasks complete <change-id> <task-id>
```

### 6. Next task or finish

If more tasks remain, repeat. When all tasks are done, run a final review and then use `ito-finish`.

## Prompt Templates

- `./implementer-prompt.md` - Dispatch implementer subagent
- `./spec-reviewer-prompt.md` - Dispatch spec compliance reviewer subagent
- `./code-quality-reviewer-prompt.md` - Dispatch code quality reviewer subagent

## Guardrails

Never:
- Start implementation on main/master without explicit user consent
- Skip reviews (spec compliance OR code quality)
- Proceed with unfixed issues
- Dispatch multiple implementation subagents in parallel (conflicts)
- Make subagent read plan file (provide full text instead)
- Skip scene-setting context
- Accept "close enough" on spec compliance
- Start code quality review before spec compliance is ✅
- Move to next task while either review has open issues

If subagents ask questions, answer clearly and fully. If reviewers find issues, send the task back for fixes and rerun the same review.

## Integration

Required workflow skills:
- `ito-using-git-worktrees` - Set up isolated workspace before starting
- `ito-proposal` - Creates the plan this skill executes
- `ito-requesting-code-review` - Code review template for reviewer subagents
- `ito-finish` - Complete development after all tasks

Subagents should use:
- `ito-test-driven-development` - Subagents follow TDD for each task

Alternative workflow:
- `ito-apply` - Use for human-in-loop execution with batch checkpoints

<!-- ITO:END -->

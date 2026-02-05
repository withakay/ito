# Ito Project Planning & Research Extension

> Tool-agnostic workflow for OpenCode, Codex CLI, Claude Code, and other AI coding assistants

## Executive Summary

This proposal extends Ito with structured project planning, research capabilities, and execution patterns that work across AI coding tools. The design prioritizes **file-based workflows** and **markdown custom commands** that any AI assistant can follow.

**Primary Target**: [OpenCode](https://opencode.ai/) - open source AI coding agent
**Also Supports**: [Codex CLI](https://github.com/openai/codex), [Claude Code](https://docs.anthropic.com/claude-code), and other terminal-based AI assistants

## Design Principles

### Tool-Agnostic Architecture

1. **File-Based State**: All context lives in markdown files, not tool-specific memory
1. **Custom Commands**: Workflows defined as markdown files any tool can load
1. **No Vendor Lock-in**: Works with any AI model (Claude, GPT, Gemini, etc.)
1. **Progressive Enhancement**: Tools with advanced features (subagents) can use them; others work sequentially

### Why File-Based?

| Approach | Pros | Cons |
|----------|------|------|
| Tool-specific APIs | Maximum capability | Vendor lock-in, fragile |
| **File-based (chosen)** | Universal, persistent, debuggable | Slightly more verbose |
| Memory/context only | Simple | Lost between sessions |

______________________________________________________________________

## Problem Statement

Current Ito workflow gaps:

1. **No Research Phase**: Proposals jump directly to specs without domain investigation
1. **No Project-Level Roadmapping**: Changes are isolated; no multi-phase planning
1. **No State Persistence**: Session context lost between restarts
1. **Limited Task Structure**: Checklists lack verification criteria
1. **No Systematic Review**: Plans aren't stress-tested

______________________________________________________________________

## Proposed Extensions

### 1. Research Phase (Pre-Proposal)

Add a research stage that runs **before** proposal creation for complex changes.

#### Directory Structure

```
ito/
├── research/                    # Domain research artifacts
│   ├── SUMMARY.md              # Synthesized findings
│   └── investigations/
│       ├── stack-analysis.md   # Technology choices
│       ├── feature-landscape.md # Table stakes vs differentiators
│       ├── architecture.md     # System design considerations
│       └── pitfalls.md         # Common mistakes and mitigations
```

#### Workflow

```
User describes goal
        │
        ▼
┌─────────────────────────────────────────────────┐
│ Research Phase                                  │
│ (sequential or parallel depending on tool)      │
│                                                 │
│ 1. Stack analysis      → stack-analysis.md     │
│ 2. Feature landscape   → feature-landscape.md  │
│ 3. Architecture review → architecture.md       │
│ 4. Pitfall research    → pitfalls.md           │
│ 5. Synthesize          → SUMMARY.md            │
└─────────────────────────────────────────────────┘
        │
        ▼
Standard Ito proposal workflow
```

#### Research Prompt Templates

Research prompts are stored as markdown files that any AI tool can load:

**`ito/commands/research-stack.md`**

```markdown
# Research: Stack Analysis

## Objective
Evaluate technology choices for the proposed feature/project.

## Process
1. Identify the domain and key technical requirements
2. Research current best practices (use web search)
3. Evaluate library ecosystem and maturity
4. Document trade-offs between options

## Output Format
Write findings to `ito/research/investigations/stack-analysis.md`:

## Stack Analysis: [Topic]

### Requirements
- [Key technical requirements]

### Options Evaluated
| Option | Pros | Cons | Maturity |
|--------|------|------|----------|
| ... | ... | ... | ... |

### Recommendation
[Choice] because [rationale]

### Alternatives
- [Option]: Use if [condition]
```

#### Research Output Template

**`ito/research/SUMMARY.md`**

```markdown
# Research Summary: [Topic]

## Key Findings
- [Critical discoveries affecting the approach]

## Stack Recommendations
- **Recommended**: [Choice] - [Rationale]
- **Alternatives**: [Options with trade-offs]

## Feature Prioritization
### Table Stakes (Must Have)
- [Expected baseline features]

### Differentiators (Competitive Advantage)
- [Features that set the project apart]

## Architecture Considerations
- [Key design decisions and implications]

## Pitfalls to Avoid
- [Risk] → [Mitigation strategy]

## Implications for Roadmap
- Phase 1 should focus on: [...]
- Ordering considerations: [...]
- Requires investigation before: [...]
```

______________________________________________________________________

### 2. Roadmap & Milestone Tracking

Add project-level planning above individual changes.

#### Directory Structure

```
ito/
├── planning/                    # Project planning artifacts
│   ├── PROJECT.md              # Vision, constraints, stakeholders
│   ├── ROADMAP.md              # Phased milestone plan
│   ├── STATE.md                # Current decisions, blockers, context
│   └── milestones/
│       ├── v1-core/
│       │   ├── milestone.md    # Milestone definition
│       │   └── phases/
│       │       ├── phase-1/    # Phase details
│       │       └── phase-2/
│       └── v2-advanced/
│           └── milestone.md
```

#### PROJECT.md Template

```markdown
# Project: [Name]

## Vision
[1-2 sentence description of what we're building and why]

## Core Value Proposition
[What makes this valuable to users]

## Constraints
- Technical: [stack, compatibility requirements]
- Resources: [team size, expertise gaps]

## Stakeholders
- [Role]: [Concerns and success criteria]

## Out of Scope
- [Explicitly excluded features/concerns]

## AI Assistant Notes
[Special instructions for AI tools working on this project]
- Preferred patterns: [...]
- Avoid: [...]
- Always check: [...]
```

#### ROADMAP.md Template

```markdown
# Roadmap

## Current Milestone: v1-core
- Status: In Progress
- Phase: 2 of 4

## Milestones

### v1-core (Current)
Target: Production-ready core functionality

| Phase | Name | Status | Changes |
|-------|------|--------|---------|
| 1 | Database Setup | Complete | add-user-schema |
| 2 | API Layer | In Progress | add-auth-api, add-user-api |
| 3 | Frontend | Pending | - |
| 4 | Integration | Pending | - |

### v2-advanced
Target: Advanced features and optimizations

| Phase | Name | Status | Changes |
|-------|------|--------|---------|
| 1 | Analytics | Pending | - |
| 2 | Caching | Pending | - |
```

#### STATE.md Template

This file persists context across sessions—critical for any AI tool:

```markdown
# Project State

Last Updated: [timestamp]

## Current Focus
[What we're working on right now]

## Recent Decisions
- [Date]: [Decision] - [Rationale]

## Open Questions
- [ ] [Question needing resolution]

## Blockers
- [Blocker]: [Impact] - [Owner]

## Session Notes
### [Date] Session
- Completed: [...]
- Next: [...]
- Issues encountered: [...]

---
## For AI Assistants
When resuming work on this project:
1. Read this STATE.md first
2. Check ROADMAP.md for current phase
3. Review any in-progress changes in `ito/changes/`
4. Continue from "Current Focus" above
```

______________________________________________________________________

### 3. Enhanced Task Structure

Replace simple checklists with structured, verifiable tasks.

#### Current Format (tasks.md)

```markdown
## 1. Implementation
- [ ] 1.1 Create database schema
- [ ] 1.2 Implement API endpoint
```

#### Proposed Format (tasks.md)

```markdown
# Tasks for: [change-id]

## Execution Notes
- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)

---

## Wave 1

### Task 1.1: Create User Schema
- **Files**: `src/db/schema/user.ts`, `src/db/migrations/001_users.sql`
- **Dependencies**: None
- **Action**:
  Create TypeScript schema definition and SQL migration for users table.
  Include fields: id (uuid), email (unique), created_at, updated_at.
- **Verify**: `bun run db:migrate && bun run test:schema`
- **Done When**: Migration runs successfully, schema types export correctly
- **Status**: [ ] pending / [ ] in-progress / [x] complete

### Task 1.2: Create Auth Schema
- **Files**: `src/db/schema/auth.ts`, `src/db/migrations/002_auth.sql`
- **Dependencies**: None
- **Action**:
  Create sessions and tokens tables for authentication.
- **Verify**: `bun run db:migrate`
- **Done When**: Tables created with foreign key to users
- **Status**: [ ] pending

---

## Wave 2 (after Wave 1 complete)

### Task 2.1: Implement Auth Service
- **Files**: `src/services/auth.ts`, `src/services/auth.test.ts`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**:
  Create authentication service with login, logout, session management.
- **Verify**: `bun run test src/services/auth.test.ts`
- **Done When**: All tests pass, service exports AuthService class
- **Status**: [ ] pending

---

## Wave 3 (Checkpoint)

### Task 3.1: Review Authentication Flow
- **Type**: checkpoint (requires human approval)
- **Files**: `src/services/auth.ts`, `docs/auth-flow.md`
- **Dependencies**: Task 2.1
- **Action**:
  Generate authentication flow diagram and request human review.
- **Done When**: User confirms flow is correct
- **Status**: [ ] pending
```

#### Task Types

| Type | Behavior |
|------|----------|
| (default) | Execute autonomously, verify automatically |
| `checkpoint` | Pause for human review before proceeding |
| `decision` | Present options, wait for user choice |
| `research` | Investigate and report, don't implement |

______________________________________________________________________

### 4. Execution Model

#### For Tools WITH Subagent Support (Claude Code)

```
Orchestrator (lean context)
├── Spawns: Executor for Task 1.1 (fresh context)
├── Spawns: Executor for Task 1.2 (fresh context)
    │
    ▼ (all complete)
├── Spawns: Executor for Task 2.1 (fresh context)
```

#### For Tools WITHOUT Subagent Support (OpenCode, Codex)

Sequential execution with context preservation via files:

```
1. Load STATE.md and tasks.md
2. Find first pending task
3. Execute task
4. Run verify command
5. Update task status in tasks.md
6. Commit changes
7. Update STATE.md with session notes
8. Repeat from step 2
```

#### Execution Command (Custom Command)

**`ito/commands/execute.md`**

```markdown
# Execute Ito Tasks

## Objective
Execute tasks from a change proposal sequentially, verifying each before proceeding.

## Process
1. Read `ito/planning/STATE.md` for current context
2. Read the specified change's `tasks.md`
3. Find the first task with status `[ ] pending`
4. Execute the task:
   a. Read the files listed
   b. Perform the action described
   c. Run the verify command
   d. If verify passes, mark status as `[x] complete`
   e. If verify fails, stop and report the issue
5. Commit the changes with message: `feat([change-id]): [task name]`
6. Update STATE.md with session notes
7. Proceed to next pending task

## On Completion
- Update ROADMAP.md if all tasks complete
- Report summary of completed work
```

______________________________________________________________________

### 5. Adversarial Review (Red Team)

Add systematic challenge phase to stress-test proposals.

#### Review Prompts

Store as custom commands that any tool can execute:

**`ito/commands/review-security.md`**

```markdown
# Security Review

## Objective
Find security vulnerabilities in the proposed changes.

## Perspective
You are a security researcher. Assume attackers are sophisticated.
Find ways to exploit, bypass, or abuse the proposed system.

## Process
1. Read the proposal and affected specs
2. Identify attack vectors:
   - Authentication/authorization bypasses
   - Injection points (SQL, XSS, command)
   - Data exposure risks
   - CSRF/SSRF vulnerabilities
3. Rate each finding: HIGH / MEDIUM / LOW
4. Suggest mitigations

## Output
Append findings to `ito/changes/[change-id]/REVIEW.md`
```

**`ito/commands/review-scale.md`**

```markdown
# Scale Review

## Objective
Identify performance bottlenecks and scaling issues.

## Perspective
What breaks at 10x, 100x, 1000x scale?

## Process
1. Review database queries for N+1 problems
2. Check for missing indexes
3. Identify memory-intensive operations
4. Look for blocking calls in hot paths
5. Evaluate caching opportunities

## Output
Append findings to `ito/changes/[change-id]/REVIEW.md`
```

#### REVIEW.md Template

```markdown
# Adversarial Review: [change-id]

## Security Review
### Issues Found
- **HIGH**: [Issue description]
  - Attack vector: [How exploited]
  - Mitigation: [Required fix]
  - Status: [ ] Addressed

### Recommendations
- [Proactive improvements]

## Scale Review
### Issues Found
- **MEDIUM**: [Performance concern]
  - Impact: [When problematic]
  - Mitigation: [Optimization]
  - Status: [ ] Addressed

## Edge Case Review
### Issues Found
- **LOW**: [Edge case]
  - Trigger: [How to reproduce]
  - Handling: [Expected behavior]
  - Status: [ ] Addressed

## Summary
- Critical issues: [count]
- Must address before implementation: [list]
- Can defer: [list]
- Approved for implementation: [ ] Yes / [ ] No
```

______________________________________________________________________

### 6. Custom Commands

Ito workflows are implemented as markdown custom commands compatible with all tools.

#### Directory Structure

```
ito/
├── commands/                    # Custom command definitions
│   ├── research-stack.md       # Stack analysis
│   ├── research-features.md    # Feature landscape
│   ├── research-pitfalls.md    # Risk identification
│   ├── research-synthesize.md  # Combine findings
│   ├── plan-init.md            # Initialize planning
│   ├── plan-milestone.md       # Create milestone
│   ├── execute.md              # Execute tasks
│   ├── review-security.md      # Security review
│   ├── review-scale.md         # Scale review
│   ├── review-edge.md          # Edge case review
│   └── state-update.md         # Update STATE.md
```

#### Tool-Specific Loading

| Tool | How to Load Custom Commands |
|------|---------------------------|
| **OpenCode** | Automatic from `.opencode/commands/` or specify path |
| **Codex CLI** | Load via `@file` or custom instructions |
| **Claude Code** | Custom slash commands or CLAUDE.md |

#### Example: OpenCode Integration

```bash
# Copy commands to OpenCode location
cp -r ito/commands .opencode/commands/ito

# Use in OpenCode
/ito/research-stack "authentication system"
/ito/execute add-user-auth
/ito/review-security add-user-auth
```

______________________________________________________________________

## CLI Commands

The `ito` CLI provides commands that work independently of AI tool:

```bash
# Research commands
ito research init                 # Create research/ structure
ito research status               # Show research progress

# Planning commands
ito plan init                     # Initialize planning/ directory
ito plan milestone [name]         # Create new milestone
ito plan phase [milestone] [name] # Add phase to milestone
ito plan status                   # Show roadmap progress

# State commands
ito state                         # Show current STATE.md
ito state decision "[text]"       # Record a decision
ito state blocker "[text]"        # Record a blocker
ito state note "[text]"           # Add session note

# Validation
ito validate [change-id] --strict # Validate including tasks format
```

______________________________________________________________________

## Implementation Phases

### Phase 1: Foundation

- Add `planning/` directory structure to `ito init`
- Create PROJECT.md, STATE.md, ROADMAP.md templates
- Add `ito state` commands
- Document tool-agnostic workflow

### Phase 2: Research

- Create research command templates
- Add `ito/commands/` structure
- Document integration with OpenCode, Codex, Claude Code

### Phase 3: Enhanced Tasks

- Extend tasks.md parser for structured format
- Add wave detection and dependency validation
- Create execution command template

### Phase 4: Adversarial Review

- Create review command templates
- Add REVIEW.md generation
- Document review workflow

______________________________________________________________________

## Tool Compatibility Matrix

| Feature | OpenCode | Codex CLI | Claude Code |
|---------|----------|-----------|-------------|
| Custom commands | ✅ Native | ✅ Via @file | ✅ Slash commands |
| File-based state | ✅ | ✅ | ✅ |
| Web search (research) | ✅ | ✅ | ✅ |
| Parallel execution | ❌ Sequential | ❌ Sequential | ✅ Subagents |
| Checkpoint pauses | ✅ | ✅ | ✅ |

______________________________________________________________________

## Comparison: Before and After

| Capability | Current Ito | With Planning Extension |
|------------|-----------------|------------------------|
| Research | None | Structured investigation |
| Project vision | project.md (basic) | PROJECT.md (structured) |
| Roadmapping | None | Milestones, phases, tracking |
| State persistence | None | STATE.md across sessions |
| Task structure | Checklists | Structured with verify/done |
| Review | Manual | Systematic adversarial |
| Tool support | Any | Any (optimized for OpenCode) |

______________________________________________________________________

## References

- [OpenCode](https://opencode.ai/) - Primary target, open source AI coding agent
- [Codex CLI](https://github.com/openai/codex) - OpenAI's terminal coding agent
- [Claude Code](https://docs.anthropic.com/claude-code) - Anthropic's CLI tool
- [GSD](https://github.com/glittercowboy/get-shit-done) - Context engineering patterns

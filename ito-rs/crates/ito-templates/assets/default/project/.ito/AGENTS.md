# Ito Instructions

Instructions for AI coding assistants using Ito for spec-driven development.

## Managed Files

Some project files are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten (especially with `--force`).

- Default guidance lives in: `.ito/AGENTS.md`
- Project-specific guidance belongs in: `.ito/user-guidance.md`, `AGENTS.md`, and/or `CLAUDE.md`
- Tool prompt wiring lives in: `.opencode/`, `.github/`, `.codex/` (and `.claude/` if present)

## TL;DR Quick Checklist

- Search existing work: `ito list --specs`, `ito list`, `ito list --modules`
- Filter by progress: `ito list --pending`, `ito list --partial`, `ito list --completed`
- Choose a module by semantic fit; create a new module if none fit (avoid dumping unrelated work into an arbitrary existing module)
- Decide scope: new capability vs modify existing capability
- For large features (epics): Create a module to group related changes
- Pick a unique `change-id`: For modular changes use `NNN-CC_name` format (e.g., `001-01_init-repo`)
- Scaffold: `proposal.md`, `tasks.md`, `design.md` (only if needed), and delta specs per affected capability
- Write deltas: use `## ADDED|MODIFIED|REMOVED|RENAMED Requirements`; include at least one `#### Scenario:` per requirement
- Validate: `ito validate [change-id] --strict` and fix issues
- Request approval: Do not start implementation until proposal is approved

## Three-Stage Workflow

### Stage 1: Creating Changes

Create proposal when you need to:

- Add features or functionality
- Make breaking changes (API, schema)
- Change architecture or patterns
- Optimize performance (changes behavior)
- Update security patterns

Triggers (examples):

- "Help me create a change proposal"
- "Help me plan a change"
- "Help me create a proposal"
- "I want to create a spec proposal"
- "I want to create a spec"

Loose matching guidance:

- Contains one of: `proposal`, `change`, `spec`
- With one of: `create`, `plan`, `make`, `start`, `help`

Skip proposal for:

- Bug fixes (restore intended behavior)
- Typos, formatting, comments
- Dependency updates (non-breaking)
- Configuration changes
- Tests for existing behavior

**Workflow**

1. Review `.ito/project.md`, `ito list`, and `ito list --specs` to understand current context.
1. Choose a unique verb-led `change-id` and scaffold `proposal.md`, `tasks.md`, optional `design.md`, and spec deltas under `.ito/changes/<id>/`.
1. Draft spec deltas using `## ADDED|MODIFIED|REMOVED Requirements` with at least one `#### Scenario:` per requirement.
1. Run `ito validate <id> --strict` and resolve any issues before sharing the proposal.

### Stage 2: Implementing Changes

Track these steps as TODOs and complete them one by one.

## Testing Policy

### TDD Workflow
- Default: RED/GREEN/REFACTOR (write a failing test, implement the minimum to pass, then refactor).

### Coverage Targets
- **Target**: 100% (aim high, more coverage == less ick)
- **Minimum**: 80% (hard floor, below this is unacceptable)

### Mocking Policy
- **Avoid mocks.** Mocking should give you the ick.
- Only mock when the alternative is *more* icky (e.g., external APIs, time-dependent behavior, paid services).
- Prefer real implementations, in-memory fakes, or test containers over mocks.
- If you find yourself mocking extensively, reconsider the design—it may indicate tight coupling.

### Integration Tests
- Write integration tests alongside unit tests.
- Integration tests catch issues that unit tests miss (wiring, configuration, real dependencies).
- A good test suite has a healthy mix of both.

### Configuration Overrides
Override per project via cascading config:
- `defaults.testing.tdd.workflow` (e.g., `red-green-refactor`)
- `defaults.testing.coverage.target_percent` (e.g., `100`)
- `defaults.testing.coverage.minimum_percent` (e.g., `80`)

1. **Read proposal.md** - Understand what's being built
1. **Read design.md** (if exists) - Review technical decisions
1. **Read tasks.md** - Get implementation checklist
1. **Implement tasks sequentially** - Complete in order
1. **Confirm completion** - Ensure every item in `tasks.md` is finished before updating statuses
1. **Update statuses** - Prefer `ito tasks start|complete` for enhanced tasks.md; for legacy checkbox lists, set `- [x]` so the list reflects reality
1. **Approval gate** - Do not start implementation until the proposal is reviewed and approved

### Stage 3: Archiving Changes

After deployment, create separate PR to:

- Move `changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
- Update `specs/` if capabilities changed
- Use `ito archive <change-id> --skip-specs --yes` for tooling-only changes (always pass the change ID explicitly)
- Run `ito validate --strict` to confirm the archived change passes checks

## Before Any Task

**Context Checklist:**

- [ ] Read relevant specs in `specs/[capability]/spec.md`
- [ ] Check pending changes in `changes/` for conflicts
  - [ ] Read `.ito/project.md` for conventions
- [ ] Run `ito list` to see active changes
- [ ] Run `ito list --specs` to see existing capabilities

**Before Creating Specs:**

- Always check if capability already exists
- Prefer modifying existing specs over creating duplicates
- Use `ito show [spec]` to review current state
- If request is ambiguous, ask 1–2 clarifying questions before scaffolding

### Search Guidance

- Enumerate specs: `ito list --specs` (or `--json` for scripts)
- Enumerate changes: `ito list` (or `--json` for scripts)
- Filter changes by progress: `ito list --pending`, `ito list --partial`, `ito list --completed`
- Show details:
  - Spec: `ito show <spec-id> --type spec` (use `--json` for filters)
  - Change: `ito show <change-id> --json --deltas-only`
- Full-text search (use ripgrep): `rg -n "Requirement:|Scenario:" .ito/specs`

## Quick Start

### CLI Commands

```bash
# Essential commands
ito list                  # List active changes
ito list --pending         # List changes with 0/N tasks complete
ito list --partial         # List changes with 1..N-1/N tasks complete
ito list --completed       # List completed changes
ito list --specs          # List specifications
ito show [item]           # Display change or spec
ito validate [item]       # Validate changes or specs
ito archive <change-id> [--yes|-y]   # Archive after deployment (add --yes for non-interactive runs)

# Task tracking (enhanced tasks.md)
ito tasks status <change-id>         # Show progress summary
ito tasks next <change-id>           # Show next ready task
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
ito tasks show <change-id>           # Print tasks.md

# Module commands
ito list --modules         # List all modules
ito create module <name>   # Create a new module
ito show module <id>       # Show module details
ito validate module <id>   # Validate a module

# Project management
ito init [path]           # Initialize Ito
ito update [path]         # Update instruction files

# Interactive mode
ito show                  # Prompts for selection
ito validate              # Bulk validation mode

# Debugging
ito show [change] --json --deltas-only
ito validate [change] --strict
ito validate --modules    # Validate all modules
```

### Command Flags

- `--json` - Machine-readable output
- `--pending` - List changes with 0/N tasks complete
- `--partial` - List changes with 1..N-1/N tasks complete
- `--completed` - List changes with N/N tasks complete
- `--type change|spec` - Disambiguate items
- `--strict` - Comprehensive validation
- `--no-interactive` - Disable prompts
- `--skip-specs` - Archive without spec updates
- `--yes`/`-y` - Skip confirmation prompts (non-interactive archive)

## Directory Structure

```
.ito/
├── project.md              # Project conventions
├── specs/                  # Current truth - what IS built
│   └── [capability]/       # Single focused capability
│       ├── spec.md         # Requirements and scenarios
│       └── design.md       # Technical patterns
├── modules/                # Module definitions (epics)
│   └── [NNN_module-name]/  # e.g., 001_project-setup
│       └── module.md       # Purpose, scope, changes list
├── changes/                # Proposals - what SHOULD change
│   ├── [NNN-CC_name]/      # Modular change (e.g., 001-01_init-repo)
│   │   ├── proposal.md     # Why, what, impact
│   │   ├── tasks.md        # Implementation checklist
│   │   ├── design.md       # Technical decisions (optional)
│   │   └── specs/          # Delta changes
│   │       └── [capability]/
│   │           └── spec.md # ADDED/MODIFIED/REMOVED
│   ├── [change-name]/      # Legacy change (no module)
│   │   └── ...
│   └── archive/            # Completed changes
```

### Module Naming Convention

- Module folder: `NNN_module-name` (e.g., `001_project-setup`)
- Modular change: `NNN-CC_change-name` (e.g., `001-01_init-repo`)
- `NNN` = 3-digit module ID
- `CC` = 2-digit change number within module
- Module `000` is reserved for ungrouped/standalone changes

## Creating Change Proposals

### Decision Tree

```
New request?
├─ Bug fix restoring spec behavior? → Fix directly
├─ Typo/format/comment? → Fix directly
├─ New feature/capability? → Create proposal
├─ Breaking change? → Create proposal
├─ Architecture change? → Create proposal
└─ Unclear? → Create proposal (safer)
```

### Proposal Structure

1. **Create directory:** `changes/[change-id]/` (kebab-case, verb-led, unique)

1. **Write proposal.md:**

```markdown
# Change: [Brief description of change]

## Why
[1-2 sentences on problem/opportunity]

## What Changes
- [Bullet list of changes]
- [Mark breaking changes with **BREAKING**]

## Impact
- Affected specs: [list capabilities]
- Affected code: [key files/systems]
```

3. **Create spec deltas:** `specs/[capability]/spec.md`

```markdown
## ADDED Requirements
### Requirement: New Feature
The system SHALL provide...

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result

## MODIFIED Requirements
### Requirement: Existing Feature
[Complete modified requirement]

## REMOVED Requirements
### Requirement: Old Feature
**Reason**: [Why removing]
**Migration**: [How to handle]
```

If multiple capabilities are affected, create multiple delta files under `changes/[change-id]/specs/<capability>/spec.md`—one per capability.

4. **Create tasks.md:**

```markdown
## 1. Implementation
- [ ] 1.1 Create database schema
- [ ] 1.2 Implement API endpoint
- [ ] 1.3 Add frontend component
- [ ] 1.4 Write tests
```

5. **Create design.md when needed:**
   Create `design.md` if any of the following apply; otherwise omit it:

- Cross-cutting change (multiple services/modules) or a new architectural pattern
- New external dependency or significant data model changes
- Security, performance, or migration complexity
- Ambiguity that benefits from technical decisions before coding

Minimal `design.md` skeleton:

```markdown
## Context
[Background, constraints, stakeholders]

## Goals / Non-Goals
- Goals: [...]
- Non-Goals: [...]

## Decisions
- Decision: [What and why]
- Alternatives considered: [Options + rationale]

## Risks / Trade-offs
- [Risk] → Mitigation

## Migration Plan
[Steps, rollback]

## Open Questions
- [...]
```

## Working with Modules

Modules group related changes into epics. Use modules for large features that span multiple changes.

Module selection guidance:

- Prefer the closest semantic fit in an existing module.
- If nothing fits, create a new module for the theme of the work.
- Only use module `000` for truly ungrouped, one-off changes.

### When to Create a Module

- Feature requires 3+ related changes
- Epic-level work spanning multiple capabilities
- Need to track dependencies between changes
- Want to enforce scope boundaries

### Creating a Module

```bash
ito create module project-setup
# Creates: .ito/modules/001_project-setup/module.md
```

### module.md Structure

```markdown
# Project Setup

## Purpose
Set up the initial project structure and tooling.

## Depends On
<!-- Optional: modules that must complete first -->
- 000

## Scope
<!-- Capabilities this module may create or modify -->
- project-config
- dev-environment

## Changes
<!-- Hybrid: existing changes auto-discovered + planned -->
- [ ] 001-01_init-repo
- [ ] 001-02_add-readme (planned)
 - [x] 001-03_setup-linting
```

### Scope Enforcement

- Changes in a module can ONLY modify specs listed in `## Scope`
- Use `*` for unrestricted scope (not recommended)
- Scope violations are validation ERRORs

### Creating Modular Changes

```bash
# Change naming: NNN-CC_name
# NNN = module ID, CC = change number
mkdir -p .ito/changes/001-01_init-repo/{specs/project-config}
```

### Module Validation

```bash
ito validate module 001                 # Validate module + scope
ito validate module 001 --with-changes  # Also validate all changes
ito validate --modules                  # Validate all modules
```

## Spec File Format

### Critical: Scenario Formatting

**CORRECT** (use #### headers):

```markdown
#### Scenario: User login success
- **WHEN** valid credentials provided
- **THEN** return JWT token
```

**WRONG** (don't use bullets or bold):

```markdown
- **Scenario: User login**  ❌
**Scenario**: User login     ❌
### Scenario: User login      ❌
```

Every requirement MUST have at least one scenario.

### Requirement Wording

- Use SHALL/MUST for normative requirements (avoid should/may unless intentionally non-normative)

### Delta Operations

- `## ADDED Requirements` - New capabilities
- `## MODIFIED Requirements` - Changed behavior
- `## REMOVED Requirements` - Deprecated features
- `## RENAMED Requirements` - Name changes

Headers matched with `trim(header)` - whitespace ignored.

#### When to use ADDED vs MODIFIED

- ADDED: Introduces a new capability or sub-capability that can stand alone as a requirement. Prefer ADDED when the change is orthogonal (e.g., adding "Slash Command Configuration") rather than altering the semantics of an existing requirement.
- MODIFIED: Changes the behavior, scope, or acceptance criteria of an existing requirement. Always paste the full, updated requirement content (header + all scenarios). The archiver will replace the entire requirement with what you provide here; partial deltas will drop previous details.
- RENAMED: Use when only the name changes. If you also change behavior, use RENAMED (name) plus MODIFIED (content) referencing the new name.

Common pitfall: Using MODIFIED to add a new concern without including the previous text. This causes loss of detail at archive time. If you aren’t explicitly changing the existing requirement, add a new requirement under ADDED instead.

Authoring a MODIFIED requirement correctly:

1. Locate the existing requirement in `.ito/specs/<capability>/spec.md`.
1. Copy the entire requirement block (from `### Requirement: ...` through its scenarios).
1. Paste it under `## MODIFIED Requirements` and edit to reflect the new behavior.
1. Ensure the header text matches exactly (whitespace-insensitive) and keep at least one `#### Scenario:`.

Example for RENAMED:

```markdown
## RENAMED Requirements
- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## Troubleshooting

### Common Errors

**"Change must have at least one delta"**

- Check `changes/[name]/specs/` exists with .md files
- Verify files have operation prefixes (## ADDED Requirements)

**"Requirement must have at least one scenario"**

- Check scenarios use `#### Scenario:` format (4 hashtags)
- Don't use bullet points or bold for scenario headers

**Silent scenario parsing failures**

- Exact format required: `#### Scenario: Name`
- Debug with: `ito show [change] --json --deltas-only`

### Validation Tips

```bash
# Always use strict mode for comprehensive checks
ito validate [change] --strict

# Debug delta parsing
ito show [change] --json | jq '.deltas'

# Check specific requirement
ito show [spec] --json -r 1
```

## Happy Path Script

```bash
# 1) Explore current state
ito list --specs
ito list
# Optional full-text search:
# rg -n "Requirement:|Scenario:" .ito/specs
# rg -n "^#|Requirement:" .ito/changes

# 2) Choose change id and scaffold
CHANGE=add-two-factor-auth
mkdir -p .ito/changes/$CHANGE/{specs/auth}
printf "## Why\n...\n\n## What Changes\n- ...\n\n## Impact\n- ...\n" > .ito/changes/$CHANGE/proposal.md
printf "## 1. Implementation\n- [ ] 1.1 ...\n" > .ito/changes/$CHANGE/tasks.md

# 3) Add deltas (example)
cat > .ito/changes/$CHANGE/specs/auth/spec.md << 'EOF'
## ADDED Requirements
### Requirement: Two-Factor Authentication
Users MUST provide a second factor during login.

#### Scenario: OTP required
- **WHEN** valid credentials are provided
- **THEN** an OTP challenge is required
EOF

# 4) Validate
ito validate $CHANGE --strict
```

## Multi-Capability Example

```
.ito/changes/add-2fa-notify/
├── proposal.md
├── tasks.md
└── specs/
    ├── auth/
    │   └── spec.md   # ADDED: Two-Factor Authentication
    └── notifications/
        └── spec.md   # ADDED: OTP email notification
```

auth/spec.md

```markdown
## ADDED Requirements
### Requirement: Two-Factor Authentication
...
```

notifications/spec.md

```markdown
## ADDED Requirements
### Requirement: OTP Email Notification
...
```

## Best Practices

### Simplicity First

- Default to \<100 lines of new code
- Single-file implementations until proven insufficient
- Avoid frameworks without clear justification
- Choose boring, proven patterns

### Complexity Triggers

Only add complexity with:

- Performance data showing current solution too slow
- Concrete scale requirements (>1000 users, >100MB data)
- Multiple proven use cases requiring abstraction

### Clear References

- Use `file.ts:42` format for code locations
- Reference specs as `specs/auth/spec.md`
- Link related changes and PRs

### Capability Naming

- Use verb-noun: `user-auth`, `payment-capture`
- Single purpose per capability
- 10-minute understandability rule
- Split if description needs "AND"

### Change ID Naming

- Use kebab-case, short and descriptive: `add-two-factor-auth`
- Prefer verb-led prefixes: `add-`, `update-`, `remove-`, `refactor-`
- Ensure uniqueness; if taken, append `-2`, `-3`, etc.

## Tool Selection Guide

| Task | Tool | Why |
|------|------|-----|
| Find files by pattern | Glob | Fast pattern matching |
| Search code content | Grep | Optimized regex search |
| Read specific files | Read | Direct file access |
| Explore unknown scope | Task | Multi-step investigation |

## Error Recovery

### Change Conflicts

1. Run `ito list` to see active changes
1. Check for overlapping specs
1. Coordinate with change owners
1. Consider combining proposals

### Validation Failures

1. Run with `--strict` flag
1. Check JSON output for details
1. Verify spec file format
1. Ensure scenarios properly formatted

### Missing Context

1. Read project.md first
1. Check related specs
1. Review recent archives
1. Ask for clarification

## Quick Reference

### Stage Indicators

- `changes/` - Proposed, not yet built
- `specs/` - Built and deployed
- `archive/` - Completed changes

### File Purposes

- `proposal.md` - Why and what
- `tasks.md` - Implementation steps
- `design.md` - Technical decisions
- `spec.md` - Requirements and behavior

### CLI Essentials

```bash
ito list              # What's in progress?
ito show [item]       # View details
ito validate --strict # Is it correct?
ito archive <change-id> [--yes|-y]  # Mark complete (add --yes for automation)
```

Remember: Specs are truth. Changes are proposals. Keep them in sync.

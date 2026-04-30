<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Ito Instructions

Instructions for AI coding assistants using Ito for change-driven development.

## Managed Files

|defaults: `.ito/AGENTS.md` |project-specific: `.ito/user-prompts/guidance.md`, `.ito/user-prompts/<artifact>.md`, `AGENTS.md`, `CLAUDE.md`
|tool wiring: `.opencode/`, `.github/`, `.codex/` (`.claude/` if present)

## TL;DR Quick Checklist

|search: `ito list --specs`, `ito list`, `ito list-archive`, `ito list --modules`; filter: `--pending|--partial|--completed`
|choose module by semantic fit; create new if none fit; avoid dumping unrelated work into arbitrary module
|scope: new capability vs modify existing; large features → create module to group changes
|change-id: unique, `NNN-CC_name` format for modular (e.g., `001-01_init-repo`)
|scaffold: `proposal.md`, `tasks.md`, `design.md` (if needed), delta specs per affected capability
|deltas: `## ADDED|MODIFIED|REMOVED|RENAMED Requirements`; ≥1 `#### Scenario:` per requirement
|validate: `ito validate [change-id] --strict` |approval gate: do not start until proposal is approved

## Three-Stage Workflow

### Stage 1: Creating Changes

Create proposal for: new features/functionality, breaking changes (API/schema), architecture/pattern changes, performance optimizations, security pattern updates.

Entrypoints:
- `ito-feature` - new capabilities, enhancements, or broader behavior changes
- `ito-fix` - bounded fixes, regressions, and supporting platform/tooling/infrastructure changes
- `ito-proposal` - neutral fallback
- `ito-brainstorming` - open-ended exploration before proposal scaffolding

Triggers: requests containing `proposal|change|spec` + `create|plan|make|start|help`

Skip proposal for: bug fixes restoring intended behavior | typos/formatting/comments | non-breaking dependency updates | config changes | tests for existing behavior. Fix-shaped but schema unclear → start with `ito-fix`.

**Workflow:**
1. Pick lane: `ito-feature`, `ito-fix`, `ito-proposal`, or `ito-brainstorming`
1. Review `.ito/project.md`, `ito list`, `ito list --specs`
1. Choose schema: `spec-driven` (new/broad/high-risk) | `minimalist` (bounded fixes/small changes) | `tdd` (regression-first) | `event-driven` (event/message-centric)
1. Choose unique verb-led `change-id`; scaffold under `.ito/changes/<id>/`
1. Draft spec deltas: `## ADDED|MODIFIED|REMOVED Requirements` with ≥1 `#### Scenario:` each
1. Run `ito validate <id> --strict`; resolve issues before sharing

### Stage 2: Implementing Changes

Track these steps as TODOs and complete them one by one.

## Testing Policy

|TDD: RED/GREEN/REFACTOR (write failing test → implement minimum → refactor)
|coverage target: 100%; minimum: 80% (hard floor)
|mocking: avoid — "gives you the ick"; only mock external APIs, time-dependent behavior, paid services; prefer real impls, in-memory fakes, test containers; extensive mocking indicates tight coupling → reconsider design
|integration tests alongside unit tests — catch wiring/config/real-dependency issues
|config overrides: `defaults.testing.tdd.workflow`, `defaults.testing.coverage.target_percent`, `defaults.testing.coverage.minimum_percent`

1. **Read proposal.md**
1. **Read design.md** (if exists)
1. **Read tasks.md**
1. **Implement tasks sequentially**
1. **Confirm completion** - every `tasks.md` item finished before updating statuses
1. **Update statuses** - MUST use `ito tasks start|complete|shelve|unshelve|add` for enhanced tasks.md (emits audit events automatically); for legacy checkbox lists set `- [x]`
1. **Reconcile if needed** - direct edit to `tasks.md` unavoidable? Run `ito audit reconcile --fix` immediately after
1. **Approval gate** - do not start until proposal is reviewed and approved

### Stage 3: Archiving Changes

After deployment, create separate PR to:

- Run `ito audit reconcile --change <change-id>` to ensure audit consistency before archiving
- Move `changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
- Update `specs/` if capabilities changed
- Use `ito archive <change-id> --skip-specs --yes` for tooling-only changes (always pass the change ID explicitly)
- Run `ito validate --strict` to confirm the archived change passes checks

## Before Any Task

**Context Checklist:**

- [ ] Read relevant specs in `specs/[capability]/spec.md`
- [ ] Check pending changes in `changes/` for conflicts
- [ ] Read `.ito/project.md` for conventions
- [ ] Run `ito list` to see active changes | `ito list --specs` to see existing capabilities

**Before Creating Specs:**

|check if capability exists; prefer modifying over creating duplicates; `ito show [spec]` to review current state
|ambiguous/unclear schema → `ito-proposal-intake`
|`minimalist`: bounded fixes, small tooling/platform changes |`tdd`: reproduce regression with failing test first |`event-driven`: event/message-centric systems

### Search Guidance

|specs: `ito list --specs` (or `--json`) |changes: `ito list --pending|--partial|--completed`
|spec detail: `ito show <spec-id> --type spec` |change detail: `ito show <change-id> --json --deltas-only`
|full-text: `rg -n "Requirement:|Scenario:" .ito/specs`

## Quick Start

### Backend-Backed Mode

When `backend.enabled=true` or persistence is remote, local active-work markdown may be absent by design. Do not create/edit `.ito/changes/*`, `.ito/specs/*`, or `tasks.md` manually. Use CLI-backed flows: `ito show <item>`, `ito patch ...`, `ito write ...`, `ito tasks ...`, `ito tasks sync pull <change-id>`, `ito archive <change-id>`. Local Git/projected files are read-oriented; mutations via CLI only.

### CLI Commands

```bash
# Essential commands
ito list                  # List active changes
ito list-archive          # List archived changes
ito list --pending         # List changes with 0/N tasks complete
ito list --partial         # List changes with 1..N-1/N tasks complete
ito list --completed       # List completed changes
ito list --specs          # List specifications
ito show [item]           # Display change or spec
ito validate [item]       # Validate changes or specs
ito patch change <id> proposal            # Patch an active change artifact from stdin
ito write change <id> design              # Replace an active change artifact from stdin
ito archive <change-id> [--yes|-y]   # Archive after deployment (add --yes for non-interactive runs)
ito trace <change-id>     # Show requirement traceability coverage (--json for machine-readable)

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

# Audit trail
ito audit log              # View audit event log
ito audit log --change <id>  # Filter by change
ito audit reconcile        # Check for drift between log and filesystem
ito audit reconcile --fix  # Fix drift with compensating events
ito audit validate         # Validate log integrity
ito audit stats            # Show audit statistics
ito audit stream           # Tail recent events

# Project management
ito init [path]           # Initialize Ito
ito init --upgrade [path] # Refresh managed template blocks (marker-scoped, preserves user content)
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

|`--json`: machine-readable |`--pending/--partial/--completed`: filter by task progress
|`--type change|spec`: disambiguate |`--strict`: comprehensive validation
|`--no-interactive`: disable prompts |`--skip-specs`: archive without spec updates |`--yes/-y`: skip confirmation

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
│   └── archive/            # Completed changes
```

### Module Naming Convention

|module folder: `NNN_module-name` (e.g., `001_project-setup`) |modular change: `NNN-CC_change-name` (e.g., `001-01_init-repo`)
|`NNN` = 3-digit module ID | `CC` = 2-digit change number within module | module `000` = ungrouped/standalone

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

- **Requirement ID**: capability:new-feature

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result

## MODIFIED Requirements
### Requirement: Existing Feature
[Complete modified requirement]

- **Requirement ID**: capability:existing-feature

## REMOVED Requirements
### Requirement: Old Feature
**Reason**: [Why removing]
**Migration**: [How to handle]
```

**Requirement ID** is optional metadata for traceability. Format: `<capability>:<requirement-name>`.
When any requirement in a change includes a Requirement ID, **all** requirements in that change must include one.
Omit the field entirely if you do not need traceability for this change.

If multiple capabilities are affected, create multiple delta files under `changes/[change-id]/specs/<capability>/spec.md`—one per capability.

4. **Create tasks.md:**

```markdown
## 1. Implementation
- [ ] 1.1 Create database schema
- [ ] 1.2 Implement API endpoint
- [ ] 1.3 Add frontend component
- [ ] 1.4 Write tests
```

For enhanced task format with traceability, add `- **Requirements**: <id>, <id>` to each task:

```markdown
### Task 1.1: Create database schema

- **Files**: `db/schema.sql`
- **Dependencies**: None
- **Action**: Create the schema
- **Verify**: `cargo test`
- **Done When**: Schema exists
- **Requirements**: capability:new-feature
- **Status**: [ ] pending
```

**Requirements** links a task to one or more Requirement IDs declared in delta specs.
Use `ito trace <change-id>` to check coverage after adding IDs.

5. **Create design.md** if any of these apply (otherwise omit): cross-cutting change or new architectural pattern, new external dependency or significant data model changes, security/performance/migration complexity, ambiguity benefiting from technical decisions before coding.

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

Modules group related changes into epics. Create when: 3+ related changes, epic-level work, dependency tracking, scope enforcement. Select closest semantic fit; create new module if nothing fits; `000` for truly one-off ungrouped changes.

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

- Changes ONLY modify specs listed in `## Scope`; `*` = unrestricted (not recommended); violations = validation ERRORs

```bash
# Change naming: NNN-CC_name (NNN = module ID, CC = change number)
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

Common pitfall: Using MODIFIED to add a new concern without including the previous text → loss of detail at archive time. If not explicitly changing existing requirement text, use ADDED instead.

Authoring a MODIFIED requirement correctly:
1. Locate the existing requirement in `.ito/specs/<capability>/spec.md`.
1. Copy the entire requirement block (from `### Requirement: ...` through its scenarios).
1. Paste it under `## MODIFIED Requirements` and edit to reflect the new behavior.
1. Ensure the header text matches exactly (whitespace-insensitive) and keep at least one `#### Scenario:`.

```markdown
## RENAMED Requirements
- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## Troubleshooting

### Common Errors

**"Change must have at least one delta"**
- Check `changes/[name]/specs/` exists with .md files; verify files have `## ADDED Requirements` prefix

**"Requirement must have at least one scenario"**
- Use `#### Scenario:` format (4 hashtags); no bullet points or bold for scenario headers

**Silent scenario parsing failures**
- Exact format required: `#### Scenario: Name`; debug: `ito show [change] --json --deltas-only`

### Validation Tips

```bash
ito validate [change] --strict
ito show [change] --json | jq '.deltas'
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

|simplicity: <100 lines new code; single-file until proven insufficient; no frameworks without justification; boring proven patterns
|add complexity only with: performance data showing current solution too slow | concrete scale requirements (>1000 users, >100MB data) | multiple proven use cases requiring abstraction
|refs: `file.ts:42` for code; `specs/auth/spec.md` for specs; link related changes and PRs
|capability naming: verb-noun (`user-auth`, `payment-capture`); single purpose; 10-min understandability; split if description needs "AND"
|change-id naming: kebab-case, verb-led (`add-`, `update-`, `remove-`, `refactor-`); unique (append `-2`, `-3` if taken)

## Tool Selection Guide

| Task | Tool |
|------|------|
| Find files by pattern | Glob |
| Search code content | Grep |
| Read specific files | Read |
| Explore unknown scope | Task |

## Error Recovery

|change conflicts: `ito list` → check overlapping specs → coordinate owners → consider combining proposals
|validation failures: `--strict` → check JSON output → verify spec file format → ensure scenarios properly formatted
|missing context: read project.md → check related specs → review recent archives → ask for clarification

## Quick Reference

|`changes/` = proposed, not yet built |`specs/` = built and deployed |`archive/` = completed
|`proposal.md` = why+what |`tasks.md` = implementation steps |`design.md` = technical decisions |`spec.md` = requirements+behavior

```bash
ito list              # What's in progress?
ito show [item]       # View details
ito validate --strict # Is it correct?
ito archive <change-id> [--yes|-y]  # Mark complete (add --yes for automation)
```

Remember: Specs are truth. Changes are proposals. Keep them in sync.

<!-- ITO:END -->

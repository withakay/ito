# Agent Workflow

This document describes the Ito workflow as used by AI coding agents. Ito provides a structured approach to managing changes through a series of actions that guide work from initial proposal to final archival.

## Core Concepts

### Changes

A **change** is the fundamental unit of work in Ito. Each change lives in `.ito/changes/<module-id>-NN_<name>/` and contains:

- **proposal.md** - Why the change is needed and what it accomplishes
- **specs/** - Detailed requirements for each capability
- **design.md** - Technical approach (optional for simple changes)
- **tasks.md** - Actionable work items with checkbox tracking

### Modules

Changes are organized into **modules** for grouping related work. Use `ito list --modules` to see existing modules, or `ito create module "<name>"` to create one. Use module `000` for small, ungrouped tasks.

### Artifacts

Each change follows a schema that defines which **artifacts** are required. The default `spec-driven` schema requires: proposal → specs → tasks (with optional design).

## The Five Actions

Ito uses five core actions that guide a change through its lifecycle:

### 1. Proposal (`/ito-proposal`)

Creates a new change with a structured proposal document.

**When to use:** Starting new work - features, fixes, refactoring, documentation.

**What it does:**

1. Checks for existing similar changes
1. Selects or creates a module
1. Creates the change directory with `ito create change "<name>" --module <id>`
1. Generates proposal.md using `ito agent instruction proposal --change "<id>"`

**Proposal structure:**

- **Why** - What problem does this solve? Who benefits?
- **What Changes** - High-level description of modifications
- **Capabilities** - List of features (each becomes a spec)
- **Impact** - Effects on existing functionality, performance, breaking changes

### 2. Research (`ito x-research`)

Conducts structured investigation before implementation.

**When to use:** Exploring options, evaluating technologies, investigating approaches.

**What it does:**

1. Creates research directory at `.ito/research/`
1. Generates SUMMARY.md with research goals
1. Creates investigation files in `investigations/` subdirectory
1. Documents findings, trade-offs, and recommendations

**Research areas:**

- Stack analysis
- Feature landscape
- Architecture patterns
- Potential pitfalls

### 3. Apply (`/ito-apply`)

Implements the tasks defined in a change.

**When to use:** Ready to write code after proposal/specs are complete.

**What it does:**

1. Verifies all required artifacts are complete
1. Reads context: proposal, specs, design, tasks
1. Works through tasks systematically
1. Marks each task complete (`- [ ]` → `- [x]`) as finished
1. Runs validation after completion

**Implementation flow:**

```
For each task in tasks.md:
  1. Mark task in_progress
  2. Read relevant specs/design
  3. Implement the changes
  4. Verify implementation
  5. Mark task complete
```

### Testing Policy (TDD + coverage)

Ito guidance assumes a disciplined TDD loop:

- RED: write a failing test first
- GREEN: implement the minimum to pass
- REFACTOR: clean up while tests stay green

Default coverage guidance is 80% (not enforced by Ito; treat as a team policy target).

You can override the defaults via the cascading project config:

- Keys: `defaults.testing.tdd.workflow`, `defaults.testing.coverage.target_percent`
- Sources (low -> high): `ito.json`, `.ito.json`, `.ito/config.json`, `$PROJECT_DIR/config.json`

Example `.ito/config.json` override:

```json
{
  "defaults": {
    "testing": {
      "tdd": { "workflow": "red-green-refactor" },
      "coverage": { "target_percent": 90 }
    }
  }
}
```

### 4. Review (`/ito-review`)

Validates changes, specs, or implementations.

**When to use:** Quality checks before merging, validating artifacts.

**What it does:**

1. Runs `ito validate` on the target
1. Categorizes issues: critical, important, minor
1. Provides actionable feedback
1. Documents assessment

**Validation targets:**

- `--changes` - Validate change artifacts
- `--specs` - Validate spec requirements

### 5. Archive (`/ito-archive`)

Completes and archives a finished change.

**When to use:** All tasks complete, implementation validated.

**What it does:**

1. Verifies change is ready (all tasks complete)
1. Confirms with user before proceeding
1. Runs `ito archive <name>`
1. Moves change to `.ito/changes/archive/`
1. Updates main specifications if applicable

## Supporting Actions

### Commit (`/ito-commit`)

Creates git commits aligned to Ito changes.

**Features:**

- Conventional commit format with change ID
- Auto-mode for immediate commits
- One commit per change preferred

## Example Workflow

Here's a complete workflow from start to finish:

```
1. User: "Add user authentication to the API"

2. /ito-proposal
   → Creates 001-03_user-authentication change
   → Generates proposal.md with Why/What/Capabilities/Impact

3. Agent creates specs for each capability:
   → specs/login-endpoint/spec.md
   → specs/token-validation/spec.md
   → specs/logout-endpoint/spec.md

4. Agent creates tasks.md with checkbox items

5. /ito-apply
   → Reads all context files
   → Implements tasks one by one
   → Marks each complete in tasks.md

6. /ito-review
   → Validates implementation
   → Checks for issues

7. /ito-commit
   → Creates conventional commit

8. /ito-archive
   → Moves to archive
   → Updates main specs
```

## Flexible ID Formats

Ito accepts flexible ID formats for both modules and changes. You don't need to remember exact zero-padding.

### Module IDs

| Input | Resolves To |
|-------|-------------|
| `1` | `001` |
| `01` | `001` |
| `001` | `001` |
| `1_foo` | module `001` (with name hint) |
| `42` | `042` |

### Change IDs

| Input | Resolves To |
|-------|-------------|
| `1-2_bar` | `001-02_bar` |
| `001-02_bar` | `001-02_bar` |
| `1-00003_bar` | `001-03_bar` |
| `0001-00002_baz` | `001-02_baz` |

These flexible formats work with all CLI commands that accept module or change IDs.

## Interactive Module Selection

When running `/ito-proposal` without specifying a module, you'll be prompted with three options:

1. **Use last worked-on module** - If you recently worked on a module, this option appears first
1. **Create a new module** - Prompts for a module name and creates it
1. **Ungrouped (module 000)** - For small, standalone changes

The system tracks your last-used module in `.ito/.state/session.json`.

## CLI Commands Reference

| Command | Purpose |
|---------|---------|
| `ito list --json` | List all changes |
| `ito status --change <id>` | Show change status and artifacts |
| `ito list --modules` | List modules |
| `ito create module "<name>"` | Create new module |
| `ito create change "<name>" --module <id>` | Create new change |
| `ito agent instruction <action> --change <id>` | Get action instructions |
| `ito validate --changes <id>` | Validate change |
| `ito archive <name>` | Archive completed change |

**Note:** All `<id>` parameters accept flexible formats (e.g., `1-2_foo` instead of `001-02_foo`).

## Directory Structure

```
.ito/
├── .state/
│   └── session.json        # Tracks last module/change worked on
├── changes/
│   ├── 000-01_small-fix/
│   │   ├── .ito.yaml
│   │   ├── proposal.md
│   │   ├── specs/
│   │   │   └── fix-description/
│   │   │       └── spec.md
│   │   └── tasks.md
│   └── archive/
│       └── 000-00_completed-change/
├── research/
│   ├── SUMMARY.md
│   └── investigations/
└── modules/
```

## Best Practices

1. **Start with a proposal** - Even small changes benefit from documenting "why"
1. **One capability = one spec** - Keep specs focused and testable
1. **Mark tasks complete immediately** - Don't batch completions
1. **Validate before archiving** - Catch issues early
1. **Use modules for related work** - Keeps changes organized
1. **Commit with change context** - Links commits to their originating change

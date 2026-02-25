---
name: migrate-openspec-to-ito
description: Use when migrating a repository from OpenSpec to Ito, replacing openspec CLI usage, directory structures, agent instructions, and slash commands with their Ito equivalents
---

# Migrate OpenSpec to Ito

Guide for migrating a project that uses OpenSpec (the predecessor tool) to Ito.

## Overview

OpenSpec was the original name for what is now Ito. Projects using OpenSpec have a `.openspec/` or similar directory, use the `openspec` CLI binary, and may have `/opsx:*` slash commands. This skill walks through replacing all OpenSpec artifacts with their Ito equivalents.

## When to Use

- Repository has an `openspec` CLI dependency or references
- Project contains `.openspec/` directory, `openspec.toml`, or `openspec.yaml`
- Agent instructions reference `openspec list`, `openspec show`, `openspec view`
- Slash commands use `/opsx:*` prefix (e.g., `/opsx:new`, `/opsx:apply`)
- Skills or commands reference `openspec` in descriptions or scripts

## Pre-Migration Checklist

Before making changes, inventory the current state:

```bash
# 1. Find all openspec references in the repo
rg -n "openspec" --type-add 'config:*.{toml,yaml,yml,json,md}' -t config .
rg -n "openspec" --type-add 'script:*.{sh,bash,zsh}' -t script .

# 2. Find OPSX slash command references
rg -n "opsx:" .

# 3. Check for openspec directory structures
ls -la .openspec/ 2>/dev/null
ls -la .openspec 2>/dev/null

# 4. Check agent instruction files
rg -n "openspec" AGENTS.md CLAUDE.md .claude/ .opencode/ .codex/ .github/ 2>/dev/null

# 5. Check for openspec binary usage in scripts
rg -n "openspec " Makefile scripts/ .github/workflows/ 2>/dev/null
```

Record all findings before proceeding. Every reference must be migrated.

## Migration Steps

### Step 1: Install Ito

```bash
# Install the Ito CLI (if not already available)
# Check: https://github.com/withakay/ito for installation instructions

# Initialize Ito in the project
ito init --tools all
```

This creates the `.ito/` directory with:

```
.ito/
├── project.md              # Project conventions (edit this)
├── config.json             # Project configuration (committed)
├── config.local.json       # Personal overrides (gitignored)
├── AGENTS.md               # AI agent instructions
├── specs/                  # Current truth specifications
│   └── [capability]/
│       └── spec.md
├── modules/                # Module (epic) definitions
│   └── [NNN_module-name]/
│       └── module.md
├── changes/                # Active change proposals
│   └── [change-id]/
│       ├── proposal.md
│       ├── tasks.md
│       ├── design.md       # Optional
│       └── specs/
│           └── [capability]/
│               └── spec.md # Delta specs
└── .state/                 # Internal state (gitignored)
```

It also installs agent integrations under `.opencode/`, `.claude/`, `.codex/`, and `.github/`.

### Step 2: Migrate Specifications

Move existing OpenSpec specs into Ito's structure:

| OpenSpec Location | Ito Location |
|---|---|
| `.openspec/specs/<name>.md` | `.ito/specs/<name>/spec.md` |
| `.openspec/specs/<name>/` | `.ito/specs/<name>/spec.md` |
| Inline specs in config files | `.ito/specs/<capability>/spec.md` |

Ito organizes specs by capability, one directory per capability:

```bash
# For each existing spec, create the Ito equivalent
mkdir -p .ito/specs/<capability-name>
# Move/adapt the spec content into spec.md
```

Spec format remains compatible. Ito uses the same requirement/scenario structure:

```markdown
### Requirement: Feature Name
The system SHALL provide...

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result
```

### Step 3: Migrate Active Changes

Move in-flight change proposals:

| OpenSpec Location | Ito Location |
|---|---|
| `.openspec/changes/<name>/` | `.ito/changes/<name>/` |
| `.openspec/changes/<name>/proposal.md` | `.ito/changes/<name>/proposal.md` |
| `.openspec/changes/<name>/tasks.md` | `.ito/changes/<name>/tasks.md` |
| `.openspec/changes/<name>/specs/` | `.ito/changes/<name>/specs/` |

If the project uses module grouping, adopt the `NNN-CC_name` convention:

- `NNN` = 3-digit module ID
- `CC` = 2-digit change number within module
- Example: `001-01_add-authentication`

For ungrouped changes, use module `000` or omit the prefix.

### Step 4: Migrate Archived Changes

```bash
# Move archived changes
mv .openspec/changes/archive/* .ito/changes/archive/ 2>/dev/null
# Or if stored differently:
mv .openspec/archive/* .ito/changes/archive/ 2>/dev/null
```

### Step 5: Replace CLI References

Replace all `openspec` CLI calls with `ito` equivalents:

| OpenSpec Command | Ito Command |
|---|---|
| `openspec list` | `ito list` |
| `openspec list --changes` | `ito list` or `ito list --changes` |
| `openspec list --specs` | `ito list --specs` |
| `openspec view` | `ito list` (no interactive dashboard; use `ito list --json` for scripts) |
| `openspec show <item>` | `ito show <item>` |
| `openspec show <item> --json` | `ito show <item> --json` |
| `openspec show <item> --json --deltas-only` | `ito show <item> --json --deltas-only` |
| `openspec show <item> -r <id>` | `ito show <item> --json -r <id>` |
| `openspec validate` | `ito validate` |

New Ito commands with no OpenSpec equivalent:

| Ito Command | Purpose |
|---|---|
| `ito list --pending` | Filter to changes with no tasks started |
| `ito list --partial` | Filter to partially complete changes |
| `ito list --completed` | Filter to completed changes |
| `ito list --modules` | List all modules |
| `ito validate --strict` | Comprehensive validation |
| `ito tasks status <id>` | Show task progress for a change |
| `ito tasks start <id> <task>` | Mark a task in-progress |
| `ito tasks complete <id> <task>` | Mark a task complete |
| `ito archive <id>` | Archive completed change and merge specs |
| `ito audit log` | View audit event log |
| `ito agent instruction <phase> --change <id>` | Get AI agent instructions |

### Step 6: Replace OPSX Slash Commands

If the project used `/opsx:*` slash commands, replace with `/ito-*`:

| OPSX Command | Ito Equivalent |
|---|---|
| `/opsx:new` | Use `ito create change` or scaffold manually |
| `/opsx:continue` | Use `ito tasks next <change-id>` |
| `/opsx:ff` | No direct equivalent; use `ito tasks status` to check progress |
| `/opsx:apply` | Use `ito agent instruction apply --change <id>` |
| `/opsx:sync` | Use `ito archive <id>` to merge deltas into main specs |
| `/opsx:archive` | `ito archive <id> --yes` |

### Step 7: Update Agent Instructions

Replace OpenSpec references in agent instruction files:

```bash
# Files to check and update:
# - AGENTS.md (root)
# - CLAUDE.md
# - .claude/CLAUDE.md
# - .opencode/AGENTS.md (will be overwritten by ito update)
# - .codex/AGENTS.md
# - .github/copilot-instructions.md

# After updating, run:
ito update
```

`ito update` refreshes all managed instruction blocks (between `<!-- ITO:START -->` and `<!-- ITO:END -->` markers). User-owned content outside those markers is preserved.

### Step 8: Update Skills and Commands

Replace any openspec-specific skills or commands:

```bash
# Remove old openspec skills/commands
rm -rf .opencode/skills/openspec-*
rm -rf .opencode/commands/openspec-*
rm -rf .claude/skills/openspec-*

# Also check global config
rm -rf ~/.config/opencode/skill/openspec-*
rm -rf ~/.config/opencode/command/openspec-*
```

Ito installs its own skills automatically via `ito init` / `ito update`.

### Step 9: Update CI/CD and Scripts

Search and replace in automation files:

```bash
# Makefile, scripts/, .github/workflows/
rg -l "openspec" Makefile scripts/ .github/workflows/ package.json 2>/dev/null
```

Replace `openspec` binary calls with `ito` equivalents per the CLI mapping table above.

### Step 10: Remove Old OpenSpec Directory

Once everything is migrated and verified:

```bash
# Final verification
ito validate --strict
ito list --specs   # Verify all specs are visible
ito list           # Verify all changes are visible

# Remove old directory
rm -rf .openspec/

# Remove any openspec config files
rm -f openspec.toml openspec.yaml openspec.json

# Commit the migration
git add -A
git commit -m "chore: migrate from OpenSpec to Ito"
```

## Post-Migration Verification

Run these checks to confirm the migration is complete:

```bash
# 1. No remaining openspec references
rg -n "openspec" . --glob '!.git' --glob '!node_modules'
# Should return nothing (or only historical references in changelogs)

# 2. Ito validates cleanly
ito validate --strict

# 3. Specs are accessible
ito list --specs

# 4. Changes are accessible
ito list

# 5. Agent instructions work
ito agent instruction apply --change <any-active-change-id>
```

## Common Issues

**"openspec: command not found" after migration**
The `openspec` binary no longer exists. All commands are now `ito`. Search for remaining references: `rg "openspec " .`

**Specs not showing up in `ito list --specs`**
Ito expects `specs/<capability>/spec.md` structure. Each capability must be in its own directory with a `spec.md` file inside.

**Changes missing after migration**
Each change needs at minimum a `proposal.md` and a `specs/` directory with at least one delta. Run `ito validate <change-id> --strict` for detailed diagnostics.

**Managed blocks overwritten by `ito update`**
Content between `<!-- ITO:START -->` and `<!-- ITO:END -->` is managed by Ito. Place custom content outside those markers or in `.ito/user-prompts/guidance.md`.

## Quick Reference

| Concept | OpenSpec | Ito |
|---|---|---|
| CLI binary | `openspec` | `ito` |
| Config directory | `.openspec/` | `.ito/` |
| Slash commands | `/opsx:*` | `/ito-*` or Ito skills |
| Spec location | `.openspec/specs/` | `.ito/specs/<capability>/spec.md` |
| Change location | `.openspec/changes/` | `.ito/changes/<id>/` |
| Archive location | `.openspec/archive/` | `.ito/changes/archive/` |
| Agent instructions | Manual AGENTS.md | `ito agent instruction <phase>` |
| Task tracking | Manual checkboxes | `ito tasks start/complete` with audit trail |
| Validation | `openspec validate` | `ito validate --strict` |
| Initialization | `openspec init` | `ito init --tools all` |
| Refresh configs | Manual | `ito update` |

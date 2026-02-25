# Migrating from OpenSpec

If your project previously used **OpenSpec** (the predecessor to Ito), this guide walks through replacing all OpenSpec artifacts with their Ito equivalents.

## What changed

OpenSpec was renamed to Ito. The core spec-driven workflow is the same, but the CLI binary, directory layout, slash commands, and agent tooling have all been updated.

| Aspect | OpenSpec | Ito |
|--------|----------|-----|
| CLI binary | `openspec` | `ito` |
| Config directory | `.openspec/` | `.ito/` |
| Slash commands | `/opsx:*` | `/ito-*` or Ito skills |
| Spec location | `.openspec/specs/` | `.ito/specs/<capability>/spec.md` |
| Change location | `.openspec/changes/` | `.ito/changes/<id>/` |
| Archive location | `.openspec/archive/` | `.ito/changes/archive/` |
| Agent instructions | Manual `AGENTS.md` | `ito agent instruction <phase>` |
| Task tracking | Manual checkboxes | `ito tasks start/complete` with audit trail |
| Validation | `openspec validate` | `ito validate --strict` |
| Initialization | `openspec init` | `ito init --tools all` |
| Refresh configs | Manual | `ito update` |

## Before you start

Inventory all OpenSpec references so nothing is missed:

```bash
# Find all openspec references
rg -n "openspec" --type-add 'config:*.{toml,yaml,yml,json,md}' -t config .
rg -n "openspec" --type-add 'script:*.{sh,bash,zsh}' -t script .

# Find OPSX slash commands
rg -n "opsx:" .

# Check for openspec directory
ls -la .openspec/ 2>/dev/null

# Check agent instruction files
rg -n "openspec" AGENTS.md CLAUDE.md .claude/ .opencode/ .codex/ .github/ 2>/dev/null

# Check CI and build scripts
rg -n "openspec " Makefile scripts/ .github/workflows/ 2>/dev/null
```

## Step 1: Install and initialize Ito

```bash
ito init --tools all
```

This creates the `.ito/` directory structure and installs agent integrations for your harnesses (OpenCode, Claude Code, Codex, GitHub Copilot).

The resulting layout:

```
.ito/
├── project.md              # Project conventions (edit this)
├── config.json             # Project config (committed)
├── config.local.json       # Personal overrides (gitignored)
├── AGENTS.md               # AI agent instructions
├── specs/                  # Specifications (current truth)
│   └── <capability>/
│       └── spec.md
├── modules/                # Module (epic) groupings
│   └── <NNN_name>/
│       └── module.md
├── changes/                # Active change proposals
│   └── <change-id>/
│       ├── proposal.md
│       ├── tasks.md
│       ├── design.md       # optional
│       └── specs/          # delta specs
│           └── <capability>/
│               └── spec.md
└── .state/                 # Internal state (gitignored)
```

## Step 2: Migrate specifications

Move existing specs into Ito's per-capability structure.

```bash
# For each spec
mkdir -p .ito/specs/<capability-name>
cp .openspec/specs/<name>.md .ito/specs/<capability-name>/spec.md
```

The requirement and scenario format is unchanged:

```markdown
### Requirement: Feature Name
The system SHALL provide...

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result
```

## Step 3: Migrate active changes

Move in-flight change proposals:

```bash
cp -r .openspec/changes/<name>/ .ito/changes/<name>/
```

If adopting module grouping, rename to the `NNN-CC_name` convention (e.g., `001-01_add-authentication`). Otherwise use module `000` or omit the prefix.

Each change directory should contain at minimum `proposal.md` and a `specs/` subdirectory with at least one delta file.

## Step 4: Migrate archived changes

```bash
cp -r .openspec/changes/archive/* .ito/changes/archive/ 2>/dev/null
# or
cp -r .openspec/archive/* .ito/changes/archive/ 2>/dev/null
```

## Step 5: Replace CLI commands

| OpenSpec | Ito |
|----------|-----|
| `openspec list` | `ito list` |
| `openspec list --changes` | `ito list` |
| `openspec list --specs` | `ito list --specs` |
| `openspec view` | `ito list` (use `--json` for scripts) |
| `openspec show <item>` | `ito show <item>` |
| `openspec show <item> --json` | `ito show <item> --json` |
| `openspec show <item> --json --deltas-only` | `ito show <item> --json --deltas-only` |
| `openspec show <item> -r <id>` | `ito show <item> --json -r <id>` |
| `openspec validate` | `ito validate --strict` |

Ito also provides commands that had no OpenSpec equivalent:

```bash
ito list --pending              # Changes with no tasks started
ito list --partial              # Partially complete changes
ito list --completed            # Completed changes
ito list --modules              # List modules
ito tasks status <id>           # Task progress for a change
ito tasks start <id> <task>     # Mark task in-progress
ito tasks complete <id> <task>  # Mark task complete
ito archive <id>                # Archive and merge specs
ito audit log                   # View audit event log
ito agent instruction apply --change <id>  # Agent instructions
```

## Step 6: Replace OPSX slash commands

If your project used the experimental `/opsx:*` commands:

| OPSX | Ito |
|------|-----|
| `/opsx:new` | `ito create change` or scaffold manually |
| `/opsx:continue` | `ito tasks next <change-id>` |
| `/opsx:ff` | `ito tasks status` to check progress |
| `/opsx:apply` | `ito agent instruction apply --change <id>` |
| `/opsx:sync` | `ito archive <id>` |
| `/opsx:archive` | `ito archive <id> --yes` |

## Step 7: Update agent instructions

Replace OpenSpec references in agent config files (`AGENTS.md`, `CLAUDE.md`, `.opencode/`, `.codex/`, `.github/copilot-instructions.md`), then run:

```bash
ito update
```

This refreshes all managed instruction blocks (content between `<!-- ITO:START -->` and `<!-- ITO:END -->` markers). Your content outside those markers is preserved.

## Step 8: Clean up old skills and commands

```bash
# Project-level
rm -rf .opencode/skills/openspec-*
rm -rf .opencode/commands/openspec-*
rm -rf .claude/skills/openspec-*

# Global config (optional)
rm -rf ~/.config/opencode/skill/openspec-*
rm -rf ~/.config/opencode/command/openspec-*
```

Ito installs its own skills via `ito init` and `ito update`.

## Step 9: Update CI/CD and scripts

Search automation files for remaining references:

```bash
rg -l "openspec" Makefile scripts/ .github/workflows/ package.json 2>/dev/null
```

Replace `openspec` calls with their `ito` equivalents from the command mapping above.

## Step 10: Remove old directory and commit

```bash
# Verify everything works
ito validate --strict
ito list --specs
ito list

# Remove old files
rm -rf .openspec/
rm -f openspec.toml openspec.yaml openspec.json

# Check for stray references
rg -n "openspec" . --glob '!.git' --glob '!node_modules'

# Commit
git add -A
git commit -m "chore: migrate from OpenSpec to Ito"
```

## Troubleshooting

**Specs not showing up in `ito list --specs`**
:   Ito expects each capability in its own directory: `.ito/specs/<capability>/spec.md`. A flat file like `.ito/specs/auth.md` won't be found.

**Changes missing after migration**
:   Each change needs `proposal.md` and at least one delta file under `specs/`. Run `ito validate <change-id> --strict` for diagnostics.

**Managed blocks overwritten by `ito update`**
:   Content between `<!-- ITO:START -->` and `<!-- ITO:END -->` is owned by Ito. Place custom instructions outside those markers or in `.ito/user-prompts/guidance.md`.

**`openspec: command not found`**
:   The binary no longer exists. All commands are now `ito`. Search for remaining references with `rg "openspec " .`.

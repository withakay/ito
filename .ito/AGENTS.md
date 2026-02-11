# Ito Instructions (Concise)

Use this file as the quick operational guide for Ito workflows.

## What lives where

- Core Ito guidance: `.ito/AGENTS.md` (this file)
- Project-specific guidance: `.ito/user-guidance.md`, `AGENTS.md`, `CLAUDE.md`
- Tool wiring files (managed by Ito): `.opencode/`, `.github/`, `.codex/`, `.claude/`

Managed files may be overwritten by `ito init` / `ito update`.

## Workflow at a glance

1. **Create** a change proposal when behavior/capabilities/architecture change.
2. **Implement** only after proposal approval.
3. **Archive** after deployment.

Skip proposal for trivial edits (typos/format/comments) and straightforward bug fixes that restore intended behavior.

## Required pre-checks

Before proposing or implementing:

- `ito list`
- `ito list --specs`
- Read `.ito/project.md`
- Check active changes for overlap/conflicts

## Create phase (proposal)

Create under `.ito/changes/<change-id>/`:

- `proposal.md` (why + what + impact)
- `tasks.md` (implementation checklist)
- `design.md` (only when complexity warrants it)
- `specs/<capability>/spec.md` deltas

Validate before review:

- `ito validate <change-id> --strict`

## Implement phase

- Read `proposal.md`, `design.md` (if present), `tasks.md`
- Execute tasks in order
- Keep task status accurate (prefer `ito tasks start/complete`)
- Do not start implementation before proposal approval

## Archive phase

After deployment:

- `ito archive <change-id> --yes`
- For tooling-only changes: `ito archive <change-id> --skip-specs --yes`
- Final check: `ito validate --strict`

## Spec delta rules (important)

Allowed delta sections:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

Each requirement must include at least one scenario header exactly like:

`#### Scenario: <name>`

Use normative wording (`SHALL`/`MUST`) for requirements.

When using `MODIFIED`, include the full updated requirement text (not partial fragments).

## Task tracking commands

```bash
ito tasks status <change-id>
ito tasks next <change-id>
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
```

## Essential commands

```bash
ito list
ito list --specs
ito show <item>
ito validate <item> --strict
ito archive <change-id> --yes
```

## Testing policy

- TDD default: RED -> GREEN -> REFACTOR
- Coverage target: 100%
- Coverage minimum: 80%
- Prefer real implementations/fakes over mocks; mock only when justified

Config overrides (low -> high precedence):

- `ito.json`
- `.ito.json`
- `.ito/config.json`
- `$PROJECT_DIR/config.json` (when used)

Keys:

- `defaults.testing.tdd.workflow`
- `defaults.testing.coverage.target_percent`
- `defaults.testing.coverage.minimum_percent`

## Naming conventions

- Module: `NNN_module-name` (example: `001_project-setup`)
- Modular change: `NNN-CC_change-name` (example: `001-01_init-repo`)
- `000` module is for ungrouped one-off work

## Troubleshooting quick fixes

- "at least one delta": ensure `.ito/changes/<id>/specs/**/spec.md` exists with valid delta headers
- "requirement must have scenario": use `#### Scenario: ...` (4 hashes)
- Parsing confusion: `ito show <change-id> --json --deltas-only`

## Source of truth commands

For detailed, phase-specific instructions, prefer:

```bash
ito agent instruction proposal --change <change-id>
ito agent instruction specs --change <change-id>
ito agent instruction tasks --change <change-id>
ito agent instruction apply --change <change-id>
ito agent instruction review --change <change-id>
ito agent instruction archive --change <change-id>
```

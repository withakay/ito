## Why

The `ito-skills/` directory contains files and directories that are not used by Ito's distribution mechanism. Ito only distributes skills from `ito-skills/skills/` via the `ITO_SKILLS` list in `distribution.rs`. The additional directories (adapters, agents, commands, hooks, lib, tests, docs) create confusion about what's actually part of Ito vs. what's legacy/external tooling.

## What Changes

Remove directories and files from `ito-skills/` that are not used by Ito:

### To Remove

| Path | Reason |
|------|--------|
| `ito-skills/adapters/` | Not used by distribution - adapter templates are embedded in ito-templates |
| `ito-skills/agents/` | Not used by distribution |
| `ito-skills/commands/` | Not used by distribution |
| `ito-skills/hooks/` | Not used by distribution |
| `ito-skills/lib/` | Not used by distribution |
| `ito-skills/tests/` | Test infrastructure, not distributed |
| `ito-skills/docs/` | Documentation, not distributed |
| `ito-skills/.claude-plugin/` | Claude plugin, not distributed |
| `ito-skills/.codex/` | Codex config, not distributed |
| `ito-skills/.github/` | GitHub config, not distributed |
| `ito-skills/.opencode/` | OpenCode config, not distributed |
| `ito-skills/README.md` | Repo readme, not distributed |
| `ito-skills/RELEASE-NOTES.md` | Release notes, not distributed |
| `ito-skills/LICENSE` | Keep - needed for attribution |
| `ito-skills/.gitignore` | Keep if skills/ remains a git repo |
| `ito-skills/.gitattributes` | Keep if skills/ remains a git repo |

### To Keep

| Path | Reason |
|------|--------|
| `ito-skills/skills/` | Source of truth for distributed skills |
| `ito-skills/LICENSE` | Legal requirement |
| `ito-skills/.gitignore` | Git config (optional) |
| `ito-skills/.gitattributes` | Git config (optional) |

## Capabilities

### New Capabilities

None - this is a cleanup/maintenance change.

### Modified Capabilities

None - no behavior changes.

## Impact

- **Code**: Only `ito-skills/` directory structure
- **Distribution**: No impact - only `skills/` is distributed
- **Risk**: Low - removing unused files
- **Dependencies**: None

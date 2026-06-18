# Ito Lite

Ito Lite is a prompt-driven, markdown-only version of the Ito change proposal workflow.

It is intended for environments where agents cannot install or run the `ito` executable. Copy the directories under `skills/` into any Agent Skills-compatible client, then use the skills to maintain `.ito-lite/` artifacts by editing markdown only.

## Contents

- `REQUIREMENTS.md` - extracted process requirements and format rules from the full Ito proposal workflow.
- `agents/` - prompt-only replicas of the agent definitions full Ito installs.
- `skills/ito-lite-setup/` - initializes a markdown-only project workspace.
- `skills/ito-lite/` - routes users through the Ito Lite workflow.
- `skills/ito-lite-proposal/` - creates proposal, spec delta, design, and task artifacts.
- `skills/ito-lite-apply/` - implements tasks with manual markdown status tracking.
- `skills/ito-lite-review/` - reviews proposal packages before implementation.
- `skills/ito-lite-archive/` - merges completed spec deltas into current specs and archives the change.

## Portable Agents

The `agents/` directory replicates the Ito-created agent surfaces as markdown prompts adapted for Ito Lite:

- `ito-quick`
- `ito-general`
- `ito-thinking`
- `ito-orchestrator`
- `ito-planner`
- `ito-researcher`
- `ito-worker`
- `ito-reviewer`
- `ito-test-runner`

Copy these files into the target agent directory for your harness, or use them as reference prompts if the harness has a different agent format. They intentionally avoid all `ito` executable calls and operate on `.ito-lite/` markdown artifacts.

## Default Layout

```text
.ito-lite/
├── project.md
├── specs/
│   └── <capability>/
│       └── spec.md
├── changes/
│   ├── <change-id>/
│   │   ├── proposal.md
│   │   ├── design.md
│   │   ├── tasks.md
│   │   └── specs/
│   │       └── <capability>/
│   │           └── spec.md
│   └── archive/
└── modules/
```

Use `.ito-lite/` by default. If a project intentionally wants full Ito-compatible paths and no real Ito CLI is present, the same rules can be applied to `.ito/` instead.

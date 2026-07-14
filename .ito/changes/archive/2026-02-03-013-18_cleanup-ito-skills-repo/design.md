# Design: Cleanup ito-skills Repository

## Current State

The `ito-skills/` directory contains:

```
ito-skills/
├── .claude-plugin/     # Not used by ito
├── .codex/             # Not used by ito
├── .github/            # Not used by ito
├── .opencode/          # Not used by ito
├── adapters/           # Not used - templates embedded in ito-templates
├── agents/             # Not used by ito
├── commands/           # Not used by ito
├── docs/               # Not used by ito
├── hooks/              # Not used by ito
├── lib/                # Not used by ito
├── skills/             # USED - source for ITO_SKILLS distribution
├── tests/              # Not used by ito
├── .gitattributes      # Git config
├── .gitignore          # Git config
├── LICENSE             # Legal
├── README.md           # Not used by ito
└── RELEASE-NOTES.md    # Not used by ito
```

## Distribution Mechanism

Ito's distribution only uses:

1. **Local mode**: Reads from `ito-skills/skills/<name>/SKILL.md`
2. **Remote mode**: Fetches from GitHub `ito-skills/skills/<name>/SKILL.md`

The `ITO_SKILLS` constant in `distribution.rs` defines the 12 skills to distribute.

## Target State

```
ito-skills/
├── skills/             # 12 skill directories
│   ├── brainstorming/
│   ├── dispatching-parallel-agents/
│   ├── finishing-a-development-branch/
│   ├── receiving-code-review/
│   ├── requesting-code-review/
│   ├── subagent-driven-development/
│   ├── systematic-debugging/
│   ├── test-driven-development/
│   ├── using-git-worktrees/
│   ├── using-ito-skills/
│   ├── verification-before-completion/
│   └── writing-skills/
├── LICENSE             # Keep for attribution
├── .gitignore          # Keep for git
└── .gitattributes      # Keep for git
```

## Decisions

1. **Keep as subdirectory**: `ito-skills/` remains a directory in the ito repo (not a separate git submodule)
2. **Minimal structure**: Only keep what's needed for skill distribution
3. **Remove adapters**: Adapter templates are already embedded in `ito-templates/assets/`

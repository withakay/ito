# Ito Lite Agents

This directory contains prompt-only replicas of the agent surfaces that full Ito installs for supported harnesses.

These files are adapted for Ito Lite:

- They do not call `ito agent instruction`, `ito patch`, `ito write`, or any other Ito executable command.
- They default to `.ito-lite/` artifacts.
- They preserve the same roles as full Ito agents: quick/general/thinking direct work, orchestration planning/research/work/review, and test execution.

## Agent Inventory

| File | Full Ito source role | Ito Lite role |
| --- | --- | --- |
| `ito-quick.md` | Fast delegated agent for simple tasks | Fast small-task prompt using plain files |
| `ito-general.md` | Balanced direct development agent | Default Ito Lite implementer/reviewer |
| `ito-thinking.md` | High-capability reasoning agent | Architecture, hard debugging, complex planning |
| `ito-orchestrator.md` | Multi-change coordinator | Prompt-only coordinator for `.ito-lite/changes/` |
| `ito-planner.md` | Orchestration plan builder | Dependency and gate planner from markdown artifacts |
| `ito-researcher.md` | Read-only context gatherer | Read-only repo and artifact researcher |
| `ito-worker.md` | Scoped implementation/remediation worker | Scoped work-packet implementer using markdown context |
| `ito-reviewer.md` | Gate and worker output reviewer | Read-only reviewer with remediation packets |
| `ito-test-runner.md` | OpenCode-only curated test runner | Non-mutating test runner prompt |

## Copying To A Harness

Use the file names as canonical names. Copy the markdown files into the target harness's project-agent directory and adjust frontmatter only if that harness requires a different schema.

Common destinations:

- OpenCode: `.opencode/agents/ito-*.md`
- Claude Code: `.claude/agents/ito-*.md`
- GitHub Copilot: `.github/agents/ito-*.md`
- Pi: `.pi/agents/ito-*.md`
- Codex-style skill agents: `.agents/skills/<agent-name>/SKILL.md` (wrap each file as a skill if needed)

If the environment already has full Ito installed, avoid overwriting its managed agent files. Keep these under a separate name such as `ito-lite-general.md` or use this directory as reference-only prompts.

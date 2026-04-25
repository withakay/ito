# ByteRover skills (local only)

This directory holds the ByteRover hub skills that all non-Claude agent
harnesses discover via `.agents/skills/`. Claude Code consumes its own
connector tree (`.claude/skills/`), so the same four skills are also
installed there — see the Mirror step below.

The `SKILL.md` at the root of this directory is the base ByteRover skill
(describes the `brv` CLI and when to query / curate). The four sub-directories
below are the **hub skills** installed by change
[`029-01_add-byterover-integration`](../../../.ito/changes/029-01_add-byterover-integration/).

## Installed hub skills

| Skill | Purpose |
| --- | --- |
| `byterover-review/` | Review staged changes or specific files against stored conventions, patterns, and architecture decisions. Flags missing tests and security concerns and curates newly observed patterns. |
| `byterover-plan/` | Produce a goal-backward implementation plan informed by what ByteRover already knows about this project. Stores the plan via `brv curate`. |
| `byterover-explore/` | Systematically map the codebase, docs, architecture, conventions, testing, integrations, and known concerns into the ByteRover context tree via `brv curate`. |
| `byterover-audit/` | Audit stored knowledge for freshness and coverage: find stale or outdated entries, identify gaps, and emit targeted `brv curate` commands to fix them. |

## How to reproduce

From the project root:

```bash
# 1. Install the Claude Code agent-skill connector (required for `.claude/skills/`)
brv connectors install "Claude Code" --type skill

# 2. Install the four hub skills into Claude Code
brv hub install byterover-review  --agent "Claude Code"
brv hub install byterover-plan    --agent "Claude Code"
brv hub install byterover-explore --agent "Claude Code"
brv hub install byterover-audit   --agent "Claude Code"

# 3. Mirror the installed skills into the shared skills directory
#    used by every other agent harness (no byterover hub-install target
#    writes to .agents/skills/ directly today).
for id in byterover-review byterover-plan byterover-explore byterover-audit; do
  rm -rf ".agents/skills/byterover/$id"
  cp -R ".claude/skills/$id" ".agents/skills/byterover/$id"
done
```

After step 3, the same four skills exist under both `.claude/skills/` and
`.agents/skills/byterover/`, and `SKILL.md` contents match byte-for-byte.

## Bootstrap curation

The change also runs a one-off `brv curate` pass so ByteRover has the
project's canonical knowledge available for `brv query` / `brv search`:

```bash
# Folder packs
brv curate --folder .ito/specs/
brv curate --folder docs/
brv curate --folder .ito/changes/archive/
brv curate --folder .ito/modules/

# Top-level agent-facing docs (file mode)
brv curate --files AGENTS.md --files .ito/AGENTS.md --files .ito/architecture.md
```

## Cloud sync — intentionally not used

This integration is **local only**. No workflow, skill, or instruction
template produced by change `029-01_add-byterover-integration` depends on
any of:

- `brv login`
- `brv push`
- `brv pull`
- `brv vc …` (ByteRover's context-tree version control)

If you want cloud sync, that is a separate, opt-in step — it is not a
prerequisite for using the skills above or the bootstrap curation.

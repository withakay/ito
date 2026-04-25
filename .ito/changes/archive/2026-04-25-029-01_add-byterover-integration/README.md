# 029-01_add-byterover-integration

Add ByteRover as the first concrete agent-memory provider: install the Claude
Code connector, install ByteRover hub skills (review, plan, explore, audit)
under `.agents/skills/byterover/` (mirrored from `.claude/skills/`), and
curate Ito's authoritative docs/specs/modules into the local ByteRover
context tree. **Local-only** (no cloud sync). Shared skills live under
`.agents/skills/` for every agent harness; Claude Code gets its own copies
under `.claude/skills/` via its connector.

## Reviewer notes

What this change installs / does:

- **Claude Code connector** under `.claude/skills/byterover/` (installed
  via `brv connectors install "Claude Code" --type skill`).
- **Four ByteRover hub skills** — `byterover-review`, `byterover-plan`,
  `byterover-explore`, `byterover-audit` — installed under Claude Code via
  `brv hub install <id> --agent "Claude Code"`, then **mirrored** (byte-for-byte
  `cp -R`) into `.agents/skills/byterover/<id>/` so every other harness that
  reads `.agents/skills/` picks them up.
- **Bootstrap curation pass** (folder + file mode) over Ito's canonical
  knowledge so `brv query` and `brv search` return useful results without
  any cloud sync:
  - Folder packs: `.ito/specs/`, `docs/`, `.ito/changes/archive/`, `.ito/modules/`
  - Files: `AGENTS.md`, `.ito/AGENTS.md`, `.ito/architecture.md`

The spec (`specs/agent-memory-byterover/spec.md`) and
`.agents/skills/byterover/README.md` are the authoritative record of which
commands were run and where the skills now live. All cloud-sync commands
(`brv login`, `brv push`, `brv pull`, `brv vc …`) are **intentionally out of
scope** — mentions in this change exist only to mark them as such.

## Reproducing the integration

See `.agents/skills/byterover/README.md` for the full command list. In
short:

```bash
brv connectors install "Claude Code" --type skill
for id in byterover-review byterover-plan byterover-explore byterover-audit; do
  brv hub install "$id" --agent "Claude Code"
  rm -rf ".agents/skills/byterover/$id"
  cp -R ".claude/skills/$id" ".agents/skills/byterover/$id"
done

brv curate --folder .ito/specs/
brv curate --folder docs/
brv curate --folder .ito/changes/archive/
brv curate --folder .ito/modules/
brv curate --files AGENTS.md --files .ito/AGENTS.md --files .ito/architecture.md
```

## Follow-on

The follow-on change
[`029-02_agent-memory-abstraction`](../029-02_agent-memory-abstraction/) makes
agent memory a first-class, provider-agnostic concept in Ito (config-driven
store/search commands or skill references; apply/finish instruction reminders
to capture memories; finish wrap-up to refresh archive and specs). It has no
default provider — this change's ByteRover installation is what a user would
point that abstraction at on this repo.

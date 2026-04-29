---
name: ito-memory
description: Use Ito's configured memory provider to capture, search, and query project knowledge. Activate when users ask to remember, recall, search memory, query memory, save learnings, or use Ito memory. Provider-agnostic: routes through `ito agent instruction memory-capture`, `memory-search`, and `memory-query` rather than calling ByteRover or another backend directly.
---

<!-- ITO:START -->
# Ito Memory

Use this skill when you need Ito's configured memory provider. It has three operations:

- `capture`: store durable knowledge from the current work.
- `search`: retrieve ranked matching memory entries.
- `query`: ask for a synthesized answer from memory.

Do not call a concrete provider directly unless the rendered instruction tells you to. The project may use ByteRover, a markdown-backed skill, a command, or no provider at all.

## Capture

Capture only durable knowledge: decisions, rationale, gotchas, recurring patterns, architecture rules, or important workflow discoveries.

```bash
ito agent instruction memory-capture \
  --context "<one-paragraph memory>" \
  --file <path> \
  --folder <path>
```

- `--context`: memory summary
- `--file`: repeatable supporting files
- `--folder`: repeatable supporting folders

Run the command printed by Ito, or invoke the skill named by Ito if the project uses a skill-backed provider.

## Search

Use search when you need likely matching memory entries or paths before reading source files.

```bash
ito agent instruction memory-search --query "<terms>" --limit 10
```

Use `--scope <scope>` when the project documents scoped memory and the query should be narrowed.

## Query

Use query when you need a synthesized answer from memory before doing broader exploration.

```bash
ito agent instruction memory-query --query "<question>"
```

Treat memory as guidance, not the source of truth. If memory conflicts with specs, code, or current instructions, trust the current source and consider capturing the correction.

## Provider Not Configured

If an operation is not configured, continue with normal repo inspection and mention that Ito memory is unavailable for that operation.

## Good Captures

- Why a design decision was made.
- Non-obvious commands or setup required to work in this repo.
- A bug pattern and its verified fix.
- A convention that future agents are likely to miss.

## Avoid Capturing

- Short-lived chat state.
- Secrets or credentials.
- Raw command output with no durable lesson.
- Information already captured unchanged in nearby specs or docs.
<!-- ITO:END -->

# Design: Machine-Readable Ito Capabilities

## Overview

Build a small capabilities layer in the CLI that turns the real Clap command tree and Ito's instruction registry into a JSON manifest. The manifest is intended for agents, shell completion generators, docs checks, and future prompt validation.

## Shape

Add a `capabilities` command with these initial surfaces:

```bash
ito capabilities --json
ito capabilities command <path> --json
ito capabilities artifacts --json
ito capabilities aliases --json
```

## Implementation Notes

- Derive command metadata from the Clap `CommandFactory` tree where possible.
- Add explicit metadata only for concepts Clap cannot know, such as example commands, semantic intent, JSON support, and instruction artifact IDs.
- Keep the response deterministic by sorting command paths, flags, aliases, examples, and artifact IDs.
- Reuse existing help-all tests as a source of coverage, but assert against JSON structure instead of text snapshots only.

## JSON Model

The root response should include:

- `schema_version`
- `ito_version`
- `commands`
- `artifacts`
- `aliases`
- `generated_at` or an explicit omission if reproducibility requires no timestamp

Command entries should include:

- `path`
- `summary`
- `description`
- `flags`
- `positionals`
- `subcommands`
- `aliases`
- `examples`
- `supports_json`
- `deprecated`
- `replacement`

## Risks

Duplicating command metadata can drift. Prefer deriving from the command tree and keep manually curated metadata small and tested.

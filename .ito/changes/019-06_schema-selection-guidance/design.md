<!-- ITO:START -->
## Context

Schemas in Ito define a workflow: which artifacts exist, their dependencies, and how validation is configured.

Schema selection is currently discoverable only by already knowing about `--schema` and/or by exporting schemas to disk. Agents therefore rarely prompt users to choose.

## Goals / Non-Goals

- Goals:
  - Make schema selection explicit during change creation.
  - Provide a single, canonical schema list + decision guide for agents.
  - Provide a machine-readable JSON form so harness adapters and skills can consume it.
- Non-Goals:
  - Changing the default schema (`spec-driven`).
  - Forcing interactive prompts in the core CLI (this change focuses on agent-facing instructions).

## Decisions

- Decision: Add `ito agent instruction schemas` as the centralized entrypoint.
  - Text output: human-friendly list + short “which should I choose?” guide.
  - JSON output: stable schema records (name, description, artifacts, source).

- Decision: The `ito-write-change-proposal` skill asks the user to choose a schema.
  - Default recommendation: `spec-driven`.
  - The chosen schema is applied by passing `--schema <name>` to `ito create change`.

## JSON Contract (draft)

```json
{
  "schemas": [
    {
      "name": "spec-driven",
      "description": "...",
      "artifacts": ["proposal", "specs", "design", "tasks"],
      "source": "embedded"
    }
  ],
  "recommended_default": "spec-driven"
}
```

## Testing Strategy

- Unit tests cover:
  - `ito agent instruction schemas` returns deterministic output.
  - `--json` output parses and contains the embedded schemas (`spec-driven`, `minimalist`, `tdd`, `event-driven`).
  - Skill template changes are reflected in embedded assets.
<!-- ITO:END -->

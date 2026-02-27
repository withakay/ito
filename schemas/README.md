# schemas/

This directory contains repository-owned JSON Schemas used by editors and tooling.

## JSON schema

- `ito-config.schema.json`
  - JSON Schema for Ito configuration files (for example: `.ito/config.json`).
  - This is published and referenced by templates as a `$schema` URL.

## Workflow schemas

Ito workflow schemas (the ones that define change artifacts like `proposal.md`, `tasks.md`, etc) are embedded in the CLI binary.

Canonical location in this repository:

- `ito-rs/crates/ito-templates/assets/schemas/`

To export embedded workflow schemas for local customization in a project:

```bash
ito templates schemas export -f .ito/templates/schemas
```

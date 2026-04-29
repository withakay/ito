# Source Guide: ito-templates

## Responsibility
`ito-templates` embeds files installed by `ito init` / `ito update`: default project/home templates, skills, commands, adapters, agents, schemas, presets, and instruction templates. It also renders Jinja-style project templates and normalizes custom Ito directory names.

## Entry Points
- `src/lib.rs`: embedded directory accessors and `.ito` path rewriting helpers.
- `src/agents.rs`: harness agent tiers, default models, and project agent install destinations.
- `src/instructions.rs`: instruction artifact template rendering.
- `src/project_templates.rs`: Jinja rendering for project templates such as `AGENTS.md`.
- `assets/**`: embedded source files compiled into the crate.

## Design
- This crate packages bytes and pure rendering helpers; filesystem writes happen in `ito-core` installers.
- Managed marker and version-stamp behavior are tested here because assets are embedded here.
- Default project prompts under `assets/default/project/.ito/user-prompts/` seed new repos.

## Flow
1. Core installers ask this crate for embedded files.
2. Rendering helpers rewrite configured Ito directory names or template variables.
3. Core writes the rendered bytes to project/home destinations.

## Integration
- `ito-core::installers` consumes most APIs.
- `ito-cli` integration tests verify installed outputs from the user perspective.

## Gotchas
- Adding, renaming, or moving assets changes compile-time embedded output.
- Markdown assets should preserve Ito managed markers when they are meant to be update-safe.
- Harness install paths should be centralized in `agents.rs` rather than duplicated in installers.

## Tests
- Targeted: `cargo test -p ito-templates`.
- Installation behavior: `cargo test -p ito-cli --test init_more`.

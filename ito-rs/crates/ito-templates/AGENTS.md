# ito-templates — L1 (Domain)

Single source of truth for all files installed by `ito init`/`ito update`. Assets embedded with `include_dir!`.
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Key Exports
|default_project_files(): assets/default/project/ |default_home_files(): assets/default/home/
|skills_files(): shared skills (all harnesses) |adapters_files(): harness bootstrap files
|commands_files(): shared command defs |agents module: rendering, harness configs, tier defs
|instructions module: instruction artifact template rendering
|normalize_ito_dir(), render_rel_path(), render_bytes(): path/content rewriting for custom .ito dir names
|ITO_START_MARKER, ITO_END_MARKER: managed block markers

## Asset Layout
```
assets/
├── default/{project/,home/}  # Installed to project root / ~/.config/
├── skills/                   # Shared skills → ALL harnesses
├── adapters/                 # Harness-specific bootstrap files
├── commands/                 # Shared command definitions
├── agents/                   # Agent prompt templates
└── instructions/             # Instruction artifact templates (Jinja)
```

## Dependencies
|none (external only: include_dir, minijinja, serde)

## Constraints
**MUST NOT:** depend on ito-core/ito-cli/ito-web | perform runtime fs I/O (compile-time embedding only) | contain business logic/domain models
**MUST:** be single source of truth for installed template content | keep harness files in sync | use managed block markers in update-safe files

## Critical Rules for Editing Templates
1. Edit template sources here, NOT installed outputs (.claude/, .opencode/, .github/, .ito/)
2. Keep harness variants equivalent — only frontmatter/tool syntax should differ
3. Skills live once under assets/skills/ — no per-harness copies unless harness requires it
4. Do NOT compact change-proposal templates (spec.md, design.md, proposal.md, tasks.md)
5. Preserve managed markers, placeholders, and code fences in prompt assets
6. Keep helper assets together inside skill directories; preserve executable bits for bundled scripts

## Verifying Changes
```bash
make install && ito init --force --tools all
```
Then inspect installed outputs. |documentation-police: template docs |rust-quality-checker: Rust code in this crate

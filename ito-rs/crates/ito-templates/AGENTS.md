# ito-templates — Layer 1 (Domain)

Embedded assets for `ito init` / `ito update`. Packages default project and home templates, shared skills, adapters, commands, agents, and instruction templates as compile-time embedded assets.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Own the canonical source files for everything `ito init` installs. Templates are compiled into the binary via `include_dir!` so `ito init` works without runtime filesystem dependencies for template content.

## Key Exports

| Export | Responsibility |
|---|---|
| `default_project_files()` | All files from `assets/default/project/` |
| `default_home_files()` | All files from `assets/default/home/` |
| `skills_files()` | Shared skills installed to all harnesses |
| `adapters_files()` | Harness-specific bootstrap files |
| `commands_files()` | Shared command definitions |
| `agents` module | Agent template rendering, harness configs, tier definitions |
| `instructions` module | Instruction artifact template rendering |
| `normalize_ito_dir()`, `render_rel_path()`, `render_bytes()` | Path/content rewriting for custom Ito directory names |
| `ITO_START_MARKER`, `ITO_END_MARKER` | Managed block markers |

## Asset Layout

```
assets/
├── default/
│   ├── project/          # Installed to project root (.ito/, .claude/, .opencode/, .github/, .codex/)
│   └── home/             # Installed to ~/.config/
├── skills/               # Shared skills — installed to ALL harnesses
├── adapters/             # Harness-specific bootstrap files
├── commands/             # Shared command definitions
├── agents/               # Agent prompt templates
└── instructions/         # Instruction artifact templates (Jinja)
```

## Workspace Dependencies

None — this is a standalone crate with only external dependencies (`include_dir`, `minijinja`, `serde`).

## Architectural Constraints

### MUST NOT

- Depend on `ito-core`, `ito-cli`, or `ito-web`
- Perform filesystem I/O at runtime — only compile-time embedding via `include_dir!`
- Contain business logic or domain models

### MUST

- Be the single source of truth for all installed template content
- Keep harness files in sync — commands/prompts under `.claude/`, `.opencode/`, `.codex/`, `.github/` must be functionally equivalent
- Use managed block markers (`<!-- ITO:START -->` / `<!-- ITO:END -->`) in files that `ito update` should refresh

## Critical Rules for Editing Templates

1. **Never edit repo-root `.claude/`, `.opencode/`, `.github/`, `.ito/` directly** — those are outputs of `ito init`. Edit the source templates here.
2. **When adding or modifying a command/prompt**, update ALL harness versions for feature parity (each has its own frontmatter format).
3. **Skills are shared** from `assets/skills/` and don't need per-harness maintenance.
4. **OpenCode uses plural directory names**: `.opencode/skills/`, `.opencode/commands/`, `.opencode/plugins/`.

## Verifying Changes

```bash
make install
ito init --force --tools all
```

Then inspect installed files to confirm they match expectations.

Use the `documentation-police` subagent to verify that any new templates include adequate documentation. Use the `rust-quality-checker` subagent for changes to the Rust source code in this crate.

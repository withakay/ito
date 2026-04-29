# ito-templates — Layer 1 (Domain)

Canonical source for files installed by `ito init` / `ito update`: project/home templates, shared skills, adapter bootstraps, commands, agents, and instruction templates.

For broader workspace guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architecture see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Keep every installed template in one crate. Assets are embedded with `include_dir!`, so `ito init` works without runtime filesystem reads for template content.

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

None — only external deps (`include_dir`, `minijinja`, `serde`).

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

1. **Edit template sources here, not installed outputs** such as repo-root `.claude/`, `.opencode/`, `.github/`, or `.ito/`.
2. **Keep harness variants equivalent** when changing commands or agent prompts; only frontmatter/tool syntax should differ.
3. **Skills live once** under `assets/skills/`; do not create per-harness copies unless the harness truly requires it.
4. **Do not compact change-proposal templates** named `spec.md`, `design.md`, `proposal.md`, or `tasks.md`.
5. **Preserve managed markers, placeholders, and code fences** in prompt assets.
6. **Keep helper assets together** inside each skill directory, and preserve executable bits for bundled scripts.

## Verifying Changes

```bash
make install
ito init --force --tools all
```

Then inspect installed outputs. Use `documentation-police` for template docs and `rust-quality-checker` for Rust code in this crate.

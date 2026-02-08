# Ito CLI Parity Matrix (TS oracle -> Rust candidate)

This matrix enumerates the current `ito` CLI surface that the Rust port must
match (flags, exit codes, error messages, JSON shapes, interactive behavior, and
filesystem writes).

Sources:

- CLI help output (TypeScript oracle): `ito --help` and `ito <cmd> --help`
- Specs (requirements guidance): `.ito/specs/*/spec.md` (notably: `cli-init`,
  `cli-update`, `cli-list`, `cli-show`, `cli-validate`, `cli-archive`,
  `cli-config`, `cli-completion`, `cli-artifact-workflow`, `cli-ralph`)

Legend:

- Mutates FS: writes or edits files
- Interactive: prompts/TTY-only behavior
- Parity tests: snapshot (text), json (structured), fs (tree/bytes), pty

| Command | Mutates FS | Interactive | JSON | Primary parity tests |
|---|---:|---:|---:|---|
| `ito --help` | no | no | no | snapshot |
| `ito --version` | no | no | no | snapshot |
| `ito init [path]` | yes | yes | no | fs + pty |
| `ito init --tools ...` | yes | no | no | fs |
| `ito init --force` | yes | maybe | no | fs |
| `ito update [path]` | yes | no | yes (`--json`) | fs + json |
| `ito list` | no | no | yes (`--json`) | snapshot + json |
| `ito list --specs` | no | no | yes (`--json`) | snapshot + json |
| `ito list --modules` | no | no | yes (`--json`) | snapshot + json |
| `ito dashboard` | no | yes (interactive UI) | no | snapshot (non-interactive fallback) + pty |
| `ito show [item]` | no | yes | yes | snapshot + json + pty |
| `ito show module <id>` | no | maybe | yes | snapshot + json |
| `ito validate [item]` | no | yes | yes | snapshot + json + pty |
| `ito validate --all/--changes/--specs/--modules` | no | no | yes | json + snapshot |
| `ito archive <change>` | yes | yes | no | fs + pty |
| `ito archive --skip-specs` | yes | no | no | fs |
| `ito config ...` | yes | maybe (editor) | yes (`list --json`) | snapshot + json + fs |
| `ito create module` | yes | yes | no | fs + pty |
| `ito create change` | yes | yes | no | fs + pty |
| `ito status --change <id>` | no | no | yes (`--json`) | snapshot + json |
| `ito instructions <artifact> --change <id>` | no | no | yes (`--json`) | snapshot + json |
| `ito x-templates` | no | no | yes (`--json`) | snapshot + json |
| `ito x-schemas` | no | no | yes (`--json`) | snapshot + json |
| `ito agent instruction ...` | no | no | yes (agent output) | json |
| `ito completions ...` | yes (install/uninstall) | yes | no | fs + pty |
| `ito ralph [prompt]` | yes (state) | yes | no | pty + fs |
| `ito split <change-id>` | yes | yes | no | pty + fs |
| `ito tasks ...` | yes (tasks.md edits) | yes | no | fs + pty |

## Installer Output Parity (Critical)

Commands `ito init` and `ito update` install and/or update tool instruction
files and marker-managed blocks. Byte-for-byte parity is required in
non-interactive mode.

Paths and notes (from specs and current repo conventions):

- Ito directory: `.ito/` (or user-selected ito dir if supported)
- OpenCode paths are plural: `.opencode/skills/`, `.opencode/commands/`, `.opencode/plugins/`
- GitHub Copilot prompts: `.github/prompts/*.prompt.md` (YAML frontmatter + `$ARGUMENTS`)
- Codex prompts: `$CODEX_HOME/prompts` or `~/.codex/prompts`

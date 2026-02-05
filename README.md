<p align="center">
  <a href="https://github.com/withakay/Ito">
    <picture>
      <source srcset="assets/logo-dark.svg" media="(prefers-color-scheme: dark)">
      <source srcset="assets/logo-light.svg" media="(prefers-color-scheme: light)">
      <img src="assets/logo-light.svg" alt="Ito logo" height="64">
    </picture>
  </a>
</p>

<p align="center">Project-centric spec + workflow system for long-running AI coding work.</p>

<p align="center">
  <a href="https://github.com/withakay/Ito/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/withakay/Ito/actions/workflows/ci.yml/badge.svg" /></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" /></a>
</p>

# Ito

Ito is a Spec Driven development tool that bring project-centric planning and an emphasis on **long-running, multi-agent tasks** to AI coding agents.

It's designed for the type of AI-assisted development where work spans multiple sessions, needs explicit verification criteria, and benefits from parallel subagents. The approach draws inspiration f[...]

## What You Get

- Project planning foundation: `PROJECT.md`, `ROADMAP.md`, `STATE.md` templates
- Research phase: parallel domain investigation + synthesis (`research/*`)
- Enhanced tasks format: waves, verification criteria, completion tracking (`tasks.md`)
- Agent configuration: per-tool models + context budgets (`config.json`)
- Workflow orchestration: YAML workflows with waves + checkpoints, plus execution status tracking
- Unified "research" and "adversarial review" workflows available as slash commands in supported tools
- Ito agent skills installed automatically during init

## Quick Start

### Prerequisites

- Rust toolchain (rustup + cargo)

### Install

**Homebrew (macOS):**

```bash
brew tap withakay/ito
brew install ito
```

**Build from source:**

```bash
make rust-install
ito --version
```

**Prebuilt binary (macOS/Linux):**

```bash
curl -fsSL https://raw.githubusercontent.com/withakay/ito/main/scripts/install.sh | sh
ito --version
```

**npm (optional):**

```bash
npm install -g @withakay/ito
ito --version
```

### Initialize In A Repo

```bash
ito init
```

This creates Ito's working directory (default: `.ito/`), installs Ito agent skills, and generates slash commands for the selected supported tools.

Note: older docs (and some templates) may refer to `ito/` as the working directory. In this fork, the default is `.ito/`, and the directory name can be customized via `ito.json`.

Ito agent skills are installed to `.claude/skills/<skill>/SKILL.md` so supported assistants can load the authoritative instructions.

## On-Disk Layout

After `ito init`, you'll typically have (default layout shown):

```text
.ito/
  AGENTS.md
  project.md
  planning/
    PROJECT.md
    ROADMAP.md
    STATE.md
  research/
    SUMMARY.md
    investigations/
      stack-analysis.md
      feature-landscape.md
      architecture.md
      pitfalls.md
  changes/
    <change-id>/
      proposal.md
      design.md
      tasks.md
      specs/
      reviews/
  workflows/
    research.yaml
    execute.yaml
    review.yaml
    .state/
      <workflow>.json
  commands/
    <prompt-templates>.md
  config.json
```

## Core Workflows

### 1) Project Planning (`ito plan`)

Project planning lives in `.ito/planning/` and is intended to survive across sessions.

```bash
ito plan init
ito plan status
ito state show
```

- `PROJECT.md`: project vision, constraints, conventions
- `ROADMAP.md`: phases/milestones
- `STATE.md`: current focus, decisions, blockers, session notes

### 2) Research Phase (`/ito … research`)

Research is meant to happen *before* proposing changes, especially when you're entering an unfamiliar domain.

The built-in research workflow runs in parallel:

- stack analysis
- feature landscape
- architecture
- pitfalls

…and then synthesizes results into `.ito/research/SUMMARY.md`.

### 3) Change Execution With Enhanced Tasks (`ito tasks`)

Ito supports an "enhanced tasks.md" format that is optimized for long-running work:

- waves (grouping and parallelizable chunks)
- explicit `Verify` commands
- `Done When` acceptance criteria
- task status tracking (pending / in-progress / complete)

Ito also supports a lightweight checkbox-only `tasks.md` format in a compatibility mode:

- `- [ ]` pending
- `- [~]` in-progress (current task)
- `- [x]` complete

```bash
ito tasks init <change-id>
ito tasks status <change-id>
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
ito tasks next <change-id>
```

### 4) Adversarial Review (`/ito … review`)

Adversarial review is multi-perspective by default:

- security review (vulnerabilities, attack vectors)
- scale review (perf bottlenecks)
- edge case review (failure modes, boundaries)

Outputs are written into the change folder under `reviews/`.

### 5) Workflow Orchestration (`ito workflow`)

Workflows are YAML files with waves, tasks, and optional checkpoints.

```bash
ito workflow init
ito workflow list
ito workflow show research
ito workflow run research --tool opencode -v topic="your topic"
ito workflow status research
```

This generates tool-specific execution instructions (OpenCode / Claude Code / Codex) and tracks progress in `.ito/workflows/.state/`.

## Agent Configuration (`ito agent-config`)

Ito can generate and manage `<ito-dir>/config.json` for per-tool model selection and context budgets.

```bash
ito agent-config init
ito agent-config summary
ito agent-config get tools.opencode.default_model
ito agent-config set agents.review.model_preference powerful
```

## Slash Commands (Where Supported)

Ito installs slash commands for tools that support them.

- Claude Code (namespace style): `/ito:proposal`, `/ito:apply`, `/ito:archive`, `/ito:research`, `/ito:review`
- OpenCode / Codex (hyphen style): `/ito-proposal`, `/ito-apply`, `/ito-archive`, `/ito-research`, `/ito-review`

Exact availability depends on which tools you selected during `ito init`.

## Command Reference (Common)

```bash
ito init
ito update
ito list
ito list --specs
ito show <change-or-spec>
ito validate [item]
ito archive <change-id> -y

# Local docs server (requires Caddy)
ito serve start
ito serve status
ito serve stop
```

## Local Docs Server (`ito serve`)

Ito can serve a local, per-project docs browser over HTTP to make reviewing `.ito/` artifacts and project docs easy.

### Prerequisite: Caddy

`ito serve start` requires the external `caddy` binary.

If `caddy` is missing, Ito prints an install hint and exits with code 1.

### What Gets Served

Only these allowlisted roots are exposed:

- `.ito/changes/`, `.ito/specs/`, `.ito/modules/`
- `.ito/planning/` and `.ito/research/` (if present)
- `docs/` and `documents/` (if present)

### Configuration

Configure in `.ito/config.json` under `serve`:

```json
{
  "serve": {
    "bind": "127.0.0.1",
    "port": 9009,
    "token": "optional-only-used-for-non-loopback",
    "caddyBin": "optional-path-to-caddy"
  }
}
```

- Default bind/port: `127.0.0.1:9009`
- If the port is busy, Ito auto-increments to the next available port.
- If `serve.bind` is non-loopback, Ito serves under a tokenized path prefix (`/t/<token>/`) and rejects requests without it.

## Test Plan

- [ ] Run `ito init` and verify `.ito/planning/` + `.ito/research/` templates exist
- [ ] Run `ito workflow init` and verify `.ito/workflows/*.yaml` are created
- [ ] Verify research and review slash commands are available in at least one supported tool
- [ ] Run `make build` to verify the Rust CLI builds

## Pre-commit Hooks (prek)

Ito uses [prek](https://prek.j178.dev/) for pre-commit hooks. prek is a drop-in replacement for pre-commit and runs the same `.pre-commit-config.yaml`.

### Quick Start

```bash
# Run hooks on staged files
prek run

# Run hooks on all files
prek run --all-files

# Install git hooks (runs automatically on commit)
prek install
```

### Already using pre-commit?

prek is fully compatible. Just replace `pre-commit` with `prek` in your workflow and reinstall hooks once:

```bash
prek install -f
```

### What the hooks check

- **Whitespace / line endings**: trailing whitespace, EOF newline, mixed line endings
- **Structured formats**: JSON syntax, YAML syntax + lint
- **Markdown**: linting via markdownlint-cli2
- **Rust**: `cargo fmt` + `cargo clippy` with the repo's lint policy

## Contributing

```bash
make build
make test
make lint
```

## License

MIT

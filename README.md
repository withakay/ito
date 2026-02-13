

<p align="center">Project-centric spec + workflow system for long-running AI coding work.</p>

<p align="center">
  <a href="https://github.com/withakay/Ito/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/withakay/Ito/actions/workflows/ci.yml/badge.svg" /></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" /></a>
</p>

# Ito

- Thread/String (糸): Used for sewing thread, yarn, or in a metaphorical sense for connections.
- Intention/Aim (意図): Often used in the context of plans, aims, or intent.

Ito is a Change Driven development tool that brings together project-centric planning, design, specifications and tasks with an emphasis on **long-running, multi-agent tasks** to AI coding agents.

It's designed for the type of AI-assisted development where work spans multiple sessions, needs explicit verification criteria, and benefits from parallel subagents. The approach draws inspiration from software development best practices, Easy Approach to Requirements Syntax and RFCs adapted for the challenges of AI-assisted coding.

Ito is not a project management tool, but rather a lightweight, flexible framework for structuring and orchestrating the work itself. It provides templates, conventions, and automation to help you plan, execute, and review changes in a way that tries to align with AI agents abilites. It tries to strike a balance between structure and flexibility, providing enough scaffolding to be useful without stiffling the ag.

## What You Get

- Project planning foundation: `PROJECT.md`, `ROADMAP.md`, `STATE.md` templates
- Research phase: parallel domain investigation + synthesis (`research/*`)
- Enhanced tasks format: waves, verification criteria, completion tracking (`tasks.md`)
- Agent configuration: per-tool models + context budgets (`config.json`)
- Workflow orchestration: YAML workflows with waves + checkpoints, plus execution status tracking
- Unified "research" and "adversarial review" workflows available as slash commands in supported tools
- Ito agent skills installed automatically during repositiroy inititialisaton
- Built in Ralph loop - create a change proposal and then use `ito ralph` to execute it in a loop with the agent, tracking progress and updating the proposal as you go.
- Docs server *:
  - `ito serve` to browse `.ito/` artifacts and project docs over HTTP.
  - Integrates with Tailscale for secure remote access, allows for a remote terminal based workflow in a cheap VM with easy access to documents in the browser.
  - Embedded terminal in the docs UI for quick commands.

*It is pretty easy to accidentally expose sensitive info with the docs server, so it is opt-in and requires explicit configuration. Use with caution and consider running in a separate VM if you want remote access.

## Quick Start

### Prerequisites

- Rust toolchain (rustup + cargo)

### Recommended Developer Tools

- `bacon` for background Rust checking and quick job switching:

```bash
cargo install --locked bacon
```

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

### 5) Workflow Instructions (`ito agent instruction`)

Ito workflow execution is instruction- and skill-driven.

```bash
ito agent instruction proposal --change <change-id>
ito agent instruction specs --change <change-id>
ito agent instruction tasks --change <change-id>
ito agent instruction apply --change <change-id>
ito agent instruction review --change <change-id>
ito agent instruction archive --change <change-id>
```

Use `ito tasks` commands to track and execute task progress.

## Agent Configuration (`ito agent-config`)

Ito can generate and manage `<ito-dir>/config.json` for per-tool model selection and context budgets.

```bash
ito agent-config init
ito agent-config summary
ito agent-config get tools.opencode.default_model
ito agent-config set agents.review.model_preference powerful
```

Generated `.ito/config.json` includes a `$schema` reference to the versioned schema artifact in this repository:

- `https://raw.githubusercontent.com/withakay/ito/v<version>/schemas/ito-config.schema.json`

Keep the committed schema artifact up-to-date with:

```bash
make config-schema
make config-schema-check
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
- [ ] Run `ito agent instruction apply --change <change-id>` and verify task guidance is generated
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

# Run architecture guardrails directly
make arch-guardrails
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
- **Architecture**: `make arch-guardrails` for crate-edge and domain API guardrails

## Contributing

```bash
make build
make test
make lint
make docs-site-check
```

### Docs Site

Build and serve the MkDocs site (includes Rust API docs generated via `mkdocs-rustdoc-plugin`):

```bash
make docs-site-build
make docs-site-serve
make bacon
```

## License

MIT

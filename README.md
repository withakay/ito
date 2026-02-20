<p align="center">Project-centric spec + workflow system for long-running AI coding work.</p>

<p align="center">
  <a href="https://github.com/withakay/Ito/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/withakay/Ito/actions/workflows/ci.yml/badge.svg" /></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" /></a>
  <a href="https://withakay.github.io/ito"><img alt="Docs" src="https://img.shields.io/badge/Docs-withakay.github.io%2Fito-2d7ff9.svg?style=flat-square" /></a>
</p>

# Ito

- Thread/String (糸): Used for sewing thread, yarn, or in a metaphorical sense for connections.
- Intention/Aim (意図): Often used in the context of plans, aims, or intent.

Ito is a Change Driven development tool that brings together project-centric planning, design, specifications and tasks with an emphasis on **long-running, multi-agent tasks** to AI coding agents.

It's designed for the type of AI-assisted development where work spans multiple sessions, needs explicit verification criteria, and benefits from parallel subagents. The approach draws inspiration from software development best practices, Easy Approach to Requirements Syntax and RFCs adapted for the challenges of AI-assisted coding.

Ito is not a project management tool, but rather a lightweight, flexible framework for structuring and orchestrating the work itself. It provides templates, conventions, and automation to help you plan, execute, and review changes in a way that tries to align with AI agents abilites. It tries to strike a balance between structure and flexibility, providing enough scaffolding to be useful without stiffling the ag.

## What You Get

Ito centers work around a small set of versioned artifacts under `.ito/`.

- Change lifecycle: proposal artifacts you iterate on, then implement, then archive into the long-term spec.
- Specs and deltas: stable capability specs under `.ito/specs/`, with per-change deltas under `.ito/changes/<change-id>/specs/`.
- Tasks: a structured `tasks.md` per change, with CLI support for status/next/start/complete.
- Modules: optional grouping of related changes with validation of scope and naming.
- Validation: checks that changes/modules/specs follow conventions and are internally consistent.
- Agent-facing instructions: generated instruction artifacts (`ito agent instruction ...`) and tool adapters installed by `ito init` / `ito update`.
- Optional project planning: templates and commands for `.ito/planning/{PROJECT,ROADMAP,STATE}.md` (`ito plan ...`, `ito state ...`).
- Optional local docs server: browse `.ito/` artifacts over HTTP (`ito serve ...`, requires `caddy`).

## Core Workflow

The intended workflow is:

1. Create a change.
2. Write/iterate on the proposal: the “why”, design notes (if needed), spec deltas, and tasks.
3. Validate the change while you iterate.
4. Implement the tasks.
5. Archive the change to merge approved deltas into the main specs.

At each step, the existing specs are the baseline. Changes are expressed as deltas, reviewed, then merged into `.ito/specs/` when archived.

## Quick Start

### Install

**Homebrew (macOS):**

```bash
brew tap withakay/ito
brew install ito-cli
```

Note: the Homebrew formula name is `ito-cli` (it installs the `ito` binary).


**Prebuilt binaries (macOS/Linux):**

```bash
curl -fsSL https://raw.githubusercontent.com/withakay/ito/main/scripts/install.sh | sh
ito --version
```

**Prebuilt binaries (Windows):**

Windows builds are published.

- Manual: download the latest Windows release from [GitHub Releases](https://github.com/withakay/ito/releases) and put `ito.exe` on your `PATH`.
- PowerShell: download and run `scripts/install.ps1` (supports `-AddToPath`).

**Build from source:**

```bash
make rust-install
ito --version
```

### Initialize In A Repo

```bash
ito init
```

This creates Ito's working directory (default: `.ito/`) and installs tool-specific adapters (skills, prompts, and instruction wiring) for the tools you select.

## Common Commands

```bash
ito create module <name>
ito create change <slug> --module <module-id>

ito list
ito list --specs
ito list --modules

ito show <change-or-spec>
ito validate <change-or-spec> --strict

ito status --change <change-id>

ito tasks status <change-id>
ito tasks next <change-id>

ito agent instruction proposal --change <change-id>
ito agent instruction apply --change <change-id>
ito agent instruction archive --change <change-id>

ito archive <change-id> -y
```

## On-Disk Layout

```text
.ito/
  project.md
  specs/                  # Current truth (capabilities)
    <capability>/
      spec.md
      design.md           # Optional
  changes/                # Proposals (deltas + tasks)
    <change-id>/
      proposal.md
      design.md           # Optional
      tasks.md
      specs/
        <capability>/
          spec.md         # Delta: ADDED/MODIFIED/REMOVED/RENAMED
  modules/                # Optional grouping of changes
    <NNN_module-name>/
      module.md
  planning/               # Optional project planning artifacts
    PROJECT.md
    ROADMAP.md
    STATE.md
```

## Contributing

```bash
make build
make test
make lint
make docs-site-check
```

### Docs Site

Build and serve the Zensical docs site (dark theme + Rust API docs). Python dependencies are isolated via `uv` and `docs/pyproject.toml`:

```bash
make docs-site-build
make docs-site-serve
```

## License

MIT

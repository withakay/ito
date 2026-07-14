# Developer Quick Start

This guide gets contributors set up to build and work on Ito locally.

## Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- `uv` for docs tooling and isolated Python environments
- `prek` for hooks

## 1) Initialize developer environment

```bash
make init
```

## 2) Build Ito CLI

```bash
make build
```

## 3) Run Ito in your repository

```bash
ito init
ito list
```

## 4) Create your first change

```bash
ito create change "my-first-change"
ito agent instruction proposal --change <change-id>
ito validate <change-id> --strict
```

Review and merge the proposal-only package into main (through a PR by default). Then start implementation from the accepted proposal:

```bash
ito change preflight <change-id> --for prepare --refresh
CHANGE_DIR=$(ito worktree ensure --change <change-id>)
cd "$CHANGE_DIR"
ito agent instruction apply --change <change-id>
```

## 5) Build docs site locally

```bash
make docs-site-build
make docs-site-serve
```

## Next Steps

- Read [Agent Workflow](agent-workflow.md)
- Read [Config](config.md)
- Browse generated [Rust API reference](api/rustdoc.md)

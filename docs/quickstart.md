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

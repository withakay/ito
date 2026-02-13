# Quick Start

This guide gets you to a working Ito setup quickly.

## Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- Python 3
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

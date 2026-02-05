# Rust Workspace / Crate Architecture

## Goals

- Keep the CLI crate thin.
- Keep core logic testable by isolating side effects behind traits.
- Embed templates and keep installer outputs deterministic.

## Proposed Workspace Layout

Create a Cargo workspace under `ito-rs/`:

```text
ito-rs/
  Cargo.toml
  crates/
    ito-cli/           # binary crate; clap; wiring
    ito-core/          # domain logic; models; operations
    ito-fs/            # fs abstraction; marker-managed edits
    ito-templates/     # embedded templates + rendering
    ito-schemas/       # schema parsing, graphs, delta apply blocks
    ito-workflow/      # workflow yaml + state
    ito-harness/       # ralph harness invocation
    ito-test-support/  # shared test helpers (oracle/candidate runner, PTY)
```

## Trait Boundaries

- Filesystem: read/write, atomic writes, globbing
- Process execution: spawn oracle/candidate, harness runners
- Terminal: isatty, width, color choice
- Clock: timestamps for archive naming and status output

Design guideline:

- Prefer passing an `Env` + `Io` object into operations rather than using
  `std::env` and global IO directly.

## Why

The Rust port requires strict behavioral parity with the existing TypeScript CLI; without an explicit parity strategy and CLI UX research, the port risks drifting in flags, exit codes, error messages, and installer outputs.

## What Changes

- Produce a command-by-command parity matrix for the TypeScript `ito` CLI.
- Define a parity testing harness strategy (oracle TS vs candidate Rust) including PTY-driven tests for interactive flows.
- Document Rust crate/workspace architecture and packaging/distribution approach.
- Collect decisions and constraints from existing Ito specs to ground the port plan.

## Capabilities

### New Capabilities

- `rust-port-research`: Research artifacts and parity strategy for the Rust port.

### Modified Capabilities

<!-- None. New documentation-only capability. -->

## Impact

- Adds research documentation under `.ito/research/`.
- Establishes constraints that later implementation changes must follow (parity, UX, installers).

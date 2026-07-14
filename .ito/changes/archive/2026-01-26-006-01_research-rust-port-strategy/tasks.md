# Tasks for: 006-01_research-rust-port-strategy

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: Parity Matrix

### Task 1.1: Produce a CLI parity matrix

- **Files**: `.ito/research/SUMMARY.md`
- **Dependencies**: None
- **Action**:
  - Enumerate all current TS `ito` commands/flags and expected outputs
  - Include `--json` shapes and known exit codes
- **Verify**: Review matrix completeness against `ito --help`
- **Done When**: matrix covers all commands listed by TypeScript CLI
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Research Investigations

### Task 2.1: Document Rust CLI UX approach

- **Files**: `.ito/research/investigations/rust-cli-ux.md`
- **Dependencies**: Task 1.1
- **Action**:
  - Decide on crates/approach for TTY, prompts, spinners, NO_COLOR, JSON/text output
- **Verify**: cross-check with existing specs under `.ito/specs/`
- **Done When**: decisions recorded with alternatives and constraints
- **Status**: \[x\] complete

### Task 2.2: Document parity testing strategy

- **Files**: `.ito/research/investigations/parity-testing.md`
- **Dependencies**: Task 2.1
- **Action**:
  - Define golden/snapshot approach and PTY strategy for interactive flows
  - Define filesystem tree comparison for installers
- **Verify**: includes deterministic guidelines
- **Done When**: harness requirements clear enough to implement
- **Status**: \[x\] complete

### Task 2.3: Document crate/workspace architecture

- **Files**: `.ito/research/investigations/rust-crate-architecture.md`
- **Dependencies**: Task 2.2
- **Action**:
  - Define workspace crate boundaries, side-effect isolation, and layering
- **Verify**: aligns to the Cargo blueprint described in the port prompt
- **Done When**: crate split and responsibilities are explicit
- **Status**: \[x\] complete

### Task 2.4: Document packaging and transition plan

- **Files**: `.ito/research/investigations/packaging-distribution.md`
- **Dependencies**: Task 2.3
- **Action**:
  - Describe how Rust `ito` replaces or coexists with npm `@withakay/ito`
- **Verify**: includes release/compat strategy
- **Done When**: plan is actionable for later change
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3: Validate Artifacts

### Task 3.1: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run strict validation and fix any issues
- **Verify**: `ito validate 006-01_research-rust-port-strategy --strict`
- **Done When**: validation passes
- **Status**: \[x\] complete

## Verify

```bash
ito validate 006-01_research-rust-port-strategy --strict
```

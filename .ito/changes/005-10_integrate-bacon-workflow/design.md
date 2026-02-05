# Design: Integrate Bacon into Development Workflow

## Overview

Bacon provides continuous background compilation feedback. This design integrates it for both human developers and AI agents.

## Configuration

### bacon.toml Location

Place in `ito-rs/bacon.toml` (next to Cargo.toml) so bacon auto-discovers it.

### Jobs Configuration

```toml
# bacon.toml - Ito development configuration

[jobs.default]
command = ["cargo", "check", "--workspace", "--all-targets", "--color", "always"]
need_stdout = false

[jobs.check]
command = ["cargo", "check", "--workspace", "--all-targets", "--color", "always"]
need_stdout = false

[jobs.clippy]
command = ["cargo", "clippy", "--workspace", "--all-targets", "--color", "always"]
need_stdout = false

[jobs.test]
command = ["cargo", "test", "--workspace", "--color", "always"]
need_stdout = true
on_success = "job:check"  # Return to check after tests pass

[jobs.test-unit]
command = ["cargo", "test", "--workspace", "--lib", "--color", "always"]
need_stdout = true

[jobs.doc]
command = ["cargo", "doc", "--workspace", "--no-deps", "--color", "always"]
need_stdout = false

[jobs.coverage]
command = ["cargo", "llvm-cov", "--workspace", "--color", "always"]
need_stdout = true
allow_failure = true

# Keybindings
[keybindings]
c = "job:clippy"
t = "job:test"
u = "job:test-unit"
d = "job:doc"
v = "job:coverage"
```

## Agent Integration

### Export Locations

Bacon can export error locations to a file with `--export-locations`:

```bash
bacon --export-locations .bacon-locations
```

This creates a file with format:
```
src/foo.rs:42:5
src/bar.rs:100:10
```

### Agent Workflow

1. **Background Mode**: Run bacon in a terminal/tmux session
2. **Error Polling**: Agent reads `.bacon-locations` to find current errors
3. **Focused Fixes**: Agent can target specific error locations
4. **Re-check**: Bacon automatically re-checks after agent saves changes

### AGENTS.md Guidance

Add section to AGENTS.md:

```markdown
## Using Bacon for Development

Bacon runs in the background watching for file changes. If available:

1. Check `.bacon-locations` for current error locations
2. Read the specific files/lines with errors
3. Fix the errors
4. Bacon will automatically re-check on save

To start bacon with location export:
```bash
cd ito-rs && bacon --export-locations .bacon-locations
```
```

## Makefile Integration

Add bacon target to Makefile:

```makefile
.PHONY: bacon
bacon:
	cd ito-rs && bacon

.PHONY: bacon-export
bacon-export:
	cd ito-rs && bacon --export-locations .bacon-locations
```

## Installation

Add to developer setup docs:

```bash
cargo install --locked bacon
```

## Alternatives Considered

### cargo-watch

- Simpler but less interactive
- No built-in TUI
- Bacon has better job switching and keybindings

### rust-analyzer

- IDE integration only
- Not suitable for terminal-based workflows
- Agents may not have access to LSP

Bacon is chosen for its terminal-first design and structured output options.

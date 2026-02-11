# Tasks: Integrate Bacon into Development Workflow

## Configuration

- [x] Create `ito-rs/bacon.toml` with standard jobs (check, clippy, test, doc, coverage)
- [x] Add keybindings for quick job switching (c=clippy, t=test, etc.)
- [x] Test bacon configuration works with `bacon` command

## Makefile Integration

- [x] Add `bacon` target to Makefile
- [x] Add `bacon-export` target for agent-friendly mode
- [x] Verify targets work from repo root

## Documentation

- [x] Add bacon to recommended tools in README.md or CONTRIBUTING.md
- [x] Add bacon usage section to AGENTS.md for AI assistants
- [x] Document `--export-locations` usage for agent workflows

## Git Integration

- [x] Add `.bacon-locations` to `.gitignore`
- [x] Add `.bacon` directory to `.gitignore` (if bacon creates one)

## Validation

- [x] Verify `bacon` starts and watches files correctly
- [x] Verify job switching works (press 'c' for clippy, 't' for test)
- [x] Verify `--export-locations` produces parseable output
- [x] Test agent workflow: error detection -> fix -> auto-recheck

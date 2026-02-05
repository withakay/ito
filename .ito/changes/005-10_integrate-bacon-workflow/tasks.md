# Tasks: Integrate Bacon into Development Workflow

## Configuration

- [ ] Create `ito-rs/bacon.toml` with standard jobs (check, clippy, test, doc, coverage)
- [ ] Add keybindings for quick job switching (c=clippy, t=test, etc.)
- [ ] Test bacon configuration works with `bacon` command

## Makefile Integration

- [ ] Add `bacon` target to Makefile
- [ ] Add `bacon-export` target for agent-friendly mode
- [ ] Verify targets work from repo root

## Documentation

- [ ] Add bacon to recommended tools in README.md or CONTRIBUTING.md
- [ ] Add bacon usage section to AGENTS.md for AI assistants
- [ ] Document `--export-locations` usage for agent workflows

## Git Integration

- [ ] Add `.bacon-locations` to `.gitignore`
- [ ] Add `.bacon` directory to `.gitignore` (if bacon creates one)

## Validation

- [ ] Verify `bacon` starts and watches files correctly
- [ ] Verify job switching works (press 'c' for clippy, 't' for test)
- [ ] Verify `--export-locations` produces parseable output
- [ ] Test agent workflow: error detection -> fix -> auto-recheck

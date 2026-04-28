## Execution Notes

- This change extends the repository-backed workflow introduced by prior `025_*` work and should reuse runtime-selected services rather than introducing command-local persistence logic.
- Treat instruction rendering and installed harness assets as part of the acceptance surface. A behavior-only implementation is incomplete.
- Keep task lifecycle commands semantic (`ito tasks ...`) even while broadening the artifact mutation surface.

## 1. Proposal and interface definition
- [ ] 1.1 Finalize the artifact target model and CLI command shape for `ito patch` and `ito write`
- [ ] 1.2 Add spec deltas covering runtime-selected artifact mutation services and generated instruction behavior
- [ ] 1.3 Validate the change proposal with `ito validate 025-11_repository-backed-artifact-mutations --strict`

## 2. Mutation services and runtime composition
- [ ] 2.1 Add domain/core artifact mutation abstractions and result/error types
- [ ] 2.2 Wire filesystem and SQLite implementations into the repository runtime
- [ ] 2.3 Extend the remote/backend client runtime to support artifact mutation operations

## 3. CLI surfaces and validation
- [ ] 3.1 Add `ito patch ...` and `ito write ...` command parsing and adapter behavior
- [ ] 3.2 Validate artifact refs, patch/write inputs, and error reporting for ambiguous or unsupported targets
- [ ] 3.3 Ensure command behavior remains repository-runtime-agnostic across modes

## 4. Instruction and harness guidance updates
- [ ] 4.1 Update generated instruction artifacts to route active Ito artifact mutations through `ito patch` / `ito write`
- [ ] 4.2 Update installed harness assets and related guidance surfaces so they teach the same workflow consistently
- [ ] 4.3 Preserve the distinction between code-file edits and Ito artifact mutations in guidance text

## 5. Verification coverage
- [ ] 5.1 Add targeted tests for filesystem, SQLite, and remote mutation behavior
- [ ] 5.2 Add CLI tests for patch/write success and failure cases
- [ ] 5.3 Add instruction/template tests proving the correct workflow is emitted everywhere relevant
- [ ] 5.4 Run `make test` and `make check` and resolve all failures

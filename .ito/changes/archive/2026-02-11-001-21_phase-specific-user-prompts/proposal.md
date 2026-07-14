## Why

Today instruction guidance is all-or-nothing in `.ito/user-guidance.md`, which makes it hard to provide different direction for proposal writing versus implementation execution. Users need phase-scoped prompt files so guidance can be precise without constant manual editing.

## What Changes

- Add support for phase-specific guidance files under a new `.ito/user-prompts/` directory.
- Define artifact-scoped guidance lookup using `.ito/user-prompts/<artifact-id>.md` (for example `proposal.md`, `apply.md`).
- Support shared guidance at `.ito/user-prompts/guidance.md`, while keeping `.ito/user-guidance.md` as backward-compatible fallback.
- Update instruction generation behavior to inject scoped guidance for the current artifact plus shared guidance.
- Document precedence and additive behavior so schema instructions remain authoritative.

## Capabilities

### New Capabilities

- `phase-specific-user-prompts`: Artifact-scoped user prompt file support and lookup conventions.

### Modified Capabilities

- `user-guidance-file`: Extend guidance file model to include `.ito/user-prompts/` artifacts.
- `instruction-guidance-injection`: Inject guidance based on artifact scope plus shared fallback.

## Impact

- Affected specs: `user-guidance-file`, `instruction-guidance-injection`, and new `phase-specific-user-prompts` spec.
- Likely affected code: instruction loading and init/update template installation paths in Rust implementation.
- UX impact: users can keep stable proposal/apply guidance without editing one monolithic file.

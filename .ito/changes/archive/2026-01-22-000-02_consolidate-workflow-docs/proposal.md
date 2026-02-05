## Why

The documentation is split between aspirational design documents (`docs/experimental-workflow.md`, `docs/experimental-release-plan.md`) and the actual implemented workflow (the ito-\* skills). This creates confusion about what Ito can actually do today vs. what was planned. Users need a single authoritative document describing the real workflow, and good ideas that haven't been implemented should be preserved separately for future consideration.

## What Changes

- **BREAKING**: Remove `docs/experimental-workflow.md` (replaced by new workflow documentation)
- **BREAKING**: Remove `docs/experimental-release-plan.md` (implementation plan is complete/obsolete)
- Add `docs/agent-workflow.md` documenting the actual implemented Ito workflow as used via agents
- Add `docs/future-ideas.md` capturing unimplemented concepts worth exploring later

## Capabilities

### New Capabilities

- `agent-workflow-docs`: Create comprehensive documentation of the actual Ito workflow as implemented in the ito-\* skills (proposal, research, apply, review, archive). This documents the real "actions on a change" model that agents use.
- `future-ideas-docs`: Create a document capturing unimplemented but valuable ideas from the experimental docs (custom schemas, additional CLI commands, the full OPSX fluid workflow model) for future consideration.

### Modified Capabilities

_None - this is purely a documentation change with no spec-level behavior modifications._

## Impact

- **Documentation**: Two experimental docs removed, two new docs added
- **User experience**: Clearer understanding of what Ito does today vs. future possibilities
- **Code**: No code changes required
- **Existing users**: Anyone referencing the experimental docs will need to use the new docs instead

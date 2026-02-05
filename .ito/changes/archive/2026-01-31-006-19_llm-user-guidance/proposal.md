## Why

Ito’s change proposal workflow relies on prompt/instruction text that is necessarily generic. Different teams and users often want strong, repeatable guidance such as:

- TDD/BDD/DDD preferences and structure
- Additional research rigor (sources, trade-offs, validation)
- Implementation conventions (commit cadence, test strategy)

Today there is no first-class, stable place for a user to add this guidance such that it reliably influences the “proposal generation” flow. As a result, guidance gets duplicated across harness prompt files or forgotten between sessions.

## What Changes

Introduce a user-owned, project-local Markdown file that Ito will (a) create during `ito init`, (b) never overwrite user edits, and (c) automatically append into Ito’s CLI-generated instruction artifacts.

This enables users to configure “how the LLM should behave” without forking template prompts or editing embedded instructions.

## Capabilities

### New Capabilities

- `user-guidance-file`: Add a stable, user-editable guidance file (created by init, preserved by update).
- `instruction-guidance-injection`: Include the guidance content in `ito agent instruction <artifact>` outputs so it is applied consistently across harnesses.

### Modified Capabilities

- `<existing-name>`: <what requirement is changing>

## Impact

- CLI/workflow output: `ito agent instruction ...` will optionally include extra guidance.
- Templates/installers: project templates will ship a new guidance file and preserve user edits.
- Testing: add unit tests for guidance injection and installer behavior.

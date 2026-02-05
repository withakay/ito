## MODIFIED Requirements

### Requirement: Invalid results SHALL include a Next steps footer in human-readable output

The CLI SHALL append a Next steps footer when the item is invalid and not using `--json`, including:

- Summary line with counts
- Top-3 guidance bullets (contextual to the most frequent or blocking errors)
- A suggestion to re-run with `--json` and/or the debug command

#### Scenario: Change invalid summary

- **WHEN** a change validation fails
- **THEN** print "Next steps" with 2-3 targeted bullets and suggest `ito show <id> --json --deltas-only`

### Requirement: Item type detection and ambiguity handling

The validate command SHALL handle ambiguous names and explicit type overrides to ensure clear, deterministic behavior.

#### Scenario: Ambiguity between change and spec names

- **GIVEN** `<item-name>` exists both as a change and as a spec
- **WHEN** executing `ito validate <item-name>`
- **THEN** print an ambiguity error explaining both matches
- **AND** suggest passing `--type change` or `--type spec`
- **AND** exit with code 1 without performing validation

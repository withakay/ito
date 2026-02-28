## MODIFIED Requirements

### Requirement: Command outputs match TypeScript

Rust MUST match TypeScript stdout/stderr/exit codes for planning commands.

#### Scenario: `tasks` output parity

- GIVEN a change with tasks
- WHEN the user runs `ito tasks --change <id>`
- THEN Rust output matches TypeScript

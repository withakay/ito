## MODIFIED Requirements

### Requirement: Instruction Argument Reconstruction

The CLI SHALL reconstruct raw argument vectors from parsed `AgentInstructionArgs` using exhaustive struct destructuring so that adding a new field to the struct produces a compile-time error if the reconstruction is not updated.

#### Scenario: New field added to AgentInstructionArgs
- **WHEN** a developer adds a new field to `AgentInstructionArgs`
- **THEN** the `to_argv()` method fails to compile until the new field is handled

#### Scenario: Instruction forwarding round-trips all flags
- **WHEN** a clap-parsed `AgentInstructionArgs` is forwarded to the string-based handler
- **THEN** all fields present in the struct are included in the reconstructed argument vector

## ADDED Requirements

### Requirement: ito-schemas contains only on-disk serde models

The `ito-schemas` crate MUST contain serde models for Ito's on-disk formats.

`ito-schemas` MUST NOT contain filesystem access or process execution.

#### Scenario: Schemas crate has crate-level documentation

- **WHEN** inspecting `ito-rs/crates/ito-schemas/src/lib.rs`
- **THEN** it MUST contain crate-level documentation describing it as serde models for on-disk formats

#### Scenario: Schemas crate has no filesystem access

- **WHEN** searching `ito-rs/crates/ito-schemas/` source code
- **THEN** it MUST NOT reference `std::fs`

#### Scenario: Schemas crate has no process execution

- **WHEN** searching `ito-rs/crates/ito-schemas/` source code
- **THEN** it MUST NOT reference `std::process::Command`

### Requirement: Schema types may be used pragmatically

Schema types MAY be used directly in domain or core code when they are pure data and align with the domain concept.

When a schema diverges from the domain concept (legacy fields, format-driven naming, or rule-heavy behavior), the domain MUST define a domain type and map at the boundary.

#### Scenario: Divergent schema is mapped at the boundary

- **GIVEN** an on-disk schema contains legacy fields or format-driven structure
- **WHEN** the data is used for business logic
- **THEN** the logic MUST operate on a domain type rather than the raw schema type

## ADDED Requirements

### Requirement: Arch guardrails reflect consolidated workspace crates

Architecture guardrails and dependency bans MUST reflect the consolidated Rust workspace crates.

At minimum:

- The guardrails tooling MUST NOT reference removed crates (`ito-schemas`, `ito-harness`, `ito-models`).
- `ito-cli` MUST NOT depend directly on `ito-domain`.
- Adapters (`ito-cli`, `ito-web`) MUST depend on `ito-core`.
- `ito-core` MUST depend on `ito-domain` and MAY depend on `ito-config`.
 - Adapters MAY depend on `ito-config`.

#### Scenario: Arch guardrails enforce consolidated boundaries

- **WHEN** running `make arch-guardrails`
- **THEN** it MUST fail if `ito-cli` depends on `ito-domain`

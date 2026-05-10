## ADDED Requirements

### Requirement: Generated agent instructions are persistence-mode aware

Generated agent instructions SHALL adapt their tooling guidance to the selected persistence mode.

#### Scenario: Remote mode guidance avoids markdown-edit recommendations

- **GIVEN** remote/API-backed persistence mode is active
- **WHEN** Ito generates instructions for an agent workflow
- **THEN** the guidance does not recommend editing active-work markdown files directly
- **AND** it recommends the repository-backed CLI interfaces that correspond to the requested workflow

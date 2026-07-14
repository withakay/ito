<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Pre-commit hook entry is opt-in for downstream projects
The pre-commit hook entry documented by Ito SHALL be opt-in for downstream projects and SHALL NOT be installed or edited automatically by `ito init`, `ito update`, or a managed helper skill. Projects MAY copy the documented entry into their chosen hook framework and SHALL verify it with `ito validate repo --staged --strict`.

#### Scenario: Ito init does not write hook configuration
- **WHEN** the user runs `ito init` on a fresh project
- **THEN** the command SHALL NOT modify `.pre-commit-config.yaml`, Husky scripts, or other hook framework files
- **AND** it does not delegate that edit to a retired helper skill

#### Scenario: Project adopts hook explicitly
- **WHEN** a project owner copies or authors an Ito validation hook entry
- **THEN** the edit is reviewed through the project's normal workflow
- **AND** direct `ito validate repo --staged --strict` verification remains available
<!-- ITO:END -->

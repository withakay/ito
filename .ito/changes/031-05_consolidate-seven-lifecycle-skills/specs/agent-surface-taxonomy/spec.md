<!-- ITO:START -->
## ADDED Requirements

### Requirement: Delegated agent roles do not expand the skill inventory
Ito MAY retain harness-native planner, researcher, worker, reviewer, or test-runner agent definitions for internal delegation, but those roles MUST NOT be installed as discoverable skills or counted as user-facing lifecycle entrypoints.

#### Scenario: Harness supports native delegated agents
- **WHEN** Ito installs delegated role definitions for a harness with a native agent location
- **THEN** the role is written only to that native agent location
- **AND** no corresponding role directory is written under the harness skill directory

#### Scenario: Harness models roles as skills only
- **WHEN** a harness has no native delegated-agent mechanism other than skill discovery
- **THEN** Ito does not install extra role skills by default
- **AND** retained lifecycle skills use inline/CLI instruction guidance or the harness's ordinary sub-agent facilities

#### Scenario: Agent and skill inventories are audited independently
- **WHEN** generated surfaces are verified
- **THEN** native agent definitions may be listed separately
- **BUT** the default Ito-managed skill assertion remains exactly seven
<!-- ITO:END -->

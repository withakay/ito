<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Orchestration Asset Names
Ito template assets SHALL install orchestration-related specialist roles and skills with the established concise `ito-*` names.

**Reason**: The requirement preserves a large family of specialist skill directories and a standalone orchestration workflow skill, which conflicts with the exact seven-skill lifecycle surface.
**Migration**: Keep delegated roles only as harness-native agent definitions when supported. Move orchestration policy into `ito agent instruction orchestrate` and expose iterative/orchestrated execution through `ito-loop`.

#### Scenario: Obsolete specialist skill paths are retired
- **WHEN** Ito refreshes managed template assets
- **THEN** it does not emit `ito-planner`, `ito-researcher`, `ito-reviewer`, `ito-worker`, `ito-orchestrator`, or `ito-orchestrator-workflow` as installed skills
- **AND** ownership-aware cleanup handles old managed paths

## ADDED Requirements

### Requirement: Harness-native agent assets remain separate from skills
Template manifests SHALL model retained delegated roles as harness-native agent assets and SHALL derive all installed skills from the canonical lifecycle inventory.

#### Scenario: Native role is emitted without skill counterpart
- **GIVEN** a harness supports a delegated-agent definition format
- **WHEN** Ito emits its template manifest
- **THEN** any retained role uses the native agent destination
- **AND** no matching role `SKILL.md` entry is emitted

#### Scenario: Harness lacks native agent roles
- **GIVEN** a harness lacks a separate delegated-agent format
- **WHEN** Ito emits its template manifest
- **THEN** no specialist role skill is synthesized as a fallback
<!-- ITO:END -->

<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Enforce Prefix in Templates Bundle
The `ito-templates` crate SHALL ship exactly the canonical seven Ito-managed skill directories and SHALL enforce Ito naming rules for other managed command, prompt, and native-agent assets. It MUST NOT preserve obsolete prefixed helpers merely because their names satisfy the prefix rule.

#### Scenario: Canonical lifecycle skills pass the bundle guard
- **WHEN** embedded skill assets are audited
- **THEN** the Ito-managed skill names are exactly `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`
- **AND** every noncanonical helper skill fails the exact-inventory guard whether prefixed or not

#### Scenario: Native agents are checked separately
- **WHEN** embedded native-agent assets are audited
- **THEN** their names follow the applicable Ito prefix convention
- **AND** those native agents are not counted or installed as skills

#### Scenario: Retired tmux helpers are absent
- **WHEN** the templates bundle is built
- **THEN** it contains no `tmux` or `ito-tmux` skill directory or helper scripts
<!-- ITO:END -->

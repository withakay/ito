<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph supports optional browser automation guidance

Ralph SHALL support injecting browser automation guidance when browser integration is enabled and available.

- **Requirement ID**: ralph-runtime-capabilities:browser-automation

#### Scenario: Browser capability is available

- **WHEN** browser automation is enabled and the required browser tool is installed
- **THEN** Ralph SHALL include browser-automation guidance in the prompt for runs that can use it

### Requirement: Ralph supports operator notifications

Ralph SHALL support optional completion and failure notifications for long-running orchestrated runs.

- **Requirement ID**: ralph-runtime-capabilities:operator-notifications

#### Scenario: Completion notification is emitted

- **WHEN** an orchestrated Ralph run finishes successfully
- **THEN** Ralph SHALL emit a supported operator notification when notifications are enabled
<!-- ITO:END -->

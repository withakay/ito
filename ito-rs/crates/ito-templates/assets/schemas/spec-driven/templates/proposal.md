<!-- ITO:START -->
## Why

<!-- Explain the motivation for this change. What problem does this solve? Why now? -->

## What Changes

<!-- Describe what will change. Be specific about new capabilities, modifications, or removals. -->

<!-- OPTIONAL: Include this block when the change would benefit from extra scope/risk signaling. -->
<!-- Allowed vocabulary:
  - Type: feature | fix | refactor | migration | contract | event-driven
  - Risk: low | medium | high
  - Stateful: yes | no
  - Public Contract: none | openapi | jsonschema | asyncapi | cli | config (comma-separated when needed)
  - Design Needed: yes | no
  - Design Reason: free text
-->
## Change Shape

- **Type**: <feature|fix|refactor|migration|contract|event-driven>
- **Risk**: <low|medium|high>
- **Stateful**: <yes|no>
- **Public Contract**: none
- **Design Needed**: <yes|no>
- **Design Reason**: <why a design doc is or is not needed>

## Capabilities

### New Capabilities

<!-- Capabilities being introduced. Replace <name> with kebab-case identifier (e.g., user-auth, data-export, api-rate-limiting). Each creates specs/<name>/spec.md -->

- `<name>`: <brief description of what this capability covers>

### Modified Capabilities

<!-- Existing capabilities whose REQUIREMENTS are changing (not just implementation).
     Only list here if spec-level behavior changes. Each needs a delta spec file.
     Use existing spec names from ito/specs/. Leave empty if no requirement changes. -->

- `<existing-name>`: <what requirement is changing>

## Impact

<!-- Affected code, APIs, dependencies, systems -->
<!-- ITO:END -->

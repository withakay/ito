<!-- ITO:START -->
## ADDED Requirements

### Requirement: PR fix workflow can be invoked from PR context

The repository automation MUST provide a `pr-fix` slash-command workflow that can run against an open pull request and may also be triggered by an `eyes` reaction.

#### Scenario: Slash command invokes workflow

- **WHEN** a user issues `/pr-fix` on a pull request thread
- **THEN** the PR fix workflow SHALL start with access to the pull request number and repository context

#### Scenario: Eyes reaction invokes workflow

- **WHEN** a user adds an `eyes` reaction in supported PR context
- **THEN** the PR fix workflow SHALL start for the associated pull request

### Requirement: Workflow analyzes CI failures and applies targeted fixes

When no explicit remediation instructions are provided, the workflow MUST analyze failing CI checks for the pull request, identify actionable root causes from logs, and apply code or configuration fixes on the pull request branch.

#### Scenario: No explicit instruction defaults to CI remediation

- **WHEN** the command input does not contain specific fix instructions
- **THEN** the workflow SHALL inspect failing workflow runs and derive fix actions from observed error output

#### Scenario: Explicit instruction is provided

- **WHEN** the command input includes sanitized user instructions
- **THEN** the workflow SHALL prioritize those instructions while still validating against pull request CI state

### Requirement: Workflow verifies and reports fixes safely

Before publishing updates, the workflow MUST run applicable repository checks and formatting tools, push only when progress is made, and post a pull request comment describing the changes and rationale.

#### Scenario: Fix validation succeeds

- **WHEN** remediation changes pass required verification steps
- **THEN** the workflow SHALL push updates to the pull request branch and add a summary comment

#### Scenario: Safe execution boundaries are enforced

- **WHEN** the workflow runs with repository permissions and safe outputs
- **THEN** the workflow MUST limit actions to declared safe operations and enforce the configured timeout
<!-- ITO:END -->

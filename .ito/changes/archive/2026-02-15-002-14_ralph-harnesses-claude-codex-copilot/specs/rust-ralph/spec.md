<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph can run Claude, Codex, and GitHub Copilot harnesses

Rust SHALL provide harness implementations that allow the Ralph loop to invoke:

- Claude Code (`claude`)
- OpenAI Codex (`codex`)
- GitHub Copilot CLI (`copilot`)

#### Scenario: Harness invocation returns captured output

- **GIVEN** a harness implementation for one of the supported CLIs
- **WHEN** the harness executes a prompt
- **THEN** the harness SHALL return captured stdout, stderr, an exit code, and a duration

### Requirement: Harness tests remain offline

Rust tests MUST run without requiring network access by using the stub harness in all test coverage.

#### Scenario: Ralph tests run offline

- **GIVEN** no network access
- **WHEN** `cargo test --workspace` runs
- **THEN** ralph tests pass using stub harnesses
<!-- ITO:END -->

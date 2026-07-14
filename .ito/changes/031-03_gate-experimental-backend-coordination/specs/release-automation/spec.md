<!-- ITO:START -->
## ADDED Requirements

### Requirement: Default and experimental feature sets have separate verification lanes

Repository automation SHALL verify the default shipping feature set independently from an explicit all-features experimental lane. Make targets and CI jobs MUST make the selected lane visible in their names and commands.

- **Requirement ID**: release-automation:split-feature-verification

#### Scenario: Default lane verifies the shipped build

- **WHEN** the default build, test, lint, documentation, or coverage lane runs
- **THEN** it uses the same default feature selection as the distributed `ito-cli` binary
- **AND** includes regression evidence that the backend crate is absent from the normal dependency graph
- **AND** verifies the default Ralph, loop, and migration-instruction surface

#### Scenario: Experimental lane verifies all features

- **WHEN** the experimental verification lane runs
- **THEN** it explicitly enables all Cargo features and workspace members needed by backend and coordination support
- **AND** runs their tests and lints without changing the default shipping feature set

### Requirement: Release artifacts contain only default CLI features

Cargo-dist, GitHub Release, installer, and Homebrew artifacts for `ito-cli` SHALL build the default CLI feature set and MUST NOT include backend or coordination-branch implementation code. Release automation SHALL retain explicit evidence of the selected feature set.

- **Requirement ID**: release-automation:default-artifact-features

#### Scenario: Cargo-dist builds the standard binary

- **WHEN** a version tag triggers cargo-dist packaging
- **THEN** cargo-dist selects `ito-cli` with its standard default features
- **AND** the packaged binary includes web and iteration behavior
- **AND** excludes backend and coordination-branch behavior

#### Scenario: Experimental backend remains buildable outside standard artifacts

- **WHEN** a developer or experimental CI job explicitly enables backend support
- **THEN** Cargo resolves a version-compatible published or workspace `ito-backend` crate
- **AND** standard GitHub Release and Homebrew artifacts remain unchanged

### Requirement: Shared default dependencies are reported accurately

Build and release evidence MUST distinguish feature-gated implementation code from dependencies that remain required by default functionality. The change MUST NOT claim that `rusqlite`, `sha2`, or `hex` disappear from the default dependency graph unless separate evidence proves that result.

- **Requirement ID**: release-automation:accurate-dependency-evidence

#### Scenario: Dependency evidence is reviewed

- **WHEN** implementation records before-and-after Cargo dependency evidence
- **THEN** it reports whether backend and coordination implementation code is compiled
- **AND** separately reports shared crates that remain for validation, task analysis, or front-matter behavior
<!-- ITO:END -->

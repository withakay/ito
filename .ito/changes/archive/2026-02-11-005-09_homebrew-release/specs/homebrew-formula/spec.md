## ADDED Requirements

### Requirement: Homebrew tap repository

A Homebrew tap repository SHALL exist at `withakay/homebrew-ito` containing the formula for installing the `ito` CLI.

#### Scenario: User adds tap and installs ito

- **WHEN** user runs `brew tap withakay/ito && brew install ito`
- **THEN** the `ito` binary is installed to the Homebrew prefix
- **AND** running `ito --version` outputs the installed version

### Requirement: Formula supports macOS architectures

The Homebrew formula SHALL support both Intel (x86_64) and Apple Silicon (arm64) macOS architectures using architecture-specific binary URLs.

#### Scenario: Install on Apple Silicon Mac

- **WHEN** user runs `brew install ito` on an arm64 Mac
- **THEN** Homebrew downloads the arm64-apple-darwin release artifact
- **AND** the installed binary runs natively without Rosetta

#### Scenario: Install on Intel Mac

- **WHEN** user runs `brew install ito` on an x86_64 Mac
- **THEN** Homebrew downloads the x86_64-apple-darwin release artifact
- **AND** the installed binary runs natively

### Requirement: Formula uses release artifacts

The formula SHALL download pre-built binaries from GitHub Releases rather than building from source.

#### Scenario: Formula downloads release binary

- **WHEN** Homebrew installs ito
- **THEN** it downloads the tarball from `https://github.com/withakay/ito/releases/download/vX.Y.Z/ito-*-apple-darwin.tar.gz`
- **AND** verifies the SHA256 checksum matches the formula

### Requirement: Automatic formula updates on release

A GitHub Actions workflow SHALL automatically update the formula when a new version is released.

#### Scenario: New release triggers formula update

- **WHEN** a new release tag (e.g., `v0.5.0`) is pushed to the ito repository
- **THEN** a workflow updates the formula version and SHA256 checksums in the tap repository
- **AND** commits and pushes the changes to the tap repository

#### Scenario: Formula update includes both architectures

- **WHEN** the formula update workflow runs
- **THEN** it updates SHA256 checksums for both x86_64 and arm64 artifacts
- **AND** updates the version string to match the release tag

### Requirement: Formula validates installation

The formula SHALL include a test block that verifies the installation succeeded.

#### Scenario: Homebrew test passes after install

- **WHEN** user runs `brew test ito`
- **THEN** Homebrew executes the test block
- **AND** the test verifies `ito --version` runs successfully

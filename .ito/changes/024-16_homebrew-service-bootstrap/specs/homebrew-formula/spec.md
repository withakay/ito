## MODIFIED Requirements

### Requirement: Homebrew tap repository

A Homebrew tap repository SHALL exist at `withakay/homebrew-ito` containing the formula for installing the `ito` CLI.

The user-facing formula name MUST be `ito`.

#### Scenario: User adds tap and installs ito

- **WHEN** user runs `brew tap withakay/ito && brew install ito`
- **THEN** the `ito` binary is installed to the Homebrew prefix
- **AND** running `ito --version` outputs the installed version

### Requirement: Formula uses release artifacts

The formula SHALL download pre-built binaries from GitHub Releases rather than building from source.

The release workflow MAY patch the dist-generated formula before committing it to the tap, but the published formula MUST continue to reference dist-produced release artifacts and checksums.

#### Scenario: Formula downloads release binary

- **WHEN** Homebrew installs `ito`
- **THEN** it downloads the tarball from `https://github.com/withakay/ito/releases/download/vX.Y.Z/ito-*-apple-darwin.tar.gz`
- **AND** verifies the SHA256 checksum matches the formula

### Requirement: Automatic formula updates on release

A GitHub Actions workflow SHALL automatically update the formula when a new version is released.

#### Scenario: New release triggers formula update

- **WHEN** a new release tag (for example `v0.5.0`) is pushed to the ito repository
- **THEN** the release workflow updates the generated Homebrew formula in the tap repository
- **AND** commits and pushes the changes to the tap repository

#### Scenario: Formula update injects service metadata

- **WHEN** the formula update workflow publishes `Formula/ito.rb`
- **THEN** it adds a Homebrew `service do` block that runs `ito serve-api --service`
- **AND** the workflow fails instead of silently publishing if the generated formula cannot be patched safely

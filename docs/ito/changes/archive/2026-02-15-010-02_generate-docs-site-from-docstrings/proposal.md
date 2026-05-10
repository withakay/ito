# Change: Generate Docs Site from Code Docstrings

## Why

Project documentation is split between code docstrings and hand-written docs, which makes onboarding slower and creates drift between API behavior and published guidance. A generated docs site consolidates API docs and curated guides so contributors can find accurate references and a fast path to first use.

## What Changes

- Add a documentation site pipeline using MkDocs with the Rustdoc plugin (`mkdocs-rustdoc-plugin`) as the default implementation.
- Generate API reference pages from code docstrings and include them in site navigation.
- Curate and include selected pages from the existing `docs/` folder in the same site.
- Add a new Quick Start guide that covers install, setup, and first successful command flow.
- Add local and CI verification commands so docs build failures are caught before release.

## Capabilities

### New Capabilities

- `docs-site-generation`: Build and serve a unified documentation site from code docstrings and curated docs content.
- `docs-quick-start`: Provide a concise Quick Start page in the site navigation for first-time contributors and users.

### Modified Capabilities

- <!-- None -->

## Impact

- Adds docs-site configuration and tooling files for MkDocs + Rustdoc plugin.
- Updates docs build workflows in local developer commands and CI checks.
- Updates documentation contributor guidance for docstrings and docs-page curation.
- Improves discoverability and reduces drift between code and published documentation.

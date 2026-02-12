# Tasks for: 010-02_generate-docs-site-from-docstrings

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use `ito tasks start/complete` for task state changes

```bash
ito tasks status 010-02_generate-docs-site-from-docstrings
ito tasks next 010-02_generate-docs-site-from-docstrings
ito tasks start 010-02_generate-docs-site-from-docstrings 1.1
ito tasks complete 010-02_generate-docs-site-from-docstrings 1.1
ito tasks show 010-02_generate-docs-site-from-docstrings
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add docs site scaffolding and build commands

- **Files**: `mkdocs.yml`, Python/docs dependency files, project build/check scripts
- **Dependencies**: None
- **Action**:
  Add MkDocs configuration, include `mkdocs-rustdoc-plugin` dependency, define site metadata/nav skeleton, and add project commands for docs build and local serve.
- **Verify**: `mkdocs build --strict`
- **Done When**: Docs site builds successfully with no missing config/pages.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

### Task 1.2: Generate API reference from code docstrings

- **Files**: `mkdocs.yml`, rustdoc plugin config, API reference output path under docs site
- **Dependencies**: Task 1.1
- **Action**:
  Configure `mkdocs-rustdoc-plugin` so API reference pages are generated from Rust docstrings as part of docs build.
- **Verify**: `mkdocs build --strict`
- **Done When**: Generated API reference pages appear in site output and nav.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Curate selected pages from docs folder into nav

- **Files**: `mkdocs.yml`, selected files under `docs/`
- **Dependencies**: None
- **Action**:
  Select and include target pages from `docs/` in deterministic nav order; exclude pages not ready for publication.
- **Verify**: `mkdocs build --strict`
- **Done When**: Site nav contains the selected docs pages in the intended order.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

### Task 2.2: Add Quick Start guide

- **Files**: `docs/quickstart.md`, `mkdocs.yml`
- **Dependencies**: Task 2.1
- **Action**:
  Create a concise Quick Start that covers prerequisites, setup, and one first-success command flow, then link it in top-level nav.
- **Verify**: `mkdocs build --strict`
- **Done When**: Quick Start is discoverable from top-level nav and renders correctly.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add docs verification to CI and contributor guidance

- **Files**: CI workflow files, contributor docs (`README.md` or equivalent)
- **Dependencies**: None
- **Action**:
  Add docs build verification in CI and document local docs workflow for contributors.
- **Verify**: CI docs check command (for example: `mkdocs build --strict` in CI job)
- **Done When**: CI fails on docs errors and contributor docs describe how to build/serve docs locally.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

### Task 3.2: Validate change artifacts

- **Files**: `.ito/changes/010-02_generate-docs-site-from-docstrings/*`
- **Dependencies**: Task 3.1
- **Action**:
  Run strict Ito validation and resolve any schema/spec/task formatting issues.
- **Verify**: `ito validate 010-02_generate-docs-site-from-docstrings --strict`
- **Done When**: Validation passes with no errors.
- **Updated At**: 2026-02-12
- **Status**: [x] complete

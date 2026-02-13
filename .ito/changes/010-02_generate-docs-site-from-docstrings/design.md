## Context

Documentation currently lives in multiple places: code docstrings for API details and markdown files under `docs/` for workflows. This split causes discoverability issues and inconsistent updates. The requested change introduces a unified docs website with generated API references plus curated narrative pages, and adds a Quick Start guide for faster onboarding.

## Goals / Non-Goals

**Goals:**

- Generate one publishable docs site from code docstrings and selected `docs/` pages.
- Keep an explicit, maintainable navigation model that controls which `docs/` pages are included.
- Add a Quick Start guide that gets a new user to first success quickly.
- Add reproducible docs build/validate commands usable locally and in CI.

**Non-Goals:**

- Rewriting all existing docs content.
- Replacing current API doc comment style in source code.
- Building a custom docs renderer when existing tooling can meet requirements.

## Decisions

- Use MkDocs as the default docs site framework and standardize API generation on the Rustdoc plugin (`mkdocs-rustdoc-plugin`). This keeps the stack aligned with Rust source docs and minimizes custom code.
- Keep docs inclusion explicit through configured navigation entries instead of auto-importing all files under `docs/`. This prevents stale or internal pages from being published unintentionally.
- Generate API references from code docstrings as part of the docs build command so drift is caught in normal verification flow.
- Add `docs/quickstart.md` (or equivalent path) as a first-class page in navigation and keep it short, task-oriented, and command-driven.

Alternatives considered:

- Docusaurus: richer app features but heavier setup and more JavaScript surface than needed.
- mdBook: strong for book-style docs but less natural for mixed generated API reference plus curated docs nav requirements.
- Bespoke static generator scripts: highest flexibility but unnecessary maintenance overhead.

## Risks / Trade-offs

- [Generated API tooling may be plugin-version sensitive] -> Pin `mkdocs-rustdoc-plugin` and MkDocs versions and document exact build prerequisites.
- [Navigation curation may lag when docs files are renamed] -> Add docs validation checks that fail on missing referenced pages.
- [Quick Start can become outdated] -> Include Quick Start verification in CI docs checks and update ownership guidance.

## Migration Plan

1. Add docs site configuration and dependencies.
2. Wire Rust docstring extraction through `mkdocs-rustdoc-plugin` in the docs build pipeline.
3. Curate selected pages from `docs/` into navigation.
4. Add and link the Quick Start guide.
5. Add local/CI docs verification commands and integrate into checks.
6. Validate the change with `ito validate 010-02_generate-docs-site-from-docstrings --strict`.

Rollback strategy: remove docs-site config, generated output hooks, and CI job additions in one revert to restore prior markdown-only docs flow.

## Open Questions

- Which subset of existing `docs/` pages should be included in the first release versus deferred?
- Should docs site deployment be tied to release tags only or every merge to main?

# Rust Port Research

```yaml
page_type: research-synthesis
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-05-27
source_refs:
  - .ito/research/SUMMARY.md
  - .ito/research/parity-matrix.md
  - .ito/research/investigations/rust-crate-architecture.md
  - .ito/research/investigations/rust-cli-ux.md
  - .ito/research/investigations/config-crate-extraction.md
  - .ito/research/investigations/packaging-distribution.md
known_gaps:
  - Does not replace the detailed parity matrix.
```

The Rust implementation emphasizes a CLI-first workflow, embedded templates,
strong validation, and parity with the previous Ito behavior while reducing
runtime dependencies. Research notes cover CLI UX, crate architecture,
configuration extraction, packaging, parity testing, and task-interface
comparisons.

## Durable Findings

- Keep CLI flows explicit and scriptable; interactive behavior should have
  non-interactive equivalents.
- Keep domain parsing and validation out of presentation surfaces when a crate
  boundary can express the behavior.
- Prefer embedded template assets with tests proving distribution and rendering
  behavior.
- Preserve artifact compatibility during migration by testing real fixtures and
  command output, not only unit-level parser behavior.

## Follow-Up

When a change touches CLI UX, crate boundaries, packaging, or parity behavior,
review the underlying research file before relying on this summary.

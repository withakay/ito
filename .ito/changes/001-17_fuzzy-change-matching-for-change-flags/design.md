# Design: Fuzzy change matching for --change/--change-id

## Context

Multiple Ito CLI commands accept a change selector flag (`--change` or `--change-id`). Users often know only a partial identifier (numeric id, slug fragment, or module + fragment). The CLI already has concepts like:

- Canonical on-disk change directory names: `NNN-<change_num>_<slug>`
- Loose ID parsing (dropping leading zeros)
- Suggestion-style error handling for ambiguous items (e.g., show/validate)

This change defines a shared resolver that all `--change` / `--change-id` flags use.

## Goals

- Accept partial inputs that uniquely identify a single active change.
- Support dropping leading zeros for module and change numbers.
- Keep canonical storage naming unchanged.
- Produce actionable errors for ambiguous or not-found inputs.
- Ensure behavior is deterministic and shared across commands.

## Non-Goals

- Changing the canonical change directory format.
- Prompting interactively when `--change` / `--change-id` is provided.

## Inputs and matching behavior

Treat the flag value as one of:

- **Exact canonical**: `001-12_project-setup-wizard`
- **Numeric identity**: `001-12` or `1-12` (with dropped zeros)
- **Slug query**: `setup-wizard`, `project-setup`, or multi-token like `"setup wizard"`
- **Module-scoped slug query**: `1:setup` / `001:setup` (module filter + slug query)
- **Module-only**: `1` / `001` (only resolves if exactly one active change is in that module)

Resolution rules:

1. Normalize module numbers and change numbers to canonical padded forms for comparison (module `NNN`, change number as digits with leading zeros trimmed for identity comparisons).
2. Search active changes by default; archived changes are excluded unless a command explicitly opts in.
3. If exactly one match remains, return its canonical change ID.
4. If zero matches, return not-found with best-effort suggestions.
5. If more than one match, return ambiguity error with a short candidate list.

## Error UX

- Ambiguous: include 5-10 candidate IDs, and suggest providing a longer value or full canonical ID.
- Not-found: include nearest-match suggestions when possible.

## Crate options (Rust)

We likely want deterministic, testable matching rather than heavy interactive search.

- `fuzzy-matcher`: straightforward fuzzy scoring API; good for small candidate sets.
- `nucleo` / `nucleo-matcher`: very fast fuzzy matcher; may be overkill but good if we want consistent scoring + future interactive pickers.
- `strsim`: similarity metrics (Levenshtein/Jaro); good for simple "nearest match" suggestions.

Recommended baseline: use an existing in-repo fuzzy helper if present (`ito-common` already has fuzzy utilities) and optionally layer `strsim` for suggestions. Add a dedicated fuzzy crate only if existing utilities are insufficient.

## Implementation sketch

- Add a shared `resolve_change_target(input, options)` utility in a common crate (likely `ito-common` or `ito-core`) and reuse it from all CLI subcommands.
- Ensure all CLI args that accept `--change` / `--change-id` call the resolver (including agent instruction commands).

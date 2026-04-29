# Source Guide: ito-common

## Responsibility
`ito-common` contains small shared utilities that are independent of Ito workflow policy: filesystem abstractions, ID parsing, file I/O helpers, fuzzy matching, canonical path builders, and git remote URL parsing.

## Entry Points
- `src/fs.rs`: filesystem abstraction for testable I/O.
- `src/id/**`: change/module/sub-module/spec ID parsing and errors.
- `src/io.rs`: convenience file operations.
- `src/match_.rs`: nearest-match helpers.
- `src/paths.rs`: canonical `.ito` path builders.
- `src/git_url.rs`: remote URL parsing.

## Design
- Keep helpers boring, deterministic, and policy-light.
- Prefer reusable newtypes/parser helpers here only when multiple crates need them.

## Flow
1. Higher-level crates call common helpers for shared low-level tasks.
2. Domain/core crates build policy on top of these primitives.

## Integration
- Used by `ito-domain`, `ito-core`, `ito-config`, and adapters.

## Gotchas
- Avoid adding dependencies or workflow knowledge that would make this crate a policy layer.
- ID parser changes can affect CLI UX, validation, and backend routes.

## Tests
- Targeted: `cargo test -p ito-common`.

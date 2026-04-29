# Zoekt Usage Log

## 2026-04-28: Source Guide Atlas Setup

### Queries Tried

- `lang:Rust "pub mod" file:ito-rs/crates/.*/src/lib.rs`
- `"Source Guide" OR "source-guide"`
- `"ito agent instruction apply" file:.ito/user-prompts/apply.md`

### What Worked

- Zoekt quickly found Rust module declarations in workspace crate `lib.rs` files.
- Zoekt found both the live `.ito/user-prompts/apply.md` and the default template copy under `ito-rs/crates/ito-templates/assets/default/project/.ito/user-prompts/apply.md`.
- It was useful for broad orientation before targeted file reads.

### Issues Observed

- JSON/JSONL results returned `Line` values as base64-encoded strings rather than readable snippets, which made direct human review harder.
- The result `Repository` field reported `main`, which is ambiguous when working from an isolated git worktree.
- A query for existing source-guide files returned no matches even after source-guide files had just been generated in the worktree; this suggests the Zoekt index may not include fresh uncommitted files until reindexed.

### Improvement Ideas

- Decode `Line` fields before displaying JSON results, or include a parallel plain-text snippet field.
- Include the indexed source path or worktree path in results to make worktree context clear.
- Surface index freshness status alongside zero-result responses, or suggest `zoekt_index` when uncommitted files may be missing.

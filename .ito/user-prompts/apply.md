<!-- ITO:START -->

# Apply Guidance

This file is for optional, user-authored guidance specific to `ito agent instruction apply`.

- Ito may update this header block over time.
- Add your apply guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Your Apply Guidance

## Source Guide Atlas

Before implementing an Ito change, check whether `source-guide.md` files exist near the code you will edit. If they are missing, stale, or too sparse for the affected subsystem, use the `source-guide` skill to set up or refresh the code atlas before making behavioral changes. Read the nearest guide first, treat it as orientation rather than authority, verify important claims against source, and update affected guides after structural changes.

After completing any logical batch of tasks (including checkbox-only task lists),
you MUST run @code-quality-squad review before reporting completion.
Treat the entire change as one wave if no wave sections exist.

When writing Rust tests that assert on structured data (structs/enums, nested graphs, vec/map payloads),
prefer `assert-struct` (https://crates.io/crates/assert-struct) over verbose field-by-field asserts.

Quick start:

```toml
[dev-dependencies]
assert-struct = "0.2"
```

```rust
use assert_struct::assert_struct;

assert_struct!(value, Type {
    important: "expected",
    nested.field: > 0,
    ..
});
```

## Showboat: Demo Documents for Completed Tasks

After completing each task (or logical batch of tasks), you MUST create a Showboat demo document
that demonstrates the work you just did. This is a key part of proving code actually works beyond
just automated tests.

### Setup

Showboat is available via `uvx showboat` (no install needed). Run `uvx showboat --help` to see
full CLI usage if needed.

### When to Create a Showboat Document

- After completing each task in `tasks.md`
- After completing a logical batch of related tasks
- Before reporting task completion

### Where to Put Demos

Place demo documents in the change directory:

```
.ito/changes/<change-id>/demos/
```

Name files descriptively: `task-1.1-schema-migration.md`, `api-endpoints.md`, etc.

### How to Build a Demo

Use the Showboat CLI commands to build the document incrementally. **Do NOT edit the markdown
file directly** -- always use the CLI so that command outputs are captured authentically.

```bash
# 1. Initialize the demo document
uvx showboat init demos/task-1.1-schema.md "Task 1.1: Database Schema Migration"

# 2. Add context notes explaining what was built
uvx showboat note demos/task-1.1-schema.md "Created the new user_sessions table with TTL support."

# 3. Run commands that prove the code works and capture their output
uvx showboat exec demos/task-1.1-schema.md bash "cargo test -p ito-core --lib schema -- --nocapture 2>&1 | tail -20"

# 4. Show the actual artifacts/output
uvx showboat exec demos/task-1.1-schema.md bash "cat ito-rs/crates/ito-core/src/schema.rs | head -30"

# 5. If something fails, remove the last entry and retry
uvx showboat pop demos/task-1.1-schema.md
```

### What to Demonstrate

- **Tests passing**: Run the relevant tests and capture output
- **CLI behavior**: Run the CLI commands that exercise the new feature
- **Code snippets**: Show key implementation details with `cat` / `head`
- **Before/after**: Show the state change your implementation caused
- **Error handling**: Demonstrate that error cases are handled correctly

### Important Rules

1. **Always use `uvx showboat exec`** to capture command output -- never fake output by editing the markdown
2. Use `uvx showboat pop` to remove failed entries rather than editing
3. Use `--workdir` if commands need to run from a specific directory
4. Keep demos focused -- one per task or logical batch, not one giant document
5. The demo is for your supervisor (the human) to quickly see what you built and verify it works

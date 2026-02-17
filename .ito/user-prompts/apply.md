<!-- ITO:START -->

# Apply Guidance

This file is for optional, user-authored guidance specific to `ito agent instruction apply`.

- Ito may update this header block over time.
- Add your apply guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Your Apply Guidance

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

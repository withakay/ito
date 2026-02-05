# Rust Coding Style for Ito

Apply these rules when writing or modifying Rust code in this project.

## Control Flow: Use `for` Loops, Not Iterator Chains

Write `for` loops with mutable accumulators instead of iterator combinators.

```rust
// DO
let mut results = Vec::new();
for item in items {
    if item.is_valid() {
        results.push(item.process());
    }
}

// DON'T
let results: Vec<_> = items
    .iter()
    .filter(|item| item.is_valid())
    .map(|item| item.process())
    .collect();
```

```rust
// DO
let mut total = 0;
for value in values {
    total += value.amount();
}

// DON'T
let total: i64 = values.iter().map(|v| v.amount()).sum();
```

```rust
// DO
let mut found = None;
for item in items {
    if item.matches(query) {
        found = Some(item);
        break;
    }
}

// DON'T
let found = items.iter().find(|item| item.matches(query));
```

## Early Returns: Use `let ... else`

Use `let ... else` to extract values and exit early on failure. This keeps the happy path unindented.

```rust
// DO
let Some(user) = get_user(id) else {
    return Err(Error::NotFound);
};
let Ok(session) = user.active_session() else {
    return Err(Error::NoSession);
};
// continue with user and session

// DON'T
if let Some(user) = get_user(id) {
    if let Ok(session) = user.active_session() {
        // deeply nested code
    } else {
        return Err(Error::NoSession);
    }
} else {
    return Err(Error::NotFound);
}
```

```rust
// DO
let Some(value) = maybe_value else { continue };
let Ok(parsed) = input.parse::<i32>() else { continue };

// DON'T
if let Some(value) = maybe_value {
    if let Ok(parsed) = input.parse::<i32>() {
        // ...
    }
}
```

## Pattern Matching: Use `if let` Chains

Use `if let` chains (combining `if` conditions with `let` bindings) to avoid nested if statements.

```rust
// DO
if !options.global_only
    && let Some(project_root) = &options.project_root
{
    // use project_root
}

// DON'T
if !options.global_only {
    if let Some(project_root) = &options.project_root {
        // nested
    }
}
```

```rust
// DO
if path.is_file()
    && let Some(ext) = path.extension()
    && ext == "bak"
{
    backups.push(path);
}

// DON'T
if path.is_file() {
    if let Some(ext) = path.extension() {
        if ext == "bak" {
            backups.push(path);
        }
    }
}
```

## Pattern Matching: Minimize Standalone `if let`

Use `if let` only when the `Some`/`Ok` branch is short and there's no else branch.

```rust
// ACCEPTABLE: short action, no else
if let Some(callback) = self.on_change {
    callback();
}

// DO: use let-else when you need the value
let Some(config) = load_config() else {
    return default_config();
};

// DO: use match for multiple cases
match result {
    Ok(value) => process(value),
    Err(Error::NotFound) => use_default(),
    Err(e) => return Err(e),
}
```

## Variable Naming: Shadow, Don't Rename

Shadow variables through transformations. Avoid prefixes like `raw_`, `parsed_`, `trimmed_`.

```rust
// DO
let input = get_raw_input();
let input = input.trim();
let input = input.to_lowercase();
let input = parse(input)?;

// DON'T
let raw_input = get_raw_input();
let trimmed_input = raw_input.trim();
let lowercase_input = trimmed_input.to_lowercase();
let parsed_input = parse(lowercase_input)?;
```

```rust
// DO
let path = args.path;
let path = path.canonicalize()?;
let path = path.join("config.toml");

// DON'T
let input_path = args.path;
let canonical_path = input_path.canonicalize()?;
let config_path = canonical_path.join("config.toml");
```

## Type Safety: Prefer Newtypes Over Strings

Wrap strings in newtypes to add semantic meaning and prevent mixing different string types.

```rust
// DO
struct UserId(String);
struct Email(String);

fn send_email(to: Email, from: UserId) { }

// DON'T
fn send_email(to: String, from: String) { }
```

## Type Safety: Prefer Strongly-Typed Enums Over Bools

Use enums with meaningful variant names instead of `bool` parameters.

```rust
// DO
enum Visibility {
    Public,
    Private,
}

fn create_repo(name: &str, visibility: Visibility) { }

// DON'T
fn create_repo(name: &str, is_public: bool) { }
```

```rust
// DO
enum Direction {
    Forward,
    Backward,
}

fn traverse(dir: Direction) { }

// DON'T
fn traverse(forward: bool) { }
```

## Pattern Matching: Avoid Wildcard Matches

Always match all variants explicitly to get compiler errors when variants are added.

```rust
// DO
match status {
    Status::Pending => handle_pending(),
    Status::Active => handle_active(),
    Status::Completed => handle_completed(),
}

// DON'T
match status {
    Status::Pending => handle_pending(),
    _ => handle_other(),
}
```

If a wildcard really seems necessary, add a comment explaining why it was used.

## Pattern Matching: Avoid `matches!` Macro

Use full `match` expressions instead of `matches!`. Full matches provide better compiler diagnostics when the matched type changes.

```rust
// DO
let is_ready = match state {
    State::Ready => true,
    State::Pending => false,
    State::Failed => false,
};

// DON'T
let is_ready = matches!(state, State::Ready);
```

## Destructuring: Always Use Explicit Destructuring

Destructure structs and tuples explicitly to get compiler errors when fields change.

```rust
// DO
let User { id, name, email } = user;
process(id, name, email);

// DON'T
process(user.id, user.name, user.email);
```

```rust
// DO
for Entry { key, value } in entries {
    map.insert(key, value);
}

// DON'T
for entry in entries {
    map.insert(entry.key, entry.value);
}
```

## Documentation: Public APIs Must Be Documented

All public items (`pub fn`, `pub struct`, `pub enum`, `pub trait`, etc.) require documentation comments. The goal is genuinely useful documentation that helps developers understand *why* and *when* to use something, not perfunctory descriptions that repeat the type signature.

### What to Document

```rust
// DO: Explain purpose, context, and non-obvious behavior
/// Resolves a change ID to its filesystem path, searching both active
/// changes and the archive. Returns `None` if the change doesn't exist
/// in either location.
///
/// Use this when you have a user-provided change ID and need to locate
/// its artifacts regardless of completion status.
pub fn resolve_change_path(id: &str) -> Option<PathBuf> { }

// DON'T: Restate the obvious
/// Resolves the change path.
///
/// # Arguments
/// * `id` - The change ID
///
/// # Returns
/// An optional PathBuf
pub fn resolve_change_path(id: &str) -> Option<PathBuf> { }
```

### Documentation Checklist

Ask yourself: "What would I want to know if I encountered this for the first time?"

- **Purpose**: What does this do and why does it exist?
- **When to use**: In what situations should someone reach for this?
- **Gotchas**: Any non-obvious behavior, edge cases, or invariants?
- **Related items**: What else should the reader look at?

### Structs and Enums

Document the type's purpose and when to use it. Document fields/variants only when their meaning isn't obvious from the name.

```rust
/// Configuration for the template installation process.
///
/// Controls which templates are installed and how conflicts are resolved.
/// Use `InstallConfig::default()` for standard interactive installation,
/// or customize for scripted/CI environments.
pub struct InstallConfig {
    /// Skip confirmation prompts (for CI/scripted use).
    pub non_interactive: bool,

    /// Overwrite existing files without prompting.
    pub force: bool,

    // No doc needed - obvious from name and type
    pub target_path: PathBuf,
}
```

### Error Types

Document what conditions cause each error variant.

```rust
/// Errors that can occur during change validation.
pub enum ValidationError {
    /// The change directory exists but contains no spec deltas.
    /// Every change must modify at least one capability.
    NoDeltas,

    /// A requirement is missing its scenarios. Each requirement
    /// must have at least one `#### Scenario:` demonstrating the behavior.
    MissingScenarios { requirement: String },

    /// The change modifies a capability outside its module's declared scope.
    ScopeViolation { capability: String, module: String },
}
```

### Module-Level Documentation

Use `//!` at the top of `lib.rs` or `mod.rs` to document the module's purpose.

```rust
//! Template installation and management.
//!
//! This module handles copying template files from embedded assets to
//! the target project. It supports both project-level templates (`.ito/`)
//! and home-level templates (`~/.config/ito/`).
//!
//! # Usage
//!
//! ```no_run
//! use ito_templates::install_project_templates;
//!
//! install_project_templates(&project_root, &config)?;
//! ```
```

### Skip Documentation For

- Private items (unless complex internal logic warrants it)
- Trait implementations where the trait docs suffice
- Simple getters/setters where the field name is self-explanatory
- Test modules and test helper functions

### Lint Enforcement

The workspace enables `#![warn(missing_docs)]` for library crates. Run `make docs` to build documentation and catch missing docs.

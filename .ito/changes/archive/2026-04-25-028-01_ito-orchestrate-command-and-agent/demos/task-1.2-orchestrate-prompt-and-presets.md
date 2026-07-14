# Task 1.2: Orchestrate Prompt Parsing and Presets

*2026-04-24T08:30:01Z by Showboat 0.6.1*
<!-- showboat-id: aede9647-5b00-447f-a5ce-4cd4318cba9b -->

Adds embedded orchestrate presets under ito-templates/assets/presets/orchestrate/*.yaml and core parsing for .ito/user-prompts/orchestrate.md (YAML front matter + MUST/PREFER/Notes sections). The orchestrate instruction now resolves a preset (default generic) and prints the configured gate order.

```bash
rtk cargo test -p ito-templates presets_files_contains_orchestrate_builtins -- --nocapture
```

```output
cargo test: 1 passed, 63 filtered out (4 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates get_preset_file_returns_contents -- --nocapture
```

```output
cargo test: 1 passed, 63 filtered out (4 suites, 0.00s)
```

```bash
rtk cargo test -p ito-cli orchestrate_ -- --nocapture
```

```output
cargo test: 3 passed, 342 filtered out (50 suites, 0.56s)
```

```bash
rtk ls ito-rs/crates/ito-templates/assets/presets/orchestrate && echo '--- rust.yaml ---' && sed -n '1,40p' ito-rs/crates/ito-templates/assets/presets/orchestrate/rust.yaml
```

```output
generic.yaml  338B
go.yaml  411B
python.yaml  461B
rust.yaml  526B
typescript.yaml  448B

5 files, 0 dirs (5 .yaml)
--- rust.yaml ---
name: rust

gate_order:
  - apply-complete
  - format
  - lint
  - tests
  - style
  - code-review
  - security-review

gates:
  apply-complete: {}
  format:
    tool: "cargo fmt --check"
  lint:
    tool: "cargo clippy -- -D warnings"
  tests:
    tool: "cargo test"
  style:
    skill: "rust-style"
  code-review: {}
  security-review: {}

recommended_skills:
  - rust-style
  - rust-code-reviewer

agent_roles:
  apply-worker: "rust-engineer"
  review-worker: "rust-code-reviewer"
  security-worker: "rust-quality-checker"
```

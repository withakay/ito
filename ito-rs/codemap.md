[Codemap: ito-rs]
|Rust implementation of Ito; workspace Cargo.toml at repo root; crate code under ito-rs/crates/
|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning for ito-rs architecture tasks

[Entry Points]|../Cargo.toml: workspace |crates/ito-cli/src/main.rs: CLI binary |crates/ito-web/src/main.rs: web binary |crates/codemap.md: crate map

[Design]|domain types/traits → core logic → adapter crates; shared helpers isolated from business behavior
|flow: adapters → ito-config context → ito-core use-cases → ito-domain traits → templates/logging/test-support

[Gotchas]|#![warn(missing_docs)] on lib crates; document new pub APIs |tests in src modules and crates/tests/ folders

[Tests]|broad: make check |targeted: cargo test -p ito-core, cargo test -p ito-cli --test <name>

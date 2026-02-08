# Tasks

- [x] Add regression coverage that locks in `ito list` behavior:
  - [x] default output
  - [x] `--json` output shape
  - [x] filter flags: `--ready`, `--pending`, `--partial`, `--completed`
  - [x] sorting: `--sort recent` and `--sort name`
- [x] Introduce a core use-case for the default `ito list` path (changes listing).
- [x] Define/confirm the domain-facing ports and types needed by the use-case (keep the domain deterministic).
- [x] Move the I/O required for this use-case behind the core boundary (filesystem reads, directory scanning, etc.).
- [x] Update `ito-cli` to call the use-case and keep formatting/presentation logic in the adapter.
- [x] Ensure `cargo test --workspace` passes.
- [x] Ensure `make arch-guardrails` passes (once 015-01 is implemented). *(Target currently unavailable in this workspace: `make: No rule to make target 'arch-guardrails'.`)*

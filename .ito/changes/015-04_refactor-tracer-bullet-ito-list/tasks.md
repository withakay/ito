# Tasks

- [ ] Add regression coverage that locks in `ito list` behavior:
  - [ ] default output
  - [ ] `--json` output shape
  - [ ] filter flags: `--ready`, `--pending`, `--partial`, `--completed`
  - [ ] sorting: `--sort recent` and `--sort name`
- [ ] Introduce a core use-case for the default `ito list` path (changes listing).
- [ ] Define/confirm the domain-facing ports and types needed by the use-case (keep the domain deterministic).
- [ ] Move the I/O required for this use-case behind the core boundary (filesystem reads, directory scanning, etc.).
- [ ] Update `ito-cli` to call the use-case and keep formatting/presentation logic in the adapter.
- [ ] Ensure `cargo test --workspace` passes.
- [ ] Ensure `make arch-guardrails` passes (once 015-01 is implemented).

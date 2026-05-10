<!-- ITO:START -->
# Tasks for: 019-10_manifesto-instruction

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-10_manifesto-instruction
ito tasks next 019-10_manifesto-instruction
ito tasks start 019-10_manifesto-instruction 1.1
ito tasks complete 019-10_manifesto-instruction 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add manifesto CLI surface

- **Files**: `ito-rs/crates/ito-cli/src/cli/agent.rs`, `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/tests/help.rs`, `ito-rs/crates/ito-cli/tests/instructions_more.rs`
- **Dependencies**: None
- **Action**: Add `manifesto` as a supported `ito agent instruction` artifact and parse the new variant, profile, change, and optional operation inputs into the instruction request path, including unsupported-combination errors.
- **Verify**: `cargo test -p ito-cli --test help --test instructions_more`
- **Done When**: The CLI accepts manifesto requests, help output exposes the artifact, request parsing covers the new flags, and invalid or incompatible request combinations fail clearly.
- **Requirements**: `agent-instructions:manifesto-artifact-availability`, `agent-instructions:manifesto-discoverability`, `agent-instructions:manifesto-variant-rendering`
- **Updated At**: 2026-04-27
- **Status**: [x] complete

### Task 1.2: Define manifesto rendering context

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/src/instructions.rs`, `ito-rs/crates/ito-templates/src/lib.rs`
- **Dependencies**: Task 1.1
- **Action**: Build the typed render context that resolves merged config, worktree and coordination settings, memory configuration, user guidance, optional change state, normalized `review_status`, and the restrictive intersection of profile, scope, and resolved workflow state.
- **Verify**: `cargo test -p ito-templates instructions_tests && cargo test -p ito-cli --test instructions_more`
- **Done When**: Manifesto rendering receives structured context for project-wide and change-scoped requests without relying on ad hoc template variables, and unresolved changes fail before rendering fabricated state.
- **Requirements**: `agent-instructions:manifesto-artifact-availability`, `agent-instructions:manifesto-config-redaction`, `agent-instructions:manifesto-state-and-profile`
- **Updated At**: 2026-04-27
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Wire light and full manifesto rendering

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/manifesto.md.j2`, `ito-rs/crates/ito-templates/src/instructions.rs`, `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**: Render the manifesto template in `light` and `full` variants with explicit source-of-truth ordering, state-machine sections, profile restrictions, and redacted config and state capsules.
- **Verify**: `cargo test -p ito-templates instructions_tests`
- **Done When**: Variant-specific output is deterministic, compact in `light`, and complete enough in `full` without leaking sensitive values.
- **Requirements**: `agent-instructions:manifesto-config-redaction`, `agent-instructions:manifesto-state-and-profile`, `agent-instructions:manifesto-variant-rendering`
- **Updated At**: 2026-04-27
- **Status**: [x] complete

### Task 2.2: Compose embedded instructions for full mode

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/src/instructions.rs`, `ito-rs/crates/ito-cli/tests/agent_instruction_context.rs`, `ito-rs/crates/ito-cli/tests/instructions_more.rs`
- **Dependencies**: Task 2.1
- **Action**: Reuse existing instruction-rendering paths to embed deterministically selected instruction artifacts in `full` mode, with `--operation` filtering when provided and manifesto-level hard rules remaining authoritative.
- **Verify**: `cargo test -p ito-cli --test agent_instruction_context --test instructions_more`
- **Done When**: Full-mode manifesto output embeds the correct instruction content for the requested scope and operation selection, rejects incompatible operation requests, and preserves manifesto precedence over embedded text.
- **Requirements**: `agent-instructions:manifesto-variant-rendering`, `agent-instructions:manifesto-discoverability`
- **Updated At**: 2026-04-27
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add profile and change-state regression coverage

- **Files**: `ito-rs/crates/ito-cli/tests/instructions_more.rs`, `ito-rs/crates/ito-cli/tests/agent_instruction_context.rs`, `ito-rs/crates/ito-core/tests/coordination_worktree.rs`
- **Dependencies**: None
- **Action**: Add regression tests for project-wide versus change-scoped rendering, profile-specific mutation restrictions, coordination-backed state resolution, and state narrowing of requested profiles.
- **Verify**: `cargo test -p ito-cli --test instructions_more --test agent_instruction_context && cargo test -p ito-core coordination_worktree`
- **Done When**: Change-scoped rendering resolves authoritative coordination state, no-change-selected rendering is constrained correctly, and profile restrictions are enforced in rendered manifesto output.
- **Requirements**: `agent-instructions:manifesto-artifact-availability`, `agent-instructions:manifesto-state-and-profile`
- **Updated At**: 2026-04-27
- **Status**: [x] complete

### Task 3.2: Add redaction and discoverability coverage

- **Files**: `ito-rs/crates/ito-templates/src/instructions_tests.rs`, `ito-rs/crates/ito-cli/tests/help.rs`, `ito-rs/crates/ito-cli/tests/instructions_more.rs`
- **Dependencies**: Task 3.1
- **Action**: Add regression coverage for secret and local-path redaction, full/profile disambiguation, and final discoverability checks in help and machine-readable responses.
- **Verify**: `cargo test -p ito-templates instructions_tests && cargo test -p ito-cli --test help --test instructions_more`
- **Done When**: Sensitive values are redacted by default, `variant=full` and `profile=full` are clearly distinguished in output, and users can discover manifesto support through the standard instruction interfaces.
- **Requirements**: `agent-instructions:manifesto-config-redaction`, `agent-instructions:manifesto-discoverability`, `agent-instructions:manifesto-variant-rendering`
- **Updated At**: 2026-04-28
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->

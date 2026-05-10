# Task 1.1: Agent Surface Inventory

*2026-04-29T14:11:36Z by Showboat 0.6.1*
<!-- showboat-id: 0c32b156-6699-4ffe-ac3a-e55a5a030a6d -->

Added a typed generated-agent surface inventory and tests that classify direct entrypoints, delegated role agents, and orchestration-adjacent workflow surfaces.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-templates surface -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-7eaa5889a2394c40)

running 3 tests
test agents::tests::agent_surface_inventory_defines_activation_boundaries ... ok
test tests::every_shipped_agent_is_in_surface_inventory ... ok
test tests::orchestration_adjacent_surfaces_are_classified ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 85 filtered out; finished in 0.00s

     Running tests/instructions_apply_memory.rs (target/debug/deps/instructions_apply_memory-d1e1807ce87f211a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-7a5705c6aff70672)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-b0565b06adac7694)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-95514587e0df9f18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-bc4ea5e74b0d0fe5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-36770e8c31892375)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-825fa89e3cbc9b79)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```

```bash
perl -ne 'print if $. >= 84 && $. <= 148' ito-rs/crates/ito-templates/src/agents.rs
```

```output
/// How a generated Ito agent is intended to be activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentActivationMode {
    /// User-facing agent that can be selected directly as a primary entrypoint.
    DirectEntryPoint,
    /// Bounded role dispatched by a direct entrypoint or orchestration workflow.
    DelegatedRole,
}

/// Canonical classification for one generated Ito agent surface.
///
/// Installers and tests use this inventory to keep generated agent templates
/// aligned with their intended user-facing or delegated role in each harness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentSurface {
    /// Agent name without harness-specific extension or `SKILL.md` suffix.
    pub name: &'static str,
    /// Expected activation mode for this agent.
    pub activation: AgentActivationMode,
}

const AGENT_SURFACE_INVENTORY: &[AgentSurface] = &[
    AgentSurface {
        name: "ito-quick",
        activation: AgentActivationMode::DelegatedRole,
    },
    AgentSurface {
        name: "ito-general",
        activation: AgentActivationMode::DirectEntryPoint,
    },
    AgentSurface {
        name: "ito-thinking",
        activation: AgentActivationMode::DirectEntryPoint,
    },
    AgentSurface {
        name: "ito-orchestrator",
        activation: AgentActivationMode::DirectEntryPoint,
    },
    AgentSurface {
        name: "ito-planner",
        activation: AgentActivationMode::DelegatedRole,
    },
    AgentSurface {
        name: "ito-researcher",
        activation: AgentActivationMode::DelegatedRole,
    },
    AgentSurface {
        name: "ito-worker",
        activation: AgentActivationMode::DelegatedRole,
    },
    AgentSurface {
        name: "ito-reviewer",
        activation: AgentActivationMode::DelegatedRole,
    },
    AgentSurface {
        name: "ito-test-runner",
        activation: AgentActivationMode::DelegatedRole,
    },
];

/// Return the canonical generated Ito agent surface inventory.
///
/// The returned slice is the source used to verify that every shipped Ito agent
/// template has an explicit activation classification. Add new generated agent
/// templates here when they become part of the supported Ito surface.
```

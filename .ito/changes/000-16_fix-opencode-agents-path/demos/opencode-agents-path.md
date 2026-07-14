# OpenCode agents plural path fix

*2026-04-28T19:16:09Z by Showboat 0.6.1*
<!-- showboat-id: d0f10373-0aac-492e-b4bf-5eaa860c642a -->

Updated Ito's OpenCode harness target path to .opencode/agents, adjusted init/update tests, and moved tracked repo agent assets out of .opencode/agent.

```bash
cargo test -p ito-cli --test init_more opencode
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.83s
     Running tests/init_more.rs (target/debug/deps/init_more-34b41e855949345c)

running 5 tests
test init_opencode_installs_audit_hook_plugin ... ok
test init_update_preserves_existing_markerless_opencode_agent_template_body ... ok
test init_with_tools_opencode_installs_orchestrator_agent_template ... ok
test init_update_refreshes_existing_opencode_orchestrator_agent_template ... ok
test init_update_preserves_existing_partial_marker_opencode_agent_template_body ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.80s

```

```bash
ito validate 000-16_fix-opencode-agents-path --strict
```

```output
Change '000-16_fix-opencode-agents-path' is valid
```

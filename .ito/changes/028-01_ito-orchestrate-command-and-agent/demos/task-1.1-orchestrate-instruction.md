# Task 1.1: Orchestrate Instruction Artifact

*2026-04-24T08:18:45Z by Showboat 0.6.1*
<!-- showboat-id: 84740a7f-259c-4b09-a11b-6e1a0cb1c57f -->

Adds , embeds , installs a default project prompt stub (), and adds focused CLI/template tests.

Adds a new agent instruction artifact: ito agent instruction orchestrate. Embeds the new template agent/orchestrate.md.j2, adds a default-project prompt stub at assets/default/project/.ito/user-prompts/orchestrate.md, and adds focused CLI/template tests.

```bash
rtk cargo test -p ito-templates orchestrate_template_renders -- --nocapture
```

```output
cargo test: 1 passed, 61 filtered out (4 suites, 0.00s)
```

```bash
rtk cargo test -p ito-templates user_prompt_stub_templates_exist -- --nocapture
```

```output
cargo test: 1 passed, 61 filtered out (4 suites, 0.00s)
```

```bash
rtk cargo test -p ito-cli orchestrate_ -- --nocapture
```

```output
cargo test: 3 passed, 342 filtered out (50 suites, 0.59s)
```

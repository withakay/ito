# Task 1.2: Regenerate config schema artifact

*2026-03-22T10:31:53Z by Showboat 0.6.1*
<!-- showboat-id: b08c5ea0-e616-4746-a329-dde728002ba4 -->

Regenerated the committed JSON schema artifact so editors and validation tooling see tools.tmux.enabled.

```bash
cd ito-rs && cargo run -p ito-cli -- config schema --output ../schemas/ito-config.schema.json 2>&1
```

```output
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/ito config schema --output ../schemas/ito-config.schema.json`
```

```bash
grep -n '"tools"\|"tmux"' schemas/ito-config.schema.json
```

```output
1051:        "tmux": {
1408:    "tools": {
1415:        "tmux": {
```

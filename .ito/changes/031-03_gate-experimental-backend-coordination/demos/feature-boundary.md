# Experimental Feature Boundary

*2026-07-13T23:45:02Z by Showboat 0.6.1*
<!-- showboat-id: 9eb4c895-4267-42e9-af49-42432e695273 -->

The root workspace selects the CLI as the primary product. The standard CLI explicitly ships the web surface while backend and coordination remain independent opt-in features.

```bash
cargo metadata --no-deps --format-version 1 | python3 -c "import json,sys; d=json.load(sys.stdin); names={p[\"id\"]:p[\"name\"] for p in d[\"packages\"]}; print(\"workspace_default_members=\" + \",\".join(names[i] for i in d[\"workspace_default_members\"]))"
```

```output
workspace_default_members=ito-cli
```

```bash
python3 -c "import tomllib; d=tomllib.load(open(\"ito-rs/crates/ito-cli/Cargo.toml\",\"rb\"))[\"features\"]; print(\"cli.default=\" + \",\".join(d[\"default\"])); print(\"cli.backend=\" + \",\".join(d[\"backend\"])); print(\"cli.coordination-branch=\" + \",\".join(d[\"coordination-branch\"]))"
```

```output
cli.default=web
cli.backend=dep:ito-backend,dep:serde_ignored,dep:tokio,dep:toml,dep:ureq,ito-core/backend
cli.coordination-branch=ito-core/coordination-branch
```

Default dependency evidence excludes the backend adapter but retains genuinely shared crates. The all-features graph activates both core capabilities explicitly.

```bash
tree=$(cargo tree -p ito-cli -e normal); for dep in ito-backend ito-web rusqlite sha2 hex; do if printf "%s\n" "$tree" | rg -q "$dep v"; then state=present; else state=absent; fi; printf "default.%s=%s\n" "$dep" "$state"; done
```

```output
default.ito-backend=absent
default.ito-web=present
default.rusqlite=present
default.sha2=present
default.hex=present
```

```bash
cargo tree -p ito-cli --all-features -e features -i ito-core | rg "ito-core feature \"(backend|coordination-branch)\"" | sed "s/.*ito-core/ito-core/" | sort -u
```

```output
ito-core feature "backend"
ito-core feature "coordination-branch"
```

Known experimental commands remain compatibility-parsed in the shipping binary and return stable typed errors instead of falling back or becoming unknown commands.

```bash
out=$(cargo run -q -p ito-cli -- backend status --json 2>/dev/null || true); printf "%s\n" "$out" | python3 -c "import json,sys; e=json.load(sys.stdin)[\"error\"]; print(\"kind=\"+e[\"kind\"]); print(\"feature=\"+e[\"feature\"]); print(\"requested_by=\"+e[\"requested_by\"])"
```

```output
kind=feature_unavailable
feature=backend
requested_by=ito backend
```

```bash
out=$(cargo run -q -p ito-cli -- sync --json 2>/dev/null || true); printf "%s\n" "$out" | python3 -c "import json,sys; e=json.load(sys.stdin)[\"error\"]; print(\"kind=\"+e[\"kind\"]); print(\"feature=\"+e[\"feature\"]); print(\"requested_by=\"+e[\"requested_by\"])"
```

```output
kind=feature_unavailable
feature=coordination-branch
requested_by=ito sync
```

The standard binary keeps iteration and recovery available: Ralph and loop remain discoverable, and migrate-to-main renders without either experimental feature.

```bash
for command in ralph loop; do if cargo run -q -p ito-cli -- "$command" --help >/dev/null 2>&1; then state=available; else state=missing; fi; printf "shipping.%s=%s\n" "$command" "$state"; done
```

```output
shipping.ralph=available
shipping.loop=available
```

```bash
cargo run -q -p ito-cli -- agent instruction migrate-to-main --json 2>/dev/null | python3 -c "import json,sys; d=json.load(sys.stdin); print(\"artifactId=\"+d[\"artifactId\"]); print(\"instruction_nonempty=\"+str(bool(d[\"instruction\"].strip())).lower()); print(\"mentions_main=\"+str(\"main\" in d[\"instruction\"]).lower())"
```

```output
artifactId=migrate-to-main
instruction_nonempty=true
mentions_main=true
```

Release automation pins the standard artifact to web only. The plan contains one CLI release and retains Homebrew, while the validator documents shared dependencies that remain for default consumers.

```bash
python3 -c "import tomllib; d=tomllib.load(open(\"dist-workspace.toml\",\"rb\"))[\"dist\"]; print(\"packages=\"+\",\".join(d[\"packages\"])); print(\"default-features=\"+str(d[\"default-features\"]).lower()); print(\"features=\"+\",\".join(d[\"features\"])); print(\"all-features=\"+str(d[\"all-features\"]).lower())"
```

```output
packages=ito-cli
default-features=false
features=web
all-features=false
```

```bash
dist plan --output-format json 2>/dev/null | python3 -c "import json,sys; d=json.load(sys.stdin); releases=d[\"releases\"]; print(\"release_count=\"+str(len(releases))); print(\"release_apps=\"+\",\".join(r[\"app_name\"] for r in releases)); print(\"homebrew_formula=\"+str(any(\"ito.rb\" in r[\"artifacts\"] for r in releases)).lower())"
```

```output
release_count=1
release_apps=ito-cli
homebrew_formula=true
```

```bash
python3 ito-rs/tools/check_release_features.py
```

```output
release feature boundary: ok
standard artifact: ito-cli with explicit web feature
experimental artifact: backend container opts into backend only
shared default dependencies retained where used: rusqlite, sha2, hex
```

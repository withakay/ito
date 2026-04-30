# ito-config — L0 (Foundation)

Configuration loading and normalization. Resolves Ito dir, reads repo-local+global config, exposes `ItoContext` per invocation.
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Modules
|config: load, defaults, schema, types (re-exported) |context: ItoContext — resolved per invocation (config dir, project root, ito path, config)
|ito_dir: resolve .ito working directory name+path |output: console/UI behaviour (color, interactivity) from CLI flags+env

## Dependencies
|ito-common only

## Constraints
**MUST NOT:** perform domain ops (no change/module/task logic) | depend on ito-domain/ito-core/ito-cli/ito-web | parse/manipulate markdown | contain business logic/workflow orchestration
**MUST:** remain thin config layer | keep #![warn(missing_docs)] | only depend on ito-common

## Gotchas
|config changes → schema+test updates |backend/worktree settings are contract-sensitive (templates render them) |keep path values portable in committed templates

## Quality
```bash
make check && make test && make arch-guardrails
```
|rust-quality-checker: style |rust-code-reviewer: catch architectural drift

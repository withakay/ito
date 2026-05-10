<!-- ITO:START -->
## Why

Change `011-05` introduces the `ito validate repo` rule engine and the first batch of rules covering `coordination/*` and `worktrees/*`. Several other configuration surfaces in `ItoConfig` benefit from the same configuration-aware guardrail:

- `audit.mirror.enabled` (`AuditMirrorConfig`) silently no-ops when its `branch` is empty or accidentally collides with `changes.coordination_branch.name`.
- `repository.mode = "sqlite"` (`RepositoryRuntimeConfig`) requires `repository.sqlite.db_path` to be set and reachable; today a missing path is only surfaced when a SQLite-backed repository is first opened, deep inside a command that should not be the discovery point.
- `backend.enabled = true` (`BackendApiConfig`) is the most security-relevant: a `backend.token` left in committed `.ito/config.json` is a leaked secret. The supported pattern is to store it in `ITO_BACKEND_TOKEN` (env var) or `.ito/config.local.json` (gitignored). This is currently documented in CLI help text only — it is not enforced.

These rules are pure additions on top of the engine landed by `011-05`: same trait, same registry, same JSON envelope.

## What Changes

Add the following rules to the existing `ito validate repo` registry. No engine API changes are required.

| Rule id | Activates when | Checks |
|---|---|---|
| `audit/mirror-branch-set` | `audit.mirror.enabled = true` | `audit.mirror.branch` non-empty and matches the `ito/internal/*` naming convention used by the rest of the workspace. |
| `audit/mirror-branch-distinct-from-coordination` | `audit.mirror.enabled = true` AND `changes.coordination_branch.storage = "worktree"` | `audit.mirror.branch != changes.coordination_branch.name` (a single branch must not be re-used for both audit mirroring and coordination). |
| `repository/sqlite-db-path-set` | `repository.mode = "sqlite"` | `repository.sqlite.db_path` is set, resolves under the project root, and its parent directory exists or is creatable. |
| `repository/sqlite-db-not-committed` | `repository.mode = "sqlite"` | `repository.sqlite.db_path` is gitignored (real binary DB files must never land in a commit). |
| `backend/token-not-committed` | `backend.enabled = true` | `backend.token` is absent from any **committed** `config.json` layer. The check must read the cascading layers individually (not the merged view) so a token in `.ito/config.local.json` or in `ITO_BACKEND_TOKEN` is allowed; only a token in committed `.ito/config.json` (or a parent shared config) fails the rule. |
| `backend/url-scheme-valid` | `backend.enabled = true` | `backend.url` parses as a valid URL with `http` or `https` scheme. |
| `backend/project-org-repo-set` | `backend.enabled = true` | `backend.project.org` and `backend.project.repo` are both non-empty (multi-tenant routing requires both). |

Each rule emits a `ValidationIssue` with `rule_id`, `level`, `path` (the config key path or the affected file path), `message` (what failed), `metadata.config_gate` (which `ItoConfig` value activated the rule), and `metadata.fix` (concrete remediation command).

`backend/token-not-committed` is `LEVEL_ERROR` regardless of `--strict` because a leaked token is a security incident. The remaining rules are warnings by default and become errors under `--strict`.

<!-- Allowed vocabulary:
  - Type: feature | fix | refactor | migration | contract | event-driven
  - Risk: low | medium | high
  - Stateful: yes | no
  - Public Contract: none | openapi | jsonschema | asyncapi | cli | config (comma-separated when needed)
  - Design Needed: yes | no
  - Design Reason: free text
-->
## Change Shape

- **Type**: feature
- **Risk**: low
- **Stateful**: no
- **Public Contract**: cli
- **Design Needed**: no
- **Design Reason**: Pure rule additions on top of the engine landed by `011-05`; no API change, no schema change, no CLI surface change beyond the registry growing. The trickiest item is `backend/token-not-committed` which needs uncascaded config layer access — to be resolved during implementation by reading `load_cascading_project_config(...).layers` rather than `.merged`.

## Capabilities

### New Capabilities

- `validate-repo-audit-rules`: `audit/mirror-branch-set` and `audit/mirror-branch-distinct-from-coordination`.
- `validate-repo-repository-rules`: `repository/sqlite-db-path-set` and `repository/sqlite-db-not-committed`.
- `validate-repo-backend-rules`: `backend/token-not-committed`, `backend/url-scheme-valid`, and `backend/project-org-repo-set`.

### Modified Capabilities

- `validate-repo-engine`: Registry grows by seven rules. No trait or runner change. The activation matrix and `--list-rules` output expand accordingly.

## Impact

- **Core (new files)**:
  - `ito-rs/crates/ito-core/src/validate_repo/audit_rules.rs`
  - `ito-rs/crates/ito-core/src/validate_repo/repository_rules.rs`
  - `ito-rs/crates/ito-core/src/validate_repo/backend_rules.rs`
- **Core (modified)**: `ito-rs/crates/ito-core/src/validate_repo/registry.rs` — register the seven new rules with their config gates.
- **Config layer access**: `backend/token-not-committed` requires per-layer access to `load_cascading_project_config`. Confirm the loader exposes `layers` (or extend it minimally if it only exposes `.merged` today).
- **Tests**:
  - Per-rule unit tests under `ito-core` exercising both the activate and skip branches.
  - Registry integration test that seeds `ItoConfig` permutations (audit on/off, sqlite/filesystem, backend on/off) and asserts the active rule set.
  - A `backend/token-not-committed` security test that:
    - Passes when the token is in `.ito/config.local.json`.
    - Passes when the token is in the `ITO_BACKEND_TOKEN` env var.
    - Fails when the token is in committed `.ito/config.json`.
- **Hard merge order**: `011-06` MUST land **after** `011-05`. The tasks file calls this out explicitly. CI config from `011-05` is reused unchanged.
- **Out of scope**: rule suppression, auto-fix, and any new CLI surface. Reserved for follow-ups.
<!-- ITO:END -->

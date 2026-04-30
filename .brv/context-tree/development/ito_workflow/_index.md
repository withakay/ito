---
children_hash: e33eb998885795c0da9736e6d03af6fb5277578a60ba84bd906f3b1cbf45e09b
compression_ratio: 0.27887571365832237
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, ddd_discovery_workflow.md, ito_orchestration_consolidation.md, obsolete_specialist_cleanup.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 4554
summary_level: d1
token_count: 1270
type: summary
---
# ito_workflow

Core workflow knowledge for Ito centers on publishing coordination-backed state into a read-only mirror, validating worktree safety, handling audit mirror synchronization safely, consolidating orchestration sources, and defining discovery behavior for DDD work.

## Mirror publishing and validation
- **Published Ito Mirror**: `published_ito_mirror.md` defines the read-only `docs/ito` mirror generated from coordination-backed state.
  - Key facts: configurable `changes.published_mirror.path` defaulting to `docs/ito`; safe path resolution rejects empty, absolute, parent-traversal, and project-root-only paths; renderer skips symlinks; output layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`.
  - Flow: configure mirror path → validate path → generate read-only mirror → detect drift → replace from coordination state.
  - Relationship: docs/ito is readable output; coordination state remains writable source of truth.

- **Worktree Validation Flow**: `worktree_validation_flow.md` documents `ito worktree validate --change <id> [--json]`.
  - Key facts: dedicated read-only validation flow; main/control checkouts hard-fail; non-main mismatches are advisory with recovery guidance; exact change-id prefix matching avoids false positives, including suffix worktrees like `<change>-review`.
  - Dependency: OpenCode pre-tool hooks rely on machine-readable status output.

- **Audit Mirror Concurrency and Temp Naming**: `audit_mirror_concurrency_and_temp_naming.md` covers the audit mirror sync path in `mirror.rs`.
  - Key facts: temporary worktree and orphan branch names include `pid`, `SystemTime` timestamp, and atomic counter; JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events; logs are truncated to the newest 30 days and max 1000 events; push/ref update conflicts retry once.
  - Flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push or update ref → retry on conflict.
  - Rules: only runs inside a Git worktree; missing remote branch uses orphan branch; non-fast-forward pushes refetch and retry once.

## Orchestration and agent surface
- **Ito Orchestration Consolidation**: `ito_orchestration_consolidation.md` folds orchestration work into change `028-02_centralize-instruction-source-of-truth`.
  - Key decision: overlapping orchestration and multi-agent skills/prompts are centralized behind `ito agent instruction orchestrate` as the authoritative source.
  - Architectural distinction: entrypoint agents (`ito-general`, `ito-orchestrator`) are separated from delegated sub-agents (`planner`, `researcher`, `worker`, `reviewer`, `test-runner`).
  - Relationship: this topic is the consolidation target for orchestration overlap and should be treated as the single source of truth.

- **Obsolete Specialist Cleanup**: `obsolete_specialist_cleanup.md` captures installer cleanup behavior for renamed orchestrator assets.
  - Key facts: cleanup runs on update and forceful init/reinstall paths; harness-level pre-pass removes legacy assets before writing new ones; broken symlinks are removed via `symlink_metadata`; plain init preserves untouched user files.
  - Scope: removes obsolete `ito-orchestrator-*` specialist markdown and `SKILL.md` assets while preserving coordinator assets like `ito-orchestrator.md` and `ito-orchestrator-workflow`.

## Discovery workflow
- **DDD Discovery Workflow**: `ddd_discovery_workflow.md` defines discovery behavior for `001-34_add-ddd-discovery-workflow`.
  - Key facts: integrates grill-with-docs ideas into Ito domain discovery; requires repository evidence before asking user questions; adds focused domain-grill questioning for ambiguous/cross-context work; includes glossary conflict challenges, scenario-based boundary probes, optional queries, consistency requirements, and named-or-provisional context relationships.
  - Lifecycle: reference material → consensus concepts → discovery depth gate → capability boundary check → context relationship patterns → consistency and optional queries → boundary-smell probes → gated domain-grill recommendation.
  - Lazy capture artifacts: `CONTEXT.md`, `CONTEXT-MAP.md`, and ADRs are used before post-approval domain-doc promotion.
  - Rule: rigorous domain-grill is gated, but auto-recommended for high-impact ambiguity or explicit user opt-in.

## Structural relationships
- `context.md` is the topic-level overview tying the workflow together: Ito publishes a read-only mirror of coordination-backed state into `docs/ito` and keeps it synchronized safely.
- `audit_mirror_concurrency_and_temp_naming.md` and `published_ito_mirror.md` both relate to safe mirror generation and synchronization.
- `worktree_validation_flow.md` and `published_ito_mirror.md` both enforce safe handling of worktree/publish operations.
- `ddd_discovery_workflow.md` connects discovery practices to `ito_orchestration_consolidation.md` and `source_guides/source_guide_workflow.md` through consensus discovery and guardrail-aware questioning.
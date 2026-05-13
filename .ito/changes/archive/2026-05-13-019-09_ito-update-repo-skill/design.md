<!-- ITO:START -->
## Context

Ito-managed harness assets are installed from multiple template sources: shared skills, shared commands, harness-specific agents/adapters, and default project seed files such as `ito-project-setup`. `ito init --update` refreshes managed blocks but intentionally does not prune stale files, so an explicit cleanup workflow is needed.

## Goals / Non-Goals

- Goals: refresh managed Ito assets, identify Ito-owned orphans safely, preserve user-owned files, and make staleness visible through version stamps.
- Non-Goals: add automatic destructive pruning to `ito init --update`, delete non-`ito-*` user assets, or migrate non-markdown YAML/JSON stamp metadata in this change.

## Decisions

- Ito ownership is name-based: `ito` and `ito-*` basenames in managed harness roots are Ito-owned; unprefixed basenames are user- or third-party-owned.
- Orphan detection is root-specific. Expected entries are derived from the current CLI templates for that root, including shared command templates and default project command seeds installed by the same binary.
- Stale detection is stamp-based. Managed markdown files carry `<!--ITO:VERSION:<semver>-->` immediately after `<!-- ITO:START -->`; missing or older stamps are stale, not deletion candidates.
- Cleanup remains skill-driven and approval-gated. The new `ito-update-repo` skill documents the workflow and requires explicit approval unless `--yes` is passed.
- The install/update renderer owns stamp insertion and normalization so generated files are idempotent once refreshed.

## Risks / Trade-offs

- Name-based ownership can miss historical unprefixed Ito assets. The rename table documents known cases, while the prefix rule prevents new unprefixed assets from shipping.
- Default project seed commands are not stored in `assets/commands/`, so the audit must not use a single global command allow-list.
- Local development builds include local version suffixes; verification must install the current branch binary before checking generated asset staleness.

## Verification Strategy

- Unit tests enforce the `ito-` prefix rule, managed marker coverage, and version-stamp idempotence.
- End-to-end smoke tests rebuild/install the CLI, run `ito init --update --tools all`, audit for Ito-owned orphans, and rerun update to verify no additional diff is produced.
<!-- ITO:END -->

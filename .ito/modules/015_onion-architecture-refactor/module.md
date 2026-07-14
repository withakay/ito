# Onion Architecture Refactor

## Purpose
Refactor the Rust workspace toward an onion-style architecture with enforced dependency direction.

This module focuses on pragmatic, incremental improvements:

- Make architectural constraints explicit and machine-checked (prek + CI).
- Keep `ito-domain` deterministic and testable (no direct disk/process/network I/O).
- Keep adapters (`ito-cli`, `ito-web`) thin and decoupled.

Guiding document: `.local/onionarchitecturerefactor_epic_plan.md`.

## Scope
- *

## Changes
- [x] 015-01_refactor-arch-guardrails
- [x] 015-02_refactor-cli-web-decouple
- [x] 015-03_update-rust-workspace-specs
- [x] 015-04_refactor-tracer-bullet-ito-list
- [x] 015-05_refactor-change-repo-ports
- [x] 015-06_refactor-module-repo-ports
- [x] 015-07_refactor-task-repo-ports
- [x] 015-08_refactor-error-boundaries
- [x] 015-09_refactor-process-exec-boundary
- [x] 015-10_refactor-adapter-thinning
- [x] 015-11_refactor-schema-usage-guidelines
- [x] 015-12_refactor-split-core-into-app-infra
- [x] 015-13_standardize-arch-guardrails-tooling
- [x] 015-14_consolidate-workspace-crates
- [x] 015-15_move-ralph-command-to-commands

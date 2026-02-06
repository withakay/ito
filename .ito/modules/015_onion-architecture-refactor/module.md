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
- [ ] 015-01_refactor-arch-guardrails
- [ ] 015-02_refactor-cli-web-decouple
- [ ] 015-03_update-rust-workspace-specs
- [ ] 015-04_refactor-tracer-bullet-ito-list
- [ ] 015-05_refactor-change-repo-ports
- [ ] 015-06_refactor-module-repo-ports
- [ ] 015-07_refactor-task-repo-ports
- [ ] 015-08_refactor-error-boundaries
- [ ] 015-09_refactor-process-exec-boundary
- [ ] 015-10_refactor-adapter-thinning
- [ ] 015-11_refactor-schema-usage-guidelines
- [ ] 015-12_refactor-split-core-into-app-infra

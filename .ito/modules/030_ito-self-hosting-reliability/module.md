# Ito Self-Hosting Reliability

## Purpose
Make building Ito with Ito reliable by moving repeated LLM workarounds into deterministic Ito capabilities.

## Scope
- *

## Rationale
This module captures improvements discovered by mining prior OpenCode sessions where agents had to work around Ito itself. The goal is to make Ito own command discovery, completion truth, coordination synchronization, archive discovery, managed-file ownership, and validation contracts so LLMs can focus on implementation and review.

## Sequencing
- Start with `030-01_machine-readable-capabilities` so agents and prompts can discover the true CLI surface.
- Implement `030-03_coordination-branch-sync` early because current change creation can reproduce remote-ahead coordination failures.
- Use `030-06_validation-contract-and-ci-doctor` as the shared validation substrate for `030-02_deterministic-completion-gate`.

## Changes
- [ ] 030-01_machine-readable-capabilities
- [ ] 030-02_deterministic-completion-gate
- [ ] 030-03_coordination-branch-sync
- [ ] 030-04_archive-and-change-discovery
- [ ] 030-05_managed-file-ownership
- [ ] 030-06_validation-contract-and-ci-doctor

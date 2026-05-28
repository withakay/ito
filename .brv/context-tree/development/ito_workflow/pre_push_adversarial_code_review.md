---
title: Pre-push Adversarial Code Review
summary: Ito agents should run an adversarial code review before pushing a change branch or opening a PR, blocking P0/P1 issues and reducing review noise.
tags: []
keywords: []
importance: 53
recency: 1
maturity: draft
accessCount: 1
createdAt: '2026-05-17T18:24:37.072Z'
updatedAt: '2026-05-17T18:24:37.072Z'
---
## Reason
Document the new pre-push review workflow guidance from the Ito general skill

## Raw Concept
**Task:**
Document pre-push adversarial code review guidance for Ito agents

**Changes:**
- Added a pre-push review step for change branches and PRs

**Flow:**
change ready -> run adversarial diff review -> fix P0/P1 issues -> optionally address low-risk findings -> push/open PR

**Author:** .agents/skills/ito-general/SKILL.md

## Narrative
### Structure
This guidance belongs in the Ito workflow as a pre-push quality gate for branches and pull requests.

### Highlights
Adversarial review is explicitly recommended before pushing or opening a PR, with a hard stop on P0/P1 findings and discretion for minor low-risk issues.

### Rules
Before pushing a change branch or opening a PR, recommend and run an adversarial code review to catch low-hanging fruit and reduce avoidable PR noise. Block P0/P1 issues until fixed. Lesser findings can be addressed at the agent's discretion when the fix is low-risk and aligned with the change.

## Facts
- **pre_push_review**: Before pushing a change branch or opening a PR, Ito agents should recommend and run an adversarial code review of the diff. [convention]
- **review_purpose**: The purpose is to catch low-hanging fruit before review. [convention]
- **review_purpose**: The purpose is to reduce avoidable PR noise. [convention]
- **severity_gate**: The purpose is to block P0/P1 issues until fixed. [convention]
- **minor_findings_handling**: Lesser findings can be addressed at the agent's discretion when the fix is low-risk and aligned with the change. [convention]

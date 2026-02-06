# Spec: ito-templates (MODIFIED)

> Updates to LLM instruction templates to promote CLI-first mutations and audit reconciliation.

## ADDED

### Requirement: Audit-aware agent instructions

The agent instruction templates SHALL include guidance on audit log awareness for LLM agents.

#### Scenario: CLI-first mutation guidance

WHEN agent instructions are generated for task apply workflows
THEN the instructions SHALL explicitly direct agents to use `ito tasks start/complete/shelve/unshelve` CLI commands for all task state changes
AND the instructions SHALL explain that CLI commands automatically emit audit events
AND the instructions SHALL discourage direct editing of `tasks.md` status fields

#### Scenario: Reconciliation after direct edits

WHEN agent instructions acknowledge that direct file edits may be necessary
THEN the instructions SHALL direct agents to run `ito audit reconcile --change <id>` after any direct edit to state files
AND this guidance SHALL appear in both apply and review instruction templates

#### Scenario: Pre-archive validation

WHEN agent instructions are generated for archive workflows
THEN the instructions SHALL include `ito audit validate --change <id>` as a verification step before archiving
AND the instructions SHALL recommend `ito audit reconcile` if validation reports drift

#### Scenario: Instruction template location

WHEN audit-aware instructions are added
THEN they SHALL be embedded in the existing instruction template assets at `ito-rs/crates/ito-templates/assets/`
AND they SHALL NOT create new standalone instruction files

<!-- ITO:START -->
## ADDED Requirements

### Requirement: Reconcile Tombstone Semantics

Audit materialization SHALL treat a `reconciled` event without a `to` value as a tombstone for the referenced entity so reconciliation can clear audit entries that no longer have a corresponding file entry.

#### Scenario: Extra task is reconciled
- **WHEN** a task exists in audit state but no longer exists in the tracking file
- **AND** reconciliation writes a `reconciled` event without a `to` value
- **THEN** subsequent audit materialization removes that task from current audit state
- **AND** a subsequent reconcile run reports no drift for that task

### Requirement: Reconcile Fix Verification

`ito audit reconcile --fix` SHALL verify the resulting drift state after writing compensating events and report failure when drift remains.

#### Scenario: Fix clears drift
- **WHEN** `ito audit reconcile --fix` writes compensating events that resolve all detected drift
- **THEN** the command reports no remaining drift

#### Scenario: Fix cannot clear drift
- **WHEN** `ito audit reconcile --fix` cannot clear detected drift
- **THEN** the command exits with failure
- **AND** callers can stop automatic repair attempts instead of appending repeated no-op events
<!-- ITO:END -->

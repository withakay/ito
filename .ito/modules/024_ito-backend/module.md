# Ito Backend

## Purpose
Introduce a shared Ito backend service so multiple harness instances can coordinate on the same project state without relying on brittle git-only synchronization. The backend provides a centralized HTTP API for change state, task progress, artifact storage, event ingestion, and project bootstrapping.

## Scope
- backend-state-api
- backend-auth
- change-leasing
- change-allocation
- backend-artifact-store
- backend-event-ingest
- backend-project-bootstrap
- backend-event-forwarding
- backend-client-runtime
- backend-change-claim
- backend-change-sync
- change-repository
- task-repository
- cli-tasks
- config

## Changes
- [x] 024-01_add-shared-state-api
- [ ] 024-02_add-cli-backend-client
- [ ] 024-03_add-backend-project-bootstrap
- [ ] 024-04_add-backend-event-forwarding
- [ ] 024-05_add-backend-archive-sync

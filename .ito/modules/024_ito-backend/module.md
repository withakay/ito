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
- cli-grep
- config
- distribution

## Changes
- [x] 024-01_add-shared-state-api
- [x] 024-02_add-cli-backend-client
- [x] 024-03_add-backend-project-bootstrap
- [x] 024-04_add-backend-event-forwarding
- [x] 024-05_add-backend-archive-sync
- [x] 024-10_import-existing-and-archived-changes
- [x] 024-10_multi-tenant-backend-server
- [x] 024-11_add-grep-command
- [x] 024-11_export-changes-zip-archive
- [x] 024-12_add-cloudflare-deployment
- [x] 024-12_add-docker-compose-backend-runtime
- [x] 024-13_add-homebrew-systemd-backend-services
- [ ] 024-15_docker-and-helm
- [ ] 024-16_homebrew-service-bootstrap

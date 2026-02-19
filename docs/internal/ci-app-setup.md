# CI GitHub App Setup (Internal)

This page documents how maintainers configure the shared GitHub App used by CI automation.

## Purpose

Use a GitHub App instead of personal access tokens so CI uses short-lived credentials, shared ownership, and permissions scoped to repositories.

## Required repositories

- `withakay/ito`
- `withakay/homebrew-ito`

## Required app permissions

- Contents: Read & Write
- Pull requests: Read & Write
- Metadata: Read-only

## Required repository secrets (`withakay/ito`)

- `ITO_CI_APP_ID`
- `ITO_CI_APP_PRIVATE_KEY`

## Validation checklist

1. CI autofix commits can push to PR branches.
2. release-plz can open/update release PRs and push tags.
3. Homebrew release workflow can push to `withakay/homebrew-ito`.

## Rotation checklist

1. Generate a new private key in GitHub App settings.
2. Update `ITO_CI_APP_PRIVATE_KEY` in repository secrets.
3. Run a CI workflow that uses app auth.
4. Revoke the old key.

If process details change, update this page directly so internal CI auth guidance stays canonical.

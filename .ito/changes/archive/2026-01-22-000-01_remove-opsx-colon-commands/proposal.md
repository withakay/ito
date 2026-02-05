# Proposal: Remove OPSX Colon Commands

## Why

- The `/opsx:*` slash commands are Claude-specific and inconsistent with the hyphenated experimental workflow command naming.
- Standardizing on `/ito-*` keeps the experimental workflow consistent with other Ito tooling.

## What Changes

- Remove all `/opsx:*` command references from templates, generated command wrappers, and docs.
- Standardize the experimental workflow slash commands to:
  - `/ito-explore`
  - `/ito-new-change`
  - `/ito-continue-change`
  - `/ito-apply-change`
  - `/ito-ff-change`
  - `/ito-sync-specs`
  - `/ito-archive-change`

## Capabilities

### New

- None (this is a rename / standardization).

### Modified

- Experimental workflow command wrappers and docs use `/ito-*`.

## Impact

- Breaking change: `/opsx:*` commands are removed (no backward compatibility).

# Proposal integration configuration

*2026-07-13T14:17:27Z by Showboat 0.6.1*
<!-- showboat-id: 6f894170-70fc-4ccf-a01f-4da1b8a4a62d -->

New repositories and existing repositories share the same safe pull-request default. The installed example makes that policy visible.

```bash
jq -c '.changes.proposal' ito-rs/crates/ito-templates/assets/default/project/.ito/config.json
```

```output
{"integration_mode":"pull_request"}
```

The generated public schema exposes exactly the two supported modes and records the default.

```bash
jq -c '{default:.definitions.ProposalConfig.properties.integration_mode.default,values:[.definitions.ProposalIntegrationMode.oneOf[].enum[0]]}' schemas/ito-config.schema.json
```

```output
{"default":"pull_request","values":["pull_request","direct_merge"]}
```

The documentation explains which immutable Git authority each mode will use.

```bash
sed -n '/### Proposal integration/,/### Worktrees/p' docs/config.md
```

````output
### Proposal integration

Ito expects a proposal to be reviewed and integrated into the configured target branch before implementation begins. `changes.proposal.integration_mode` selects the authority used to prove that hand-off:

- `pull_request` (default) uses the target branch's tracked upstream ref, normally `refs/remotes/origin/main`.
- `direct_merge` is an explicit opt-in that uses the local target branch, normally `refs/heads/main`.

```json
{
  "changes": {
    "proposal": {
      "integration_mode": "pull_request"
    }
  }
}
```

There is no fallback from a missing pull-request authority to local `main`. Repositories that deliberately integrate proposals without a remote pull-request workflow must select `direct_merge`.

The lifecycle is the same in both modes:

1. Author and strictly validate a proposal-only package.
2. Review and integrate that package into the configured target branch.
3. Run `ito change preflight <change-id> --for prepare --refresh`.
4. Create/reuse the implementation checkout with `ito worktree ensure --change <change-id>`.
5. Run implementation commands only after `ito change preflight <change-id> --for execute` passes.

`prepare` reads required artifacts directly from one captured authority commit. `execute` additionally proves that the selected change worktree contains the proposal integration commit and belongs to the full change ID. Local copies, coordination links, and backend state cannot satisfy either gate.

### Worktrees
````

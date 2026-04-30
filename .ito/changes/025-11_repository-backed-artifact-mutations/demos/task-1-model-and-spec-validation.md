# Task 1: Artifact model and spec validation

*2026-04-30T22:23:05Z by Showboat 0.6.1*
<!-- showboat-id: 4af224f7-e08c-412f-8fde-87654223db11 -->

Validated the active-change artifact mutation command model: repository-runtime-backed `ito patch` and `ito write` operate on change proposal, design, tracking, and spec delta artifacts via Ito artifact refs instead of filesystem paths.

```bash
ito patch --help
```

```output
Apply a targeted patch to an active change artifact

Uses repository-runtime-selected persistence to patch an active-work
change artifact such as `proposal.md`, `design.md`, the tracking
artifact, or a change-local spec delta.

Examples:
  printf '%s' '`<patch>`' | ito patch change 025-11_repository-backed-artifact-mutations proposal
  printf '%s' '`<patch>`' | ito patch change 025-11_repository-backed-artifact-mutations spec backend-agent-instructions

Usage: ito patch [OPTIONS] <COMMAND>

Commands:
  change  Mutate an artifact inside an active change

Options:
      --no-color
          Disable color output

      --help-all
          Print the full CLI reference (equivalent to `ito help --all`)

  -h, --help
          Print help (see a summary with '-h')
```

```bash
ito write --help
```

```output
Replace an active change artifact completely

Uses repository-runtime-selected persistence to write an active-work
change artifact such as `proposal.md`, `design.md`, the tracking
artifact, or a change-local spec delta.

Examples:
  printf '%s' '`<content>`' | ito write change 025-11_repository-backed-artifact-mutations proposal
  printf '%s' '`<content>`' | ito write change 025-11_repository-backed-artifact-mutations spec backend-agent-instructions

Usage: ito write [OPTIONS] <COMMAND>

Commands:
  change  Mutate an artifact inside an active change

Options:
      --no-color
          Disable color output

      --help-all
          Print the full CLI reference (equivalent to `ito help --all`)

  -h, --help
          Print help (see a summary with '-h')
```

```bash
ito validate 025-11_repository-backed-artifact-mutations --strict
```

```output
Change '025-11_repository-backed-artifact-mutations' is valid
```

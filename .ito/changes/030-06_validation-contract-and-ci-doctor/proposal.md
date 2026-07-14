# Change: Validation Contract And CI Doctor

## Why

Many prior sessions spent time discovering whether to run `make check`, `make test`, coverage, cargo-deny, rustdoc, GitHub Actions checks, or a narrower targeted command. Agents also had to manually parse CI logs and extract the useful failure excerpt.

Ito should expose the project's validation contract and CI diagnostics in a deterministic, machine-readable form.

## What

Add validation and CI doctor surfaces:

```bash
ito check --json
ito test affected --json
ito doctor ci --json
```

`ito check` should compute and run the configured validation plan. `ito test affected` should provide a narrower plan where Ito can determine one. `ito doctor ci` should summarize GitHub Actions or configured CI failures with actionable excerpts.

## Impact

Agents stop guessing validation commands and stop pasting large logs back into reasoning loops. Ralph and completion verification can depend on a shared validation contract.

## Out Of Scope

This change does not replace all project Makefiles or CI systems. It standardizes how Ito discovers, records, runs, and reports validation for agent workflows.

## Success Criteria

- `ito check --json` reports command source, executed commands, pass/fail, duration, and failure excerpts.
- `ito test affected --json` reports a deterministic plan or a reason it fell back to full validation.
- `ito doctor ci --json` summarizes failing checks with links, failed steps, and concise actionable excerpts.
- Ralph and completion verifier can use `ito check --json` as the project validation step.

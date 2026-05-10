<!-- ITO:START -->
## Context

Agents frequently rely on `rg`/`grep` workflows to search through change proposals, specs, and tasks. In backend mode, artifacts may be remote and not present on local disk.

We want a consistent `ito grep` UX that behaves the same in filesystem mode and backend mode.

## Goals / Non-Goals

**Goals:**

- Provide `ito grep` that supports searching:
  - a single change
  - a module (across changes)
  - the whole project (across changes)
- Implement the search engine in `ito-core` using ripgrep’s crate ecosystem (`grep-*`, `ignore`).
- In backend mode, materialize artifacts to a local cache directory and search cached files on disk.
- Limit output via a simple `--limit` flag.

**Non-Goals:**

- Full parity with all ripgrep flags.
- PCRE2 regex support.
- Long-running local daemon for push updates.

## Decisions

### Decision: Search cached files on disk

In backend mode, the CLI will ensure local cached artifact files exist for the selected scope and are fresh via HTTP conditional requests (`ETag` / `If-None-Match`). The grep engine then searches local paths.

Rationale: simplest mental model and reuses mature search crates.

### Decision: Minimal output limiting

Provide a single `--limit <N>` that caps printed matching lines.

Rationale: agents can further trim output with shell tools (`head`, `sed`) without Ito owning complex paging behavior.

## Risks / Trade-offs

- Searching `--all` may require downloading many artifacts the first time → mitigate via conditional revalidation and persistent cache.
- Output size can still be large even with `--limit` if line lengths are huge → mitigate later with max line length if needed.
<!-- ITO:END -->

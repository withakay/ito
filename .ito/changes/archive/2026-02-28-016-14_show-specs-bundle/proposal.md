<!-- ITO:START -->
## Why

Sometimes we need to provide an LLM (or a human reviewer) with the entire set of Ito truth specifications as a single prompt/document.

Today, `ito show <spec-id>` prints one spec at a time. There is no first-class command that produces a single markdown stream containing all main specs, with per-spec metadata that preserves the spec folder name (capability id) and source file path.

## What Changes

- Add a new `ito show specs` (plural) mode that renders a single markdown document by concatenating all main specs under `.ito/specs/*/spec.md`.
- Include per-spec metadata so the spec id (folder name) and source file path are preserved in the output.
- Support `--json` for machine-readable consumption (with absolute path fields).

## Capabilities

### Modified Capabilities

- `cli-show`: Add `ito show specs` output for bundling all truth specs into one stream.

## Impact

- Adds a new CLI affordance; no behavior changes to existing `ito show <item>` flows.
- Output includes absolute filesystem paths (consistent with `absolute-path-output`).
<!-- ITO:END -->

<!-- ITO:START -->
## Why

Once a change proposal is written, there is no ergonomic way to review it without navigating the filesystem manually. Agents and users alike need a quick, workflow-integrated way to (re)view a completed proposal package (proposal.md, specs/, tasks.md) without leaving the terminal. A dedicated `ito view proposal <change-id>` command closes this gap and sets up an extensible viewer dispatch layer that future viewer backends (e.g., HTML/browser rendering — see 001-30) can plug into.

## What Changes

- Add `ito view proposal <change-id>` subcommand under the existing `ito view` / `ito dashboard` surface.
- The command collects all change artifacts — `proposal.md`, `specs/*.md` (delta specs), and `tasks.md` — and concatenates them into a single document for viewing.
- An interactive prompt asks the user to choose a viewer each time (no persistence):
  - **tmux popover (neovim)** — opens a tmux popup window with neovim displaying the document (uses the `tmux` skill; depends on 001-28)
  - **bat** — renders the document with syntax highlighting in the terminal
  - **glow** — renders markdown in the terminal with glow formatting
- The viewer dispatch architecture SHALL be extensible: adding a new viewer requires only implementing a `ViewerBackend` trait and registering it — no changes to the core command logic.
- `--viewer <bat|glow|tmux-nvim>` flag allows bypassing the prompt for scripted / agent use.

## Capabilities

### New Capabilities

- `proposal-viewer`: The `ito view proposal <change-id>` command and its viewer dispatch layer. Collects change artifacts, prompts for a viewer, and opens the content in the selected viewer. Extensible via a `ViewerBackend` trait for future viewer additions.

### Modified Capabilities

- `cli-view`: The `ito view` command surface gains a `proposal` subcommand. The existing `ito dashboard` / `ito view` behavior is unchanged.

## Impact

- `ito-rs/crates/ito-cli/src/` — new `view proposal` subcommand handler
- `ito-rs/crates/ito-core/` — artifact collection logic (proposal.md + specs/ + tasks.md for a given change-id); `ViewerBackend` trait and viewer implementations
- External tool dependencies: `neovim`, `tmux`, `bat`, `glow` (all optional; graceful error if missing)
- Depends on 001-28 (`tmux-skill-integration`) for agent guidance on the tmux popover path
- 001-30 (`proposal-viewer-html`) extends this by adding a `pandoc` → browser viewer backend
<!-- ITO:END -->

<!-- ITO:START -->
## ADDED Requirements

### Requirement: Proposal viewer command

The system SHALL provide an `ito view proposal <change-id>` command that collects all change artifacts and opens them in a user-selected viewer.

#### Scenario: Command resolves change artifacts

- **WHEN** user runs `ito view proposal <change-id>`
- **THEN** the system locates the change directory
- **AND** collects `proposal.md`, any `specs/**/*.md` delta files, and `tasks.md` (if present)
- **AND** concatenates them into a single ordered document with section separators

#### Scenario: Change not found

- **WHEN** user runs `ito view proposal <change-id>` with an unknown change ID
- **THEN** the system displays an error: "✗ Change '<change-id>' not found"
- **AND** exits with a non-zero status code

#### Scenario: No Ito directory

- **WHEN** user runs `ito view proposal` outside an Ito-initialized project
- **THEN** the system displays an error: "✗ No ito directory found"
- **AND** exits with a non-zero status code

### Requirement: Interactive viewer selection

The system SHALL prompt the user to select a viewer each time `ito view proposal` is invoked, unless a viewer is specified via flag.

#### Scenario: Viewer prompt shown

- **WHEN** user runs `ito view proposal <change-id>` without `--viewer`
- **THEN** the system presents an interactive selection prompt listing available viewers
- **AND** the prompt includes only viewers whose backing tool is detected on the system

#### Scenario: Viewer flag bypasses prompt

- **WHEN** user runs `ito view proposal <change-id> --viewer <name>`
- **THEN** the system skips the interactive prompt and opens the document directly in the specified viewer

#### Scenario: Specified viewer not installed

- **WHEN** user passes `--viewer <name>` and the backing tool is not found on PATH
- **THEN** the system displays an error naming the missing tool and how to install it
- **AND** exits with a non-zero status code

#### Scenario: No viewers available

- **WHEN** none of the supported viewer tools are detected on the system
- **THEN** the system displays an error listing the supported tools and how to install them
- **AND** exits with a non-zero status code

### Requirement: Tmux/neovim popover viewer

The system SHALL support a `tmux-nvim` viewer backend that opens the collected document in a neovim instance inside a tmux popup window.

#### Scenario: Document opened in tmux popup

- **WHEN** user selects or specifies `--viewer tmux-nvim`
- **THEN** the system writes the collected document to a temporary file
- **AND** opens a tmux popup window running `nvim <tmpfile>` in read-only mode
- **AND** the popup is sized to fill the terminal

#### Scenario: Tmux not running

- **WHEN** `$TMUX` is not set (no active tmux session)
- **THEN** the system displays an error: "✗ tmux-nvim viewer requires an active tmux session"
- **AND** exits with a non-zero status code

#### Scenario: Neovim not installed

- **WHEN** `nvim` is not found on PATH
- **THEN** the system displays an error naming `nvim` as a missing dependency
- **AND** exits with a non-zero status code

### Requirement: Bat viewer

The system SHALL support a `bat` viewer backend that renders the collected document with syntax highlighting in the terminal.

#### Scenario: Document rendered with bat

- **WHEN** user selects or specifies `--viewer bat`
- **THEN** the system pipes the collected document through `bat --language=markdown`
- **AND** bat output is displayed in the terminal with paging

#### Scenario: Bat not installed

- **WHEN** `bat` is not found on PATH
- **THEN** the system displays an error naming `bat` as a missing dependency with an install hint
- **AND** exits with a non-zero status code

### Requirement: Glow viewer

The system SHALL support a `glow` viewer backend that renders the collected document as styled markdown in the terminal.

#### Scenario: Document rendered with glow

- **WHEN** user selects or specifies `--viewer glow`
- **THEN** the system pipes the collected document through `glow -`
- **AND** glow output is displayed in the terminal

#### Scenario: Glow not installed

- **WHEN** `glow` is not found on PATH
- **THEN** the system displays an error naming `glow` as a missing dependency with an install hint
- **AND** exits with a non-zero status code

### Requirement: Extensible viewer backend architecture

The system SHALL implement viewer dispatch via a `ViewerBackend` trait so that new viewer backends can be added without modifying core command logic.

#### Scenario: New viewer registered without core changes

- **WHEN** a new type implementing `ViewerBackend` is created and registered in the viewer registry
- **THEN** it appears automatically in the interactive viewer prompt
- **AND** it is selectable via `--viewer <name>` without changes to the command parser
<!-- ITO:END -->

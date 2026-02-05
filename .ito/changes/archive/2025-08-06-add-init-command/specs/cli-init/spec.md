# CLI Init Specification

## Purpose

The `ito init` command SHALL create a complete Ito directory structure in any project, enabling immediate adoption of Ito conventions with support for multiple AI coding assistants.

## Behavior

### Progress Indicators

WHEN executing initialization steps
THEN validate environment silently in background (no output unless error)
AND display progress with ora spinners:

- Show spinner: "⠋ Creating Ito structure..."
- Then success: "✔ Ito structure created"
- Show spinner: "⠋ Configuring AI tools..."
- Then success: "✔ AI tools configured"

### Directory Creation

WHEN `ito init` is executed
THEN create the following directory structure:

```
ito/
├── project.md
├── README.md
├── specs/
└── changes/
    └── archive/
```

### File Generation

The command SHALL generate:

- `README.md` containing complete Ito instructions for AI assistants
- `project.md` with project context template

### AI Tool Configuration

WHEN run interactively
THEN prompt user to select AI tools to configure:

- Claude Code (updates/creates CLAUDE.md with Ito markers)
- Cursor (future)
- Aider (future)

### AI Tool Configuration Details

WHEN Claude Code is selected
THEN create or update `CLAUDE.md` in the project root directory (not inside ito/)

WHEN CLAUDE.md does not exist
THEN create new file with Ito content wrapped in markers:

```markdown
<!-- ITO:START -->
# Ito Project

This document provides instructions for AI coding assistants on how to use Ito conventions for spec-driven development. Follow these rules precisely when working on Ito-enabled projects.

This project uses Ito for spec-driven development. Specifications are the source of truth.

See @ito/README.md for detailed conventions and guidelines.
<!-- ITO:END -->
```

WHEN CLAUDE.md already exists
THEN preserve all existing content
AND insert Ito content at the beginning of the file using markers
AND ensure markers don't duplicate if they already exist

The marker system SHALL:

- Use `<!-- ITO:START -->` to mark the beginning of managed content
- Use `<!-- ITO:END -->` to mark the end of managed content
- Allow Ito to update its content without affecting user customizations
- Preserve all content outside the markers intact

WHY use markers:

- Users may have existing CLAUDE.md instructions they want to keep
- Ito can update its instructions in future versions
- Clear boundary between Ito-managed and user-managed content

### Interactive Mode

WHEN run
THEN prompt user with: "Which AI tool do you use?"
AND show single-select menu with available tools:

- Claude Code
  AND show disabled options as "coming soon" (not selectable):
- Cursor (coming soon)
- Aider (coming soon)
- Continue (coming soon)

User navigation:

- Use arrow keys to move between options
- Press Enter to select the highlighted option

### Safety Checks

WHEN `ito/` directory already exists
THEN display error with ora fail indicator:
"✖ Error: Ito seems to already be initialized. Use 'ito update' to update the structure."

WHEN checking initialization feasibility
THEN verify write permissions in the target directory silently
AND only display error if permissions are insufficient

### Success Output

WHEN initialization completes successfully
THEN display actionable prompts for AI-driven workflow:

```
✔ Ito initialized successfully!

Next steps - Copy these prompts to Claude:

────────────────────────────────────────────────────────────
1. Populate your project context:
   "Please read ito/project.md and help me fill it out
    with details about my project, tech stack, and conventions"

2. Create your first change proposal:
   "I want to add [YOUR FEATURE HERE]. Please create an
    Ito change proposal for this feature"

3. Learn the Ito workflow:
   "Please explain the Ito workflow from ito/README.md
    and how I should work with you on this project"
────────────────────────────────────────────────────────────
```

The prompts SHALL:

- Be copy-pasteable for immediate use with AI tools
- Guide users through the AI-driven workflow
- Replace placeholder text (\[YOUR FEATURE HERE\]) with actual features

### Exit Codes

- 0: Success
- 1: General error (including when Ito directory already exists)
- 2: Insufficient permissions (reserved for future use)
- 3: User cancelled operation (reserved for future use)

## Why

Manual creation of Ito structure is error-prone and creates adoption friction. A standardized init command ensures:

- Consistent structure across all projects
- Proper AI instruction files are always included
- Quick onboarding for new projects
- Clear conventions from the start

---
name: ito-lite-setup
description: Initialize a markdown-only Ito Lite workspace with no executables. Use when setting up prompt-driven change proposals, creating .ito-lite/project.md, or preparing a repo for Ito-style specs, changes, and tasks without the ito CLI.
compatibility: No external dependencies; markdown and file editing only.
---

# Ito Lite Setup

Use this skill to set up a repo for prompt-driven Ito-style change management when the `ito` executable is unavailable.

## Default Path

Use `.ito-lite/` unless the user explicitly asks for `.ito/` and the real Ito CLI is not installed in the target environment.

Do not create executable scripts. Do not require package managers, shell commands, or network access.

## Setup Workflow

1. Inspect the repo enough to identify the primary stack and existing project conventions.
2. Ask the user only for missing setup facts:
   - What does this project do in one sentence?
   - Who is it for?
   - What are the primary languages/frameworks?
   - How do you build, test, lint, and run it today?
   - What is the most important constraint future agents should know?
3. Create the Ito Lite directory skeleton.
4. Create `.ito-lite/project.md` from the template below.
5. Mark setup complete with `<!-- ITO-LITE:PROJECT_SETUP:COMPLETE -->`.

## Directory Skeleton

```text
.ito-lite/
├── project.md
├── specs/
├── changes/
│   └── archive/
├── modules/
└── wiki/
```

## project.md Template

```markdown
# Ito Lite Project

<!-- ITO-LITE:PROJECT_SETUP:COMPLETE -->

## Purpose

<What this project does in one sentence.>

## Audience

<Internal tool, public API, end-user app, library, etc.>

## Stack

- **Primary language(s)**: <languages>
- **Frameworks/libraries**: <frameworks>
- **Package/build tools**: <tools>

## Dev Commands

- **Build**: <command or N/A>
- **Test**: <command or N/A>
- **Lint/Check**: <command or N/A>
- **Run**: <command or N/A>

## Agent Guidance

<The single most important constraint future agents should know.>

## Ito Lite Notes

- Specs under `.ito-lite/specs/` describe current intended behavior.
- Changes under `.ito-lite/changes/` describe proposed behavior.
- Archive completed changes only after implementation and verification.
```

## Completion Check

Setup is complete when:

- `.ito-lite/project.md` exists.
- `.ito-lite/project.md` contains `<!-- ITO-LITE:PROJECT_SETUP:COMPLETE -->`.
- `.ito-lite/specs/`, `.ito-lite/changes/archive/`, and `.ito-lite/modules/` exist.

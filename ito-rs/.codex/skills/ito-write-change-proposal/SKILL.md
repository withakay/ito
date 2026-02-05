---
name: ito-write-change-proposal
description: Use when creating, designing, planning, proposing, or specifying a feature, change, requirement, enhancement, fix, modification, or spec. Use when writing tasks, proposals, specifications, or requirements for new work.
---

Create or continue a change, then generate proposal/spec/design/tasks using the CLI instruction artifacts.

**Testing Policy (TDD + coverage)**

- Default workflow: RED/GREEN/REFACTOR (write a failing test, implement the minimum to pass, then refactor).
- Coverage target: 80% (guidance; projects may override in `.ito/config.json`).
- When available, follow the "Testing Policy" section emitted by `ito agent instruction proposal|apply`; it reflects project configuration.

**Steps**

1. If the user provided an existing change ID, use it.
   Otherwise, create a new change:

   - Pick a module (default to `000` if unsure).
   - Run:
     ```bash
     ito create change "<change-name>" --module <module-id>
     ```

2. Generate the artifacts (source of truth):

   ```bash
   ito agent instruction proposal --change "<change-id>"
   ito agent instruction specs --change "<change-id>"
   ito agent instruction design --change "<change-id>"
   ito agent instruction tasks --change "<change-id>"
   ```

3. Follow the printed instructions for each artifact exactly.

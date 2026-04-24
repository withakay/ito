# 029-01_add-byterover-integration

Add ByteRover as the first concrete agent-memory provider: install the Claude Code connector, install ByteRover hub skills (review, plan, explore, audit) under .agents/skills/byterover/, and curate Ito's authoritative docs/specs/modules into the local ByteRover context tree. Local-only (no cloud sync). Shared skills live under .agents/skills/ except for Claude Code which uses its own connector.

# Ito Documentation

Ito is a change-driven development tool for your terminal that brings together project-centric planning, design notes, specifications, and tasks.

It is designed for AI-assisted development where work spans multiple sessions and benefits from explicit artifacts, validation, and repeatable workflows.

Ito integrates with terminal based AI coding agents by installing commands, prompts, skills and instructions that agents can use to interact with the change lifecycle and project artifacts. It provides templates, conventions, and automation to help you plan, execute, and review changes in a way that tries to align with AI agents abilities.

Currently the following agents are supported to a greater or lesser extent, with more on the way:

- Claude Code
- Codex CLI
- GitHub Copilot CLI
- OpenCode

Ito is a personal project and not affiliated with any of the above agents or their respective companies. 
There will be rough edges and different agents will have different levels of support and reliability. 
The goal is to provide a flexible framework that can be adapted as agents evolve, rather than a rigid system that tries to predict the future of AI coding.
I hope it can be a useful tool for people experimenting with AI-assisted development and multi-agent workflows, and I welcome contributions and feedback to help improve it.
It is open source under the MIT License.

## Why Ito

I wanted a short easy to type name that evokes the idea of threads of work, connections between ideas, and intentionality in planning and execution. The name "Ito" is inspired by the following Japanese words:

- Thread/String (糸): Used for sewing thread, yarn, or in a metaphorical sense for connections.
- Intention/Aim (意図): Often used in the context of plans, aims, or intent.

This site combines:

- curated project guides from `docs/`
- generated API reference from `cargo doc`, published under `docs/rustdoc/`

If you are new to contributing to Ito, start with [Developer Quick Start](quickstart.md).

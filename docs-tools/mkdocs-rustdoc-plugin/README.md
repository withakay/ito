# mkdocs-rustdoc-plugin

Local MkDocs plugin for Ito documentation builds.

It runs `cargo doc`, generates a markdown index page at `docs/api/rustdoc.md`, and copies rendered Rustdoc HTML into the final MkDocs site under `/rustdoc/`.

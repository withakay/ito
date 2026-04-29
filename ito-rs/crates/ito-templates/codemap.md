[Codemap: ito-templates]|L1: embeds files installed by ito init/update (project/home templates, skills, commands, adapters, agents, schemas, presets, instruction templates); Jinja rendering; no workspace deps beyond external crates

[Entry Points]|src/lib.rs: embedded dir accessors + .ito path rewriting |src/agents.rs: harness tiers, default models, install destinations
|src/instructions.rs: instruction artifact rendering |src/project_templates.rs: Jinja rendering (e.g. AGENTS.md) |assets/**: embedded source compiled into crate

[Design]|bytes + pure rendering only; fs writes in ito-core installers; default prompts in assets/default/project/.ito/user-prompts/

[Gotchas]|adding/renaming/moving assets changes compile-time output |markdown assets must preserve ITO managed markers
|harness install paths → centralize in agents.rs, not duplicated in installers

[Tests]|targeted: cargo test -p ito-templates |installation: cargo test -p ito-cli --test init_more

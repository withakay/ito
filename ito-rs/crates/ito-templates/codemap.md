[Codemap: ito-templates]|L1: embeds files installed by ito init/update (project/home templates, skills, commands, adapters, agents, schemas, presets, instruction templates); Jinja rendering; no workspace deps beyond external crates

[Entry Points]|src/lib.rs: embedded dir accessors + .ito path rewriting |src/agents.rs: harness tiers, default models, install destinations
|src/instructions.rs: instruction artifact rendering |src/project_templates.rs: Jinja rendering (e.g. AGENTS.md)
|src/legacy.rs: retired lifecycle/historical surface registry |assets/**: embedded source compiled into crate

[Design]|bytes + pure rendering only; fs writes in ito-core installers; default prompts in assets/default/project/.ito/user-prompts/
|LIFECYCLE_SKILL_NAMES is the ordered exact 7-skill contract; skills_files() filters embedded assets through it

[Gotchas]|adding/renaming/moving assets changes compile-time output |markdown assets must preserve ITO managed markers
|native agent files must never fall back to SKILL.md; Codex has no native agent destination

[Tests]|targeted: cargo test -p ito-templates |installation: cargo test -p ito-cli --test init_more

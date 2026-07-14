## Why

The ito CLI and ito-\* skills are currently separate entry points, making it harder for users to discover and use ito commands from within agent harnesses. This change unifies the ito experience by creating a master 'ito' skill that provides intelligent command routing and fallback to the CLI, while also adding a slash command for easier access.

## What Changes

- Create a new OpenCode skill called 'ito' that handles command routing
- The 'ito' skill will attempt to match commands to existing ito-\* skills first (e.g., 'ito archive' matches 'ito-archive' skill)
- If no matching ito-\* skill exists, the skill will fallback to calling the ito CLI directly
- Create a slash command 'ito.md' that is installed during 'ito init' to enable '/ito <command>' syntax in agent harnesses like opencode
- The skill will pass through command arguments to either the matched skill or the CLI
- Both the ito skill and ito.md slash command will be automatically installed as part of ito init

## Capabilities

### New Capabilities

- `ito-skill-routing`: Intelligent routing of ito commands to matching ito-\* skills with fallback to CLI
- `ito-slash-command`: Installation and execution of '/ito' slash command for agent harness integration

### Modified Capabilities

None

## Impact

- New skill file at `.opencode/skill/ito/`
- New slash command at `.opencode/command/ito.md`
- No breaking changes to existing ito-\* skills or CLI
- Enhances user experience by providing a unified entry point for all ito functionality
- Requires agents to have the 'ito' skill available (automatic via skill system)

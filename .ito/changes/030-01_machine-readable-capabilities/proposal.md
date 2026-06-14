# Change: Machine-Readable Ito Capabilities

## Why

Session mining showed that agents repeatedly infer reasonable Ito commands that do not exist or have different flags. Examples included `ito tasks --json`, `ito list --changes --module 001`, and `ito agent instruction review --change ...`. Each failure forces the LLM to inspect help text, recover intent, and retry.

Ito should make the actual command surface discoverable in a stable, machine-readable form. Agents should ask Ito what exists instead of guessing from prose guidance.

## What

Add a capabilities surface that reports commands, flags, aliases, JSON support, artifact IDs, examples, deprecations, and suggested replacements.

The primary command is:

```bash
ito capabilities --json
```

The command should also support focused queries:

```bash
ito capabilities command tasks --json
ito capabilities artifacts --json
ito capabilities aliases --json
```

## Impact

Agents can validate command availability before using uncommon commands. Prompt templates can steer agents to a deterministic API instead of embedding stale command examples.

## Out Of Scope

This change does not add every missing alias or command. It creates the discovery substrate. Follow-up changes can add specific aliases using the capabilities schema as the contract.

## Success Criteria

- `ito capabilities --json` returns valid JSON without requiring a TTY.
- Command entries include path, summary, flags, positional args, aliases, examples, and whether JSON output is supported.
- Artifact entries include valid `ito agent instruction <artifact>` IDs and required flags.
- Deprecated or compatibility commands can name preferred replacements.
- Tests prove the manifest includes known surfaces such as `list`, `tasks`, `archive`, `agent instruction`, and `validate`.

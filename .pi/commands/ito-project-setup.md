Run `ito agent instruction project-setup` and follow the guide to configure this project for Ito.

Pi installs the Ito extension at `.pi/extensions/ito-skills.ts`.
The extension runs `ito audit validate` and `ito audit reconcile` before tool execution,
injects Ito bootstrap context into the system prompt, and provides the `/ito` command.

Optional environment flags:
- `ITO_PI_AUDIT_DISABLED=1` disables the pre-tool audit hook.
- `ITO_PI_AUDIT_FIX=1` enables `ito audit reconcile --fix` when drift is detected.
- `ITO_PI_AUDIT_TTL_MS=<milliseconds>` overrides the short audit cache TTL (default 10s).
- `ITO_PI_CONTEXT_DISABLED=1` disables Ito context loading.
- `ITO_PI_COMPACTION_DISABLED=1` disables continuation context on session compaction.
- `ITO_PI_DEBUG=1` enables debug logging to stderr.

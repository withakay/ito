Run `ito agent instruction project-setup` and follow the guide to configure this project for Ito.

OpenCode installs the Ito audit hook plugin at `.opencode/plugins/ito-skills.js`.
The plugin runs `ito audit validate` and `ito audit reconcile` before tool execution.

Optional environment flags:
- `ITO_OPENCODE_AUDIT_DISABLED=1` disables the pre-tool audit hook.
- `ITO_OPENCODE_AUDIT_FIX=1` enables `ito audit reconcile --fix` when drift is detected.
- `ITO_OPENCODE_AUDIT_TTL_MS=<milliseconds>` overrides the short audit cache TTL.

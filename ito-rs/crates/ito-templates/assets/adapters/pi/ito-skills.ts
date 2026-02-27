/**
 * Ito Pi Extension
 *
 * Integrates Ito workflows into the Pi coding agent harness.
 *
 * - Injects Ito bootstrap context via system prompt (before_agent_start)
 * - Runs Ito audit checks on tool_call hook with short TTL caching
 * - Warns when mutating tools touch Ito-managed files
 * - Injects Ito continuation context on session compaction
 * - Registers /ito command for direct CLI access
 *
 * Skills are installed to .pi/skills/ by `ito init --tools pi`.
 */

import type { ExtensionAPI } from "@mariozechner/pi-coding-agent";
import { execFileSync } from "node:child_process";

// ─── Configuration ───────────────────────────────────────────────────────────

const DEFAULT_AUDIT_TTL_MS = 10_000;
const ITO_EXEC_TIMEOUT_MS = 20_000;
const ITO_CONTEXT_TTL_MS = 5_000;
const DRIFT_RELATED_TEXT = /(drift|reconcile|mismatch|missing|out\s+of\s+sync)/i;

const ITO_MANAGED_FILE_RULES = [
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/tasks\.md$/,
    advice:
      "[Ito Guardrail] Direct edits to tasks.md detected. Prefer `ito tasks start/complete/shelve/unshelve/add` so audit stays consistent.",
  },
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/(proposal|design)\.md$/,
    advice:
      "[Ito Guardrail] Direct edits to change artifacts detected. Prefer `ito agent instruction proposal|tasks|specs --change <id>` and then `ito validate <id> --strict`.",
  },
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/specs\/[^/]+\/spec\.md$/,
    advice:
      "[Ito Guardrail] Direct edits to spec deltas detected. Prefer `ito agent instruction specs --change <id>` and validate with `ito validate <id> --strict`.",
  },
  {
    pattern: /(^|\/)\.ito\/specs\/[^/]+\/spec\.md$/,
    advice:
      "[Ito Guardrail] Direct edits to canonical specs detected. Prefer change-proposal workflow and validate via `ito validate --specs --strict`.",
  },
];

// Pi built-in tool names that mutate the filesystem.
const MUTATING_TOOLS = new Set(["bash", "write", "edit"]);
const FILE_EDITING_TOOLS = new Set(["write", "edit"]);

// ─── Helpers ─────────────────────────────────────────────────────────────────

interface ExecResult {
  ok: boolean;
  code: number;
  stdout: string;
  stderr: string;
}

function runCmd(cmd: string, args: string[], cwd: string): ExecResult {
  try {
    const stdout = execFileSync(cmd, args, {
      cwd,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
      timeout: ITO_EXEC_TIMEOUT_MS,
    });
    return { ok: true, code: 0, stdout: (stdout || "").trim(), stderr: "" };
  } catch (error: any) {
    return {
      ok: false,
      code: typeof error.status === "number" ? error.status : 1,
      stdout: (typeof error.stdout === "string" ? error.stdout : "").trim(),
      stderr: (typeof error.stderr === "string" ? error.stderr : "").trim(),
    };
  }
}

function summarize(result: ExecResult): string {
  const output = [result.stdout, result.stderr].filter(Boolean).join("\n").trim();
  if (!output) return `exit ${result.code}`;
  const first = output.split(/\r?\n/)[0].trim();
  return first.length > 280 ? `${first.slice(0, 277)}...` : first;
}

function formatTarget(ctx: any): string | null {
  const kind = ctx?.target?.kind;
  const id = ctx?.target?.id;
  if (typeof kind === "string" && typeof id === "string" && id.trim()) {
    return `${kind} ${id}`;
  }
  return null;
}

// ─── Managed-file guardrail helpers ──────────────────────────────────────────

function matchManagedFileAdvice(toolName: string, text: string): string | null {
  if (!text) return null;

  if (toolName === "bash") {
    const maybeMutates =
      /(\>|\>\>|\btee\b|\bsed\s+-i\b|\bcp\b|\bmv\b|\btouch\b|\brm\b|\btruncate\b)/.test(text);
    if (!maybeMutates) return null;
  }

  const normalized = text.replace(/\\/g, "/");
  for (const rule of ITO_MANAGED_FILE_RULES) {
    if (rule.pattern.test(normalized)) {
      return rule.advice;
    }
  }
  return null;
}

function collectLikelyPaths(toolName: string, input: any): string[] {
  const out: string[] = [];
  const push = (v: any) => {
    if (typeof v === "string" && v.trim()) out.push(v.trim());
  };

  if (toolName === "bash") {
    push(input?.command);
    return out;
  }

  // write / edit tools
  push(input?.path);
  push(input?.filePath);
  push(input?.newPath);
  push(input?.oldPath);
  push(input?.to);
  push(input?.patchText);
  return out;
}

// ─── Extension ───────────────────────────────────────────────────────────────

export default function itoSkills(pi: ExtensionAPI) {
  const directory = process.cwd();

  // Environment-driven configuration (mirrors OpenCode plugin env vars).
  const ttlMs = Number.parseInt(process.env.ITO_PI_AUDIT_TTL_MS || "", 10);
  const auditTtlMs = Number.isFinite(ttlMs) && ttlMs > 0 ? ttlMs : DEFAULT_AUDIT_TTL_MS;
  const autoFixDrift = process.env.ITO_PI_AUDIT_FIX !== "0";
  const disableAuditHook = process.env.ITO_PI_AUDIT_DISABLED === "1";
  const disableContext = process.env.ITO_PI_CONTEXT_DISABLED === "1";
  const disableCompactionContext = process.env.ITO_PI_COMPACTION_DISABLED === "1";
  const debugEnabled = process.env.ITO_PI_DEBUG === "1";

  // ── State ────────────────────────────────────────────────────────────────

  let lastAuditAt = 0;
  let lastAudit: { hardFailure: boolean; notice: string | null; message?: string } | null = null;
  let pendingContinuationNotice: string | null = null;
  let bootstrapInjected = false;

  // ── Debug logging ────────────────────────────────────────────────────────

  const debug = (...parts: any[]) => {
    if (!debugEnabled) return;
    const line = `[${new Date().toISOString()}] [ito-pi] ${parts
      .map((p) => {
        if (p == null) return "";
        if (typeof p === "string") return p;
        try {
          return JSON.stringify(p);
        } catch {
          return String(p);
        }
      })
      .join(" ")}`;
    console.error(line); // stderr so it doesn't pollute stdout
  };

  // ── Ito CLI wrappers ────────────────────────────────────────────────────

  const runIto = (args: string[]) => runCmd("ito", args, directory);
  const runGit = (args: string[]) => runCmd("git", args, directory);

  // ── Context loader (cached) ──────────────────────────────────────────────

  let lastContextAt = 0;
  let lastContext: any = null;

  const loadContext = () => {
    if (disableContext) return null;

    const now = Date.now();
    if (lastContext && now - lastContextAt < ITO_CONTEXT_TTL_MS) return lastContext;

    debug("context:load");
    const result = runIto(["agent", "instruction", "context", "--json"]);
    if (!result.ok || !result.stdout) {
      debug("context:failed", summarize(result));
      lastContext = null;
      lastContextAt = now;
      return null;
    }

    try {
      const parsed = JSON.parse(result.stdout);
      debug("context:ok", parsed?.target || null);
      lastContext = parsed;
      lastContextAt = now;
      return parsed;
    } catch {
      debug("context:parse_error");
      lastContext = null;
      lastContextAt = now;
      return null;
    }
  };

  // ── Audit runner ─────────────────────────────────────────────────────────

  const detectDrift = (reconcileResult: ExecResult): boolean => {
    if (!reconcileResult.ok) return true;
    const output = [reconcileResult.stdout, reconcileResult.stderr].join("\n");
    return DRIFT_RELATED_TEXT.test(output);
  };

  const runAuditChecks = () => {
    const validateResult = runIto(["audit", "validate"]);
    if (!validateResult.ok) {
      return {
        hardFailure: true,
        notice: null,
        message: `Ito audit validation failed: ${summarize(validateResult)}`,
      };
    }

    const reconcileResult = runIto(["audit", "reconcile"]);
    const driftDetected = detectDrift(reconcileResult);

    if (!driftDetected) {
      return { hardFailure: false, notice: null };
    }

    if (autoFixDrift) {
      const fixResult = runIto(["audit", "reconcile", "--fix"]);
      const fixSummary = summarize(fixResult);
      return {
        hardFailure: false,
        // Silent on success — only warn when auto-fix fails.
        notice: fixResult.ok
          ? null
          : `[Ito Audit] Drift detected; auto-fix failed: ${fixSummary}`,
      };
    }

    return {
      hardFailure: false,
      notice: `[Ito Audit] Drift detected: ${summarize(reconcileResult)}`,
    };
  };

  const maybeRunAudit = (toolName: string) => {
    const now = Date.now();
    const isMutating = MUTATING_TOOLS.has(toolName);
    const cacheExpired = now - lastAuditAt > auditTtlMs;

    if (!lastAudit || cacheExpired || isMutating) {
      lastAudit = runAuditChecks();
      lastAuditAt = now;
    }
    return lastAudit;
  };

  // ── Bootstrap content ────────────────────────────────────────────────────

  const getBootstrapContent = (): string => {
    try {
      const bootstrap = execFileSync(
        "ito",
        ["agent", "instruction", "bootstrap", "--tool", "pi"],
        {
          cwd: directory,
          encoding: "utf8",
          stdio: ["ignore", "pipe", "ignore"],
          timeout: ITO_EXEC_TIMEOUT_MS,
        }
      ).trim();

      const fallback = `You have access to Ito workflows via skills prefixed with \`ito-\`.

Load a skill with Pi's native skill command. Start with:
\`\`\`
/skill:using-ito-skills
\`\`\`

Skills are in \`.pi/skills/\`, commands in \`.pi/commands/\`.`;

      return bootstrap.length > 0 ? bootstrap : fallback;
    } catch {
      return `Ito integration is configured, but the Ito CLI is not available.

Use \`/skill:using-ito-skills\` to load Ito workflows if skills are installed.`;
    }
  };

  // ── Event handlers ─────────────────────────────────────────────────────

  debug("plugin:init", { directory });

  // Inject Ito bootstrap into system prompt on every agent turn.
  pi.on("before_agent_start", async (event, ctx) => {
    const parts: string[] = [];

    // Bootstrap preamble (always inject — Pi doesn't persist system prompt
    // modifications across turns the way OpenCode does).
    const bootstrap = getBootstrapContent();
    parts.push(bootstrap);

    // Pending continuation context from compaction.
    if (pendingContinuationNotice) {
      parts.push(pendingContinuationNotice);
      pendingContinuationNotice = null;
    }

    // Pending audit notice.
    if (lastAudit?.notice) {
      parts.push(lastAudit.notice);
    }

    if (!bootstrapInjected) {
      bootstrapInjected = true;
      const ctx2 = loadContext();
      const target = formatTarget(ctx2);
      ctx.ui.notify(
        target ? `Ito: target ${target}` : "Ito bootstrap injected",
        "success"
      );

      // Worktree detection toast
      const gitDirResult = runGit(["rev-parse", "--git-dir"]);
      if (gitDirResult.ok && gitDirResult.stdout.includes("/worktrees/")) {
        ctx.ui.notify(`Git worktree: ${gitDirResult.stdout}`, "info");
      }
    }

    if (parts.length === 0) return;

    const injection = parts.join("\n\n");
    return {
      systemPrompt: event.systemPrompt + "\n\n" + injection,
    };
  });

  // Audit + managed-file guardrails on every tool call.
  pi.on("tool_call", async (event, ctx) => {
    if (disableAuditHook) return;

    const toolName = event.toolName;

    // Managed-file write warnings — collect and notify.
    if (FILE_EDITING_TOOLS.has(toolName) || toolName === "bash") {
      const paths = collectLikelyPaths(toolName, event.input);
      for (const p of paths) {
        const advice = matchManagedFileAdvice(toolName, p);
        if (advice) {
          ctx.ui.notify(advice, "warning");
        }
      }
    }

    // Run audit checks (TTL-cached).
    const audit = maybeRunAudit(toolName);

    if (audit?.hardFailure) {
      ctx.ui.notify(
        `${audit.message}. Run \`ito audit validate\` and \`ito audit reconcile --fix\`.`,
        "error"
      );
      // Block the tool call on hard audit failure.
      return { block: true, reason: audit.message || "Ito audit validation failed" };
    }

    if (audit?.notice) {
      ctx.ui.notify(audit.notice, "warning");
    }
  });

  // Inject Ito continuation context into compaction.
  pi.on("session_compact", async (_event, ctx) => {
    if (disableCompactionContext) return;

    const itoCtx = loadContext();
    if (itoCtx?.nudge) {
      pendingContinuationNotice = `[Ito Continuation] ${itoCtx.nudge}`;
    }

    const target = formatTarget(itoCtx);
    ctx.ui.notify(
      target ? `Session compacted — continue: ${target}` : "Session compacted",
      "info"
    );
  });

  // Register /ito command for quick access.
  pi.registerCommand("ito", {
    description: "Run an ito CLI command (e.g., /ito audit validate)",
    handler: async (args, ctx) => {
      if (!args?.trim()) {
        ctx.ui.notify("Usage: /ito <command> [args...]", "info");
        return;
      }
      const argv = args.trim().split(/\s+/);
      const result = runIto(argv);
      const output = [result.stdout, result.stderr].filter(Boolean).join("\n").trim();
      ctx.ui.notify(
        output ? `ito ${argv[0]}: ${output.slice(0, 200)}` : `ito ${argv[0]}: exit ${result.code}`,
        result.ok ? "success" : "error"
      );
    },
  });
}

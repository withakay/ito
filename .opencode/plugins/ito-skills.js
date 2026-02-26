/**
 * Ito OpenCode Plugin
 *
 * Injects Ito bootstrap context via system prompt transform.
 * Runs Ito audit checks on pre-tool hook with short TTL caching.
 * Skills are resolved from ${OPENCODE_CONFIG_DIR}/skills/ito-skills/
 * (never via relative paths to the plugin file).
 */

import os from 'os';
import path from 'path';
import { execFileSync } from 'child_process';

const DEFAULT_AUDIT_TTL_MS = 10000;
const ITO_EXEC_TIMEOUT_MS = 20000;
const ITO_CONTEXT_TTL_MS = 5000;
const DRIFT_RELATED_TEXT = /(drift|reconcile|mismatch|missing|out\s+of\s+sync)/i;

const ITO_MANAGED_FILE_RULES = [
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/tasks\.md/,
    advice: '[Ito Guardrail] Direct edits to tasks.md detected. Prefer `ito tasks start/complete/shelve/unshelve/add` so audit stays consistent.'
  },
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/(proposal|design)\.md/,
    advice: '[Ito Guardrail] Direct edits to change artifacts detected. Prefer `ito agent instruction proposal|tasks|specs --change <id>` and then `ito validate <id> --strict`.'
  },
  {
    pattern: /(^|\/)\.ito\/changes\/[^/]+\/specs\/[^/]+\/spec\.md/,
    advice: '[Ito Guardrail] Direct edits to spec deltas detected. Prefer `ito agent instruction specs --change <id>` and validate with `ito validate <id> --strict`.'
  },
  {
    pattern: /(^|\/)\.ito\/specs\/[^/]+\/spec\.md/,
    advice: '[Ito Guardrail] Direct edits to canonical specs detected. Prefer change-proposal workflow and validate via `ito validate --specs --strict`.'
  }
];

const MUTATING_TOOLS = new Set([
  'Edit',
  'Write',
  'Bash',
  'MultiEdit',
  'Task',
  'TodoWrite',
  'apply_patch'
]);

const FILE_EDITING_TOOLS = new Set(['Edit', 'Write', 'MultiEdit', 'apply_patch']);

export const ItoPlugin = async ({ client, directory }) => {
  const homeDir = os.homedir();
  const envConfigDir = process.env.OPENCODE_CONFIG_DIR?.trim();
  const configDir = envConfigDir || path.join(homeDir, '.config/opencode');
  const skillsDir = path.join(configDir, 'skills');
  const ttlMs = Number.parseInt(process.env.ITO_OPENCODE_AUDIT_TTL_MS || '', 10);
  const auditTtlMs = Number.isFinite(ttlMs) && ttlMs > 0 ? ttlMs : DEFAULT_AUDIT_TTL_MS;
  const autoFixDrift = process.env.ITO_OPENCODE_AUDIT_FIX === '1';
  const disableAuditHook = process.env.ITO_OPENCODE_AUDIT_DISABLED === '1';

  let lastAuditAt = 0;
  let lastAudit = null;
  let pendingAuditNotice = null;

  let bootstrapToastSent = false;
  let worktreeToastSent = false;
  let pendingContinuationNotice = null;

  let lastContextAt = 0;
  let lastContext = null;

  const showToast = async ({ title, message, variant = 'info', duration }) => {
    if (!client?.tui?.showToast) {
      return;
    }

    try {
      await client.tui.showToast({
        body: {
          title,
          message,
          variant,
          duration
        }
      });
    } catch {
      // Best-effort only.
    }
  };

  const runGit = (args) => {
    try {
      const stdout = execFileSync('git', args, {
        cwd: directory,
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'pipe'],
        timeout: ITO_EXEC_TIMEOUT_MS
      });

      return {
        ok: true,
        code: 0,
        stdout: (stdout || '').trim(),
        stderr: ''
      };
    } catch (error) {
      const stdout = typeof error.stdout === 'string' ? error.stdout : '';
      const stderr = typeof error.stderr === 'string' ? error.stderr : '';
      const code = typeof error.status === 'number' ? error.status : 1;

      return {
        ok: false,
        code,
        stdout: stdout.trim(),
        stderr: stderr.trim()
      };
    }
  };

  const runIto = (args) => {
    try {
      const stdout = execFileSync('ito', args, {
        cwd: directory,
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'pipe'],
        timeout: ITO_EXEC_TIMEOUT_MS
      });

      return {
        ok: true,
        code: 0,
        stdout: (stdout || '').trim(),
        stderr: ''
      };
    } catch (error) {
      const stdout = typeof error.stdout === 'string' ? error.stdout : '';
      const stderr = typeof error.stderr === 'string' ? error.stderr : '';
      const code = typeof error.status === 'number' ? error.status : 1;

      return {
        ok: false,
        code,
        stdout: stdout.trim(),
        stderr: stderr.trim()
      };
    }
  };

  const summarize = (result) => {
    const output = [result.stdout, result.stderr].filter(Boolean).join('\n').trim();
    if (output.length === 0) {
      return `exit ${result.code}`;
    }

    const firstLine = output.split(/\r?\n/)[0].trim();
    return firstLine.length > 280 ? `${firstLine.slice(0, 277)}...` : firstLine;
  };

  const formatTarget = (ctx) => {
    const kind = ctx?.target?.kind;
    const id = ctx?.target?.id;
    if (typeof kind === 'string' && typeof id === 'string' && id.trim()) {
      return `${kind} ${id}`;
    }
    return null;
  };

  const loadContext = () => {
    const now = Date.now();
    if (lastContext && now - lastContextAt < ITO_CONTEXT_TTL_MS) {
      return lastContext;
    }

    const result = runIto(['agent', 'instruction', 'context', '--json']);
    if (!result.ok || !result.stdout) {
      lastContext = null;
      lastContextAt = now;
      return null;
    }

    try {
      const parsed = JSON.parse(result.stdout);
      lastContext = parsed;
      lastContextAt = now;
      return parsed;
    } catch {
      lastContext = null;
      lastContextAt = now;
      return null;
    }
  };

  const maybeToastWorktree = async () => {
    if (worktreeToastSent) {
      return;
    }

    const gitDirResult = runGit(['rev-parse', '--git-dir']);
    if (!gitDirResult.ok) {
      return;
    }

    const gitDir = (gitDirResult.stdout || '').replace(/\\/g, '/');
    if (!gitDir.includes('/worktrees/')) {
      return;
    }

    worktreeToastSent = true;
    await showToast({
      title: 'Git Worktree Detected',
      message: gitDirResult.stdout || 'worktree',
      variant: 'info',
      duration: 5000
    });
  };

  const detectDrift = (reconcileResult) => {
    if (!reconcileResult.ok) {
      return true;
    }

    const output = [reconcileResult.stdout, reconcileResult.stderr].join('\n');
    return DRIFT_RELATED_TEXT.test(output);
  };

  const addSystemNotice = (output, notice) => {
    if (!output || typeof output !== 'object') {
      return;
    }
    if (!Array.isArray(output.system)) {
      output.system = [];
    }
    output.system.push(notice);
  };

  const coerceString = (value) => {
    if (typeof value === 'string') {
      return value;
    }
    if (typeof value === 'number' || typeof value === 'boolean') {
      return String(value);
    }
    return '';
  };

  const collectLikelyPaths = (toolName, input) => {
    const out = [];
    const push = (value) => {
      const text = coerceString(value).trim();
      if (!text) {
        return;
      }
      out.push(text);
    };

    const toolInput = input?.tool?.input || input?.toolInput || input?.input || {};
    if (toolName === 'Bash') {
      push(toolInput.command || input?.tool?.command || input?.command);
      return out;
    }

    push(toolInput.filePath);
    push(toolInput.path);
    push(toolInput.newPath);
    push(toolInput.oldPath);
    push(toolInput.to);
    push(toolInput.patchText);

    return out;
  };

  const matchManagedFileAdvice = (toolName, text) => {
    if (!text) {
      return null;
    }

    if (toolName === 'Bash') {
      const maybeMutates = /(\>|\>\>|\btee\b|\bsed\s+-i\b|\bcp\b|\bmv\b|\btouch\b|\brm\b|\btruncate\b)/.test(text);
      if (!maybeMutates) {
        return null;
      }
    }

    const normalized = text.replace(/\\/g, '/');
    for (const rule of ITO_MANAGED_FILE_RULES) {
      if (rule.pattern.test(normalized)) {
        return rule.advice;
      }
    }

    return null;
  };

  const maybeWarnForManagedFileWrites = (toolName, input, output) => {
    const paths = collectLikelyPaths(toolName, input);
    const notices = new Set();

    for (const value of paths) {
      const advice = matchManagedFileAdvice(toolName, value);
      if (advice) {
        notices.add(advice);
      }
    }

    for (const notice of notices) {
      addSystemNotice(output, notice);
    }
  };

  const runAuditChecks = () => {
    const validateResult = runIto(['audit', 'validate']);
    if (!validateResult.ok) {
      return {
        hardFailure: true,
        message: `Ito audit validation failed: ${summarize(validateResult)}`
      };
    }

    const reconcileResult = runIto(['audit', 'reconcile']);
    const driftDetected = detectDrift(reconcileResult);

    if (!driftDetected) {
      return {
        hardFailure: false,
        notice: null
      };
    }

    if (autoFixDrift) {
      const fixResult = runIto(['audit', 'reconcile', '--fix']);
      const fixSummary = summarize(fixResult);
      return {
        hardFailure: false,
        notice: fixResult.ok
          ? `[Ito Audit] Drift detected and reconciled: ${fixSummary}`
          : `[Ito Audit] Drift detected; auto-fix failed: ${fixSummary}`
      };
    }

    return {
      hardFailure: false,
      notice: `[Ito Audit] Drift detected: ${summarize(reconcileResult)}`
    };
  };

  const maybeRunAudit = (toolName) => {
    const now = Date.now();
    const isMutatingTool = MUTATING_TOOLS.has(toolName);
    const cacheExpired = now - lastAuditAt > auditTtlMs;

    if (!lastAudit || cacheExpired || isMutatingTool) {
      lastAudit = runAuditChecks();
      lastAuditAt = now;
    }

    return lastAudit;
  };

  // Get bootstrap content from Ito CLI
  const getBootstrapContent = () => {
    try {
      const bootstrap = execFileSync('ito', ['agent', 'instruction', 'bootstrap', '--tool', 'opencode'], {
        cwd: directory,
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'ignore'],
        timeout: ITO_EXEC_TIMEOUT_MS
      }).trim();

      const fallback = `You have access to Ito workflows via skills prefixed with \`ito-\`.

Load a skill with OpenCode's native \`skill\` tool. Start with:
\`\`\`
use skill tool to load using-ito-skills
\`\`\`

Skills are installed to: \`${skillsDir}\``;

      const content = bootstrap.length > 0 ? bootstrap : fallback;
      return `<EXTREMELY_IMPORTANT>
 ${content}
 </EXTREMELY_IMPORTANT>`;
    } catch (error) {
      // Graceful degradation if CLI is not available
      return `<EXTREMELY_IMPORTANT>
Ito integration is configured, but the Ito CLI is not available.

Ito skills should be installed to: \`${skillsDir}\`

Use OpenCode's native \`skill\` tool to load Ito workflows.
</EXTREMELY_IMPORTANT>`;
    }
  };

  await maybeToastWorktree();

  return {
    event: async ({ event }) => {
      if (!event || typeof event.type !== 'string') {
        return;
      }

      if (event.type === 'session.compacted') {
        const ctx = loadContext();
        if (ctx?.nudge) {
          pendingContinuationNotice = `[Ito Continuation] ${ctx.nudge}`;
        }

        const target = formatTarget(ctx);
        await showToast({
          title: 'Session Compacted',
          message: target ? `Continue: ${target}` : 'Continue',
          variant: 'info',
          duration: 4500
        });
      }
    },

    'experimental.session.compacting': async (_input, output) => {
      const ctx = loadContext();
      if (!ctx?.nudge) {
        return;
      }

      if (!Array.isArray(output.context)) {
        output.context = [];
      }
      output.context.push(`## Ito Continuation\n${ctx.nudge}`);
    },

    'tool.execute.before': async (input, output) => {
      if (disableAuditHook) {
        return;
      }

      const toolName = input?.tool?.name || input?.toolName || '';

      if (pendingContinuationNotice) {
        addSystemNotice(output, pendingContinuationNotice);
        pendingContinuationNotice = null;
      }

      if (FILE_EDITING_TOOLS.has(toolName) || toolName === 'Bash') {
        maybeWarnForManagedFileWrites(toolName, input, output);
      }

      const audit = maybeRunAudit(toolName);

      if (audit?.hardFailure) {
        throw new Error(`${audit.message}. Run \`ito audit validate\` and \`ito audit reconcile --fix\`.`);
      }

      if (audit?.notice) {
        pendingAuditNotice = audit.notice;
        addSystemNotice(output, audit.notice);
      }
    },

    // Use system prompt transform to inject bootstrap
    'experimental.chat.system.transform': async (_input, output) => {
      const bootstrap = getBootstrapContent();
      if (bootstrap) {
        if (!Array.isArray(output.system)) {
          output.system = [];
        }
        output.system.push(bootstrap);

        if (!bootstrapToastSent) {
          bootstrapToastSent = true;
          const ctx = loadContext();
          const target = formatTarget(ctx);
          await showToast({
            title: 'Ito Prompt Injected',
            message: target ? `Target: ${target}` : 'Bootstrap injected',
            variant: 'success',
            duration: 3500
          });
        }
      }

      if (pendingAuditNotice) {
        if (!Array.isArray(output.system)) {
          output.system = [];
        }
        output.system.push(pendingAuditNotice);
        pendingAuditNotice = null;
      }
    }
  };
};

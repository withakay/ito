/**
 * Ito OpenCode Plugin
 *
 * Injects Ito bootstrap context via system prompt transform.
 * Skills are resolved from ${OPENCODE_CONFIG_DIR}/skills/ito-skills/
 * (never via relative paths to the plugin file).
 */

import os from 'os';
import path from 'path';
import { execSync } from 'child_process';

export const ItoPlugin = async ({ client, directory }) => {
  const homeDir = os.homedir();
  const envConfigDir = process.env.OPENCODE_CONFIG_DIR?.trim();
  const configDir = envConfigDir || path.join(homeDir, '.config/opencode');
  const skillsDir = path.join(configDir, 'skills', 'ito-skills');

  // Get bootstrap content from Ito CLI
  const getBootstrapContent = () => {
    try {
      const bootstrap = execSync('ito agent instruction bootstrap --tool opencode', {
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'ignore']
      }).trim();

      const fallback = `You have access to Ito workflows.

To load a Ito workflow, use OpenCode's native \`skill\` tool:
\`\`\`
use skill tool to load ito-skills/<workflow-name>
\`\`\`

Ito skills are available at: \`${skillsDir}\`

**Tool Mapping for OpenCode:**
When Ito workflows reference Claude Code tools, use these OpenCode equivalents:
- \`TodoWrite\` → \`update_plan\`
- \`Task\` tool with subagents → Use OpenCode's subagent system (@mention)
- \`Skill\` tool → OpenCode's native \`skill\` tool
- \`Read\`, \`Write\`, \`Edit\`, \`Bash\` → Your native tools

**Getting Started:**
List available Ito skills:
\`\`\`
use skill tool to list skills
\`\`\`

Load a specific workflow:
\`\`\`
use skill tool to load ito-skills/using-ito-skills
\`\`\``;

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

  return {
    // Use system prompt transform to inject bootstrap
    'experimental.chat.system.transform': async (_input, output) => {
      const bootstrap = getBootstrapContent();
      if (bootstrap) {
        (output.system ||= []).push(bootstrap);
      }
    }
  };
};

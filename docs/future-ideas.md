# Future Ideas

This document captures concepts and features that have been considered for Ito but are **not yet implemented**. These ideas may be explored in future versions.

> **Note:** Everything in this document represents aspirational features. For documentation of current Ito capabilities, see [agent-workflow.md](./agent-workflow.md).

## Custom Schemas

Currently, Ito uses a single `spec-driven` schema. Future versions could support multiple workflow schemas:

### Research-First Schema

- Prioritize research artifacts before proposals
- Useful for exploratory work where the solution isn't yet clear
- Flow: research → proposal → specs → tasks

### TDD Schema

- Test-driven development workflow
- Write test specs before implementation specs
- Flow: proposal → test-specs → implementation-specs → tasks

### Minimal Schema

- Lightweight workflow for small changes
- Skip specs entirely for trivial fixes
- Flow: proposal → tasks

### Schema Customization

- User-defined schemas at `~/.local/share/ito/schemas/`
- `ito x-schemas` command to list and manage available schemas
- Project-level schema overrides in `.ito/config.yaml`

## Additional CLI Commands

### Explore Command (`/ito-explore`)

- Interactive codebase exploration tied to changes
- Capture exploration findings directly into research artifacts
- Link discoveries to specific capabilities

### Sync Specs Command (`/ito-sync-specs`)

- Synchronize specs with implementation
- Detect drift between spec requirements and actual code
- Generate reports on spec coverage

### Fast-Forward Command (`/ito-ff-change`)

- Skip to specific artifact in workflow
- Useful when some artifacts are already prepared
- Validate that skipped artifacts would pass

## Granular Artifact Creation

### Continue Change (`/ito-continue-change`)

- Create artifacts one at a time interactively
- Better control over artifact generation
- Pause and resume artifact creation

### Artifact Dependencies

- Explicit dependency tracking between artifacts
- Automatic re-validation when dependencies change
- Visual dependency graph

## Enhanced Validation

### Validation Feedback

- Detailed feedback on why validation failed
- Suggestions for fixing validation errors
- Progressive validation during artifact creation

### Spec Coverage Analysis

- Track which specs are covered by tests
- Identify gaps in implementation
- Generate coverage reports

## Workflow Improvements

### Context Setting

- Persistent context across agent sessions
- Remember previous work on a change
- Automatic context restoration

### Change Templates

- Pre-built templates for common change types
- Feature, bugfix, refactor, documentation templates
- Custom project-specific templates

### Multi-Change Coordination

- Track dependencies between changes
- Coordinate implementation order
- Conflict detection between parallel changes

## Integration Ideas

### Git Integration

- Automatic branch creation for changes
- Commit message generation from change context
- PR description generation from proposals

### IDE Integration

- VS Code extension for Ito
- Inline spec viewing
- Task progress in sidebar

### CI/CD Integration

- Validation in CI pipelines
- Automatic spec verification on PR
- Change tracking in deployment logs

## OPSX Fluid Workflow Model

The original "OPSX" (Operations Specification) model envisioned a more fluid approach:

### Granular Artifacts

- Smaller, more focused artifact types
- Mix and match artifacts based on need
- No rigid schema requirements

### Dynamic Dependency Tracking

- Real-time dependency resolution
- Automatic artifact ordering
- Lazy artifact generation

### Collaborative Workflows

- Multiple agents working on same change
- Conflict resolution between agents
- Parallel task execution

______________________________________________________________________

## Contributing Ideas

Have an idea for Ito? Consider:

1. Does it fit the spec-driven philosophy?
1. Would it simplify agent workflows?
1. Is it generally useful or too specific?

Open an issue or discussion to propose new features.

//! Agent memory provider resolution and instruction rendering.
//!
//! Memory in Ito is intentionally provider-agnostic. The user configures one
//! of three operations (`capture`, `search`, `query`) under
//! `ItoConfig.memory`; each operation independently picks either an inline
//! command template or a delegated skill. This module turns those
//! configurations plus runtime inputs into a [`RenderedInstruction`] that
//! the agent-facing CLI can print verbatim.
//!
//! See `.ito/specs/agent-memory-abstraction/` for the authoritative spec.

use std::collections::BTreeMap;

use ito_config::types::{MemoryConfig, MemoryOpConfig};
use serde_json::Value;

mod rendering;
#[cfg(test)]
mod rendering_tests;

pub use rendering::shell_quote;

/// Default value applied to `memory-search`'s `--limit` flag when the caller
/// does not supply one.
pub const DEFAULT_SEARCH_LIMIT: u64 = 10;

/// Inputs accepted by `ito agent instruction memory-capture`.
#[derive(Debug, Clone, Default)]
pub struct CaptureInputs {
    /// Free-form context describing what to remember.
    pub context: Option<String>,
    /// Files to include when the configured provider supports file context.
    pub files: Vec<String>,
    /// Folders to include when the configured provider supports folder packs.
    pub folders: Vec<String>,
}

/// Inputs accepted by `ito agent instruction memory-search`.
#[derive(Debug, Clone)]
pub struct SearchInputs {
    /// Search query (required).
    pub query: String,
    /// Maximum results. `None` falls back to [`DEFAULT_SEARCH_LIMIT`].
    pub limit: Option<u64>,
    /// Optional path-prefix scope (e.g. `auth/`).
    pub scope: Option<String>,
}

/// Inputs accepted by `ito agent instruction memory-query`.
#[derive(Debug, Clone)]
pub struct QueryInputs {
    /// Natural-language question (required).
    pub query: String,
}

/// Identifies a memory operation for diagnostics and template rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// `capture` — store / curate a memory.
    Capture,
    /// `search` — structured BM25-style ranked lookup.
    Search,
    /// `query` — synthesized natural-language answer over stored memory.
    Query,
}

impl Operation {
    /// Lower-case, brv-compatible key for this operation.
    #[must_use]
    pub fn as_key(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Search => "search",
            Self::Query => "query",
        }
    }
}

/// Output of resolving and rendering a memory operation.
///
/// Three render branches mirror the three states a single operation can be
/// in (configured-as-command / configured-as-skill / not-configured); see
/// the `agent-memory-abstraction` spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderedInstruction {
    /// Operation is configured with `kind: "command"` — the rendered shell
    /// command line is ready for the agent to execute.
    Command {
        /// Final command line with placeholders substituted.
        line: String,
    },
    /// Operation is configured with `kind: "skill"` — the agent should
    /// invoke the named skill with the listed inputs and opaque options.
    Skill {
        /// Skill identifier as configured.
        skill_id: String,
        /// Structured inputs for this operation, named by the operation's
        /// schema (`context`, `files`, `folders`, `query`, `limit`,
        /// `scope`).
        inputs: BTreeMap<String, Value>,
        /// Opaque per-skill options from configuration.
        ///
        /// `None` when no `options` field was supplied; the skill receives
        /// nothing for it and decides on its own defaults.
        options: Option<Value>,
    },
    /// Operation is not configured — the artifact should print provider
    /// setup guidance (the caller embeds [`Operation`] context to build a
    /// useful hint).
    NotConfigured {
        /// Which operation is missing.
        operation: Operation,
    },
}

/// Render the `memory-capture` instruction for the given config and inputs.
///
/// Returns [`RenderedInstruction::NotConfigured`] when `memory.capture` is
/// absent, regardless of how `search` and `query` are configured.
#[must_use]
pub fn render_capture(
    config: Option<&MemoryConfig>,
    inputs: &CaptureInputs,
) -> RenderedInstruction {
    let op_cfg = config.and_then(|c| c.capture.as_ref());
    let Some(op_cfg) = op_cfg else {
        return RenderedInstruction::NotConfigured {
            operation: Operation::Capture,
        };
    };
    match op_cfg {
        MemoryOpConfig::Command { command } => RenderedInstruction::Command {
            line: rendering::render_capture_command(command, inputs),
        },
        MemoryOpConfig::Skill { skill, options } => RenderedInstruction::Skill {
            skill_id: skill.clone(),
            inputs: rendering::capture_inputs_as_structured(inputs),
            options: options.clone(),
        },
    }
}

/// Render the `memory-search` instruction for the given config and inputs.
#[must_use]
pub fn render_search(
    config: Option<&MemoryConfig>,
    inputs: &SearchInputs,
) -> RenderedInstruction {
    let op_cfg = config.and_then(|c| c.search.as_ref());
    let Some(op_cfg) = op_cfg else {
        return RenderedInstruction::NotConfigured {
            operation: Operation::Search,
        };
    };
    match op_cfg {
        MemoryOpConfig::Command { command } => RenderedInstruction::Command {
            line: rendering::render_search_command(command, inputs),
        },
        MemoryOpConfig::Skill { skill, options } => RenderedInstruction::Skill {
            skill_id: skill.clone(),
            inputs: rendering::search_inputs_as_structured(inputs),
            options: options.clone(),
        },
    }
}

/// Render the `memory-query` instruction for the given config and inputs.
#[must_use]
pub fn render_query(
    config: Option<&MemoryConfig>,
    inputs: &QueryInputs,
) -> RenderedInstruction {
    let op_cfg = config.and_then(|c| c.query.as_ref());
    let Some(op_cfg) = op_cfg else {
        return RenderedInstruction::NotConfigured {
            operation: Operation::Query,
        };
    };
    match op_cfg {
        MemoryOpConfig::Command { command } => RenderedInstruction::Command {
            line: rendering::render_query_command(command, inputs),
        },
        MemoryOpConfig::Skill { skill, options } => RenderedInstruction::Skill {
            skill_id: skill.clone(),
            inputs: rendering::query_inputs_as_structured(inputs),
            options: options.clone(),
        },
    }
}

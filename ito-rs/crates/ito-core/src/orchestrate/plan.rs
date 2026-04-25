//! Orchestrator run planning.
//!
//! Builds a deterministic, dependency-aware execution plan for a multi-change
//! orchestration run.

use crate::errors::{CoreError, CoreResult};
use crate::orchestrate::gates::default_gate_order;
use crate::orchestrate::types::{
    FailurePolicy, GatePolicy, OrchestrateRunConfig, parse_max_parallel,
};
use ito_domain::changes::ChangeOrchestrateMetadata;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// One gate in a planned pipeline.
pub struct PlannedGate {
    /// Gate identifier (e.g. `tests`, `code-review`).
    pub name: String,
    /// Whether the gate should run or be recorded as skipped.
    pub policy: GatePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Planned execution for a single change.
pub struct PlannedChange {
    /// Canonical change id.
    pub id: String,
    /// Canonical change ids that must complete before this change begins.
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// Gate pipeline for this change.
    pub gates: Vec<PlannedGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Resolved plan for an orchestration run.
pub struct RunPlan {
    /// Run id.
    pub run_id: String,
    /// Preset name used for this run.
    pub preset: String,
    /// Maximum number of concurrent change pipelines.
    pub max_parallel: usize,
    /// Run-level failure policy.
    pub failure_policy: FailurePolicy,
    /// Planned changes in dependency order.
    pub changes: Vec<PlannedChange>,
}

#[derive(Debug, Clone)]
/// Input required to plan a change.
pub struct ChangePlanInput {
    /// Canonical change id.
    pub id: String,
    /// Per-change orchestration metadata.
    pub orchestrate: ChangeOrchestrateMetadata,
}

/// Build a dependency-aware run plan.
///
/// Dependencies are enforced via a topological sort. Cycles are rejected.
pub fn build_run_plan(
    run_id: &str,
    preset: &str,
    config: OrchestrateRunConfig,
    changes: Vec<ChangePlanInput>,
) -> CoreResult<RunPlan> {
    let max_parallel = parse_max_parallel(config.max_parallel, config.max_parallel_cap)?;
    let failure_policy = config.failure_policy.unwrap_or(FailurePolicy::Remediate);

    let change_ids = collect_change_ids(&changes);
    let deps = build_dependency_map(&changes, &change_ids);

    let ordered_ids = topo_sort(&deps)?;
    let mut changes_by_id: BTreeMap<String, ChangePlanInput> = BTreeMap::new();
    for c in changes {
        changes_by_id.insert(c.id.clone(), c);
    }

    let default_order = if config.gate_order.is_empty() {
        default_gate_order()
    } else {
        config.gate_order.clone()
    };
    let skip_gates = config.skip_gates;

    let mut planned = Vec::new();
    for id in ordered_ids {
        let Some(input) = changes_by_id.remove(&id) else {
            continue;
        };

        let gate_names = resolve_gate_names(&input, &default_order);
        let gates = build_planned_gates(gate_names, &skip_gates);

        let depends_on = deps
            .get(&input.id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();

        planned.push(PlannedChange {
            id: input.id,
            depends_on,
            gates,
        });
    }

    Ok(RunPlan {
        run_id: run_id.to_string(),
        preset: preset.to_string(),
        max_parallel,
        failure_policy,
        changes: planned,
    })
}

fn collect_change_ids(changes: &[ChangePlanInput]) -> BTreeSet<String> {
    let mut change_ids = BTreeSet::new();
    for change in changes {
        change_ids.insert(change.id.clone());
    }
    change_ids
}

fn build_dependency_map(
    changes: &[ChangePlanInput],
    change_ids: &BTreeSet<String>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut deps = BTreeMap::new();
    for change in changes {
        let depends_on = change
            .orchestrate
            .depends_on
            .iter()
            .filter(|dep| change_ids.contains(dep.as_str()))
            .cloned()
            .collect();
        deps.insert(change.id.clone(), depends_on);
    }
    deps
}

fn resolve_gate_names(input: &ChangePlanInput, default_order: &[String]) -> Vec<String> {
    if !input.orchestrate.preferred_gates.is_empty() {
        return input.orchestrate.preferred_gates.clone();
    }

    default_order.to_vec()
}

fn build_planned_gates(gate_names: Vec<String>, skip_gates: &BTreeSet<String>) -> Vec<PlannedGate> {
    gate_names
        .into_iter()
        .map(|name| PlannedGate {
            policy: gate_policy_for_name(skip_gates, &name),
            name,
        })
        .collect()
}

fn gate_policy_for_name(skip_gates: &BTreeSet<String>, gate_name: &str) -> GatePolicy {
    if skip_gates.contains(gate_name) {
        return GatePolicy::Skip;
    }

    GatePolicy::Run
}

fn topo_sort(deps: &BTreeMap<String, BTreeSet<String>>) -> CoreResult<Vec<String>> {
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    let mut reverse: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (node, node_deps) in deps {
        indegree.insert(node.clone(), node_deps.len());
        for dep in node_deps {
            reverse.entry(dep.clone()).or_default().insert(node.clone());
        }
    }

    let mut q: VecDeque<String> = VecDeque::new();
    for (node, d) in &indegree {
        if *d == 0 {
            q.push_back(node.clone());
        }
    }

    let mut out = Vec::new();
    while let Some(n) = q.pop_front() {
        out.push(n.clone());
        let Some(children) = reverse.get(&n) else {
            continue;
        };
        for child in children {
            if let Some(d) = indegree.get_mut(child) {
                *d = d.saturating_sub(1);
                if *d == 0 {
                    q.push_back(child.clone());
                }
            }
        }
    }

    if out.len() == deps.len() {
        return Ok(out);
    }

    // Cycle detection for a clearer error message.
    let cycle = find_cycle(deps).unwrap_or_else(|| vec!["<unknown>".to_string()]);
    Err(CoreError::Validation(format!(
        "circular depends_on cycle detected: {}",
        cycle.join(" -> ")
    )))
}

fn find_cycle(deps: &BTreeMap<String, BTreeSet<String>>) -> Option<Vec<String>> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mark {
        Temp,
        Perm,
    }

    fn dfs(
        node: &str,
        deps: &BTreeMap<String, BTreeSet<String>>,
        marks: &mut BTreeMap<String, Mark>,
        stack: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        match marks.get(node) {
            Some(Mark::Temp) => {
                return Some(build_cycle(stack, node));
            }
            Some(Mark::Perm) => {
                return None;
            }
            None => {}
        }

        marks.insert(node.to_string(), Mark::Temp);
        stack.push(node.to_string());
        if let Some(next) = deps.get(node) {
            for dep in next {
                if let Some(cycle) = dfs(dep, deps, marks, stack) {
                    return Some(cycle);
                }
            }
        }
        stack.pop();
        marks.insert(node.to_string(), Mark::Perm);
        None
    }

    let mut marks: BTreeMap<String, Mark> = BTreeMap::new();
    for node in deps.keys() {
        let mut stack = Vec::new();
        if let Some(cycle) = dfs(node, deps, &mut marks, &mut stack) {
            return Some(cycle);
        }
    }
    None
}

fn build_cycle(stack: &[String], node: &str) -> Vec<String> {
    let start = stack
        .iter()
        .position(|candidate| candidate == node)
        .unwrap_or(0);
    let mut cycle = stack[start..].to_vec();
    cycle.push(node.to_string());
    cycle
}

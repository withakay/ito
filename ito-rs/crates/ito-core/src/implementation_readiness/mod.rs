//! Main-first proposal authority and implementation readiness evaluation.

mod git;
mod render_source;
mod tree_validation;
mod types;

use ito_config::types::{ItoConfig, ProposalIntegrationMode};

pub use render_source::{AuthoritativeChangeSource, AuthoritativeSourceError};
pub use types::{
    AuthorityEvidence, AuthoritySnapshot, ReadinessCondition, ReadinessPhase, ReadinessReport,
    ReadinessRequest,
};

use self::git::{CheckoutState, ReadinessGit, SystemReadinessGit};
use self::tree_validation::{PrepareFailureKind, validate_authoritative_change};

/// Evaluate main-first readiness using the production Git object adapter.
pub fn evaluate_readiness(request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport {
    evaluate_readiness_with_git(request, config, &SystemReadinessGit)
}

/// Materialize apply inputs from the immutable authority snapshot captured by
/// a successful prepare report.
pub fn materialize_authoritative_change(
    prepare: &ReadinessReport,
    repository_root: &std::path::Path,
    guidance_artifacts: &[&str],
) -> Result<AuthoritativeChangeSource, AuthoritativeSourceError> {
    render_source::materialize_authoritative_change(
        prepare,
        repository_root,
        guidance_artifacts,
        &SystemReadinessGit,
    )
}

/// Render a readiness report for human-facing CLI and runtime failures.
#[must_use]
pub fn render_readiness_text(report: &ReadinessReport) -> String {
    let status = if report.ready { "ready" } else { "not ready" };
    let mut lines = vec![
        format!(
            "Change '{}' is {status} for {:?}.",
            report.change_id, report.phase
        ),
        format!("Integration mode: {}", report.authority.integration_mode),
        format!(
            "Authority ref: {}",
            report
                .authority
                .target_ref
                .as_deref()
                .unwrap_or("unresolved")
        ),
        format!(
            "Authority OID: {}",
            report.authority.oid.as_deref().unwrap_or("unresolved")
        ),
        format!(
            "Proposal integration OID: {}",
            report
                .proposal_integration_oid
                .as_deref()
                .unwrap_or("unresolved")
        ),
        String::new(),
        "Conditions:".to_string(),
    ];
    for condition in &report.conditions {
        let mark = if condition.passed { "PASS" } else { "FAIL" };
        lines.push(format!(
            "  [{mark}] {}: {}",
            condition.code, condition.message
        ));
        if let Some(remediation) = &condition.remediation {
            lines.push(format!("         Fix: {remediation}"));
        }
    }
    lines.join("\n")
}

/// Evaluate execute readiness from one already-successful prepare snapshot.
///
/// This preserves the authority and integration OIDs captured before a new
/// worktree is created, so a concurrently moving target ref cannot change the
/// base or proof halfway through that operation.
pub fn evaluate_execute_from_prepare(
    prepare: &ReadinessReport,
    request: &ReadinessRequest,
    config: &ItoConfig,
) -> ReadinessReport {
    evaluate_execute_from_prepare_with_git(prepare, request, config, &SystemReadinessGit)
}

fn evaluate_execute_from_prepare_with_git(
    prepare: &ReadinessReport,
    request: &ReadinessRequest,
    config: &ItoConfig,
    git: &dyn ReadinessGit,
) -> ReadinessReport {
    let mut report = prepare.clone();
    report.phase = ReadinessPhase::Execute;
    report.ready = false;
    let valid_prepare = prepare.phase == ReadinessPhase::Prepare
        && prepare.ready
        && prepare.change_id == request.change_id
        && request.phase == ReadinessPhase::Execute
        && prepare.authority.integration_mode == config.changes.proposal.integration_mode;
    let Some(integration_oid) = prepare.proposal_integration_oid.as_deref() else {
        report.conditions.push(ReadinessCondition::failed(
            "prepare_snapshot",
            "The prepare report does not contain a proposal integration commit.",
            "Run prepare readiness successfully immediately before creating the implementation worktree.",
        ));
        return report;
    };
    if !valid_prepare || prepare.authority_snapshot().is_none() {
        report.conditions.push(ReadinessCondition::failed(
            "prepare_snapshot",
            "The supplied prepare report is not a successful compatible authority snapshot for this execute request.",
            "Run prepare readiness for this exact change and integration mode, then retry without substituting another report.",
        ));
        return report;
    }

    evaluate_execute_conditions(&mut report, request, config, git, integration_oid);
    report
}

pub(crate) fn evaluate_readiness_with_git(
    request: &ReadinessRequest,
    config: &ItoConfig,
    git: &dyn ReadinessGit,
) -> ReadinessReport {
    let mut report = evaluate_authority_with_git(request, config, git);
    if !report.ready {
        return report;
    }
    report.ready = false;

    let snapshot = report
        .authority_snapshot()
        .expect("successful authority resolution returns a complete snapshot");
    let canonical_change_id = match resolve_authoritative_change_target(git, request, &snapshot) {
        Ok(change_id) => change_id,
        Err((message, remediation)) => {
            report.conditions.push(ReadinessCondition::failed(
                "change_target",
                message,
                remediation,
            ));
            return report;
        }
    };
    report.change_id = canonical_change_id.clone();
    report.conditions.push(ReadinessCondition::passed(
        "change_target",
        format!(
            "Resolved change target '{}' to authoritative change '{}'.",
            request.change_id, canonical_change_id
        ),
    ));
    let mut request = request.clone();
    request.change_id = canonical_change_id;
    let proof = match validate_authoritative_change(git, &request, &snapshot) {
        Ok(proof) => proof,
        Err(failure) => {
            let remediation = format!(
                "Correct '{}' in the reviewed proposal, integrate the fix through {}, and retry readiness.",
                failure.path, snapshot.integration_mode
            );
            let condition = match failure.kind {
                PrepareFailureKind::Artifacts => ReadinessCondition::failed_artifact(
                    format!("{}: {}", failure.path, failure.message),
                    remediation,
                    failure.path,
                ),
                PrepareFailureKind::Validation => ReadinessCondition::failed_validation(
                    failure.message,
                    remediation,
                    failure.path,
                    failure.validator_code,
                ),
            };
            report.conditions.push(condition);
            return report;
        }
    };
    report.conditions.push(ReadinessCondition::passed(
        "authoritative_artifacts",
        format!(
            "Loaded {} apply prerequisite file(s) for schema '{}' from authority commit '{}'.",
            proof.artifact_paths.len(),
            proof.schema_name,
            snapshot.oid
        ),
    ));
    report.conditions.push(ReadinessCondition::passed(
        "authoritative_validation",
        format!(
            "Authoritative proposal files passed strict '{}' schema validation.",
            proof.schema_name
        ),
    ));

    let marker_path = format!(".ito/changes/{}/.ito.yaml", request.change_id);
    let integration_oid = match git.find_introduction_commit(
        &request.repository_root,
        &snapshot.oid,
        &marker_path,
    ) {
        Ok(oid) => {
            report.proposal_integration_oid = Some(oid.clone());
            report.conditions.push(ReadinessCondition::passed(
                "proposal_integration",
                format!(
                    "Target first-parent history introduced the proposal marker at commit '{oid}'."
                ),
            ));
            oid
        }
        Err(error) => {
            report.conditions.push(ReadinessCondition::failed(
                "proposal_integration",
                format!("Cannot prove proposal integration in target history: {error}"),
                format!(
                    "Integrate the reviewed proposal into '{}' with complete history available, then retry.",
                    snapshot.target_ref
                ),
            ));
            return report;
        }
    };

    if request.phase == ReadinessPhase::Prepare {
        report.ready = true;
        return report;
    }

    evaluate_execute_conditions(&mut report, &request, config, git, &integration_oid);
    report
}

fn resolve_authoritative_change_target(
    git: &dyn ReadinessGit,
    request: &ReadinessRequest,
    snapshot: &AuthoritySnapshot,
) -> Result<String, (String, String)> {
    let entries = git
        .list_tree(&request.repository_root, &snapshot.oid, ".ito/changes")
        .map_err(|error| {
            (
                format!("Cannot list authoritative changes: {error}"),
                "Make the accepted proposal tree available locally and retry.".to_string(),
            )
        })?;
    let mut change_ids = entries
        .iter()
        .filter(|entry| entry.path.ends_with("/.ito.yaml"))
        .filter_map(|entry| {
            entry
                .path
                .strip_prefix(".ito/changes/")
                .and_then(|path| path.strip_suffix("/.ito.yaml"))
        })
        .filter(|change_id| !change_id.is_empty() && !change_id.contains('/'))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    change_ids.sort();
    change_ids.dedup();

    let input = request.change_id.trim();
    if let Some(exact) = change_ids
        .iter()
        .find(|change_id| change_id.as_str() == input)
    {
        return Ok(exact.clone());
    }
    let matches = change_ids
        .into_iter()
        .filter(|change_id| change_id.starts_with(input))
        .collect::<Vec<_>>();
    if matches.len() == 1 {
        return Ok(matches[0].clone());
    }
    if matches.is_empty() {
        return Err((
            format!(
                "Change target '{}' was not found in authority commit '{}'.",
                request.change_id, snapshot.oid
            ),
            "Integrate the reviewed proposal into the authoritative target branch, or use its canonical change ID."
                .to_string(),
        ));
    }
    Err((
        format!(
            "Change target '{}' is ambiguous in authority commit '{}'. Matches: {}.",
            request.change_id,
            snapshot.oid,
            matches.join(", ")
        ),
        "Use a longer prefix or the full canonical change ID.".to_string(),
    ))
}

fn evaluate_execute_conditions(
    report: &mut ReadinessReport,
    request: &ReadinessRequest,
    config: &ItoConfig,
    git: &dyn ReadinessGit,
    integration_oid: &str,
) {
    let Some(checkout_path) = request.current_checkout.as_deref() else {
        report.conditions.push(ReadinessCondition::failed(
            "checkout_identity",
            "Execute readiness requires a current implementation checkout.",
            "Run execute preflight from the dedicated change worktree, or provide that checkout explicitly.",
        ));
        return;
    };
    let repository_state = match git.inspect_checkout(&request.repository_root) {
        Ok(state) => state,
        Err(error) => {
            report.conditions.push(ReadinessCondition::failed(
                "checkout_identity",
                format!("Cannot inspect the authority repository identity: {error}"),
                "Run readiness from a valid Git repository and retry.",
            ));
            return;
        }
    };
    let checkout_state = match git.inspect_checkout(checkout_path) {
        Ok(state) => state,
        Err(error) => {
            report.conditions.push(ReadinessCondition::failed(
                "checkout_identity",
                format!(
                    "Cannot inspect implementation checkout '{}': {error}",
                    checkout_path.display()
                ),
                "Select a valid dedicated Git worktree for this change and retry.",
            ));
            return;
        }
    };

    let same_repository = repository_state.common_dir == checkout_state.common_dir;
    let ancestry_passed = if same_repository {
        match git.is_ancestor(
            &checkout_state.root,
            integration_oid,
            &checkout_state.head_oid,
        ) {
            Ok(true) => {
                report.conditions.push(ReadinessCondition::passed(
                    "implementation_ancestry",
                    format!(
                        "Implementation HEAD '{}' descends from proposal integration commit '{}'.",
                        checkout_state.head_oid, integration_oid
                    ),
                ));
                true
            }
            Ok(false) => {
                report.conditions.push(ReadinessCondition::failed(
                    "implementation_ancestry",
                    format!(
                        "Implementation HEAD '{}' does not contain proposal integration commit '{}'.",
                        checkout_state.head_oid, integration_oid
                    ),
                    "Recreate the change worktree from the verified authority commit, or rebase/merge the accepted proposal history before implementation.",
                ));
                false
            }
            Err(error) => {
                report.conditions.push(ReadinessCondition::failed(
                    "implementation_ancestry",
                    format!("Cannot inspect implementation ancestry: {error}"),
                    "Make the complete proposal and implementation history available locally, then retry.",
                ));
                false
            }
        }
    } else {
        report.conditions.push(ReadinessCondition::failed(
            "implementation_ancestry",
            "The selected checkout belongs to a different Git repository, so proposal ancestry cannot be proven.",
            "Select a dedicated change worktree from the authoritative repository and retry.",
        ));
        false
    };

    let identity_passed =
        evaluate_checkout_identity(report, request, config, &checkout_state, same_repository);
    report.ready = ancestry_passed && identity_passed;
}

fn evaluate_checkout_identity(
    report: &mut ReadinessReport,
    request: &ReadinessRequest,
    config: &ItoConfig,
    checkout: &CheckoutState,
    same_repository: bool,
) -> bool {
    let branch = checkout.branch.as_deref();
    let failure = if !same_repository {
        Some("The selected checkout is not a worktree of the authoritative repository.".to_string())
    } else if checkout.is_bare {
        Some(
            "The selected checkout is a bare control repository, not an implementation worktree."
                .to_string(),
        )
    } else if branch == Some(config.worktrees.default_branch.as_str()) {
        Some(format!(
            "The selected checkout is the authoritative target branch '{}', not a dedicated implementation worktree.",
            config.worktrees.default_branch
        ))
    } else if !crate::worktree_validate::checkout_matches_change_id(
        &checkout.root,
        branch,
        &request.change_id,
    ) {
        Some(format!(
            "Checkout '{}'{} is not associated with full change ID '{}'.",
            checkout.root.display(),
            branch
                .map(|name| format!(" on branch '{name}'"))
                .unwrap_or_default(),
            request.change_id
        ))
    } else {
        None
    };

    if let Some(message) = failure {
        report.conditions.push(ReadinessCondition::failed(
            "checkout_identity",
            message,
            format!(
                "Use the dedicated '{}' worktree (a suffixed review worktree is also accepted) and retry.",
                request.change_id
            ),
        ));
        return false;
    }

    report.conditions.push(ReadinessCondition::passed(
        "checkout_identity",
        format!(
            "Checkout '{}'{} is associated with change '{}'.",
            checkout.root.display(),
            branch
                .map(|name| format!(" on branch '{name}'"))
                .unwrap_or_default(),
            request.change_id
        ),
    ));
    true
}

/// Resolve the configured proposal authority to an immutable commit snapshot.
///
/// This is the production entry point. It performs no durable writes. When
/// `request.refresh_authority` is true in pull-request mode, the one configured
/// upstream target ref is fetched before its commit is resolved.
#[cfg(test)]
pub(crate) fn evaluate_authority(
    request: &ReadinessRequest,
    config: &ItoConfig,
) -> ReadinessReport {
    evaluate_authority_with_git(request, config, &SystemReadinessGit)
}

/// Resolve proposal authority using an injected Git boundary.
///
/// The authority ref is resolved exactly once. All later readiness phases must
/// consume the returned OID rather than resolving the mutable ref again.
pub(crate) fn evaluate_authority_with_git(
    request: &ReadinessRequest,
    config: &ItoConfig,
    git: &dyn ReadinessGit,
) -> ReadinessReport {
    let mode = config.changes.proposal.integration_mode;
    let local_target_ref = format!("refs/heads/{}", config.worktrees.default_branch);
    let mut report = ReadinessReport::new(request, mode);

    let upstream = match mode {
        ProposalIntegrationMode::DirectMerge => {
            report.authority.target_ref = Some(local_target_ref.clone());
            report.conditions.push(ReadinessCondition::passed(
                "authority_ref",
                format!("Using local target ref '{local_target_ref}' for direct-merge authority."),
            ));
            None
        }
        ProposalIntegrationMode::PullRequest => {
            match git.tracked_upstream(&request.repository_root, &local_target_ref) {
                Ok(upstream) => {
                    report.authority.target_ref = Some(upstream.tracking_ref.clone());
                    report.conditions.push(ReadinessCondition::passed(
                        "authority_ref",
                        format!(
                            "Using tracked upstream ref '{}' for pull-request authority.",
                            upstream.tracking_ref
                        ),
                    ));
                    Some(upstream)
                }
                Err(error) => {
                    report.conditions.push(ReadinessCondition::failed(
                        "authority_ref",
                        format!(
                            "Cannot resolve the tracked upstream for target ref '{local_target_ref}': {error}"
                        ),
                        "Configure and fetch the target branch's tracked upstream, or explicitly set changes.proposal.integration_mode to direct_merge if this repository does not use a pull_request workflow.",
                    ));
                    return report;
                }
            }
        }
    };

    if request.refresh_authority {
        if let Some(upstream) = upstream.as_ref() {
            match git.refresh_upstream(&request.repository_root, upstream) {
                Ok(()) => report.conditions.push(ReadinessCondition::passed(
                    "authority_refresh",
                    format!("Refreshed authority ref '{}'.", upstream.tracking_ref),
                )),
                Err(error) => {
                    report.conditions.push(ReadinessCondition::failed(
                        "authority_refresh",
                        format!(
                            "Cannot refresh authority ref '{}': {error}",
                            upstream.tracking_ref
                        ),
                        format!(
                            "Check access to remote '{}' and retry the refreshed preflight.",
                            upstream.remote
                        ),
                    ));
                    return report;
                }
            }
        } else {
            report.conditions.push(ReadinessCondition::passed(
                "authority_refresh",
                "Direct-merge authority is local and does not require a remote refresh.",
            ));
        }
    }

    let target_ref = report
        .authority
        .target_ref
        .as_deref()
        .expect("successful authority selection always records a target ref");
    match git.resolve_commit(&request.repository_root, target_ref) {
        Ok(oid) => {
            report.authority.oid = Some(oid.clone());
            report.conditions.push(ReadinessCondition::passed(
                "authority_oid",
                format!("Resolved authority ref '{target_ref}' to commit '{oid}'."),
            ));
            report.ready = true;
        }
        Err(error) => report.conditions.push(ReadinessCondition::failed(
            "authority_oid",
            format!("Cannot resolve authority ref '{target_ref}' to a commit: {error}"),
            match mode {
                ProposalIntegrationMode::PullRequest => {
                    format!("Fetch the configured upstream target ref '{target_ref}' and retry.")
                }
                ProposalIntegrationMode::DirectMerge => format!(
                    "Create or update the local target branch '{}' and retry.",
                    config.worktrees.default_branch
                ),
            },
        )),
    }

    report
}

#[cfg(test)]
#[path = "implementation_readiness_tests.rs"]
mod tests;

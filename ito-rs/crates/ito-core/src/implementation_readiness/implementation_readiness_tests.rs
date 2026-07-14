use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::Command;

use ito_config::types::{ItoConfig, ProposalIntegrationMode, WorktreeStrategy};

use super::git::{ReadinessGit, ReadinessGitError, TrackedUpstream};
use super::*;

const FIRST_OID: &str = "1111111111111111111111111111111111111111";
const SECOND_OID: &str = "2222222222222222222222222222222222222222";

#[derive(Debug)]
struct FakeGit {
    upstream: RefCell<Option<Result<TrackedUpstream, ReadinessGitError>>>,
    refresh: RefCell<Option<Result<(), ReadinessGitError>>>,
    resolved_oids: RefCell<VecDeque<Result<String, ReadinessGitError>>>,
    upstream_calls: RefCell<Vec<String>>,
    refresh_calls: RefCell<Vec<TrackedUpstream>>,
    resolve_calls: RefCell<Vec<String>>,
}

impl FakeGit {
    fn direct(oids: &[&str]) -> Self {
        Self {
            upstream: RefCell::new(None),
            refresh: RefCell::new(None),
            resolved_oids: RefCell::new(oids.iter().map(|oid| Ok((*oid).to_string())).collect()),
            upstream_calls: RefCell::new(Vec::new()),
            refresh_calls: RefCell::new(Vec::new()),
            resolve_calls: RefCell::new(Vec::new()),
        }
    }

    fn pull_request(
        upstream: Result<TrackedUpstream, ReadinessGitError>,
        refresh: Result<(), ReadinessGitError>,
        oids: &[&str],
    ) -> Self {
        Self {
            upstream: RefCell::new(Some(upstream)),
            refresh: RefCell::new(Some(refresh)),
            resolved_oids: RefCell::new(oids.iter().map(|oid| Ok((*oid).to_string())).collect()),
            upstream_calls: RefCell::new(Vec::new()),
            refresh_calls: RefCell::new(Vec::new()),
            resolve_calls: RefCell::new(Vec::new()),
        }
    }
}

impl ReadinessGit for FakeGit {
    fn tracked_upstream(
        &self,
        _repository_root: &Path,
        local_branch_ref: &str,
    ) -> Result<TrackedUpstream, ReadinessGitError> {
        self.upstream_calls
            .borrow_mut()
            .push(local_branch_ref.to_string());
        self.upstream
            .borrow_mut()
            .take()
            .expect("unexpected upstream lookup")
    }

    fn refresh_upstream(
        &self,
        _repository_root: &Path,
        upstream: &TrackedUpstream,
    ) -> Result<(), ReadinessGitError> {
        self.refresh_calls.borrow_mut().push(upstream.clone());
        self.refresh
            .borrow_mut()
            .take()
            .expect("unexpected refresh")
    }

    fn resolve_commit(
        &self,
        _repository_root: &Path,
        target_ref: &str,
    ) -> Result<String, ReadinessGitError> {
        self.resolve_calls.borrow_mut().push(target_ref.to_string());
        self.resolved_oids
            .borrow_mut()
            .pop_front()
            .expect("unexpected commit resolution")
    }
}

fn config(mode: ProposalIntegrationMode) -> ItoConfig {
    let mut config = ItoConfig::default();
    config.changes.proposal.integration_mode = mode;
    config.worktrees.default_branch = "main".to_string();
    config
}

fn request(repository_root: impl Into<PathBuf>) -> ReadinessRequest {
    ReadinessRequest::new(
        "031-02_enforce-main-first-implementation",
        ReadinessPhase::Prepare,
        repository_root,
    )
}

fn upstream() -> TrackedUpstream {
    TrackedUpstream {
        tracking_ref: "refs/remotes/origin/main".to_string(),
        remote: "origin".to_string(),
        remote_ref: "refs/heads/main".to_string(),
    }
}

#[test]
fn direct_merge_resolves_local_target_ref_once() {
    let git = FakeGit::direct(&[FIRST_OID]);

    let report = evaluate_authority_with_git(
        &request("/repo"),
        &config(ProposalIntegrationMode::DirectMerge),
        &git,
    );

    assert!(report.ready);
    assert_eq!(
        report.authority.integration_mode,
        ProposalIntegrationMode::DirectMerge
    );
    assert_eq!(
        report.authority.target_ref.as_deref(),
        Some("refs/heads/main")
    );
    assert_eq!(report.authority.oid.as_deref(), Some(FIRST_OID));
    assert_eq!(
        report.authority_snapshot(),
        Some(AuthoritySnapshot {
            integration_mode: ProposalIntegrationMode::DirectMerge,
            target_ref: "refs/heads/main".to_string(),
            oid: FIRST_OID.to_string(),
        })
    );
    assert!(git.upstream_calls.borrow().is_empty());
    assert_eq!(git.resolve_calls.borrow().as_slice(), ["refs/heads/main"]);
}

#[test]
fn pull_request_resolves_tracked_upstream_without_local_fallback() {
    let git = FakeGit::pull_request(Ok(upstream()), Ok(()), &[FIRST_OID]);

    let report = evaluate_authority_with_git(
        &request("/repo"),
        &config(ProposalIntegrationMode::PullRequest),
        &git,
    );

    assert!(report.ready);
    assert_eq!(
        report.authority.target_ref.as_deref(),
        Some("refs/remotes/origin/main")
    );
    assert_eq!(report.authority.oid.as_deref(), Some(FIRST_OID));
    assert_eq!(git.upstream_calls.borrow().as_slice(), ["refs/heads/main"]);
    assert_eq!(
        git.resolve_calls.borrow().as_slice(),
        ["refs/remotes/origin/main"]
    );
}

#[test]
fn missing_pull_request_upstream_returns_actionable_failure() {
    let git = FakeGit::pull_request(
        Err(ReadinessGitError::new(
            "target branch has no tracked upstream",
        )),
        Ok(()),
        &[],
    );

    let report = evaluate_authority_with_git(
        &request("/repo"),
        &config(ProposalIntegrationMode::PullRequest),
        &git,
    );

    assert!(!report.ready);
    assert!(report.authority.target_ref.is_none());
    assert!(report.authority.oid.is_none());
    let condition = report
        .conditions
        .iter()
        .find(|condition| condition.code == "authority_ref")
        .expect("authority ref condition");
    assert!(!condition.passed);
    assert!(condition.message.contains("tracked upstream"));
    assert!(
        condition
            .remediation
            .as_deref()
            .unwrap()
            .contains("pull_request")
    );
    assert!(git.resolve_calls.borrow().is_empty());
}

#[test]
fn refresh_failure_stops_before_authority_resolution() {
    let git = FakeGit::pull_request(
        Ok(upstream()),
        Err(ReadinessGitError::new("network unavailable")),
        &[FIRST_OID],
    );
    let request = request("/repo").with_refresh_authority(true);

    let report = evaluate_authority_with_git(
        &request,
        &config(ProposalIntegrationMode::PullRequest),
        &git,
    );

    assert!(!report.ready);
    assert_eq!(
        report.authority.target_ref.as_deref(),
        Some("refs/remotes/origin/main")
    );
    assert!(report.authority.oid.is_none());
    assert_eq!(git.refresh_calls.borrow().as_slice(), [upstream()]);
    assert!(git.resolve_calls.borrow().is_empty());
}

#[test]
fn moving_ref_changes_only_a_later_evaluation() {
    let git = FakeGit::direct(&[FIRST_OID, SECOND_OID]);
    let config = config(ProposalIntegrationMode::DirectMerge);
    let request = request("/repo");

    let first = evaluate_authority_with_git(&request, &config, &git);
    assert_eq!(first.authority.oid.as_deref(), Some(FIRST_OID));
    assert_eq!(first.authority.oid.as_deref(), Some(FIRST_OID));

    let second = evaluate_authority_with_git(&request, &config, &git);
    assert_eq!(second.authority.oid.as_deref(), Some(SECOND_OID));
    assert_eq!(git.resolve_calls.borrow().len(), 2);
}

#[test]
fn system_git_resolves_direct_and_pull_request_authority() {
    let fixture = tempfile::tempdir().expect("fixture root");
    let remote = fixture.path().join("remote.git");
    let repository = fixture.path().join("repository");
    run_git(
        fixture.path(),
        &["init", "--bare", remote.to_str().unwrap()],
    );
    run_git(
        fixture.path(),
        &[
            "init",
            "--initial-branch=main",
            repository.to_str().unwrap(),
        ],
    );
    run_git(&repository, &["config", "user.name", "Ito Test"]);
    run_git(
        &repository,
        &["config", "user.email", "ito@example.invalid"],
    );
    std::fs::write(repository.join("README.md"), "fixture\n").unwrap();
    run_git(&repository, &["add", "README.md"]);
    run_git(&repository, &["commit", "-m", "initial"]);
    run_git(
        &repository,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    run_git(&repository, &["push", "--set-upstream", "origin", "main"]);

    let expected_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);
    for mode in [
        ProposalIntegrationMode::DirectMerge,
        ProposalIntegrationMode::PullRequest,
    ] {
        let report = evaluate_authority(&request(&repository), &config(mode));
        assert!(report.ready, "{report:#?}");
        assert_eq!(report.authority.oid.as_deref(), Some(expected_oid.as_str()));
    }

    std::fs::write(repository.join("README.md"), "fixture changed\n").unwrap();
    run_git(&repository, &["add", "README.md"]);
    run_git(&repository, &["commit", "-m", "move local main"]);
    let moved_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);
    let moved = evaluate_authority(
        &request(&repository),
        &config(ProposalIntegrationMode::DirectMerge),
    );
    assert_ne!(moved_oid, expected_oid);
    assert_eq!(moved.authority.oid.as_deref(), Some(moved_oid.as_str()));

    let missing_upstream = fixture.path().join("missing-upstream");
    run_git(
        fixture.path(),
        &[
            "init",
            "--initial-branch=main",
            missing_upstream.to_str().unwrap(),
        ],
    );
    run_git(&missing_upstream, &["config", "user.name", "Ito Test"]);
    run_git(
        &missing_upstream,
        &["config", "user.email", "ito@example.invalid"],
    );
    std::fs::write(missing_upstream.join("README.md"), "fixture\n").unwrap();
    run_git(&missing_upstream, &["add", "README.md"]);
    run_git(&missing_upstream, &["commit", "-m", "initial"]);
    let missing = evaluate_authority(
        &request(&missing_upstream),
        &config(ProposalIntegrationMode::PullRequest),
    );
    assert!(!missing.ready);
    assert!(missing.authority.target_ref.is_none());

    std::fs::remove_dir_all(&remote).unwrap();
    let refresh_failure = evaluate_authority(
        &request(&repository).with_refresh_authority(true),
        &config(ProposalIntegrationMode::PullRequest),
    );
    assert!(!refresh_failure.ready);
    assert!(refresh_failure.authority.oid.is_none());
    assert!(
        refresh_failure
            .conditions
            .iter()
            .any(|condition| condition.code == "authority_refresh" && !condition.passed)
    );
}

#[test]
fn report_serializes_a_stable_authority_shape() {
    let git = FakeGit::direct(&[FIRST_OID]);
    let report = evaluate_authority_with_git(
        &request("/repo"),
        &config(ProposalIntegrationMode::DirectMerge),
        &git,
    );

    let value = serde_json::to_value(report).unwrap();
    assert_eq!(value["phase"], "prepare");
    assert_eq!(value["ready"], true);
    assert_eq!(value["authority"]["integration_mode"], "direct_merge");
    assert_eq!(value["authority"]["target_ref"], "refs/heads/main");
    assert_eq!(value["authority"]["oid"], FIRST_OID);
    assert!(value["proposal_integration_oid"].is_null());
}

#[test]
fn prepare_reads_and_validates_only_the_authoritative_tree() {
    let fixture = tempfile::tempdir().expect("fixture root");
    let repository = fixture.path().join("repository");
    run_git(
        fixture.path(),
        &[
            "init",
            "--initial-branch=main",
            repository.to_str().unwrap(),
        ],
    );
    run_git(&repository, &["config", "user.name", "Ito Test"]);
    run_git(
        &repository,
        &["config", "user.email", "ito@example.invalid"],
    );
    std::fs::write(repository.join("README.md"), "fixture\n").unwrap();
    run_git(&repository, &["add", "README.md"]);
    run_git(&repository, &["commit", "-m", "initial"]);

    write_change(&repository, VALID_TASKS);
    let copied_only = evaluate_readiness(
        &request(&repository),
        &config(ProposalIntegrationMode::DirectMerge),
    );
    assert!(!copied_only.ready);
    assert_failed_condition(&copied_only, "change_target");
    run_git(&repository, &["clean", "-fd"]);

    let change_dir = repository.join(".ito/changes").join(CHANGE_ID);
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::write(change_dir.join(".ito.yaml"), "schema: spec-driven\n").unwrap();
    std::fs::write(change_dir.join("proposal.md"), PROPOSAL).unwrap();
    run_git(&repository, &["add", ".ito"]);
    run_git(
        &repository,
        &["commit", "-m", "integrate incomplete proposal"],
    );
    let integration_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);

    let incomplete = evaluate_readiness(
        &request(&repository),
        &config(ProposalIntegrationMode::DirectMerge),
    );
    assert!(!incomplete.ready);
    assert_failed_condition(&incomplete, "authoritative_artifacts");

    write_change(&repository, "# no recognizable tasks\n");
    run_git(&repository, &["add", ".ito"]);
    run_git(
        &repository,
        &["commit", "-m", "add invalid proposal artifacts"],
    );
    let invalid = evaluate_readiness(
        &request(&repository),
        &config(ProposalIntegrationMode::DirectMerge),
    );
    assert!(!invalid.ready);
    assert_failed_condition(&invalid, "authoritative_validation");

    std::fs::write(change_dir.join("tasks.md"), VALID_TASKS).unwrap();
    run_git(&repository, &["add", ".ito"]);
    run_git(
        &repository,
        &["commit", "-m", "validate proposal artifacts"],
    );
    let ready = evaluate_readiness(
        &request(&repository),
        &config(ProposalIntegrationMode::DirectMerge),
    );
    assert!(ready.ready, "{ready:#?}");
    assert_eq!(
        ready.proposal_integration_oid.as_deref(),
        Some(integration_oid.as_str())
    );
}

#[test]
fn prepare_runs_warning_rules_against_authority_and_ignores_local_tampering() {
    let (_fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    write_project_validation(&repository, "warning");
    commit_all(&repository, "integrate proposal with warning rule");

    std::fs::write(
        repository
            .join(".ito/changes")
            .join(CHANGE_ID)
            .join("specs/main-first/spec.md"),
        DELTA_SPEC_MISSING_THEN,
    )
    .unwrap();

    let report = evaluate_direct_readiness(&repository);

    assert!(report.ready, "{report:#?}");
    assert_passed_condition(&report, "authoritative_validation");
}

#[test]
fn prepare_rejects_error_rule_failure_from_the_authority_tree() {
    let (_fixture, repository) = init_readiness_repository();
    write_change_with_spec(&repository, VALID_TASKS, DELTA_SPEC_MISSING_THEN);
    write_project_validation(&repository, "error");
    commit_all(&repository, "integrate proposal failing error rule");

    let report = evaluate_direct_readiness(&repository);

    assert!(!report.ready, "{report:#?}");
    let condition = failed_condition(&report, "authoritative_validation");
    assert!(condition.message.contains("missing THEN"), "{condition:#?}");
    assert_eq!(
        condition.validator_code.as_deref(),
        Some("ito.delta-specs.v1")
    );
}

#[test]
fn direct_commit_reports_the_marker_addition_as_proposal_integration() {
    let (_fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);

    let direct_oid = commit_all(&repository, "integrate proposal directly");
    let report = evaluate_direct_readiness(&repository);

    assert!(report.ready, "{report:#?}");
    assert_eq!(
        report.proposal_integration_oid.as_deref(),
        Some(direct_oid.as_str())
    );
}

#[test]
fn no_ff_merge_reports_the_merge_commit_as_proposal_integration() {
    let (_fixture, repository) = init_readiness_repository();
    run_git(&repository, &["switch", "-c", "proposal"]);
    write_change(&repository, VALID_TASKS);
    let proposal_oid = commit_all(&repository, "author proposal");

    run_git(&repository, &["switch", "main"]);
    std::fs::write(
        repository.join("main-only.txt"),
        "force divergent history\n",
    )
    .unwrap();
    commit_all(&repository, "advance main before integration");
    run_git(
        &repository,
        &[
            "merge",
            "--no-ff",
            "proposal",
            "-m",
            "merge reviewed proposal",
        ],
    );
    let merge_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);

    let report = evaluate_direct_readiness(&repository);

    assert!(report.ready, "{report:#?}");
    assert_ne!(merge_oid, proposal_oid);
    assert_eq!(
        report.proposal_integration_oid.as_deref(),
        Some(merge_oid.as_str())
    );
}

#[test]
fn squash_merge_reports_the_squash_commit_as_proposal_integration() {
    let (_fixture, repository) = init_readiness_repository();
    run_git(&repository, &["switch", "-c", "proposal"]);
    write_change(&repository, VALID_TASKS);
    let proposal_oid = commit_all(&repository, "author proposal");

    run_git(&repository, &["switch", "main"]);
    run_git(&repository, &["merge", "--squash", "proposal"]);
    let squash_oid = commit_all(&repository, "squash reviewed proposal");

    let report = evaluate_direct_readiness(&repository);

    assert!(report.ready, "{report:#?}");
    assert_ne!(squash_oid, proposal_oid);
    assert_eq!(
        report.proposal_integration_oid.as_deref(),
        Some(squash_oid.as_str())
    );
}

#[cfg(unix)]
#[test]
fn prepare_rejects_a_committed_symlink_marker() {
    use std::os::unix::fs::symlink;

    let (_fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let change_dir = repository.join(".ito/changes").join(CHANGE_ID);
    std::fs::remove_file(change_dir.join(".ito.yaml")).unwrap();
    std::fs::write(
        repository.join("shared-marker.yaml"),
        "schema: spec-driven\n",
    )
    .unwrap();
    symlink("../../../shared-marker.yaml", change_dir.join(".ito.yaml")).unwrap();
    commit_all(&repository, "commit symlink proposal marker");
    assert_committed_symlink(&repository, &format!(".ito/changes/{CHANGE_ID}/.ito.yaml"));

    let report = evaluate_direct_readiness(&repository);

    assert!(!report.ready, "{report:#?}");
    let condition = failed_condition(&report, "authoritative_artifacts");
    assert!(condition.message.contains(".ito.yaml"));
    assert!(condition.message.contains("regular Git blob"));
    assert!(report.proposal_integration_oid.is_none());
}

#[cfg(unix)]
#[test]
fn prepare_rejects_a_committed_symlink_artifact() {
    use std::os::unix::fs::symlink;

    let (_fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let change_dir = repository.join(".ito/changes").join(CHANGE_ID);
    std::fs::remove_file(change_dir.join("proposal.md")).unwrap();
    std::fs::write(repository.join("shared-proposal.md"), PROPOSAL).unwrap();
    symlink(
        "../../../shared-proposal.md",
        change_dir.join("proposal.md"),
    )
    .unwrap();
    commit_all(&repository, "commit symlink proposal artifact");
    assert_committed_symlink(
        &repository,
        &format!(".ito/changes/{CHANGE_ID}/proposal.md"),
    );

    let report = evaluate_direct_readiness(&repository);

    assert!(!report.ready, "{report:#?}");
    let condition = failed_condition(&report, "authoritative_artifacts");
    assert!(condition.message.contains("proposal.md"));
    assert!(condition.message.contains("regular Git blob"));
    assert!(report.proposal_integration_oid.is_none());
}

#[test]
fn execute_passes_for_post_integration_suffixed_change_worktree() {
    let (fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let branch = format!("{CHANGE_ID}-review");
    let worktree = add_linked_worktree(fixture.path(), &repository, &branch, &integration_oid);

    assert!(git_succeeds(
        &worktree,
        &["merge-base", "--is-ancestor", &integration_oid, "HEAD"],
    ));

    let report = evaluate_execute_readiness(&repository, &worktree);

    assert!(report.ready, "{report:#?}");
    assert_passed_condition(&report, "implementation_ancestry");
    assert_passed_condition(&report, "checkout_identity");
}

#[test]
fn execute_rejects_pre_integration_branch_with_committed_copied_artifacts() {
    let (fixture, repository) = init_readiness_repository();
    let pre_integration_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);
    let worktree =
        add_linked_worktree(fixture.path(), &repository, CHANGE_ID, &pre_integration_oid);

    write_change(&repository, VALID_TASKS);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");

    write_change(&worktree, VALID_TASKS);
    commit_all(&worktree, "copy proposal artifacts into old branch");
    assert!(git_stdout(&worktree, &["status", "--porcelain"]).is_empty());
    assert!(!git_succeeds(
        &worktree,
        &["merge-base", "--is-ancestor", &integration_oid, "HEAD"],
    ));

    let report = evaluate_execute_readiness(&repository, &worktree);

    assert!(!report.ready, "{report:#?}");
    assert_passed_condition(&report, "checkout_identity");
    let condition = failed_condition(&report, "implementation_ancestry");
    let remediation = condition.remediation.as_deref().unwrap_or_default();
    assert!(
        ["recreate", "rebase", "merge"]
            .iter()
            .any(|action| remediation.contains(action)),
        "ancestry remediation should explain how to update the old branch: {condition:#?}"
    );
}

#[test]
fn execute_rejects_pre_integration_branch_with_uncommitted_copied_artifacts() {
    let (fixture, repository) = init_readiness_repository();
    let pre_integration_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);
    let branch = format!("{CHANGE_ID}-local-copy");
    let worktree = add_linked_worktree(fixture.path(), &repository, &branch, &pre_integration_oid);

    write_change(&repository, VALID_TASKS);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");

    write_change(&worktree, VALID_TASKS);
    assert!(
        git_stdout(&worktree, &["status", "--porcelain"]).contains(".ito/"),
        "fixture must contain uncommitted proposal artifacts"
    );
    assert!(!git_succeeds(
        &worktree,
        &["merge-base", "--is-ancestor", &integration_oid, "HEAD"],
    ));

    let report = evaluate_execute_readiness(&repository, &worktree);

    assert!(!report.ready, "{report:#?}");
    assert_passed_condition(&report, "checkout_identity");
    failed_condition(&report, "implementation_ancestry");
}

#[test]
fn execute_rejects_authoritative_target_checkout() {
    let (_fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    assert!(git_succeeds(
        &repository,
        &["merge-base", "--is-ancestor", &integration_oid, "HEAD"],
    ));

    let report = evaluate_execute_readiness(&repository, &repository);

    assert!(!report.ready, "{report:#?}");
    assert_passed_condition(&report, "implementation_ancestry");
    let condition = failed_condition(&report, "checkout_identity");
    assert!(condition.message.contains("main") || condition.message.contains("target"));
}

#[test]
fn execute_rejects_unrelated_branch_and_mismatched_worktree() {
    let (fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let worktree = add_linked_worktree(
        fixture.path(),
        &repository,
        "999-99_unrelated",
        &integration_oid,
    );
    assert!(git_succeeds(
        &worktree,
        &["merge-base", "--is-ancestor", &integration_oid, "HEAD"],
    ));

    let report = evaluate_execute_readiness(&repository, &worktree);

    assert!(!report.ready, "{report:#?}");
    assert_passed_condition(&report, "implementation_ancestry");
    let condition = failed_condition(&report, "checkout_identity");
    assert!(condition.message.contains(CHANGE_ID));
}

#[test]
fn execute_from_prepare_keeps_the_captured_authority_when_target_moves() {
    let (fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let captured_oid = commit_all(&repository, "integrate reviewed proposal");
    let prepare = evaluate_direct_readiness(&repository);
    assert!(prepare.ready, "{prepare:#?}");

    let branch = format!("{CHANGE_ID}-captured");
    let worktree = add_linked_worktree(fixture.path(), &repository, &branch, &captured_oid);
    std::fs::write(repository.join("main-moved.txt"), "new target state\n").unwrap();
    let moved_oid = commit_all(&repository, "move main after prepare");
    assert_ne!(captured_oid, moved_oid);

    let execute_request = ReadinessRequest::new(CHANGE_ID, ReadinessPhase::Execute, &repository)
        .with_current_checkout(&worktree);
    let execute = evaluate_execute_from_prepare(
        &prepare,
        &execute_request,
        &config(ProposalIntegrationMode::DirectMerge),
    );

    assert!(execute.ready, "{execute:#?}");
    assert_eq!(
        execute.authority.oid.as_deref(),
        Some(captured_oid.as_str())
    );
    assert_eq!(
        execute.proposal_integration_oid.as_deref(),
        Some(captured_oid.as_str())
    );
    assert_passed_condition(&execute, "implementation_ancestry");
    assert_passed_condition(&execute, "checkout_identity");
}

#[test]
fn readded_change_id_uses_latest_integration_and_rejects_stale_worktree() {
    let (fixture, repository) = init_readiness_repository();
    write_change(&repository, VALID_TASKS);
    let old_integration = commit_all(&repository, "integrate original proposal");
    let stale_branch = format!("{CHANGE_ID}-stale");
    let stale_worktree =
        add_linked_worktree(fixture.path(), &repository, &stale_branch, &old_integration);

    std::fs::remove_dir_all(repository.join(".ito/changes").join(CHANGE_ID)).unwrap();
    commit_all(&repository, "remove abandoned proposal");
    write_change(&repository, VALID_TASKS);
    let new_integration = commit_all(&repository, "integrate replacement proposal");

    let prepare = evaluate_direct_readiness(&repository);
    assert!(prepare.ready, "{prepare:#?}");
    assert_eq!(
        prepare.proposal_integration_oid.as_deref(),
        Some(new_integration.as_str())
    );
    assert_ne!(old_integration, new_integration);

    let execute = evaluate_execute_readiness(&repository, &stale_worktree);
    assert!(!execute.ready, "{execute:#?}");
    assert_failed_condition(&execute, "implementation_ancestry");
    assert_passed_condition(&execute, "checkout_identity");
}

const CHANGE_ID: &str = "031-02_enforce-main-first-implementation";
const PROPOSAL: &str = "# Proposal\n\nIntegrate reviewed intent before implementation.\n";
const DESIGN: &str = "# Design\n\nResolve one immutable authority commit.\n";
const DELTA_SPEC: &str = r#"## ADDED Requirements

### Requirement: Main-first implementation
Ito SHALL require accepted proposal history before implementation begins.

#### Scenario: Accepted proposal
- **GIVEN** a reviewed proposal
- **WHEN** implementation readiness is evaluated
- **THEN** the accepted proposal commit is present
"#;
const DELTA_SPEC_MISSING_THEN: &str = r#"## ADDED Requirements

### Requirement: Main-first implementation
Ito SHALL require accepted proposal history before implementation begins.

#### Scenario: Accepted proposal
- **GIVEN** a reviewed proposal
- **WHEN** implementation readiness is evaluated
"#;
const VALID_TASKS: &str = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Implement the accepted proposal
- **Dependencies**: None
- **Updated At**: 2026-07-13
- **Status**: [ ] pending
"#;

fn write_change(repository: &Path, tasks: &str) {
    write_change_with_spec(repository, tasks, DELTA_SPEC);
}

fn write_change_with_spec(repository: &Path, tasks: &str, spec: &str) {
    let change_dir = repository.join(".ito/changes").join(CHANGE_ID);
    std::fs::create_dir_all(change_dir.join("specs/main-first")).unwrap();
    std::fs::write(change_dir.join(".ito.yaml"), "schema: spec-driven\n").unwrap();
    std::fs::write(change_dir.join("proposal.md"), PROPOSAL).unwrap();
    std::fs::write(change_dir.join("design.md"), DESIGN).unwrap();
    std::fs::write(change_dir.join("tasks.md"), tasks).unwrap();
    std::fs::write(change_dir.join("specs/main-first/spec.md"), spec).unwrap();
}

fn write_project_validation(repository: &Path, scenario_grammar_level: &str) {
    let schema_dir = repository.join(".ito/templates/schemas/spec-driven");
    std::fs::create_dir_all(&schema_dir).unwrap();
    let schema = ito_templates::get_schema_file("spec-driven/schema.yaml")
        .expect("embedded spec-driven schema");
    std::fs::write(schema_dir.join("schema.yaml"), schema).unwrap();
    std::fs::write(
        schema_dir.join("validation.yaml"),
        format!(
            r#"version: 1
defaults:
  missing_required_artifact_level: error
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: {scenario_grammar_level}
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
"#
        ),
    )
    .unwrap();
}

fn init_readiness_repository() -> (tempfile::TempDir, PathBuf) {
    let fixture = tempfile::tempdir().expect("fixture root");
    let repository = fixture.path().join("repository");
    run_git(
        fixture.path(),
        &[
            "init",
            "--initial-branch=main",
            repository.to_str().unwrap(),
        ],
    );
    run_git(&repository, &["config", "user.name", "Ito Test"]);
    run_git(
        &repository,
        &["config", "user.email", "ito@example.invalid"],
    );
    std::fs::write(repository.join("README.md"), "fixture\n").unwrap();
    commit_all(&repository, "initial");
    (fixture, repository)
}

fn commit_all(repository: &Path, message: &str) -> String {
    run_git(repository, &["add", "."]);
    run_git(repository, &["commit", "-m", message]);
    git_stdout(repository, &["rev-parse", "HEAD"])
}

fn evaluate_direct_readiness(repository: &Path) -> ReadinessReport {
    evaluate_readiness(
        &request(repository),
        &config(ProposalIntegrationMode::DirectMerge),
    )
}

fn evaluate_execute_readiness(repository: &Path, current_checkout: &Path) -> ReadinessReport {
    let mut config = config(ProposalIntegrationMode::DirectMerge);
    config.worktrees.enabled = true;
    config.worktrees.strategy = WorktreeStrategy::CheckoutSiblings;
    let request = ReadinessRequest::new(CHANGE_ID, ReadinessPhase::Execute, repository)
        .with_current_checkout(current_checkout);
    evaluate_readiness(&request, &config)
}

fn add_linked_worktree(
    fixture_root: &Path,
    repository: &Path,
    branch: &str,
    start_point: &str,
) -> PathBuf {
    let worktrees_root = fixture_root.join("repository-ito-worktrees");
    std::fs::create_dir_all(&worktrees_root).unwrap();
    let worktree = worktrees_root.join(branch);
    run_git(
        repository,
        &[
            "worktree",
            "add",
            "-b",
            branch,
            worktree.to_str().unwrap(),
            start_point,
        ],
    );
    worktree
}

#[cfg(unix)]
fn assert_committed_symlink(repository: &Path, path: &str) {
    let entry = git_stdout(repository, &["ls-tree", "HEAD", "--", path]);
    assert!(
        entry.starts_with("120000 blob "),
        "expected a committed symlink for '{path}', got: {entry}"
    );
}

fn failed_condition<'a>(report: &'a ReadinessReport, code: &str) -> &'a ReadinessCondition {
    report
        .conditions
        .iter()
        .find(|condition| condition.code == code && !condition.passed)
        .unwrap_or_else(|| panic!("missing failed condition '{code}': {report:#?}"))
}

fn assert_failed_condition(report: &ReadinessReport, code: &str) {
    assert!(
        report
            .conditions
            .iter()
            .any(|condition| condition.code == code && !condition.passed),
        "missing failed condition '{code}': {report:#?}"
    );
}

fn assert_passed_condition(report: &ReadinessReport, code: &str) {
    assert!(
        report
            .conditions
            .iter()
            .any(|condition| condition.code == code && condition.passed),
        "missing passed condition '{code}': {report:#?}"
    );
}

fn git_succeeds(cwd: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(cwd)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .status()
        .expect("git should run")
        .success()
}

fn run_git(cwd: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(cwd)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(cwd: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(cwd)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

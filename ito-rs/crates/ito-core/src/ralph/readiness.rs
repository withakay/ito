use super::runner::{RalphOptions, run_ralph_with_readiness};
use crate::errors::{CoreError, CoreResult};
use crate::harness::Harness;
use crate::implementation_readiness::{
    ReadinessPhase, ReadinessRequest, evaluate_readiness, render_readiness_text,
};
use ito_config::{ConfigContext, load_cascading_project_config};
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;
use std::path::{Path, PathBuf};

/// Effective checkout and `.ito` state directory for a Ralph invocation.
#[derive(Debug, Clone)]
pub struct ResolvedCwd {
    /// Directory where the harness and Git commands execute.
    pub path: PathBuf,
    /// `.ito` directory used for state writes.
    pub ito_path: PathBuf,
}

/// Readiness boundary used by Ralph before a change-scoped operation.
pub trait RalphReadinessGate {
    /// Reject before any iteration, task update, Git automation, or state write.
    fn require(&self, ito_path: &Path, change_id: &str, checkout: &ResolvedCwd) -> CoreResult<()>;
}

#[derive(Debug, Default)]
pub(super) struct SystemRalphReadinessGate;

impl RalphReadinessGate for SystemRalphReadinessGate {
    fn require(&self, ito_path: &Path, change_id: &str, checkout: &ResolvedCwd) -> CoreResult<()> {
        let repository_root = ito_path.parent().unwrap_or_else(|| Path::new("."));
        let loaded = load_cascading_project_config(
            repository_root,
            ito_path,
            &ConfigContext::from_process_env(),
        );
        let config = serde_json::from_value(loaded.merged).map_err(|error| {
            CoreError::Validation(format!(
                "Cannot evaluate Ralph implementation readiness because Ito configuration is invalid: {error}"
            ))
        })?;
        let request = ReadinessRequest::new(change_id, ReadinessPhase::Execute, repository_root)
            .with_current_checkout(&checkout.path);
        let report = evaluate_readiness(&request, &config);
        if report.ready {
            return Ok(());
        }
        Err(CoreError::Validation(format!(
            "Ralph did not start because implementation readiness failed before any iteration or state mutation.\n\n{}",
            render_readiness_text(&report)
        )))
    }
}

/// Run Ralph with the system main-first readiness boundary.
pub fn run_ralph(
    ito_path: &Path,
    change_repo: &(impl ChangeRepository + ?Sized),
    task_repo: &dyn TaskRepository,
    module_repo: &(impl ModuleRepository + ?Sized),
    opts: RalphOptions,
    harness: &mut dyn Harness,
) -> CoreResult<()> {
    run_ralph_with_readiness(
        ito_path,
        change_repo,
        task_repo,
        module_repo,
        opts,
        harness,
        &SystemRalphReadinessGate,
    )
}

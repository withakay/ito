use ito_core::harness::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use ito_core::ralph::{RalphOptions, run_ralph};
use ito_domain::changes::{
    Change, ChangeRepository, ChangeSummary, ChangeTargetResolution, ResolveTargetOptions,
};
use ito_domain::errors::DomainResult;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

static CWD_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug)]
struct FixedHarness {
    name: HarnessName,
    outputs: Vec<(String, String, i32)>,
    idx: usize,
}

impl FixedHarness {
    fn new(name: HarnessName, outputs: Vec<(String, String, i32)>) -> Self {
        Self {
            name,
            outputs,
            idx: 0,
        }
    }

    fn next(&mut self) -> (String, String, i32) {
        if self.outputs.is_empty() {
            return (String::new(), String::new(), 0);
        }
        let v = self
            .outputs
            .get(self.idx)
            .cloned()
            .unwrap_or_else(|| self.outputs.last().cloned().unwrap());
        self.idx = self.idx.saturating_add(1);
        v
    }
}

impl Harness for FixedHarness {
    fn name(&self) -> HarnessName {
        self.name.clone()
    }

    fn run(&mut self, _config: &HarnessRunConfig) -> miette::Result<HarnessRunResult> {
        let (stdout, stderr, exit_code) = self.next();
        Ok(HarnessRunResult {
            stdout,
            stderr,
            exit_code,
            duration: Duration::from_millis(1),
            timed_out: false,
        })
    }

    fn stop(&mut self) {
        // No-op
    }
}

/// Harness that captures the cwd it receives from HarnessRunConfig.
#[derive(Debug)]
struct CwdCapturingHarness {
    captured_cwd: Option<PathBuf>,
}

impl Harness for CwdCapturingHarness {
    fn name(&self) -> HarnessName {
        HarnessName::STUB
    }

    fn run(&mut self, config: &HarnessRunConfig) -> miette::Result<HarnessRunResult> {
        self.captured_cwd = Some(config.cwd.clone());
        Ok(HarnessRunResult {
            stdout: "<promise>COMPLETE</promise>\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
            duration: Duration::from_millis(1),
            timed_out: false,
        })
    }

    fn stop(&mut self) {
        // No-op
    }
}

fn write_fixture_ito(ito_path: &Path, change_id: &str) {
    std::fs::create_dir_all(ito_path.join("changes").join(change_id)).unwrap();
    std::fs::write(
        ito_path.join("changes").join(change_id).join("proposal.md"),
        "# fixture\n",
    )
    .unwrap();

    // Provide module.md for module 006.
    let module_dir = ito_path.join("modules").join("006_ito-rs-port");
    std::fs::create_dir_all(&module_dir).unwrap();
    std::fs::write(module_dir.join("module.md"), "# 006_ito-rs-port\n").unwrap();
}

fn write_tasks(ito_path: &Path, change_id: &str, contents: &str) {
    let dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("tasks.md"), contents).unwrap();
}

fn write_spec(ito_path: &Path, change_id: &str) {
    let spec_dir = ito_path
        .join("changes")
        .join(change_id)
        .join("specs")
        .join("alpha");
    std::fs::create_dir_all(&spec_dir).unwrap();
    std::fs::write(
        spec_dir.join("spec.md"),
        "## Requirements\n\n### Requirement: Test\nThe system SHALL be testable.\n",
    )
    .unwrap();
}

fn write_ready_change(ito_path: &Path, change_id: &str) {
    write_fixture_ito(ito_path, change_id);
    write_spec(ito_path, change_id);
    write_tasks(ito_path, change_id, "# Tasks\n\n- [ ] todo\n");
}

fn default_opts() -> RalphOptions {
    RalphOptions {
        prompt: "do the thing".to_string(),
        change_id: None,
        module_id: None,
        model: None,
        min_iterations: 1,
        max_iterations: Some(3),
        completion_promise: "COMPLETE".to_string(),
        allow_all: false,
        no_commit: true,
        interactive: false,
        status: false,
        add_context: None,
        clear_context: false,
        verbose: false,
        continue_module: false,
        continue_ready: false,
        inactivity_timeout: None,
        skip_validation: false,
        validation_command: None,
        exit_on_error: false,
        error_threshold: 10,
        worktree: ito_core::ralph::WorktreeConfig::default(),
    }
}

fn run_ralph_for_test(
    ito_path: &Path,
    opts: RalphOptions,
    harness: &mut dyn Harness,
) -> ito_core::errors::CoreResult<()> {
    let change_repo = ito_core::change_repository::FsChangeRepository::new(ito_path);
    let task_repo = ito_core::task_repository::FsTaskRepository::new(ito_path);
    let module_repo = ito_core::module_repository::FsModuleRepository::new(ito_path);
    run_ralph(
        ito_path,
        &change_repo,
        &task_repo,
        &module_repo,
        opts,
        harness,
    )
}

fn run_ralph_for_test_with_change_repo(
    ito_path: &Path,
    change_repo: &impl ChangeRepository,
    opts: RalphOptions,
    harness: &mut dyn Harness,
) -> ito_core::errors::CoreResult<()> {
    let task_repo = ito_core::task_repository::FsTaskRepository::new(ito_path);
    let module_repo = ito_core::module_repository::FsModuleRepository::new(ito_path);
    run_ralph(
        ito_path,
        change_repo,
        &task_repo,
        &module_repo,
        opts,
        harness,
    )
}

#[derive(Debug, Clone)]
enum DriftAction {
    CompleteChange(String),
    CompleteAll,
    RemoveSpecs(String),
}

#[derive(Debug)]
struct DriftingChangeRepo {
    ito_path: std::path::PathBuf,
    action: DriftAction,
    did_drift: std::sync::Mutex<bool>,
}

impl DriftingChangeRepo {
    fn new(ito_path: &Path, action: DriftAction) -> Self {
        Self {
            ito_path: ito_path.to_path_buf(),
            action,
            did_drift: std::sync::Mutex::new(false),
        }
    }

    fn repo(&self) -> ito_core::change_repository::FsChangeRepository<'_> {
        ito_core::change_repository::FsChangeRepository::new(&self.ito_path)
    }

    fn maybe_drift(&self) {
        let Ok(mut did) = self.did_drift.lock() else {
            return;
        };
        if *did {
            return;
        }
        *did = true;

        match &self.action {
            DriftAction::CompleteChange(change_id) => {
                write_tasks(&self.ito_path, change_id, "# Tasks\n\n- [x] done\n");
            }
            DriftAction::CompleteAll => {
                let Ok(changes) = self.repo().list() else {
                    return;
                };
                for change in changes {
                    write_tasks(&self.ito_path, &change.id, "# Tasks\n\n- [x] done\n");
                }
            }
            DriftAction::RemoveSpecs(change_id) => {
                let specs_dir = self.ito_path.join("changes").join(change_id).join("specs");
                let _ = std::fs::remove_dir_all(specs_dir);
            }
        }
    }
}

impl ChangeRepository for DriftingChangeRepo {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        self.repo().resolve_target_with_options(input, options)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        self.repo().suggest_targets(input, max)
    }

    fn exists(&self, id: &str) -> bool {
        self.repo().exists(id)
    }

    fn get(&self, id: &str) -> DomainResult<Change> {
        self.repo().get(id)
    }

    fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        let out = self.repo().list();
        self.maybe_drift();
        out
    }

    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        self.repo().list_by_module(module_id)
    }

    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.repo().list_incomplete()
    }

    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.repo().list_complete()
    }

    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        self.repo().get_summary(id)
    }
}

#[test]
fn run_ralph_completion_promise_trims_whitespace() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![(
            ("<promise>\n  COMPLETE\n</promise>\n").to_string(),
            String::new(),
            0,
        )],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(1);
    run_ralph_for_test(&ito, opts, &mut h).unwrap();
    assert_eq!(h.idx, 1);
}

#[test]
fn run_ralph_continues_when_completion_validation_fails() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");
    write_tasks(
        &ito,
        "006-09_fixture",
        "# Tasks\n\n- [x] done\n- [ ] todo\n",
    );

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![
            (
                "<promise>COMPLETE</promise>\n".to_string(),
                String::new(),
                0,
            ),
            ("still working\n".to_string(), String::new(), 0),
        ],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(2);
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    // If validation incorrectly allowed completion, the loop would exit after 1 run.
    assert_eq!(h.idx, 2);

    let raw = std::fs::read_to_string(ito.join(".state/ralph/006-09_fixture/state.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v.get("iteration").and_then(|v| v.as_u64()).unwrap(), 2);
}

#[test]
fn run_ralph_skip_validation_exits_immediately() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");
    write_tasks(&ito, "006-09_fixture", "# Tasks\n\n- [ ] todo\n");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![
            (
                "<promise>COMPLETE</promise>\n".to_string(),
                String::new(),
                0,
            ),
            ("should not run\n".to_string(), String::new(), 0),
        ],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(2);
    opts.skip_validation = true;
    run_ralph_for_test(&ito, opts, &mut h).unwrap();
    assert_eq!(h.idx, 1);
}

#[test]
fn run_ralph_loop_writes_state_and_honors_min_iterations() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![
            (
                "<promise>COMPLETE</promise>\n".to_string(),
                String::new(),
                0,
            ),
            (
                "<promise>COMPLETE</promise>\n".to_string(),
                String::new(),
                0,
            ),
        ],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 2;
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    let state_path = ito.join(".state/ralph/006-09_fixture/state.json");
    assert!(state_path.exists());

    let raw = std::fs::read_to_string(state_path).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v.get("iteration").and_then(|v| v.as_u64()).unwrap(), 2);
    assert_eq!(
        v.get("history").and_then(|v| v.as_array()).unwrap().len(),
        2
    );
}

#[test]
fn run_ralph_errors_when_max_iterations_is_zero() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.max_iterations = Some(0);
    let err = run_ralph_for_test(&ito, opts, &mut h).unwrap_err();
    assert!(err.to_string().contains("--max-iterations"));
}

#[test]
fn run_ralph_returns_error_on_harness_failure() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![("boom".to_string(), "nope".to_string(), 2)],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.exit_on_error = true;
    let err = run_ralph_for_test(&ito, opts, &mut h).unwrap_err();
    assert!(err.to_string().contains("exited with code"));
}

#[test]
fn run_ralph_continues_after_harness_failure_by_default() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![
            ("build failed".to_string(), "compiler error".to_string(), 2),
            (
                "<promise>COMPLETE</promise>\n".to_string(),
                String::new(),
                0,
            ),
        ],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.skip_validation = true;
    run_ralph_for_test(&ito, opts, &mut h).unwrap();
    assert_eq!(h.idx, 2);
}

#[test]
fn run_ralph_fails_after_error_threshold() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![
            ("fail-1".to_string(), "err-1".to_string(), 2),
            ("fail-2".to_string(), "err-2".to_string(), 3),
            ("fail-3".to_string(), "err-3".to_string(), 4),
        ],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.error_threshold = 3;
    opts.max_iterations = Some(20);
    let err = run_ralph_for_test(&ito, opts, &mut h).unwrap_err();
    assert!(err.to_string().contains("exceeded non-zero exit threshold"));
}

#[test]
#[ignore = "Flaky in pre-commit: counts real uncommitted changes instead of test fixture"]
fn run_ralph_opencode_counts_git_changes_when_in_repo() {
    let _guard = CWD_LOCK.lock().unwrap();
    let original = std::env::current_dir().unwrap();

    let repo_td = tempfile::tempdir().unwrap();
    let repo = repo_td.path();

    // Keep the ito dir outside the git repo so it doesn't affect `git status`.
    let ito_td = tempfile::tempdir().unwrap();
    let ito = ito_td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    // Init git repo and create exactly one change.
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo)
        .status()
        .unwrap();
    std::fs::write(repo.join("untracked.txt"), "hi\n").unwrap();

    std::env::set_current_dir(repo).unwrap();

    let mut h = FixedHarness::new(
        HarnessName::OPENCODE,
        vec![(
            "<promise>COMPLETE</promise>\n".to_string(),
            String::new(),
            0,
        )],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(1);
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    let raw = std::fs::read_to_string(ito.join(".state/ralph/006-09_fixture/state.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let history = v.get("history").and_then(|v| v.as_array()).unwrap();
    let file_changes = history[0]
        .get("fileChangesCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    assert_eq!(file_changes, 1);

    std::env::set_current_dir(original).unwrap();
}

#[test]
fn state_helpers_append_and_clear_context() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    ito_core::ralph::state::append_context(&ito, "006-09_fixture", "hello").unwrap();
    ito_core::ralph::state::append_context(&ito, "006-09_fixture", "world").unwrap();

    let ctx = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx.contains("hello"));
    assert!(ctx.contains("world"));

    ito_core::ralph::state::clear_context(&ito, "006-09_fixture").unwrap();
    let ctx2 = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx2.trim().is_empty());
}

#[test]
fn run_ralph_status_path_works_with_no_state() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.status = true;
    run_ralph_for_test(&ito, opts, &mut h).unwrap();
}

#[test]
fn run_ralph_add_and_clear_context_paths() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);

    let mut add = default_opts();
    add.change_id = Some("006-09_fixture".to_string());
    add.add_context = Some("hello".to_string());
    add.prompt = String::new();
    run_ralph_for_test(&ito, add, &mut h).unwrap();

    let ctx = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx.contains("hello"));

    let mut clear = default_opts();
    clear.change_id = Some("006-09_fixture".to_string());
    clear.clear_context = true;
    clear.prompt = String::new();
    run_ralph_for_test(&ito, clear, &mut h).unwrap();

    let ctx2 = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx2.trim().is_empty());
}

#[test]
fn run_ralph_module_resolves_single_change() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_ready_change(&ito, "006-01_only");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.status = true;
    opts.module_id = Some("006".to_string());
    opts.prompt = String::new();
    run_ralph_for_test(&ito, opts, &mut h).unwrap();
}

#[test]
fn run_ralph_module_multiple_changes_errors_when_non_interactive() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_ready_change(&ito, "006-01_a");
    write_ready_change(&ito, "006-02_b");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![(
            "<promise>COMPLETE</promise>\n".to_string(),
            String::new(),
            0,
        )],
    );
    let mut opts = default_opts();
    opts.module_id = Some("006".to_string());
    opts.max_iterations = Some(1);
    opts.skip_validation = true;
    opts.prompt = String::new();
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    assert!(ito.join(".state/ralph/006-01_a/state.json").exists());
    assert!(!ito.join(".state/ralph/006-02_b/state.json").exists());
}

#[derive(Debug)]
struct CompletingHarness {
    complete_in_order: Vec<String>,
    ito_path: std::path::PathBuf,
    idx: usize,
}

fn extract_change_id_from_prompt(prompt: &str) -> Option<String> {
    let marker = "## Change Proposal (";
    let start = prompt.find(marker)?;
    let rest = &prompt[start + marker.len()..];
    let end = rest.find(')')?;
    Some(rest[..end].to_string())
}

#[derive(Debug)]
struct RecordingCompletingHarness {
    complete_in_order: Vec<String>,
    ito_path: std::path::PathBuf,
    idx: usize,
    seen_change_ids: Vec<String>,
}

impl Harness for RecordingCompletingHarness {
    fn name(&self) -> HarnessName {
        HarnessName::STUB
    }

    fn run(&mut self, config: &HarnessRunConfig) -> miette::Result<HarnessRunResult> {
        if let Some(change_id) = extract_change_id_from_prompt(&config.prompt) {
            self.seen_change_ids.push(change_id);
        }

        if let Some(change_id) = self.complete_in_order.get(self.idx) {
            write_tasks(&self.ito_path, change_id, "# Tasks\n\n- [x] done\n");
        }
        self.idx = self.idx.saturating_add(1);

        Ok(HarnessRunResult {
            stdout: "<promise>COMPLETE</promise>\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
            duration: Duration::from_millis(1),
            timed_out: false,
        })
    }

    fn stop(&mut self) {}
}

impl Harness for CompletingHarness {
    fn name(&self) -> HarnessName {
        HarnessName::STUB
    }

    fn run(&mut self, _config: &HarnessRunConfig) -> miette::Result<HarnessRunResult> {
        if let Some(change_id) = self.complete_in_order.get(self.idx) {
            write_tasks(&self.ito_path, change_id, "# Tasks\n\n- [x] done\n");
        }
        self.idx = self.idx.saturating_add(1);

        Ok(HarnessRunResult {
            stdout: "<promise>COMPLETE</promise>\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
            duration: Duration::from_millis(1),
            timed_out: false,
        })
    }

    fn stop(&mut self) {}
}

#[test]
fn run_ralph_continue_module_processes_all_ready_changes() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    write_ready_change(&ito, "006-01_a");
    write_ready_change(&ito, "006-02_b");

    let mut h = CompletingHarness {
        complete_in_order: vec!["006-01_a".to_string(), "006-02_b".to_string()],
        ito_path: ito.clone(),
        idx: 0,
    };

    let mut opts = default_opts();
    opts.module_id = Some("006".to_string());
    opts.continue_module = true;
    opts.max_iterations = Some(1);
    opts.skip_validation = true;
    opts.prompt = String::new();

    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    assert_eq!(h.idx, 2);
    assert!(ito.join(".state/ralph/006-01_a/state.json").exists());
    assert!(ito.join(".state/ralph/006-02_b/state.json").exists());
}

#[test]
fn run_ralph_continue_ready_processes_all_eligible_changes_across_repo() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    // Ready change.
    write_ready_change(&ito, "006-02_b");

    // In-progress change (eligible under continue-ready).
    write_fixture_ito(&ito, "007-01_a");
    write_spec(&ito, "007-01_a");
    write_tasks(&ito, "007-01_a", "# Tasks\n\n- [>] doing\n");

    let mut h = RecordingCompletingHarness {
        complete_in_order: vec!["006-02_b".to_string(), "007-01_a".to_string()],
        ito_path: ito.clone(),
        idx: 0,
        seen_change_ids: Vec::new(),
    };

    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.max_iterations = Some(1);
    opts.skip_validation = true;
    opts.prompt = String::new();

    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    assert_eq!(h.idx, 2);
    assert_eq!(
        h.seen_change_ids,
        vec!["006-02_b".to_string(), "007-01_a".to_string()]
    );
    assert!(ito.join(".state/ralph/006-02_b/state.json").exists());
    assert!(ito.join(".state/ralph/007-01_a/state.json").exists());
}

#[test]
fn run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    // Draft change (missing proposal/specs).
    write_tasks(&ito, "006-09_fixture", "# Tasks\n\n- [ ] todo\n");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.max_iterations = Some(1);
    opts.prompt = String::new();

    let err = run_ralph_for_test(&ito, opts, &mut h).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("no eligible changes"));
    assert!(msg.contains("006-09_fixture"));
}

#[test]
fn run_ralph_continue_ready_reorients_when_repo_state_shifts() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    write_ready_change(&ito, "006-01_a");
    write_ready_change(&ito, "006-02_b");

    let change_repo = DriftingChangeRepo::new(&ito, DriftAction::CompleteChange("006-01_a".into()));

    let mut h = RecordingCompletingHarness {
        complete_in_order: vec!["006-02_b".to_string()],
        ito_path: ito.clone(),
        idx: 0,
        seen_change_ids: Vec::new(),
    };

    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.max_iterations = Some(1);
    opts.skip_validation = true;
    opts.prompt = String::new();

    run_ralph_for_test_with_change_repo(&ito, &change_repo, opts, &mut h).unwrap();

    assert_eq!(h.seen_change_ids, vec!["006-02_b".to_string()]);
    assert!(!ito.join(".state/ralph/006-01_a/state.json").exists());
    assert!(ito.join(".state/ralph/006-02_b/state.json").exists());
}

#[test]
fn run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    write_ready_change(&ito, "006-01_a");

    let change_repo = DriftingChangeRepo::new(&ito, DriftAction::CompleteAll);

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.max_iterations = Some(1);
    opts.prompt = String::new();

    run_ralph_for_test_with_change_repo(&ito, &change_repo, opts, &mut h).unwrap();
    assert_eq!(h.idx, 0);
}

#[test]
fn run_ralph_continue_ready_errors_when_targeting_change_or_module() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    write_ready_change(&ito, "006-01_a");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.change_id = Some("006-01_a".to_string());
    opts.prompt = String::new();

    let err = run_ralph_for_test(&ito, opts, &mut h).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("--continue-ready"));
    assert!(msg.contains("--change") || msg.contains("--module"));
}

#[test]
fn run_ralph_worktree_disabled_uses_fallback_cwd() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = CwdCapturingHarness { captured_cwd: None };

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(1);
    // worktree is disabled by default
    assert!(!opts.worktree.enabled);
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    // When worktrees are disabled, cwd should be the process cwd (fallback)
    let captured = h.captured_cwd.unwrap();
    let process_cwd = std::env::current_dir().unwrap();
    assert_eq!(
        captured,
        process_cwd,
        "Expected fallback to process cwd, got: {}",
        captured.display()
    );
}

#[test]
fn run_ralph_worktree_enabled_state_written_to_effective_ito() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-09_fixture");

    let mut h = FixedHarness::new(
        HarnessName::STUB,
        vec![(
            "<promise>COMPLETE</promise>\n".to_string(),
            String::new(),
            0,
        )],
    );

    let mut opts = default_opts();
    opts.change_id = Some("006-09_fixture".to_string());
    opts.min_iterations = 1;
    opts.max_iterations = Some(1);
    // Worktree enabled but no actual worktree exists, so it falls back
    opts.worktree = ito_core::ralph::WorktreeConfig {
        enabled: true,
        dir_name: "ito-worktrees".to_string(),
    };
    run_ralph_for_test(&ito, opts, &mut h).unwrap();

    // State should still be written to the fallback ito_path since no worktree was found
    let state_path = ito.join(".state/ralph/006-09_fixture/state.json");
    assert!(
        state_path.exists(),
        "State should be written to fallback .ito when no worktree found"
    );
}

#[test]
fn run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    write_ready_change(&ito, "006-01_a");

    let change_repo = DriftingChangeRepo::new(&ito, DriftAction::RemoveSpecs("006-01_a".into()));

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.continue_ready = true;
    opts.prompt = String::new();

    let err = run_ralph_for_test_with_change_repo(&ito, &change_repo, opts, &mut h).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("changed during selection"), "{msg}");
}

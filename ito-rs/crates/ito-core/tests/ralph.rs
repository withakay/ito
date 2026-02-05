use ito_core::ralph::{RalphOptions, run_ralph};
use ito_harness::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use std::path::Path;
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
        inactivity_timeout: None,
    }
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
    run_ralph(&ito, opts, &mut h).unwrap();

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
    let err = run_ralph(&ito, opts, &mut h).unwrap_err();
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
    let err = run_ralph(&ito, opts, &mut h).unwrap_err();
    assert!(err.to_string().contains("exited with code"));
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
    run_ralph(&ito, opts, &mut h).unwrap();

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
    run_ralph(&ito, opts, &mut h).unwrap();
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
    run_ralph(&ito, add, &mut h).unwrap();

    let ctx = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx.contains("hello"));

    let mut clear = default_opts();
    clear.change_id = Some("006-09_fixture".to_string());
    clear.clear_context = true;
    clear.prompt = String::new();
    run_ralph(&ito, clear, &mut h).unwrap();

    let ctx2 = ito_core::ralph::state::load_context(&ito, "006-09_fixture").unwrap();
    assert!(ctx2.trim().is_empty());
}

#[test]
fn run_ralph_module_resolves_single_change() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-01_only");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.status = true;
    opts.module_id = Some("006".to_string());
    opts.prompt = String::new();
    run_ralph(&ito, opts, &mut h).unwrap();
}

#[test]
fn run_ralph_module_multiple_changes_errors_when_non_interactive() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();
    write_fixture_ito(&ito, "006-01_a");
    write_fixture_ito(&ito, "006-02_b");

    let mut h = FixedHarness::new(HarnessName::STUB, vec![]);
    let mut opts = default_opts();
    opts.module_id = Some("006".to_string());
    opts.status = true;
    opts.prompt = String::new();
    let err = run_ralph(&ito, opts, &mut h).unwrap_err();
    assert!(err.to_string().contains("Multiple changes"));
}

use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRunner};
use std::sync::Mutex as StdMutex;

// -- resolve_effective_cwd -------------------------------------------

#[test]
fn resolve_cwd_worktree_found() {
    let ito_path = Path::new("/project/.ito");
    let change_id = Some("002-16_ralph-worktree-awareness");
    let worktree = WorktreeConfig {
        enabled: true,
        dir_name: "ito-worktrees".to_string(),
    };
    let fallback = PathBuf::from("/project");
    let wt_path = PathBuf::from("/project/ito-worktrees/002-16_ralph-worktree-awareness");

    let result = resolve_effective_cwd_with(ito_path, change_id, &worktree, fallback, |_branch| {
        Some(wt_path.clone())
    });

    assert_eq!(result.path, wt_path);
    assert_eq!(result.ito_path, wt_path.join(".ito"));
}

#[test]
fn resolve_cwd_no_worktree_found_fallback() {
    let ito_path = Path::new("/project/.ito");
    let change_id = Some("005-01_some-change");
    let worktree = WorktreeConfig {
        enabled: true,
        dir_name: "ito-worktrees".to_string(),
    };
    let fallback = PathBuf::from("/project");

    let result = resolve_effective_cwd_with(
        ito_path,
        change_id,
        &worktree,
        fallback.clone(),
        |_branch| None,
    );

    assert_eq!(result.path, fallback);
    assert_eq!(result.ito_path, ito_path.to_path_buf());
}

#[test]
fn resolve_cwd_worktrees_not_enabled_fallback() {
    let ito_path = Path::new("/project/.ito");
    let change_id = Some("002-16_ralph-worktree-awareness");
    let worktree = WorktreeConfig {
        enabled: false,
        dir_name: "ito-worktrees".to_string(),
    };
    let fallback = PathBuf::from("/project");

    let result = resolve_effective_cwd_with(
        ito_path,
        change_id,
        &worktree,
        fallback.clone(),
        |_branch| {
            panic!("lookup should not be called when worktrees disabled");
        },
    );

    assert_eq!(result.path, fallback);
    assert_eq!(result.ito_path, ito_path.to_path_buf());
}

#[test]
fn resolve_cwd_no_change_targeted_fallback() {
    let ito_path = Path::new("/project/.ito");
    let worktree = WorktreeConfig {
        enabled: true,
        dir_name: "ito-worktrees".to_string(),
    };
    let fallback = PathBuf::from("/project");

    let result =
        resolve_effective_cwd_with(ito_path, None, &worktree, fallback.clone(), |_branch| {
            panic!("lookup should not be called without a change id");
        });

    assert_eq!(result.path, fallback);
    assert_eq!(result.ito_path, ito_path.to_path_buf());
}

// -- completion_promise_found ----------------------------------------

#[test]
fn promise_single_match() {
    assert!(completion_promise_found("x\n<promise>T</promise>\ny", "T"));
}

#[test]
fn promise_no_tags() {
    assert!(!completion_promise_found("no tags here", "T"));
}

#[test]
fn promise_second_match() {
    assert!(completion_promise_found(
        "<promise>W</promise><promise>T</promise>",
        "T"
    ));
}

#[test]
fn promise_empty_token() {
    assert!(completion_promise_found("<promise></promise>", ""));
}

#[test]
fn promise_empty_stdout() {
    assert!(!completion_promise_found("", "T"));
}

#[test]
fn promise_whitespace_trimmed() {
    assert!(completion_promise_found("<promise>  T  </promise>", "T"));
}

#[test]
fn promise_nested() {
    assert!(!completion_promise_found(
        "<promise><promise>T</promise></promise>",
        "T"
    ));
}

#[test]
fn promise_incomplete() {
    assert!(!completion_promise_found("<promise>T", "T"));
}

// -- infer_module_from_change ----------------------------------------

#[test]
fn infer_module_ok() {
    assert_eq!(infer_module_from_change("003-05_foo").unwrap(), "003");
    assert_eq!(infer_module_from_change("003-05").unwrap(), "003");
}

#[test]
fn infer_module_no_hyphen() {
    let CoreError::Validation(msg) = infer_module_from_change("x").unwrap_err() else {
        panic!("expected Validation");
    };
    assert!(msg.contains("Invalid change ID"));
}

// -- render helpers --------------------------------------------------

#[test]
fn render_validation_pass() {
    let r = validation::ValidationResult {
        success: true,
        message: "ok".into(),
        output: None,
    };
    let s = render_validation_result("T", &r);
    assert!(s.contains("PASS") && s.contains("### T"));
}

#[test]
fn render_validation_fail_with_output() {
    let r = validation::ValidationResult {
        success: false,
        message: "bad".into(),
        output: Some("detail".into()),
    };
    let s = render_validation_result("T", &r);
    assert!(s.contains("FAIL") && s.contains("```text"));
}

#[test]
fn render_validation_whitespace_output() {
    let r = validation::ValidationResult {
        success: true,
        message: "ok".into(),
        output: Some("  \n ".into()),
    };
    assert!(!render_validation_result("T", &r).contains("```"));
}

#[test]
fn render_failure_both() {
    let s = render_harness_failure("h", 1, "out", "err");
    assert!(s.contains("Stdout:") && s.contains("Stderr:"));
}

#[test]
fn render_failure_empty() {
    let s = render_harness_failure("h", 1, "", "");
    assert!(!s.contains("Stdout:") && !s.contains("Stderr:"));
}

// -- ChangeSummary filter helpers ------------------------------------

fn summary(id: &str, c: u32, ip: u32, p: u32, sh: u32, plan: bool) -> ChangeSummary {
    ChangeSummary {
        id: id.into(),
        module_id: None,
        completed_tasks: c,
        shelved_tasks: sh,
        in_progress_tasks: ip,
        pending_tasks: p,
        total_tasks: c + ip + p + sh,
        last_modified: chrono::Utc::now(),
        has_proposal: plan,
        has_design: false,
        has_specs: plan,
        has_tasks: plan,
    }
}

#[test]
fn filter_ready() {
    let c = vec![
        summary("a", 0, 0, 2, 0, true),
        summary("b", 3, 0, 0, 0, true),
    ];
    assert_eq!(module_ready_change_ids(&c), vec!["a"]);
}

#[test]
fn filter_eligible() {
    let c = vec![
        summary("z", 3, 0, 0, 0, true),
        summary("a", 0, 0, 2, 0, true),
        summary("m", 1, 1, 0, 0, true),
    ];
    assert_eq!(repo_eligible_change_ids(&c), vec!["a", "m"]);
}

#[test]
fn filter_incomplete() {
    let c = vec![
        summary("b", 0, 0, 0, 0, false),
        summary("a", 3, 0, 0, 0, true),
        summary("c", 0, 0, 1, 0, true),
    ];
    assert_eq!(repo_incomplete_change_ids(&c), vec!["b", "c"]);
}

#[test]
fn filter_module_incomplete() {
    let c = vec![
        summary("a", 3, 0, 0, 0, true),
        summary("b", 0, 0, 1, 0, true),
    ];
    assert_eq!(module_incomplete_change_ids(&c), vec!["b"]);
}

// -- print helpers (coverage only) -----------------------------------

#[test]
fn print_helpers() {
    print_eligible_changes(&[]);
    print_eligible_changes(&["a".into(), "b".into()]);
    print_ready_changes("x", &[]);
    print_ready_changes("x", &["a".into(), "b".into()]);
}

// -- Process helpers via mock ----------------------------------------

struct MockRunner(StdMutex<Vec<Result<ProcessOutput, ProcessExecutionError>>>);

impl MockRunner {
    fn new(r: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
        Self(StdMutex::new(r))
    }
}

impl ProcessRunner for MockRunner {
    fn run(
        &self,
        _req: &crate::process::ProcessRequest,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        self.0.lock().unwrap().remove(0)
    }

    fn run_with_timeout(
        &self,
        req: &crate::process::ProcessRequest,
        _t: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        self.run(req)
    }
}

fn ok(stdout: &str, code: i32) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: code,
        success: code == 0,
        stdout: stdout.into(),
        stderr: String::new(),
        timed_out: false,
    })
}

#[test]
fn git_changes_and_commit() {
    let cwd = Path::new("/tmp");
    assert_eq!(
        count_git_changes(&MockRunner::new(vec![ok(" M a\n M b\n", 0)]), cwd).unwrap(),
        2
    );
    assert_eq!(
        count_git_changes(&MockRunner::new(vec![ok("", 0)]), cwd).unwrap(),
        0
    );
    let fail = MockRunner::new(vec![Ok(ProcessOutput {
        exit_code: 128,
        success: false,
        stdout: String::new(),
        stderr: "fatal".into(),
        timed_out: false,
    })]);
    assert_eq!(count_git_changes(&fail, cwd).unwrap(), 0);
    commit_iteration(&MockRunner::new(vec![ok("", 0), ok("", 0)]), 1, cwd).unwrap();
    let bad = MockRunner::new(vec![Ok(ProcessOutput {
        exit_code: 1,
        success: false,
        stdout: String::new(),
        stderr: "e".into(),
        timed_out: false,
    })]);
    assert!(commit_iteration(&bad, 1, cwd).is_err());
    assert!(now_ms().unwrap() > 0);
}

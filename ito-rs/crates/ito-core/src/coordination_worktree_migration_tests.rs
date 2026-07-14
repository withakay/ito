use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput};
use std::cell::RefCell;

fn output(success: bool, stderr: &str) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: i32::from(!success),
        success,
        stdout: String::new(),
        stderr: stderr.to_string(),
        timed_out: false,
    })
}

fn configure_worktree(project_root: &Path, coord_wt: &Path) {
    let json = serde_json::json!({
        "changes": {
            "coordination_branch": {
                "storage": "worktree",
                "name": "ito/internal/changes",
                "worktree_path": coord_wt.to_str().unwrap()
            }
        }
    });
    std::fs::write(project_root.join("ito.json"), json.to_string()).unwrap();
}

fn create_legacy_authority_links(ito_path: &Path, coord_ito: &Path) {
    for subdir in ITO_SUBDIRS
        .iter()
        .chain(crate::coordination::AUTHORITATIVE_GIT_DIRS)
    {
        std::fs::create_dir_all(coord_ito.join(subdir)).unwrap();
    }
    crate::coordination::create_dir_link(&coord_ito.join("changes"), &ito_path.join("changes"))
        .unwrap();
    crate::coordination::create_dir_link(&coord_ito.join("specs"), &ito_path.join("specs"))
        .unwrap();
}

#[test]
#[cfg(unix)]
fn sync_fast_forwards_before_migrating_legacy_authority_links() {
    struct MergeWritingRunner {
        remote_change: PathBuf,
        calls: RefCell<Vec<Vec<String>>>,
    }

    impl ProcessRunner for MergeWritingRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.calls.borrow_mut().push(request.args.clone());
            if request.args.iter().any(|arg| arg == "merge") {
                std::fs::create_dir_all(self.remote_change.parent().unwrap()).unwrap();
                std::fs::write(&self.remote_change, "remote accepted proposal\n").unwrap();
            }
            output(true, "")
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!()
        }
    }

    let tmp = tempfile::TempDir::new().unwrap();
    let project_root = tmp.path();
    let ito_path = project_root.join(".ito");
    let coord_wt = tmp.path().join("coord-wt");
    let coord_ito = coord_wt.join(".ito");
    let wrong_modules = tmp.path().join("wrong-modules");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::create_dir_all(&wrong_modules).unwrap();
    create_legacy_authority_links(&ito_path, &coord_ito);
    crate::coordination::create_dir_link(&wrong_modules, &ito_path.join("modules")).unwrap();
    configure_worktree(project_root, &coord_wt);
    let runner = MergeWritingRunner {
        remote_change: coord_ito.join("changes/999-01_remote-only/proposal.md"),
        calls: RefCell::new(Vec::new()),
    };

    let error = sync_coordination_worktree_with_runner(&runner, project_root, &ito_path, false)
        .expect_err("wrong compatibility link should stop after authority migration");

    assert!(error.to_string().contains("should point to"));
    assert!(ito_path.join("changes").is_dir());
    assert!(!ito_path.join("changes").is_symlink());
    assert_eq!(
        std::fs::read_to_string(ito_path.join("changes/999-01_remote-only/proposal.md")).unwrap(),
        "remote accepted proposal\n"
    );
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0], ["fetch", "origin", "ito/internal/changes"]);
    assert!(calls[1].iter().any(|arg| arg == "merge"));
}

#[test]
#[cfg(unix)]
fn sync_without_origin_still_repairs_local_legacy_layout() {
    struct NoOriginRunner {
        calls: RefCell<Vec<Vec<String>>>,
    }

    impl ProcessRunner for NoOriginRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.calls.borrow_mut().push(request.args.clone());
            output(
                false,
                "fatal: 'origin' does not appear to be a git repository",
            )
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!()
        }
    }

    let tmp = tempfile::TempDir::new().unwrap();
    let project_root = tmp.path();
    let ito_path = project_root.join(".ito");
    let coord_wt = tmp.path().join("coord-wt");
    let coord_ito = coord_wt.join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    create_legacy_authority_links(&ito_path, &coord_ito);
    std::fs::create_dir_all(coord_ito.join("changes/031-01_local")).unwrap();
    std::fs::write(
        coord_ito.join("changes/031-01_local/proposal.md"),
        "local proposal\n",
    )
    .unwrap();
    configure_worktree(project_root, &coord_wt);
    let runner = NoOriginRunner {
        calls: RefCell::new(Vec::new()),
    };

    let outcome = sync_coordination_worktree_with_runner(&runner, project_root, &ito_path, false)
        .expect("local repair should succeed without an origin remote");

    assert_eq!(outcome, CoordinationSyncOutcome::RateLimited);
    assert_eq!(
        std::fs::read_to_string(ito_path.join("changes/031-01_local/proposal.md")).unwrap(),
        "local proposal\n"
    );
    assert!(!ito_path.join("changes").is_symlink());
    assert!(!ito_path.join("specs").is_symlink());
    for subdir in ITO_SUBDIRS {
        assert_eq!(
            std::fs::read_link(ito_path.join(subdir)).unwrap(),
            coord_ito.join(subdir)
        );
    }
    assert_eq!(runner.calls.borrow().len(), 1);
}

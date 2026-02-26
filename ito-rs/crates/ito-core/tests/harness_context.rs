use ito_core::harness_context::{InferredItoTarget, InferredItoTargetKind, infer_context_from_cwd};
use std::path::Path;

fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn write_dir(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
}

#[test]
fn infer_context_from_cwd_infers_change_from_path() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();

    let cwd = root
        .join("ito-worktrees")
        .join("023-07_harness-context-inference");
    write_dir(&cwd);

    let inferred = infer_context_from_cwd(&cwd).expect("infer");
    assert_eq!(
        inferred.target,
        Some(InferredItoTarget {
            kind: InferredItoTargetKind::Change,
            id: "023-07_harness-context-inference".to_string(),
        })
    );
    assert!(
        inferred
            .nudge
            .contains("ito tasks next 023-07_harness-context-inference")
    );
}

#[test]
fn infer_context_from_cwd_infers_module_from_ito_modules_path() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();

    let cwd = root.join(".ito").join("modules").join("023_harness-hooks");
    write_dir(&cwd);

    let inferred = infer_context_from_cwd(&cwd).expect("infer");
    assert_eq!(
        inferred.target,
        Some(InferredItoTarget {
            kind: InferredItoTargetKind::Module,
            id: "023".to_string(),
        })
    );
    assert!(inferred.nudge.contains("ito show module 023"));
}

#[test]
fn infer_context_from_cwd_infers_change_from_git_branch() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();
    write_dir(root);

    run_git(root, &["init"]);
    run_git(root, &["config", "user.email", "test@example.com"]);
    run_git(root, &["config", "user.name", "Test User"]);
    run_git(root, &["config", "commit.gpgsign", "false"]);
    std::fs::write(root.join("README.md"), "test\n").expect("write");
    run_git(root, &["add", "README.md"]);
    run_git(
        root,
        &["commit", "--no-verify", "--no-gpg-sign", "-m", "initial"],
    );

    run_git(
        root,
        &["checkout", "-b", "023-07_harness-context-inference"],
    );

    let inferred = infer_context_from_cwd(root).expect("infer");
    assert_eq!(
        inferred.target,
        Some(InferredItoTarget {
            kind: InferredItoTargetKind::Change,
            id: "023-07_harness-context-inference".to_string(),
        })
    );
}

#[test]
fn infer_context_from_cwd_prefers_path_over_git_branch() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();
    write_dir(root);

    run_git(root, &["init"]);
    run_git(root, &["config", "user.email", "test@example.com"]);
    run_git(root, &["config", "user.name", "Test User"]);
    run_git(root, &["config", "commit.gpgsign", "false"]);
    std::fs::write(root.join("README.md"), "test\n").expect("write");
    run_git(root, &["add", "README.md"]);
    run_git(
        root,
        &["commit", "--no-verify", "--no-gpg-sign", "-m", "initial"],
    );

    run_git(root, &["checkout", "-b", "001-01_branch-change"]);

    let cwd = root.join("ito-worktrees").join("002-01_path-change");
    write_dir(&cwd);

    let inferred = infer_context_from_cwd(&cwd).expect("infer");
    assert_eq!(
        inferred.target,
        Some(InferredItoTarget {
            kind: InferredItoTargetKind::Change,
            id: "002-01_path-change".to_string(),
        })
    );
}

#[test]
fn infer_context_from_cwd_returns_no_target_when_inconclusive() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();
    write_dir(root);

    let inferred = infer_context_from_cwd(root).expect("infer");
    assert_eq!(inferred.target, None);
    assert!(inferred.nudge.contains("No Ito change/module inferred"));
}

#[test]
fn infer_context_from_cwd_infers_module_from_git_branch() {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();
    write_dir(root);

    run_git(root, &["init"]);
    run_git(root, &["config", "user.email", "test@example.com"]);
    run_git(root, &["config", "user.name", "Test User"]);
    run_git(root, &["config", "commit.gpgsign", "false"]);
    std::fs::write(root.join("README.md"), "test\n").expect("write");
    run_git(root, &["add", "README.md"]);
    run_git(
        root,
        &["commit", "--no-verify", "--no-gpg-sign", "-m", "initial"],
    );

    run_git(root, &["checkout", "-b", "023_harness-hooks"]);

    let inferred = infer_context_from_cwd(root).expect("infer");
    assert_eq!(
        inferred.target,
        Some(InferredItoTarget {
            kind: InferredItoTargetKind::Module,
            id: "023".to_string(),
        })
    );
}

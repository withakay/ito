use super::*;

#[test]
fn parse_single_worktree() {
    let output = "worktree /home/user/project\nHEAD abc1234\nbranch refs/heads/main\n\n";
    let wts = parse_worktree_list(output);
    assert_eq!(wts.len(), 1);
    assert_eq!(wts[0].path, PathBuf::from("/home/user/project"));
    assert_eq!(wts[0].branch, Some("main".to_string()));
    assert!(wts[0].is_main);
}

#[test]
fn parse_multiple_worktrees() {
    let output = "\
worktree /home/user/project
HEAD abc1234
branch refs/heads/main

worktree /home/user/wt-feature
HEAD def5678
branch refs/heads/feature-x

";
    let wts = parse_worktree_list(output);
    assert_eq!(wts.len(), 2);
    assert!(wts[0].is_main);
    assert!(!wts[1].is_main);
    assert_eq!(wts[1].branch, Some("feature-x".to_string()));
}

#[test]
fn parse_bare_worktree_excluded() {
    let output = "\
worktree /home/user/project.git
bare

worktree /home/user/wt-main
HEAD abc1234
branch refs/heads/main

";
    let wts = parse_worktree_list(output);
    assert_eq!(wts.len(), 1);
    assert_eq!(wts[0].path, PathBuf::from("/home/user/wt-main"));
}

#[test]
fn parse_detached_head() {
    let output = "worktree /home/user/project\nHEAD abc1234\ndetached\n\n";
    let wts = parse_worktree_list(output);
    assert_eq!(wts.len(), 1);
    assert!(wts[0].branch.is_none());
}

#[test]
fn worktree_audit_log_path_resolves() {
    let wt = WorktreeInfo {
        path: PathBuf::from("/project/wt-feature"),
        branch: Some("feature".to_string()),
        is_main: false,
    };
    let path = worktree_audit_log_path(&wt);
    assert_eq!(
        path,
        PathBuf::from("/project/wt-feature/.ito/.state/audit/events.jsonl")
    );
}

#[test]
fn find_worktree_matching_branch() {
    let output = "\
worktree /home/user/project
HEAD abc1234
branch refs/heads/main

worktree /home/user/wt-feature
HEAD def5678
branch refs/heads/002-16_ralph-worktree-awareness

";
    let result = find_worktree_for_branch_in_output(output, "002-16_ralph-worktree-awareness");
    assert_eq!(result, Some(PathBuf::from("/home/user/wt-feature")));
}

#[test]
fn find_worktree_no_match() {
    let output = "\
worktree /home/user/project
HEAD abc1234
branch refs/heads/main

";
    let result = find_worktree_for_branch_in_output(output, "nonexistent-branch");
    assert!(result.is_none());
}

#[test]
fn find_worktree_bare_excluded() {
    let output = "\
worktree /home/user/project.git
bare

worktree /home/user/wt-main
HEAD abc1234
branch refs/heads/main

";
    // Even though the bare repo is listed first, it should be excluded
    let result = find_worktree_for_branch_in_output(output, "main");
    assert_eq!(result, Some(PathBuf::from("/home/user/wt-main")));
}

#[test]
fn find_worktree_multiple_returns_first_match() {
    let output = "\
worktree /home/user/project.git
bare

worktree /home/user/wt-main
HEAD abc1234
branch refs/heads/main

worktree /home/user/wt-feature-a
HEAD def5678
branch refs/heads/feature-a

worktree /home/user/wt-feature-b
HEAD 9ab0123
branch refs/heads/feature-b

";
    let result = find_worktree_for_branch_in_output(output, "feature-b");
    assert_eq!(result, Some(PathBuf::from("/home/user/wt-feature-b")));

    // Non-matching returns None
    let result = find_worktree_for_branch_in_output(output, "feature-c");
    assert!(result.is_none());
}

#[test]
fn ralph_worktree_prefers_worktrunk_json_match() {
    let output = r#"[
  {"path": "/home/user/ito-worktrees/002-16_ralph-worktree-awareness", "branch": "refs/heads/002-16_ralph-worktree-awareness"},
  {"path": "/home/user/ito-worktrees/other", "branch": "other"}
]"#;

    let result =
        find_worktree_for_branch_in_worktrunk_json(output, "002-16_ralph-worktree-awareness");
    assert_eq!(
        result,
        Some(PathBuf::from(
            "/home/user/ito-worktrees/002-16_ralph-worktree-awareness"
        ))
    );
}

#[test]
fn ralph_worktree_worktrunk_json_ignores_bare_entries() {
    let output = r#"{"worktrees":[
  {"path": "/home/user/project.git", "branch": "refs/heads/main", "bare": true},
  {"path": "/home/user/ito-worktrees/main", "branch": "main", "bare": false}
]}"#;

    let result = find_worktree_for_branch_in_worktrunk_json(output, "main");
    assert_eq!(result, Some(PathBuf::from("/home/user/ito-worktrees/main")));
}

#[test]
fn aggregate_empty_worktrees() {
    let results = aggregate_worktree_events(&[]);
    assert!(results.is_empty());
}

#[test]
fn aggregate_worktree_with_events() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let wt_path = tmp.path().join("wt");
    std::fs::create_dir_all(&wt_path).expect("create wt dir");

    // Write an event to this worktree's audit log
    let wt_ito_path = wt_path.join(".ito");
    let writer = crate::audit::writer::FsAuditWriter::new(&wt_ito_path);
    let event = ito_domain::audit::event::AuditEvent {
        v: 1,
        ts: "2026-02-08T14:30:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: "1.1".to_string(),
        scope: Some("ch".to_string()),
        op: "create".to_string(),
        from: None,
        to: Some("pending".to_string()),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        count: 1,
        ctx: ito_domain::audit::event::EventContext {
            session_id: "test".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    };
    ito_domain::audit::writer::AuditWriter::append(&writer, &event).unwrap();

    let wt_info = WorktreeInfo {
        path: wt_path,
        branch: Some("main".to_string()),
        is_main: true,
    };

    let results = aggregate_worktree_events(&[wt_info]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.len(), 1);
}

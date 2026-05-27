use super::*;

#[test]
fn removes_regular_specialist_files_and_prunes_empty_dirs() {
    let tempdir = tempfile::tempdir().expect("tempdir");
    let agent_dir = tempdir.path().join(".agents/skills");
    let obsolete_dir = agent_dir.join("ito-orchestrator-planner");
    std::fs::create_dir_all(&obsolete_dir).expect("obsolete dir");

    let obsolete = obsolete_dir.join("SKILL.md");
    std::fs::write(&obsolete, "legacy specialist asset").expect("obsolete file");

    remove_obsolete_specialist_agent(&agent_dir, "ito-orchestrator-planner/SKILL.md")
        .expect("cleanup succeeds");

    assert!(!obsolete.exists(), "obsolete file should be removed");
    assert!(
        !obsolete_dir.exists(),
        "empty legacy specialist directory should be pruned"
    );
}

#[cfg(unix)]
#[test]
fn removes_broken_specialist_symlinks_and_prunes_empty_dirs() {
    use std::os::unix::fs::symlink;

    let tempdir = tempfile::tempdir().expect("tempdir");
    let agent_dir = tempdir.path().join(".agents/skills");
    let obsolete_dir = agent_dir.join("ito-orchestrator-planner");
    std::fs::create_dir_all(&obsolete_dir).expect("obsolete dir");

    let obsolete = obsolete_dir.join("SKILL.md");
    symlink("missing-target.md", &obsolete).expect("symlink");

    remove_obsolete_specialist_agent(&agent_dir, "ito-orchestrator-planner/SKILL.md")
        .expect("cleanup succeeds");

    assert!(
        !obsolete.exists(),
        "broken obsolete symlink should be removed"
    );
    assert!(
        std::fs::symlink_metadata(&obsolete).is_err(),
        "removed symlink should no longer have metadata"
    );
    assert!(
        !obsolete_dir.exists(),
        "empty legacy specialist directory should be pruned"
    );
}

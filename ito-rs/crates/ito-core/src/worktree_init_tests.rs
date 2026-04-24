use std::fs;

use ito_config::types::WorktreeInitConfig;

use super::*;

#[test]
fn parse_worktree_include_file_strips_comments_and_blanks() {
    let content = "# This is a comment\n\
                   .env\n\
                   \n\
                   # Another comment\n\
                   .envrc\n\
                   \n";
    let patterns = parse_worktree_include_file(content);
    assert_eq!(patterns, vec![".env", ".envrc"]);
}

#[test]
fn parse_worktree_include_file_trims_whitespace() {
    let content = "  .env  \n  # comment \n  .envrc  \n";
    let patterns = parse_worktree_include_file(content);
    assert_eq!(patterns, vec![".env", ".envrc"]);
}

#[test]
fn parse_worktree_include_file_empty_content() {
    let patterns = parse_worktree_include_file("");
    assert!(patterns.is_empty());
}

#[test]
fn parse_worktree_include_file_comments_only() {
    let content = "# comment\n# another\n";
    let patterns = parse_worktree_include_file(content);
    assert!(patterns.is_empty());
}

#[test]
fn resolve_include_files_config_only() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    // Create files in the source root
    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(
        files,
        vec![PathBuf::from(".env"), PathBuf::from(".envrc")]
    );
}

#[test]
fn resolve_include_files_file_only() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(
        root.join(".worktree-include"),
        "# Copy env files\n.env\n",
    )
    .unwrap();

    let config = WorktreeInitConfig::default();

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_union_of_config_and_file() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".envrc"), "use nix").unwrap();
    fs::write(root.join(".worktree-include"), ".envrc\n").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(
        files,
        vec![PathBuf::from(".env"), PathBuf::from(".envrc")]
    );
}

#[test]
fn resolve_include_files_deduplicates() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".worktree-include"), ".env\n").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_missing_include_file_ok() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    // No .worktree-include file exists — should still work
    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_glob_expansion() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("app.local.toml"), "key=1").unwrap();
    fs::write(root.join("db.local.toml"), "key=2").unwrap();
    fs::write(root.join("app.toml"), "key=3").unwrap(); // should NOT match

    let config = WorktreeInitConfig {
        include: vec!["*.local.toml".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(
        files,
        vec![
            PathBuf::from("app.local.toml"),
            PathBuf::from("db.local.toml"),
        ]
    );
}

#[test]
fn resolve_include_files_no_match_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    // .env doesn't exist
    let files = resolve_include_files(&config, root).unwrap();
    assert!(files.is_empty());
}

#[test]
fn resolve_include_files_ignores_directories() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::create_dir(root.join(".env")).unwrap(); // directory, not file
    fs::write(root.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".envrc")]);
}

#[test]
fn copy_include_files_copies_to_dest() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "SECRET=abc").unwrap();
    fs::write(src.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let copied = copy_include_files(&config, src, dst).unwrap();
    assert_eq!(
        copied,
        vec![PathBuf::from(".env"), PathBuf::from(".envrc")]
    );

    assert_eq!(fs::read_to_string(dst.join(".env")).unwrap(), "SECRET=abc");
    assert_eq!(fs::read_to_string(dst.join(".envrc")).unwrap(), "use nix");
}

#[test]
fn copy_include_files_overwrites_existing() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "NEW_SECRET").unwrap();
    fs::write(dst.join(".env"), "OLD_SECRET").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    copy_include_files(&config, src, dst).unwrap();
    assert_eq!(fs::read_to_string(dst.join(".env")).unwrap(), "NEW_SECRET");
}

#[test]
fn copy_include_files_skips_missing_source() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let copied = copy_include_files(&config, src, dst).unwrap();
    assert!(copied.is_empty());
    assert!(!dst.join(".env").exists());
}

#[test]
fn copy_include_files_empty_config_and_no_file() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();

    let config = WorktreeInitConfig::default();

    let copied =
        copy_include_files(&config, src_dir.path(), dst_dir.path()).unwrap();
    assert!(copied.is_empty());
}

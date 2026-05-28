use super::*;
use std::path::PathBuf;

#[test]
fn normalize_strips_ansi_and_crlf() {
    let home = PathBuf::from("/tmp/home");
    let input = "\u{1b}[31mred\u{1b}[0m\r\nnext\r\n";
    let out = normalize_text(input, &home);
    assert_eq!(out, "red\nnext\n");
}

#[test]
fn normalize_replaces_home_path() {
    let home = PathBuf::from("/tmp/some/home");
    let input = "path=/tmp/some/home/.ito";
    let out = normalize_text(input, &home);
    assert_eq!(out, "path=<HOME>/.ito");
}

#[test]
fn copy_dir_all_copies_nested_files() {
    let src = tempfile::tempdir().expect("src");
    let dst = tempfile::tempdir().expect("dst");

    std::fs::create_dir_all(src.path().join("a/b")).unwrap();
    std::fs::write(src.path().join("a/b/file.txt"), "hello").unwrap();

    copy_dir_all(src.path(), dst.path()).unwrap();

    let copied = std::fs::read_to_string(dst.path().join("a/b/file.txt")).unwrap();
    assert_eq!(copied, "hello");
}

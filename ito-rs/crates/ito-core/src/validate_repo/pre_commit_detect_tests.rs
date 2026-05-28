use super::*;
use std::fs;
use tempfile::TempDir;

fn new_root() -> TempDir {
    TempDir::new().expect("tempdir")
}

fn write_file(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, body).expect("write fixture");
}

fn snapshot_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    // Capture the tree contents so we can prove the detector did not
    // mutate anything (pre-commit-hook-detection:read-only).
    fn walk(root: &Path, base: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(root).expect("read_dir") {
            let entry = entry.expect("entry");
            let path = entry.path();
            let rel = path.strip_prefix(base).expect("strip prefix").to_path_buf();
            if path.is_dir() {
                walk(&path, base, out);
            } else if path.is_file() {
                let bytes = fs::read(&path).expect("read");
                out.push((rel, bytes));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

#[test]
fn empty_repo_returns_none() {
    let tmp = new_root();
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::None,
    );
}

#[test]
fn pre_commit_config_alone_returns_pre_commit() {
    let tmp = new_root();
    write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::PreCommit,
    );
}

#[test]
fn pre_commit_config_with_prek_in_yaml_returns_prek() {
    let tmp = new_root();
    write_file(
        tmp.path(),
        ".pre-commit-config.yaml",
        "# prek: enabled\nrepos: []\n",
    );
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Prek,
    );
}

#[test]
fn prek_on_path_promotes_pre_commit_to_prek() {
    let tmp = new_root();
    write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), true),
        PreCommitSystem::Prek,
    );
}

#[test]
fn mise_mentioning_prek_returns_prek() {
    let tmp = new_root();
    write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
    write_file(tmp.path(), "mise.toml", "[tools]\nprek = \"latest\"\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Prek,
    );
}

#[test]
fn dot_mise_mentioning_prek_returns_prek() {
    let tmp = new_root();
    write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
    write_file(tmp.path(), ".mise.toml", "[tools]\nprek = \"latest\"\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Prek,
    );
}

#[test]
fn husky_directory_returns_husky() {
    let tmp = new_root();
    fs::create_dir(tmp.path().join(".husky")).expect("create .husky");
    write_file(tmp.path(), ".husky/pre-commit", "echo hi\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Husky,
    );
}

#[test]
fn package_json_with_husky_key_returns_husky() {
    let tmp = new_root();
    write_file(
        tmp.path(),
        "package.json",
        "{\n  \"name\": \"foo\",\n  \"husky\": {}\n}\n",
    );
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Husky,
    );
}

#[test]
fn lefthook_yml_returns_lefthook() {
    let tmp = new_root();
    write_file(tmp.path(), "lefthook.yml", "pre-commit:\n  commands: {}\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Lefthook,
    );
}

#[test]
fn dot_lefthook_yaml_returns_lefthook() {
    let tmp = new_root();
    write_file(tmp.path(), ".lefthook.yaml", "pre-commit: {}\n");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::Lefthook,
    );
}

#[test]
fn detection_is_read_only_for_every_variant() {
    // Build one fixture per variant and confirm bytes are unchanged
    // across the detector call.
    let cases: &[(&str, &[(&str, &str)])] = &[
        ("none", &[]),
        ("pre_commit", &[(".pre-commit-config.yaml", "repos: []\n")]),
        (
            "prek",
            &[(".pre-commit-config.yaml", "# prek\nrepos: []\n")],
        ),
        ("husky", &[("package.json", "{\"husky\": {}}\n")]),
        ("lefthook", &[("lefthook.yml", "pre-commit: {}\n")]),
    ];
    for (label, files) in cases {
        let tmp = new_root();
        for (rel, body) in *files {
            write_file(tmp.path(), rel, body);
        }
        let before = snapshot_tree(tmp.path());
        let _ = detect_pre_commit_system_with(tmp.path(), false);
        let after = snapshot_tree(tmp.path());
        assert_eq!(before, after, "detector mutated tree for fixture: {label}");
    }
}

#[test]
fn pre_commit_classification_overrides_husky() {
    // When both a husky directory AND a pre-commit config exist, the
    // pre-commit branch wins per the documented detection order.
    let tmp = new_root();
    write_file(tmp.path(), ".pre-commit-config.yaml", "repos: []\n");
    fs::create_dir(tmp.path().join(".husky")).expect("create .husky");
    assert_eq!(
        detect_pre_commit_system_with(tmp.path(), false),
        PreCommitSystem::PreCommit,
    );
}

#[test]
fn pre_commit_system_as_str_round_trips() {
    for variant in [
        PreCommitSystem::Prek,
        PreCommitSystem::PreCommit,
        PreCommitSystem::Husky,
        PreCommitSystem::Lefthook,
        PreCommitSystem::None,
    ] {
        assert_eq!(format!("{variant}"), variant.as_str());
    }
}

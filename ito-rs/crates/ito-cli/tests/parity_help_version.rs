use ito_test_support::run_rust_candidate;

fn make_fixture_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("tempdir");

    // Minimal ito repo layout.
    let m0 = td.path().join(".ito/modules/000_ungrouped");
    let m6 = td.path().join(".ito/modules/006_ito-rs-port");
    std::fs::create_dir_all(&m0).unwrap();
    std::fs::create_dir_all(&m6).unwrap();
    std::fs::write(m0.join("module.md"), "# 000_ungrouped\n").unwrap();
    std::fs::write(m6.join("module.md"), "# 006_ito-rs-port\n").unwrap();
    std::fs::create_dir_all(td.path().join(".ito/changes")).unwrap();

    td
}

#[test]
fn version_prints_workspace_version() {
    let repo = make_fixture_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let rs = run_rust_candidate(rust_path, &["--version"], repo.path(), home.path())
        .normalized(home.path());

    assert_eq!(rs.code, 0);
    assert!(rs.stderr.is_empty());

    let ver = option_env!("ITO_WORKSPACE_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let out = rs.stdout.trim();
    // Debug builds include git SHA suffix like "(abc1234)" or "(abc1234-dirty)"
    // Accept: "0.1.0", "ito 0.1.0", "0.1.0 (abc1234)", "0.1.0 (abc1234-dirty)"
    assert!(
        out == ver
            || out == format!("ito {ver}")
            || out.starts_with(&format!("{ver} ("))
            || out.starts_with(&format!("ito {ver} (")),
        "unexpected --version output: {out:?}"
    );
}

#[test]
fn help_prints_usage() {
    let repo = make_fixture_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let rs = run_rust_candidate(rust_path, &["--help"], repo.path(), home.path())
        .normalized(home.path());

    assert_eq!(rs.code, 0);
    assert!(rs.stdout.contains("Usage:"));
}

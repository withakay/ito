use ito_test_support::run_rust_candidate;

fn make_fixture_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("tempdir");

    // Minimal ito repo layout.
    let m0 = td.path().join(".ito/modules/000_ungrouped");
    std::fs::create_dir_all(&m0).unwrap();
    std::fs::write(m0.join("module.md"), "# 000_ungrouped\n").unwrap();
    std::fs::create_dir_all(td.path().join(".ito/changes")).unwrap();

    td
}

/// Normalizes version text by replacing concrete versions with `<VERSION>` and removing debug or placeholder suffixes.
///
/// This replaces the workspace build version (when `ITO_WORKSPACE_VERSION` is present) and the package version
/// (`CARGO_PKG_VERSION`) with `"<VERSION>"`. If the resulting text contains a trailing suffix of the form
/// `" (<git-sha>)"`, `" (<git-sha>-dirty)"`, or `" (VERGEN_...)"`, that suffix and the preceding space and
/// parentheses are removed.
///
/// # Examples
///
/// ```
/// let s = "ito 1.2.3 (abc1234-dirty)".to_string();
/// assert_eq!(normalize_version(s), "ito <VERSION>");
///
/// let s2 = "ito 1.2.3".to_string();
/// // package/workspace version replaced
/// assert_eq!(normalize_version(s2), "ito <VERSION>");
/// ```
fn normalize_version(text: String) -> String {
    // The CLI prints the workspace version (via build.rs) when available.
    // Debug builds also include git SHA suffix like "(abc1234-dirty)".
    // Snapshot tests should normalize all version-related output.
    let mut out = text;
    if let Some(ver) = option_env!("ITO_WORKSPACE_VERSION") {
        out = out.replace(ver, "<VERSION>");
    }
    out = out.replace(env!("CARGO_PKG_VERSION"), "<VERSION>");

    // Strip debug suffixes from builds: "<VERSION> (abc1234)",
    // "<VERSION> (abc1234-dirty)", or fallback placeholders like "<VERSION> (VERGEN_)".
    if let Some(pos) = out.find(" (")
        && let Some(close_pos) = out[pos..].find(')')
    {
        let content = &out[pos + 2..pos + close_pos];
        let is_git_sha = if let Some(dash_pos) = content.find('-') {
            content[..dash_pos].chars().all(|c| c.is_ascii_hexdigit())
                && &content[dash_pos..] == "-dirty"
        } else {
            content.chars().all(|c| c.is_ascii_hexdigit())
        };
        let is_vergen_placeholder = content.starts_with("VERGEN_");
        if is_git_sha || is_vergen_placeholder {
            out = out[..pos].to_string();
        }
    }
    out
}

fn normalize_trailing_whitespace(text: String) -> String {
    text.split('\n')
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn snapshot(args: &[&str]) -> String {
    let repo = make_fixture_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let rs = run_rust_candidate(rust_path, args, repo.path(), home.path()).normalized(home.path());
    assert_eq!(rs.code, 0, "unexpected exit code for args={args:?}");
    assert!(rs.stderr.is_empty(), "unexpected stderr for args={args:?}");

    normalize_version(normalize_trailing_whitespace(rs.stdout))
}

#[test]
fn snapshot_version() {
    insta::assert_snapshot!("ito_version", snapshot(&["--version"]));
}

#[test]
fn snapshot_help() {
    insta::assert_snapshot!("ito_help", snapshot(&["--help"]));
}

#[test]
fn snapshot_help_all_global_flag() {
    insta::assert_snapshot!("ito_help_all", snapshot(&["--help-all"]));
}

#[test]
fn snapshot_help_all_subcommand() {
    insta::assert_snapshot!("ito_help_subcommand_all", snapshot(&["help", "--all"]));
}

#[test]
fn snapshot_tasks_help() {
    insta::assert_snapshot!("ito_tasks_help", snapshot(&["tasks", "--help"]));
}

#[test]
fn snapshot_create_help() {
    insta::assert_snapshot!("ito_create_help", snapshot(&["create", "--help"]));
}

#[test]
fn snapshot_agent_help() {
    insta::assert_snapshot!("ito_agent_help", snapshot(&["agent", "--help"]));
}

#[test]
fn snapshot_agent_instruction_help() {
    insta::assert_snapshot!(
        "ito_agent_instruction_help",
        snapshot(&["agent", "instruction", "-h"])
    );
}

#[test]
fn snapshot_list_help() {
    insta::assert_snapshot!("ito_list_help", snapshot(&["list", "--help"]));
}

#[test]
fn snapshot_validate_help() {
    insta::assert_snapshot!("ito_validate_help", snapshot(&["validate", "--help"]));
}

#[test]
fn snapshot_init_help() {
    insta::assert_snapshot!("ito_init_help", snapshot(&["init", "--help"]));
}

#[test]
fn snapshot_ralph_help() {
    insta::assert_snapshot!("ito_ralph_help", snapshot(&["ralph", "--help"]));
}
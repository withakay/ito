use predicates::str::contains;

/// Verifies that the `ito templates schemas export -f <target>` command writes the embedded schema
/// and template files into the target directory.
///
/// The test runs the CLI to export templates into a temporary `.ito/templates/schemas` target,
/// asserts the command reports "Exported schemas", and checks that the following files exist:
/// `spec-driven/schema.yaml`, `spec-driven/templates/proposal.md`, and `tdd/schema.yaml`.
///
/// # Examples
///
/// ```
/// // Runs the test which performs the export and checks the generated files.
/// templates_schemas_export_writes_embedded_files();
/// ```
#[test]
fn templates_schemas_export_writes_embedded_files() {
    let td = tempfile::tempdir().expect("tempdir");
    let target = td.path().join(".ito/templates/schemas");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.current_dir(td.path())
        .arg("templates")
        .arg("schemas")
        .arg("export")
        .arg("-f")
        .arg(&target)
        .assert()
        .success()
        .stdout(contains("Exported schemas"));

    assert!(target.join("spec-driven/schema.yaml").exists());
    assert!(target.join("spec-driven/templates/proposal.md").exists());
    assert!(target.join("tdd/schema.yaml").exists());
}

/// Ensures exporting embedded template schemas does not overwrite existing files unless `--force` is used, and that using `--force` restores embedded defaults.
///
/// The test performs an initial export to a temporary target, mutates `spec-driven/schema.yaml`, verifies a subsequent non-forced export reports a skip and preserves the modification, then verifies a forced export replaces the file with the embedded default.
///
/// # Examples
///
/// ```
/// let td = tempfile::tempdir().expect("tempdir");
/// let target = td.path().join(".ito/templates/schemas");
///
/// let mut first = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// first
///     .current_dir(td.path())
///     .arg("templates")
///     .arg("schemas")
///     .arg("export")
///     .arg("-f")
///     .arg(&target)
///     .assert()
///     .success();
///
/// std::fs::write(
///     target.join("spec-driven/schema.yaml"),
///     "name: spec-driven\nversion: 1\ndescription: modified\nartifacts: []\n",
/// )
/// .expect("write override");
///
/// let mut second = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// second
///     .current_dir(td.path())
///     .arg("templates")
///     .arg("schemas")
///     .arg("export")
///     .arg("-f")
///     .arg(&target)
///     .assert()
///     .success()
///     .stdout(predicates::str::contains("Skipped:"));
///
/// let content = std::fs::read_to_string(target.join("spec-driven/schema.yaml"))
///     .expect("read after non-force export");
/// assert!(content.contains("description: modified"));
///
/// let mut forced = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// forced
///     .current_dir(td.path())
///     .arg("templates")
///     .arg("schemas")
///     .arg("export")
///     .arg("-f")
///     .arg(&target)
///     .arg("--force")
///     .assert()
///     .success();
///
/// let forced_content = std::fs::read_to_string(target.join("spec-driven/schema.yaml"))
///     .expect("read after force export");
/// assert!(!forced_content.contains("description: modified"));
/// assert!(forced_content.contains("description: Default Ito workflow"));
/// ```
fn templates_schemas_export_skips_without_force_then_overwrites_with_force() {
    let td = tempfile::tempdir().expect("tempdir");
    let target = td.path().join(".ito/templates/schemas");

    let mut first = assert_cmd::cargo::cargo_bin_cmd!("ito");
    first
        .current_dir(td.path())
        .arg("templates")
        .arg("schemas")
        .arg("export")
        .arg("-f")
        .arg(&target)
        .assert()
        .success();

    std::fs::write(
        target.join("spec-driven/schema.yaml"),
        "name: spec-driven\nversion: 1\ndescription: modified\nartifacts: []\n",
    )
    .expect("write override");

    let mut second = assert_cmd::cargo::cargo_bin_cmd!("ito");
    second
        .current_dir(td.path())
        .arg("templates")
        .arg("schemas")
        .arg("export")
        .arg("-f")
        .arg(&target)
        .assert()
        .success()
        .stdout(contains("Skipped:"));

    let content = std::fs::read_to_string(target.join("spec-driven/schema.yaml"))
        .expect("read after non-force export");
    assert!(content.contains("description: modified"));

    let mut forced = assert_cmd::cargo::cargo_bin_cmd!("ito");
    forced
        .current_dir(td.path())
        .arg("templates")
        .arg("schemas")
        .arg("export")
        .arg("-f")
        .arg(&target)
        .arg("--force")
        .assert()
        .success();

    let forced_content = std::fs::read_to_string(target.join("spec-driven/schema.yaml"))
        .expect("read after force export");
    assert!(!forced_content.contains("description: modified"));
    assert!(forced_content.contains("description: Default Ito workflow"));
}

#[test]
fn templates_help_includes_schemas_export() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("templates")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("schemas"));
}
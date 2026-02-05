use ito_core::validate::validate_change_dirs_repo_integrity;
use std::path::Path;

fn mkdir(path: &Path) {
    std::fs::create_dir_all(path).unwrap();
}

#[test]
fn duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    mkdir(&ito.join("modules").join("008_demo"));
    mkdir(&ito.join("changes").join("008-01_foo"));
    mkdir(&ito.join("changes").join("008-01_bar"));

    let issues = validate_change_dirs_repo_integrity(&ito).unwrap();
    let foo = issues.get("008-01_foo").expect("foo dir issues");
    let bar = issues.get("008-01_bar").expect("bar dir issues");

    assert!(
        foo.iter()
            .any(|i| i.message.contains("Duplicate numeric change id 008-01"))
    );
    assert!(foo.iter().any(|i| i.message.contains("008-01_bar")));
    assert!(
        bar.iter()
            .any(|i| i.message.contains("Duplicate numeric change id 008-01"))
    );
    assert!(bar.iter().any(|i| i.message.contains("008-01_foo")));
}

#[test]
fn change_referring_to_missing_module_is_an_error() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    mkdir(&ito.join("modules").join("008_demo"));
    mkdir(&ito.join("changes").join("009-01_test"));

    let issues = validate_change_dirs_repo_integrity(&ito).unwrap();
    let d = issues.get("009-01_test").expect("missing module issues");
    assert!(
        d.iter()
            .any(|i| i.message.contains("refers to missing module '009'"))
    );
}

#[test]
fn invalid_change_dir_names_are_reported() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    mkdir(&ito.join("changes").join("not-a-change"));

    let issues = validate_change_dirs_repo_integrity(&ito).unwrap();
    let d = issues.get("not-a-change").expect("invalid dir issues");
    assert!(d.iter().any(|i| {
        i.message
            .contains("Invalid change directory name 'not-a-change'")
    }));
}

#[test]
fn read_to_string_optional_returns_none_for_missing_file() {
    let td = tempfile::tempdir().unwrap();
    let p = td.path().join("missing.txt");
    let v = ito_common::io::read_to_string_optional(&p).unwrap();
    assert!(v.is_none());
}

#[test]
fn read_to_string_or_default_returns_empty_for_missing_file() {
    let td = tempfile::tempdir().unwrap();
    let p = td.path().join("missing.txt");
    let v = ito_common::io::read_to_string_or_default(&p);
    assert_eq!(v, "");
}

#[test]
fn write_atomic_std_creates_parent_and_replaces_contents() {
    let td = tempfile::tempdir().unwrap();
    let p = td.path().join("a").join("b").join("c.txt");

    ito_common::io::write_atomic_std(&p, "one\n").unwrap();
    let one = std::fs::read_to_string(&p).unwrap();
    assert_eq!(one, "one\n");

    ito_common::io::write_atomic_std(&p, "two\n").unwrap();
    let two = std::fs::read_to_string(&p).unwrap();
    assert_eq!(two, "two\n");

    let Some(parent) = p.parent() else {
        panic!("no parent");
    };
    assert!(parent.exists());
}

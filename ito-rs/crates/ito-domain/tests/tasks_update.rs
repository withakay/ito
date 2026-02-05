use ito_domain::tasks;

#[test]
fn update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting() {
    let md = "# Tasks\n\n  - [ ] first\n\t* [ ] second\n- [x] third\n";

    let out = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::InProgress)
        .expect("expected update to succeed");

    assert!(out.ends_with('\n'));
    assert!(out.contains("  - [ ] first"));
    assert!(out.contains("\t* [~] second"));
    assert!(out.contains("- [x] third"));
}

#[test]
fn update_checkbox_task_status_rejects_shelving() {
    let md = "- [ ] one\n";
    let err = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Shelved)
        .expect_err("expected error");
    assert!(err.to_lowercase().contains("does not support"));
}

#[test]
fn update_checkbox_task_status_errors_for_invalid_or_missing_task_id() {
    let md = "- [ ] one\n";

    assert!(tasks::update_checkbox_task_status(md, "nope", tasks::TaskStatus::Complete).is_err());
    assert!(tasks::update_checkbox_task_status(md, "0", tasks::TaskStatus::Complete).is_err());
    assert!(tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Complete).is_err());
}

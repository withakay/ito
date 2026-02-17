use ito_core::archive::{
    TaskStatus as ArchiveTaskStatus, archive_exists, categorize_specs, change_exists,
    check_task_completion, copy_specs_to_main, discover_change_specs, generate_archive_name,
    list_available_changes, move_to_archive,
};
use ito_core::module_repository::FsModuleRepository;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

#[test]
fn check_task_completion_handles_checkbox_and_enhanced_formats() {
    assert_eq!(check_task_completion(""), ArchiveTaskStatus::NoTasks);
    assert_eq!(
        check_task_completion("- [x] done\n* [X] done\n"),
        ArchiveTaskStatus::AllComplete
    );
    assert_eq!(
        check_task_completion("- [ ] todo\n- [x] done\n"),
        ArchiveTaskStatus::HasIncomplete {
            pending: 1,
            total: 2
        }
    );

    let enhanced = "- **Status**: [ ] pending\n- **Status**: [x] complete\n";
    assert_eq!(
        check_task_completion(enhanced),
        ArchiveTaskStatus::HasIncomplete {
            pending: 1,
            total: 2
        }
    );
}

#[test]
fn discover_and_copy_specs_and_archive_change() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    // Module + change setup.
    let change_name = "001-01_demo";
    let change_dir = ito.join("changes").join(change_name);
    write(change_dir.join("proposal.md").as_path(), "# proposal\n");

    // Module.md includes a checklist entry for the change.
    let module_dir = ito.join("modules").join("001_demo");
    write(
        module_dir.join("module.md").as_path(),
        &format!("# Demo\n\n## Changes\n- [ ] {change_name}\n"),
    );

    // Spec deltas under change.
    write(
        change_dir
            .join("specs")
            .join("auth")
            .join("spec.md")
            .as_path(),
        "# auth delta\n",
    );
    write(
        change_dir
            .join("specs")
            .join("billing")
            .join("spec.md")
            .as_path(),
        "# billing delta\n",
    );

    assert!(change_exists(&ito, change_name));

    let changes = list_available_changes(&ito).expect("list_available_changes");
    assert!(changes.contains(&change_name.to_string()));

    let specs = discover_change_specs(&ito, change_name).expect("discover_change_specs");
    assert_eq!(specs, vec!["auth".to_string(), "billing".to_string()]);

    // Categorize specs based on existence in main specs tree.
    write(
        ito.join("specs").join("auth").join("spec.md").as_path(),
        "# auth main\n",
    );
    let (new_specs, existing_specs) = categorize_specs(&ito, &specs);
    assert_eq!(existing_specs, vec!["auth".to_string()]);
    assert_eq!(new_specs, vec!["billing".to_string()]);

    // Copy deltas back to main specs.
    let updated = copy_specs_to_main(&ito, change_name, &specs).expect("copy_specs_to_main");
    assert_eq!(updated, specs);
    assert!(ito.join("specs").join("billing").join("spec.md").exists());

    // Archive.
    let module_repo = FsModuleRepository::new(&ito);
    let archive_name = format!("2026-01-01-{change_name}");
    move_to_archive(&module_repo, &ito, change_name, &archive_name).expect("move_to_archive");

    assert!(archive_exists(&ito, &archive_name));
    assert!(!ito.join("changes").join(change_name).exists());
    assert!(
        ito.join("changes")
            .join("archive")
            .join(&archive_name)
            .exists()
    );

    let module_md = std::fs::read_to_string(module_dir.join("module.md")).expect("read module");
    assert!(
        module_md.contains("- [x]"),
        "change should be marked complete"
    );
}

#[test]
fn generate_archive_name_prefixes_with_date() {
    let name = generate_archive_name("001-01_demo");
    assert!(name.ends_with("-001-01_demo"));
    assert!(name.len() > "001-01_demo".len());
}

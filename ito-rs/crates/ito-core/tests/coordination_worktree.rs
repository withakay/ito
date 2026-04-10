//! Integration tests verifying that filesystem repository adapters work
//! correctly when the canonical `.ito/` subdirectories are replaced by
//! symlinks pointing into a coordination worktree.
//!
//! Layout under test:
//!
//! ```text
//! tmp/
//! ├── project/
//! │   └── .ito/
//! │       ├── config.json          (real file)
//! │       ├── changes  ──────────► worktree/.ito/changes/
//! │       ├── modules  ──────────► worktree/.ito/modules/
//! │       └── specs    ──────────► worktree/.ito/specs/
//! └── worktree/
//!     └── .ito/
//!         ├── changes/
//!         ├── modules/
//!         └── specs/
//! ```
//!
//! All tests are gated with `#[cfg(unix)]` because directory symlinks require
//! Unix semantics.

#[cfg(unix)]
mod symlink_tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use ito_core::change_repository::FsChangeRepository;
    use ito_core::module_repository::FsModuleRepository;
    use ito_core::task_repository::FsTaskRepository;
    use ito_domain::tasks::TaskRepository as DomainTaskRepository;

    // ── Fixture helpers ───────────────────────────────────────────────────────

    /// Paths for the two sides of the simulated coordination layout.
    struct CoordLayout {
        /// Root temp directory (keeps the `TempDir` alive for the test).
        _tmp: TempDir,
        /// `<tmp>/project/.ito` — the project-side `.ito` directory.
        project_ito: PathBuf,
        /// `<tmp>/worktree/.ito` — the worktree-side `.ito` directory.
        worktree_ito: PathBuf,
    }

    /// Build the coordination layout and wire symlinks for `changes`, `modules`,
    /// and `specs`.
    ///
    /// The worktree directories are created first so the symlinks resolve
    /// immediately.  A minimal `config.json` is written into the project `.ito`
    /// directory so that any config-reading code has something to parse.
    fn setup_coord_layout() -> CoordLayout {
        let tmp = TempDir::new().expect("tempdir should be created");

        let project_ito = tmp.path().join("project").join(".ito");
        let worktree_ito = tmp.path().join("worktree").join(".ito");

        // Create the real directories on the worktree side first.
        for dir in ["changes", "modules", "specs"] {
            fs::create_dir_all(worktree_ito.join(dir))
                .expect("worktree subdirectory should be created");
        }

        // Create the project `.ito` directory (without the subdirs — they will
        // be symlinks).
        fs::create_dir_all(&project_ito).expect("project .ito should be created");

        // Write a minimal config so config-reading code does not error.
        fs::write(project_ito.join("config.json"), r#"{"version": 1}"#)
            .expect("config.json should be written");

        // Wire symlinks: project/.ito/<dir> → worktree/.ito/<dir>
        for dir in ["changes", "modules", "specs"] {
            let link = project_ito.join(dir);
            let target = worktree_ito.join(dir);
            symlink(&target, &link).expect("symlink should be created");
        }

        CoordLayout {
            _tmp: tmp,
            project_ito,
            worktree_ito,
        }
    }

    /// Create a minimal change directory directly inside `parent_changes_dir`.
    ///
    /// Writes a `proposal.md` so the change is non-empty and detectable.
    fn create_change_dir(parent_changes_dir: &Path, id: &str) {
        let change_dir = parent_changes_dir.join(id);
        fs::create_dir_all(&change_dir).expect("change directory should be created");
        fs::write(change_dir.join("proposal.md"), "# Proposal\n")
            .expect("proposal.md should be written");
    }

    /// Create a minimal module directory directly inside `parent_modules_dir`.
    fn create_module_dir(parent_modules_dir: &Path, id: &str, name: &str) {
        let module_dir = parent_modules_dir.join(format!("{id}_{name}"));
        fs::create_dir_all(&module_dir).expect("module directory should be created");
    }

    // ── FsChangeRepository through symlinks ───────────────────────────────────

    /// A change written into the worktree's `changes/` directory is visible
    /// when the repository is rooted at the project's `.ito/` path (which
    /// reaches `changes/` through a symlink).
    #[test]
    fn change_repo_exists_through_symlink() {
        let layout = setup_coord_layout();

        // Write the change directly into the worktree.
        create_change_dir(&layout.worktree_ito.join("changes"), "001-01_symlink-test");

        // Read through the project symlink.
        let repo = FsChangeRepository::new(&layout.project_ito);
        assert!(
            repo.exists("001-01_symlink-test"),
            "change written to worktree should be visible through project symlink"
        );
    }

    /// `list()` returns the change that was written into the worktree directory,
    /// accessed through the project-side symlink.
    #[test]
    fn change_repo_list_through_symlink() {
        let layout = setup_coord_layout();

        create_change_dir(&layout.worktree_ito.join("changes"), "002-03_list-via-link");

        let repo = FsChangeRepository::new(&layout.project_ito);
        let summaries = repo.list().expect("list should succeed through symlink");

        let ids: Vec<&str> = summaries.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["002-03_list-via-link"],
            "list should return the change written to the worktree"
        );
    }

    /// Multiple changes written to the worktree are all returned by `list()`.
    #[test]
    fn change_repo_list_multiple_through_symlink() {
        let layout = setup_coord_layout();

        let changes_dir = layout.worktree_ito.join("changes");
        create_change_dir(&changes_dir, "003-01_alpha");
        create_change_dir(&changes_dir, "003-02_beta");
        create_change_dir(&changes_dir, "003-03_gamma");

        let repo = FsChangeRepository::new(&layout.project_ito);
        let summaries = repo.list().expect("list should succeed");

        let mut ids: Vec<String> = summaries.into_iter().map(|s| s.id).collect();
        ids.sort();
        assert_eq!(
            ids,
            vec!["003-01_alpha", "003-02_beta", "003-03_gamma"],
            "all three changes should be visible through the symlink"
        );
    }

    /// `get()` loads the full change (including proposal) through the symlink.
    #[test]
    fn change_repo_get_through_symlink() {
        let layout = setup_coord_layout();

        create_change_dir(&layout.worktree_ito.join("changes"), "004-01_get-test");

        let repo = FsChangeRepository::new(&layout.project_ito);
        let change = repo
            .get("004-01_get-test")
            .expect("get should succeed through symlink");

        assert_eq!(change.id, "004-01_get-test");
        assert!(
            change.proposal.is_some(),
            "proposal.md should be readable through symlink"
        );
    }

    /// A change written through the project symlink is physically stored in the
    /// worktree directory.
    #[test]
    fn change_written_through_symlink_lands_in_worktree() {
        let layout = setup_coord_layout();

        // Write through the project symlink path.
        let via_link = layout
            .project_ito
            .join("changes")
            .join("005-01_written-via-link");
        fs::create_dir_all(&via_link).expect("create via symlink should succeed");
        fs::write(via_link.join("proposal.md"), "# Via link\n")
            .expect("write via symlink should succeed");

        // Verify the file physically exists in the worktree.
        let in_worktree = layout
            .worktree_ito
            .join("changes")
            .join("005-01_written-via-link")
            .join("proposal.md");
        assert!(
            in_worktree.exists(),
            "file written through symlink should physically reside in the worktree"
        );

        // And the repository can read it back.
        let repo = FsChangeRepository::new(&layout.project_ito);
        assert!(repo.exists("005-01_written-via-link"));
    }

    // ── FsModuleRepository through symlinks ───────────────────────────────────

    /// A module written into the worktree's `modules/` directory is visible
    /// when the repository is rooted at the project's `.ito/` path.
    #[test]
    fn module_repo_exists_through_symlink() {
        let layout = setup_coord_layout();

        create_module_dir(&layout.worktree_ito.join("modules"), "010", "core-engine");

        let repo = FsModuleRepository::new(&layout.project_ito);
        assert!(
            repo.exists("010"),
            "module written to worktree should be visible through project symlink"
        );
    }

    /// `list()` returns the module that was written into the worktree directory.
    #[test]
    fn module_repo_list_through_symlink() {
        let layout = setup_coord_layout();

        create_module_dir(&layout.worktree_ito.join("modules"), "011", "auth");

        let repo = FsModuleRepository::new(&layout.project_ito);
        let summaries = repo.list().expect("list should succeed through symlink");

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].id, "011");
        assert_eq!(summaries[0].name, "auth");
    }

    /// `get()` loads the full module through the symlink.
    #[test]
    fn module_repo_get_through_symlink() {
        let layout = setup_coord_layout();

        create_module_dir(&layout.worktree_ito.join("modules"), "012", "payments");

        let repo = FsModuleRepository::new(&layout.project_ito);
        let module = repo.get("012").expect("get should succeed through symlink");

        assert_eq!(module.id, "012");
        assert_eq!(module.name, "payments");
    }

    /// Multiple modules written to the worktree are all returned by `list()`,
    /// sorted by id.
    #[test]
    fn module_repo_list_multiple_through_symlink() {
        let layout = setup_coord_layout();

        let modules_dir = layout.worktree_ito.join("modules");
        create_module_dir(&modules_dir, "020", "billing");
        create_module_dir(&modules_dir, "021", "reporting");
        create_module_dir(&modules_dir, "022", "notifications");

        let repo = FsModuleRepository::new(&layout.project_ito);
        let summaries = repo.list().expect("list should succeed");

        let ids: Vec<&str> = summaries.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["020", "021", "022"],
            "all modules should be visible and sorted by id"
        );
    }

    /// Module change counts are computed correctly when changes are stored in
    /// the worktree and accessed through symlinks.
    #[test]
    fn module_repo_change_counts_through_symlink() {
        let layout = setup_coord_layout();

        create_module_dir(&layout.worktree_ito.join("modules"), "030", "search");

        let changes_dir = layout.worktree_ito.join("changes");
        create_change_dir(&changes_dir, "030-01_index-rebuild");
        create_change_dir(&changes_dir, "030-02_query-cache");

        let repo = FsModuleRepository::new(&layout.project_ito);
        let summaries = repo.list().expect("list should succeed");

        let module = summaries
            .iter()
            .find(|m| m.id == "030")
            .expect("module 030 should be present");
        assert_eq!(
            module.change_count, 2,
            "change count should reflect changes in the worktree"
        );
    }

    // ── FsTaskRepository through symlinks ─────────────────────────────────────

    /// Tasks written into a change directory in the worktree are readable
    /// through the project-side symlink.
    #[test]
    fn task_repo_load_tasks_through_symlink() {
        let layout = setup_coord_layout();

        let change_id = "040-01_task-symlink";
        let change_dir = layout.worktree_ito.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).expect("change dir should be created");
        fs::write(
            change_dir.join("tasks.md"),
            "# Tasks\n- [x] Done task\n- [ ] Pending task\n",
        )
        .expect("tasks.md should be written");

        let repo = FsTaskRepository::new(&layout.project_ito);
        let (completed, total) = repo
            .get_task_counts(change_id)
            .expect("get_task_counts should succeed through symlink");

        assert_eq!(completed, 1, "one task should be complete");
        assert_eq!(total, 2, "two tasks total");
    }

    /// `has_tasks()` returns `true` for a change with tasks in the worktree,
    /// accessed through the project symlink.
    #[test]
    fn task_repo_has_tasks_through_symlink() {
        let layout = setup_coord_layout();

        let change_id = "041-01_has-tasks-check";
        let change_dir = layout.worktree_ito.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).expect("change dir should be created");
        fs::write(change_dir.join("tasks.md"), "# Tasks\n- [ ] One task\n")
            .expect("tasks.md should be written");

        let repo = FsTaskRepository::new(&layout.project_ito);
        assert!(
            repo.has_tasks(change_id)
                .expect("has_tasks should succeed through symlink"),
            "has_tasks should return true for a change with tasks in the worktree"
        );
    }

    /// A tasks file written through the project symlink is physically stored in
    /// the worktree and is readable back through the repository.
    #[test]
    fn task_written_through_symlink_lands_in_worktree() {
        let layout = setup_coord_layout();

        let change_id = "042-01_write-tasks-via-link";

        // Write through the project symlink path.
        let via_link = layout.project_ito.join("changes").join(change_id);
        fs::create_dir_all(&via_link).expect("create via symlink should succeed");
        fs::write(
            via_link.join("tasks.md"),
            "# Tasks\n- [x] Task A\n- [x] Task B\n- [ ] Task C\n",
        )
        .expect("write via symlink should succeed");

        // Verify the file physically exists in the worktree.
        let in_worktree = layout
            .worktree_ito
            .join("changes")
            .join(change_id)
            .join("tasks.md");
        assert!(
            in_worktree.exists(),
            "tasks.md written through symlink should physically reside in the worktree"
        );

        // Read back through the repository.
        let repo = FsTaskRepository::new(&layout.project_ito);
        let (completed, total) = repo
            .get_task_counts(change_id)
            .expect("get_task_counts should succeed");

        assert_eq!(completed, 2);
        assert_eq!(total, 3);
    }

    /// A change with no tasks file returns zero counts (not an error) when
    /// accessed through a symlink.
    #[test]
    fn task_repo_missing_tasks_file_returns_zero_through_symlink() {
        let layout = setup_coord_layout();

        let change_id = "043-01_no-tasks";
        let change_dir = layout.worktree_ito.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).expect("change dir should be created");
        // Intentionally no tasks.md.

        let repo = FsTaskRepository::new(&layout.project_ito);
        let (completed, total) = repo
            .get_task_counts(change_id)
            .expect("get_task_counts should succeed even without tasks.md");

        assert_eq!(completed, 0);
        assert_eq!(total, 0);
    }

    // ── Cross-repository consistency ──────────────────────────────────────────

    /// All three repositories (change, module, task) can be used together
    /// against the same project `.ito/` path that has symlinked subdirectories,
    /// and they all see the data written into the worktree.
    #[test]
    fn all_repos_consistent_through_symlinks() {
        let layout = setup_coord_layout();

        // Set up a module.
        create_module_dir(&layout.worktree_ito.join("modules"), "050", "integration");

        // Set up a change with tasks.
        let change_id = "050-01_full-stack-test";
        let change_dir = layout.worktree_ito.join("changes").join(change_id);
        fs::create_dir_all(&change_dir).expect("change dir should be created");
        fs::write(change_dir.join("proposal.md"), "# Full stack\n")
            .expect("proposal.md should be written");
        fs::write(
            change_dir.join("tasks.md"),
            "# Tasks\n- [x] Step 1\n- [ ] Step 2\n- [ ] Step 3\n",
        )
        .expect("tasks.md should be written");

        let change_repo = FsChangeRepository::new(&layout.project_ito);
        let module_repo = FsModuleRepository::new(&layout.project_ito);
        let task_repo = FsTaskRepository::new(&layout.project_ito);

        // Change repository sees the change.
        assert!(change_repo.exists(change_id));
        let change = change_repo.get(change_id).expect("get change");
        assert_eq!(change.id, change_id);
        assert!(change.proposal.is_some());

        // Module repository sees the module with the correct change count.
        assert!(module_repo.exists("050"));
        let modules = module_repo.list().expect("list modules");
        let module = modules.iter().find(|m| m.id == "050").expect("module 050");
        assert_eq!(module.change_count, 1);

        // Task repository reads the tasks correctly.
        let (completed, total) = task_repo
            .get_task_counts(change_id)
            .expect("get task counts");
        assert_eq!(completed, 1);
        assert_eq!(total, 3);
    }
}

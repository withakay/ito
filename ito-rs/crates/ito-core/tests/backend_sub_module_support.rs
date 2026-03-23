//! Integration tests for sub-module support in backend repositories and stores.
//!
//! Covers Tasks 5.7 and 5.8: verifying that sub-module IDs are correctly
//! stored, retrieved, and resolved through the SQLite backend store and the
//! backend module/change repository adapters.

use std::path::PathBuf;

use ito_core::ModuleRepository;
use ito_core::backend_module_repository::BackendModuleRepository;
use ito_core::sqlite_project_store::{SqliteBackendProjectStore, UpsertChangeParams};
use ito_domain::backend::{BackendModuleReader, BackendProjectStore};
use ito_domain::changes::ChangeLifecycleFilter;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleSummary, SubModule, SubModuleSummary};

// ── Fake backend module reader with sub-module support ──────────────

struct FakeModuleReader {
    summaries: Vec<ModuleSummary>,
    modules: Vec<Module>,
}

impl FakeModuleReader {
    fn new(summaries: Vec<ModuleSummary>, modules: Vec<Module>) -> Self {
        Self { summaries, modules }
    }
}

impl BackendModuleReader for FakeModuleReader {
    fn list_modules(&self) -> DomainResult<Vec<ModuleSummary>> {
        Ok(self.summaries.clone())
    }

    fn get_module(&self, module_id: &str) -> DomainResult<Module> {
        self.modules
            .iter()
            .find(|m| m.id == module_id || m.name == module_id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("module", module_id))
    }
}

// ── Task 5.7: SQLite backend store sub-module round-trip ────────────

#[test]
fn sqlite_store_persists_sub_module_id_on_change() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store.ensure_project("org", "repo").expect("ensure project");

    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "005.01-03_my-change",
            module_id: Some("005"),
            sub_module_id: Some("005.01"),
            proposal: Some("# Sub-module change"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("upsert sub-module change");

    let change_repo = store.change_repository("org", "repo").expect("change repo");

    // List returns the change with sub_module_id populated.
    let changes = change_repo.list().expect("list changes");
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].id, "005.01-03_my-change");
    assert_eq!(changes[0].module_id.as_deref(), Some("005"));
    assert_eq!(changes[0].sub_module_id.as_deref(), Some("005.01"));

    // Get returns the full change with sub_module_id populated.
    let change = change_repo.get("005.01-03_my-change").expect("get change");
    assert_eq!(change.id, "005.01-03_my-change");
    assert_eq!(change.module_id.as_deref(), Some("005"));
    assert_eq!(change.sub_module_id.as_deref(), Some("005.01"));
}

#[test]
fn sqlite_store_sub_module_change_roundtrips_through_artifact_bundle() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store.ensure_project("org", "repo").expect("ensure project");

    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "024.02-01_api-gateway",
            module_id: Some("024"),
            sub_module_id: Some("024.02"),
            proposal: Some("# API Gateway"),
            design: None,
            tasks_md: Some("## 1. Tasks\n- [ ] 1.1 Implement"),
            specs: &[("api", "## ADDED\n### Gateway endpoint")],
        })
        .expect("upsert sub-module change");

    // Pull the artifact bundle — change_id must be preserved with dots.
    let bundle = store
        .pull_artifact_bundle("org", "repo", "024.02-01_api-gateway")
        .expect("pull bundle");
    assert_eq!(bundle.change_id, "024.02-01_api-gateway");
    assert_eq!(bundle.proposal.as_deref(), Some("# API Gateway"));
    assert_eq!(bundle.specs.len(), 1);
    assert_eq!(bundle.specs[0].0, "api");
}

#[test]
fn sqlite_store_legacy_change_has_no_sub_module_id() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store.ensure_project("org", "repo").expect("ensure project");

    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "005-01_legacy-change",
            module_id: Some("005"),
            sub_module_id: None,
            proposal: Some("# Legacy change"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("upsert legacy change");

    let change_repo = store.change_repository("org", "repo").expect("change repo");
    let changes = change_repo.list().expect("list changes");
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].id, "005-01_legacy-change");
    assert_eq!(changes[0].module_id.as_deref(), Some("005"));
    assert_eq!(changes[0].sub_module_id, None);
}

// ── Task 5.7: Backend module repository sub-module resolution ───────

#[test]
fn backend_module_repository_list_sub_modules_returns_sorted_summaries() {
    let module = Module {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        description: None,
        path: PathBuf::new(),
        sub_modules: vec![
            SubModule {
                id: "005.02".to_string(),
                parent_module_id: "005".to_string(),
                sub_id: "02".to_string(),
                name: "ci-cd".to_string(),
                description: None,
                change_count: 1,
                path: PathBuf::new(),
            },
            SubModule {
                id: "005.01".to_string(),
                parent_module_id: "005".to_string(),
                sub_id: "01".to_string(),
                name: "core-api".to_string(),
                description: Some("Core API sub-module".to_string()),
                change_count: 3,
                path: PathBuf::new(),
            },
        ],
    };
    let summary = ModuleSummary {
        id: "005".to_string(),
        name: "dev-tooling".to_string(),
        change_count: 4,
        sub_modules: vec![
            SubModuleSummary {
                id: "005.02".to_string(),
                name: "ci-cd".to_string(),
                change_count: 1,
            },
            SubModuleSummary {
                id: "005.01".to_string(),
                name: "core-api".to_string(),
                change_count: 3,
            },
        ],
    };

    let repo = BackendModuleRepository::new(FakeModuleReader::new(vec![summary], vec![module]));

    let sub_modules = repo.list_sub_modules("005").expect("list sub-modules");
    assert_eq!(sub_modules.len(), 2);
    // Should be sorted by ID.
    assert_eq!(sub_modules[0].id, "005.01");
    assert_eq!(sub_modules[0].name, "core-api");
    assert_eq!(sub_modules[0].change_count, 3);
    assert_eq!(sub_modules[1].id, "005.02");
    assert_eq!(sub_modules[1].name, "ci-cd");
}

#[test]
fn backend_module_repository_get_sub_module_by_composite_id() {
    let module = Module {
        id: "024".to_string(),
        name: "repository-backends".to_string(),
        description: None,
        path: PathBuf::new(),
        sub_modules: vec![
            SubModule {
                id: "024.01".to_string(),
                parent_module_id: "024".to_string(),
                sub_id: "01".to_string(),
                name: "sqlite".to_string(),
                description: Some("SQLite backend".to_string()),
                change_count: 2,
                path: PathBuf::new(),
            },
            SubModule {
                id: "024.02".to_string(),
                parent_module_id: "024".to_string(),
                sub_id: "02".to_string(),
                name: "r2".to_string(),
                description: None,
                change_count: 1,
                path: PathBuf::new(),
            },
        ],
    };
    let summary = ModuleSummary {
        id: "024".to_string(),
        name: "repository-backends".to_string(),
        change_count: 3,
        sub_modules: vec![],
    };

    let repo = BackendModuleRepository::new(FakeModuleReader::new(vec![summary], vec![module]));

    let sub = repo.get_sub_module("024.01").expect("get sub-module");
    assert_eq!(sub.id, "024.01");
    assert_eq!(sub.parent_module_id, "024");
    assert_eq!(sub.sub_id, "01");
    assert_eq!(sub.name, "sqlite");
    assert_eq!(sub.description.as_deref(), Some("SQLite backend"));
    assert_eq!(sub.change_count, 2);
}

#[test]
fn backend_module_repository_get_sub_module_not_found_returns_error() {
    let module = Module {
        id: "024".to_string(),
        name: "repository-backends".to_string(),
        description: None,
        path: PathBuf::new(),
        sub_modules: vec![SubModule {
            id: "024.01".to_string(),
            parent_module_id: "024".to_string(),
            sub_id: "01".to_string(),
            name: "sqlite".to_string(),
            description: None,
            change_count: 0,
            path: PathBuf::new(),
        }],
    };
    let summary = ModuleSummary {
        id: "024".to_string(),
        name: "repository-backends".to_string(),
        change_count: 0,
        sub_modules: vec![],
    };

    let repo = BackendModuleRepository::new(FakeModuleReader::new(vec![summary], vec![module]));

    let err = repo.get_sub_module("024.99").expect_err("should not find");
    assert!(
        matches!(err, DomainError::NotFound { .. }),
        "expected NotFound, got: {err:?}"
    );
}

#[test]
fn backend_module_repository_list_sub_modules_for_unknown_module_returns_error() {
    let repo = BackendModuleRepository::new(FakeModuleReader::new(vec![], vec![]));
    let err = repo
        .list_sub_modules("999")
        .expect_err("should not find module");
    assert!(
        matches!(err, DomainError::NotFound { .. }),
        "expected NotFound, got: {err:?}"
    );
}

// ── Task 5.8: Remote-mode list/show resolves sub-modules ────────────

#[test]
fn backend_module_repository_list_includes_sub_module_summaries() {
    let modules = vec![
        ModuleSummary {
            id: "001".to_string(),
            name: "core".to_string(),
            change_count: 5,
            sub_modules: vec![
                SubModuleSummary {
                    id: "001.01".to_string(),
                    name: "auth".to_string(),
                    change_count: 2,
                },
                SubModuleSummary {
                    id: "001.02".to_string(),
                    name: "api".to_string(),
                    change_count: 3,
                },
            ],
        },
        ModuleSummary {
            id: "002".to_string(),
            name: "infra".to_string(),
            change_count: 1,
            sub_modules: vec![],
        },
    ];

    let repo = BackendModuleRepository::new(FakeModuleReader::new(modules, vec![]));

    let listed = repo.list().expect("list modules");
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, "001");
    assert_eq!(listed[0].sub_modules.len(), 2);
    assert_eq!(listed[0].sub_modules[0].id, "001.01");
    assert_eq!(listed[0].sub_modules[1].id, "001.02");
    assert_eq!(listed[1].id, "002");
    assert_eq!(listed[1].sub_modules.len(), 0);
}

#[test]
fn sqlite_store_list_changes_filters_by_sub_module_id() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store.ensure_project("org", "repo").expect("ensure project");

    // Insert a sub-module change and a legacy change.
    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "005.01-01_sub-change",
            module_id: Some("005"),
            sub_module_id: Some("005.01"),
            proposal: Some("# Sub change"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("upsert sub-module change");

    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "005-01_direct-change",
            module_id: Some("005"),
            sub_module_id: None,
            proposal: Some("# Direct change"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("upsert direct change");

    let change_repo = store.change_repository("org", "repo").expect("change repo");
    let all = change_repo
        .list_with_filter(ChangeLifecycleFilter::Active)
        .expect("list all");
    assert_eq!(all.len(), 2);

    // The sub-module change should have sub_module_id set.
    let sub_change = all
        .iter()
        .find(|c| c.id == "005.01-01_sub-change")
        .expect("sub-module change");
    assert_eq!(sub_change.sub_module_id.as_deref(), Some("005.01"));
    assert_eq!(sub_change.module_id.as_deref(), Some("005"));

    // The direct change should have no sub_module_id.
    let direct_change = all
        .iter()
        .find(|c| c.id == "005-01_direct-change")
        .expect("direct change");
    assert_eq!(direct_change.sub_module_id, None);
    assert_eq!(direct_change.module_id.as_deref(), Some("005"));
}

use std::path::PathBuf;

use ito_core::ModuleRepository;
use ito_core::backend_module_repository::BackendModuleRepository;
use ito_core::list::list_modules;
use ito_core::show::read_module_markdown;
use ito_domain::backend::BackendModuleReader;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleSummary};

struct FakeBackendModuleReader {
    summaries: Vec<ModuleSummary>,
    modules: Vec<Module>,
}

impl FakeBackendModuleReader {
    fn new(summaries: Vec<ModuleSummary>, modules: Vec<Module>) -> Self {
        Self { summaries, modules }
    }
}

impl BackendModuleReader for FakeBackendModuleReader {
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

#[test]
fn backend_module_repository_normalizes_full_name_inputs() {
    let module = Module {
        id: "005".to_string(),
        name: "demo".to_string(),
        description: Some("Remote module".to_string()),
        path: PathBuf::new(),
        sub_modules: Vec::new(),
    };
    let summary = ModuleSummary {
        id: "005".to_string(),
        name: "demo".to_string(),
        change_count: 2,
        sub_modules: Vec::new(),
    };
    let repo =
        BackendModuleRepository::new(FakeBackendModuleReader::new(vec![summary], vec![module]));

    assert!(repo.exists("5_demo"));
    let resolved = repo.get("005_demo").expect("module should resolve");
    assert_eq!(resolved.id, "005");
}

#[test]
fn backend_module_repository_accepts_name_inputs() {
    let module = Module {
        id: "013".to_string(),
        name: "dev-tooling".to_string(),
        description: None,
        path: PathBuf::new(),
        sub_modules: Vec::new(),
    };
    let summary = ModuleSummary {
        id: "013".to_string(),
        name: "dev-tooling".to_string(),
        change_count: 1,
        sub_modules: Vec::new(),
    };
    let repo =
        BackendModuleRepository::new(FakeBackendModuleReader::new(vec![summary], vec![module]));

    let resolved = repo.get("dev-tooling").expect("module should resolve");
    assert_eq!(resolved.id, "013");
    assert_eq!(resolved.name, "dev-tooling");
}

#[test]
fn backend_module_repository_list_sorts_by_id() {
    let repo = BackendModuleRepository::new(FakeBackendModuleReader::new(
        vec![
            ModuleSummary {
                id: "010".to_string(),
                name: "zeta".to_string(),
                change_count: 1,
                sub_modules: Vec::new(),
            },
            ModuleSummary {
                id: "002".to_string(),
                name: "alpha".to_string(),
                change_count: 3,
                sub_modules: Vec::new(),
            },
        ],
        vec![],
    ));

    let modules = repo.list().expect("list modules should succeed");
    assert_eq!(modules.len(), 2);
    assert_eq!(modules[0].id, "002");
    assert_eq!(modules[1].id, "010");
}

#[test]
fn backend_module_repository_list_sorts_deterministically() {
    let repo = BackendModuleRepository::new(FakeBackendModuleReader::new(
        vec![
            ModuleSummary {
                id: "010".to_string(),
                name: "zeta".to_string(),
                change_count: 1,
                sub_modules: Vec::new(),
            },
            ModuleSummary {
                id: "002".to_string(),
                name: "alpha".to_string(),
                change_count: 3,
                sub_modules: Vec::new(),
            },
        ],
        vec![
            Module {
                id: "010".to_string(),
                name: "zeta".to_string(),
                description: None,
                path: PathBuf::new(),
                sub_modules: Vec::new(),
            },
            Module {
                id: "002".to_string(),
                name: "alpha".to_string(),
                description: None,
                path: PathBuf::new(),
                sub_modules: Vec::new(),
            },
        ],
    ));

    let modules = list_modules(&repo).expect("list modules should succeed");
    assert_eq!(modules.len(), 2);
    assert_eq!(modules[0].full_name, "002_alpha");
    assert_eq!(modules[1].full_name, "010_zeta");
}

#[test]
fn read_module_markdown_falls_back_without_local_file() {
    let module = Module {
        id: "025".to_string(),
        name: "repository-backends".to_string(),
        description: Some("Remote-only module".to_string()),
        path: PathBuf::new(),
        sub_modules: Vec::new(),
    };
    let summary = ModuleSummary {
        id: "025".to_string(),
        name: "repository-backends".to_string(),
        change_count: 4,
        sub_modules: Vec::new(),
    };
    let repo =
        BackendModuleRepository::new(FakeBackendModuleReader::new(vec![summary], vec![module]));

    let markdown = read_module_markdown(&repo, "025").expect("module markdown");
    assert!(markdown.contains("# repository-backends"));
    assert!(markdown.contains("## Purpose"));
    assert!(markdown.contains("Remote-only module"));
}

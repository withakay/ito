use std::path::PathBuf;

use chrono::Utc;

use ito_core::show::{
    bundle_specs_markdown_from_repository, bundle_specs_show_json_from_repository,
    read_spec_markdown_from_repository,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::specs::{SpecDocument, SpecRepository, SpecSummary};

struct FakeSpecRepo {
    specs: Vec<SpecDocument>,
}

impl FakeSpecRepo {
    fn new() -> Self {
        Self {
            specs: vec![
                SpecDocument {
                    id: "beta".to_string(),
                    path: PathBuf::from("/remote/specs/beta/spec.md"),
                    markdown: "# Beta\n".to_string(),
                    last_modified: Utc::now(),
                },
                SpecDocument {
                    id: "alpha".to_string(),
                    path: PathBuf::from("/remote/specs/alpha/spec.md"),
                    markdown: "# Alpha\n".to_string(),
                    last_modified: Utc::now(),
                },
            ],
        }
    }
}

impl SpecRepository for FakeSpecRepo {
    fn list(&self) -> DomainResult<Vec<SpecSummary>> {
        Ok(self
            .specs
            .iter()
            .map(|spec| SpecSummary {
                id: spec.id.clone(),
                path: spec.path.clone(),
                last_modified: spec.last_modified,
            })
            .collect())
    }

    fn get(&self, id: &str) -> DomainResult<SpecDocument> {
        self.specs
            .iter()
            .find(|spec| spec.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("spec", id))
    }
}

#[test]
fn bundle_specs_show_json_from_repository_sorts_ids() {
    let repo = FakeSpecRepo::new();

    let bundle = bundle_specs_show_json_from_repository(&repo).expect("bundle specs");

    assert_eq!(bundle.spec_count, 2);
    assert_eq!(bundle.specs[0].id, "alpha");
    assert_eq!(bundle.specs[1].id, "beta");
    assert_eq!(bundle.specs[0].path, "/remote/specs/alpha/spec.md");
    assert_eq!(bundle.specs[1].path, "/remote/specs/beta/spec.md");
}

#[test]
fn bundle_specs_markdown_from_repository_adds_metadata_comments() {
    let repo = FakeSpecRepo::new();

    let markdown = bundle_specs_markdown_from_repository(&repo).expect("bundle markdown");

    assert!(markdown.contains("<!-- spec-id: alpha; source: /remote/specs/alpha/spec.md -->"));
    assert!(markdown.contains("<!-- spec-id: beta; source: /remote/specs/beta/spec.md -->"));
    assert!(markdown.contains("# Alpha"));
    assert!(markdown.contains("# Beta"));
}

#[test]
fn read_spec_markdown_from_repository_reads_remote_spec() {
    let repo = FakeSpecRepo::new();

    let markdown = read_spec_markdown_from_repository(&repo, "beta").expect("read spec");

    assert_eq!(markdown, "# Beta\n");
}

//! Backend-backed promoted spec repository adapter.

use ito_domain::backend::BackendSpecReader;
use ito_domain::errors::DomainResult;
use ito_domain::specs::{SpecDocument, SpecRepository, SpecSummary};

/// Backend-backed promoted spec repository.
pub struct BackendSpecRepository<R: BackendSpecReader> {
    reader: R,
}

impl<R: BackendSpecReader> BackendSpecRepository<R> {
    /// Create a backend-backed promoted spec repository.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BackendSpecReader> SpecRepository for BackendSpecRepository<R> {
    fn list(&self) -> DomainResult<Vec<SpecSummary>> {
        self.reader.list_specs()
    }

    fn get(&self, id: &str) -> DomainResult<SpecDocument> {
        self.reader.get_spec(id)
    }
}

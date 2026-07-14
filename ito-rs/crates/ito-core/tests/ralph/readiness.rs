use ito_core::ralph::{RalphReadinessGate, ResolvedCwd};
use std::path::Path;

#[derive(Debug)]
pub(super) struct TestReadiness;

impl RalphReadinessGate for TestReadiness {
    fn require(
        &self,
        _ito_path: &Path,
        _change_id: &str,
        _checkout: &ResolvedCwd,
    ) -> ito_core::errors::CoreResult<()> {
        Ok(())
    }
}

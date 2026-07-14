use std::io::ErrorKind;
use std::path::Path;

use crate::errors::{CoreError, CoreResult};

const RETIRED_DEFAULT_GUIDANCE: &str = r#"
<!-- ITO:INTERNAL:START -->
## Project Guidance

[Subagents]|first-class tools; delegate independent work in parallel; ≥2 review passes for non-trivial changes
|explore: codebase nav/search |ito-test-runner: project tests/checks curated output
|rust-quality-checker: style/idioms |rust-code-reviewer: safety/idioms/arch
|rust-test-engineer: test strategy |codex-review: diff correctness+edge cases
|documentation-police: docs quality |code-simplifier: refactor for clarity
|code-quality-squad: parallel quality |perplexity-researcher[-pro]: web research+citations
|multi-agent: explore multiple approaches and synthesize
<!-- ITO:INTERNAL:END -->"#;

pub(super) fn remove_retired_default_guidance(path: &Path) -> CoreResult<bool> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(CoreError::io(format!("reading {}", path.display()), error)),
    };
    if !contents.contains(RETIRED_DEFAULT_GUIDANCE) {
        return Ok(false);
    }

    let updated = contents.replacen(RETIRED_DEFAULT_GUIDANCE, "", 1);
    std::fs::write(path, updated)
        .map_err(|error| CoreError::io(format!("writing {}", path.display()), error))?;
    Ok(true)
}

#[cfg(test)]
#[path = "project_guidance_cleanup_tests.rs"]
mod project_guidance_cleanup_tests;

use std::path::Path;
use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() {
    // Emit git and build metadata via vergen
    // This sets VERGEN_GIT_SHA, VERGEN_GIT_BRANCH, VERGEN_GIT_DIRTY, VERGEN_BUILD_TIMESTAMP
    let build = BuildBuilder::default().build_timestamp(true).build();
    let gitcl = GitclBuilder::default()
        .sha(true)
        .branch(true)
        .dirty(true)
        .build();

    let mut emitter = Emitter::default();
    if let Ok(build) = build {
        let _ = emitter.add_instructions(&build);
    }
    if let Ok(gitcl) = gitcl {
        let _ = emitter.add_instructions(&gitcl);
    }
    let _ = emitter.emit();

    // Keep `ito --version` in sync with the workspace version.
    // This avoids touching `crates/ito-cli/Cargo.toml` (release-please managed)
    // while still reflecting local version bumps in `ito-rs/Cargo.toml`.
    let workspace_manifest = Path::new("../../Cargo.toml");
    println!("cargo:rerun-if-changed={}", workspace_manifest.display());

    let Ok(contents) = std::fs::read_to_string(workspace_manifest) else {
        return;
    };
    let Some(version) = workspace_package_version(&contents) else {
        return;
    };

    println!("cargo:rustc-env=ITO_WORKSPACE_VERSION={version}");
}

fn workspace_package_version(contents: &str) -> Option<String> {
    let mut in_section = false;
    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed == "[workspace.package]" {
            in_section = true;
            continue;
        }

        if in_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            break;
        }

        if !in_section {
            continue;
        }

        let Some(rest) = trimmed.strip_prefix("version") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let value = rest.trim();
        let value = value.strip_prefix('"')?.strip_suffix('"')?;
        if value.trim().is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

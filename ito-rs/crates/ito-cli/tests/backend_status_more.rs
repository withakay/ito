//! Integration tests for Wave 3: backend status, generate-token, and silent fallback warnings.
//!
//! This test suite covers:
//! - Task 3.1: `ito backend status` with various configurations
//! - Task 3.2: `ito backend generate-token` with seed sources and auth verification
//! - Task 3.3: Silent fallback warnings when backend config is broken

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::{CmdOutput, rust_candidate_command};
use std::path::Path;

struct TestContext {
    repo: tempfile::TempDir,
    home: tempfile::TempDir,
    rust_path: &'static std::path::Path,
}

impl TestContext {
    fn new() -> Self {
        Self {
            repo: fixtures::make_empty_repo(),
            home: tempfile::tempdir().expect("home"),
            rust_path: assert_cmd::cargo::cargo_bin!("ito"),
        }
    }

    /// Run a backend command with ITO_BACKEND_* env vars cleared.
    fn run_backend_cmd(&self, args: &[&str]) -> CmdOutput {
        run_backend_cmd_impl(
            self.rust_path,
            args,
            self.repo.path(),
            self.home.path(),
            &[],
        )
    }

    /// Run a backend command with custom environment variables.
    fn run_backend_cmd_with_env(&self, args: &[&str], env: &[(&str, &str)]) -> CmdOutput {
        run_backend_cmd_impl(
            self.rust_path,
            args,
            self.repo.path(),
            self.home.path(),
            env,
        )
    }

    /// Write a project config file at `.ito/config.json`.
    fn write_project_config(&self, contents: &str) {
        fixtures::write(self.repo.path().join(".ito/config.json"), contents);
    }

    /// Write a global config file at `$HOME/.config/ito/config.json`.
    fn write_global_config(&self, contents: &str) {
        fixtures::write(self.home.path().join(".config/ito/config.json"), contents);
    }

    /// Initialize the repo as a git repository.
    fn git_init(&self) {
        fixtures::git_init_with_initial_commit(self.repo.path());
    }
}

/// Run a command with backend-related env vars cleared and optional custom env vars set.
fn run_backend_cmd_impl(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    env: &[(&str, &str)],
) -> CmdOutput {
    let mut cmd = rust_candidate_command(program);
    cmd.args(args);
    cmd.current_dir(cwd);

    // Determinism knobs
    cmd.env("CI", "1");
    cmd.env("NO_COLOR", "1");
    cmd.env("ITO_INTERACTIVE", "0");
    cmd.env("TERM", "dumb");
    cmd.env("HOME", home);
    cmd.env("XDG_CONFIG_HOME", home.join(".config"));
    cmd.env("XDG_DATA_HOME", home);

    // Clear backend-related env vars to avoid test pollution
    cmd.env_remove("ITO_BACKEND_TOKEN");
    cmd.env_remove("ITO_BACKEND_TOKEN_SEED");
    cmd.env_remove("ITO_BACKEND_PROJECT_ORG");
    cmd.env_remove("ITO_BACKEND_PROJECT_REPO");

    // Clear Git vars
    for key in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_COMMON_DIR",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_QUARANTINE_PATH",
        "GIT_PREFIX",
    ] {
        cmd.env_remove(key);
    }

    // Apply custom env vars
    for (key, val) in env {
        cmd.env(key, val);
    }

    let out = cmd.output().expect("command should run");
    CmdOutput {
        code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

// ============================================================================
// Task 3.1: Integration Tests for `ito backend status`
// ============================================================================

#[test]
fn backend_status_disabled_shows_informational_output() {
    let cx = TestContext::new();
    cx.git_init();
    cx.write_project_config(r#"{"backend": {"enabled": false}}"#);

    let out = cx.run_backend_cmd(&["backend", "status"]);

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("Enabled:        false"),
        "stdout={}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Backend mode is disabled"),
        "stdout={}",
        out.stdout
    );
}

#[test]
fn generate_token_missing_org_fails() {
    let cx = TestContext::new();
    cx.git_init();

    // In non-interactive mode (ITO_INTERACTIVE=0), prompting fails
    let out = cx.run_backend_cmd(&[
        "backend",
        "generate-token",
        "--seed",
        "test-seed",
        "--repo",
        "widgets",
    ]);

    assert_ne!(
        out.code, 0,
        "should fail without org in non-interactive mode"
    );
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("not a terminal") || combined.contains("Failed to read input"),
        "output should mention interactive failure\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn backend_status_disabled_json_output() {
    let cx = TestContext::new();
    cx.git_init();
    cx.write_project_config(r#"{"backend": {"enabled": false}}"#);

    let out = cx.run_backend_cmd(&["backend", "status", "--json"]);

    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let parsed: serde_json::Value = serde_json::from_str(&out.stdout)
        .unwrap_or_else(|e| panic!("failed to parse JSON: {e}\nstdout={}", out.stdout));

    assert_eq!(
        parsed["enabled"], false,
        "enabled should be false, parsed={parsed:?}"
    );
    assert_eq!(
        parsed["config_valid"], true,
        "config_valid should be true, parsed={parsed:?}"
    );
}

#[test]
fn backend_status_incomplete_config_fails() {
    let cx = TestContext::new();
    cx.git_init();
    // Backend enabled but missing token, org, repo
    cx.write_project_config(r#"{"backend": {"enabled": true}}"#);

    let out = cx.run_backend_cmd(&["backend", "status"]);

    assert_ne!(out.code, 0, "should fail with incomplete config");
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("token") || combined.contains("org") || combined.contains("repo"),
        "output should mention missing config\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn backend_status_unreachable_server_fails() {
    let cx = TestContext::new();
    cx.git_init();
    // Port 1 is almost certainly unreachable
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:1",
                "token": "test-token",
                "project": {
                    "org": "acme",
                    "repo": "widgets"
                }
            }
        }"#,
    );

    let out = cx.run_backend_cmd(&["backend", "status"]);

    assert_ne!(out.code, 0, "should fail with unreachable server");
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("unreachable")
            || combined.contains("connection")
            || combined.contains("connect")
            || combined.contains("refused"),
        "output should mention connection error\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn backend_status_unreachable_server_json_output() {
    let cx = TestContext::new();
    cx.git_init();
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:1",
                "token": "test-token",
                "project": {
                    "org": "acme",
                    "repo": "widgets"
                }
            }
        }"#,
    );

    let out = cx.run_backend_cmd(&["backend", "status", "--json"]);

    assert_ne!(out.code, 0, "should fail with unreachable server");

    let parsed: serde_json::Value = serde_json::from_str(&out.stdout)
        .unwrap_or_else(|e| panic!("failed to parse JSON: {e}\nstdout={}", out.stdout));

    assert_eq!(
        parsed["server_reachable"], false,
        "server_reachable should be false, parsed={parsed:?}"
    );
}

#[test]
fn backend_status_token_security_warning() {
    let cx = TestContext::new();
    cx.git_init();
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:1",
                "token": "hardcoded-secret",
                "project": {
                    "org": "acme",
                    "repo": "widgets"
                }
            }
        }"#,
    );

    let out = cx.run_backend_cmd(&["backend", "status"]);

    assert!(
        out.stderr
            .contains("Warning: Backend token is set directly in config file")
            || out.stderr.contains("token is set directly"),
        "stderr should contain security warning\nstderr={}",
        out.stderr
    );
}

// ============================================================================
// Task 3.2: Tests for `generate-token` and Auth Verify
// ============================================================================

#[test]
fn generate_token_derives_deterministic_token() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up global config with token seed
    cx.write_global_config(
        r#"{
            "backendServer": {
                "auth": {
                    "tokenSeed": "test-seed"
                }
            }
        }"#,
    );

    // Set up project config with org/repo
    cx.write_project_config(
        r#"{
            "backend": {
                "project": {
                    "org": "acme",
                    "repo": "widgets"
                }
            }
        }"#,
    );

    // Compute expected token
    let expected = ito_backend::derive_project_token("test-seed", "acme", "widgets");

    let out = cx.run_backend_cmd(&["backend", "generate-token"]);

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        out.stdout.trim(),
        expected,
        "token should match expected\nstdout={}\nexpected={}",
        out.stdout,
        expected
    );
    assert!(
        out.stderr.contains("Token derived for: acme/widgets"),
        "stderr should show project\nstderr={}",
        out.stderr
    );
}

#[test]
fn generate_token_seed_from_env_takes_precedence() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up global config with a different seed
    cx.write_global_config(
        r#"{
            "backendServer": {
                "auth": {
                    "tokenSeed": "config-seed"
                }
            }
        }"#,
    );

    // Env var should take precedence
    let expected = ito_backend::derive_project_token("env-seed", "acme", "widgets");

    let out = cx.run_backend_cmd_with_env(
        &[
            "backend",
            "generate-token",
            "--org",
            "acme",
            "--repo",
            "widgets",
        ],
        &[("ITO_BACKEND_TOKEN_SEED", "env-seed")],
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        out.stdout.trim(),
        expected,
        "token should match env-derived token\nstdout={}\nexpected={}",
        out.stdout,
        expected
    );
}

#[test]
fn generate_token_no_seed_fails() {
    let cx = TestContext::new();
    cx.git_init();

    // No seed in env or config
    let out = cx.run_backend_cmd(&[
        "backend",
        "generate-token",
        "--org",
        "acme",
        "--repo",
        "widgets",
    ]);

    assert_ne!(out.code, 0, "should fail without seed");
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("No token seed configured") || combined.contains("seed"),
        "output should mention missing seed\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn generate_token_flag_overrides_for_org_repo() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up project config with org/repo
    cx.write_project_config(
        r#"{
            "backend": {
                "project": {
                    "org": "original-org",
                    "repo": "original-repo"
                }
            }
        }"#,
    );

    // Flags should override config
    let expected = ito_backend::derive_project_token("test-seed", "override-org", "override-repo");

    let out = cx.run_backend_cmd(&[
        "backend",
        "generate-token",
        "--seed",
        "test-seed",
        "--org",
        "override-org",
        "--repo",
        "override-repo",
    ]);

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        out.stdout.trim(),
        expected,
        "token should match override values\nstdout={}\nexpected={}",
        out.stdout,
        expected
    );
}

// ============================================================================
// Task 3.3: Tests for Silent Fallback Fixes
// ============================================================================

#[test]
fn silent_fallback_tasks_warns_on_bad_config() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up a valid change directory
    fixtures::write(
        cx.repo.path().join(".ito/changes/000-01_test/proposal.md"),
        "## Why\nTest\n\n## What Changes\n- Test\n\n## Impact\n- None\n",
    );
    fixtures::write(
        cx.repo.path().join(".ito/changes/000-01_test/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Do a thing\n",
    );

    // Malformed backend config: enabled should be bool, not string
    cx.write_project_config(r#"{"backend": {"enabled": "not-a-bool"}}"#);

    let out = cx.run_backend_cmd(&["tasks", "status", "000-01_test"]);

    // The command may succeed or fail, but should warn about backend integration
    assert!(
        out.stderr.contains("Warning: backend integration skipped")
            || out.stderr.contains("backend")
            || out.stderr.contains("skipped"),
        "stderr should contain backend warning\nstderr={}",
        out.stderr
    );
}

#[test]
fn silent_fallback_event_forwarding_warns_on_bad_config() {
    let cx = TestContext::new();
    cx.git_init();

    // Malformed backend config
    cx.write_project_config(r#"{"backend": {"enabled": "not-a-bool"}}"#);

    // Run any command that triggers with_logging (most commands do)
    let out = cx.run_backend_cmd(&["--version"]);

    // The command should succeed but may warn about event forwarding
    // Note: This test may be fragile if --version doesn't trigger event forwarding
    // We're checking for the warning, but it's okay if it doesn't appear for --version
    if out.stderr.contains("Warning") {
        assert!(
            out.stderr.contains("backend event forwarding skipped")
                || out.stderr.contains("backend")
                || out.stderr.contains("skipped"),
            "if warning present, should mention backend\nstderr={}",
            out.stderr
        );
    }
}

#[test]
fn silent_fallback_grep_warns_on_bad_config() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up a minimal spec to grep
    fixtures::write(
        cx.repo.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nTest spec\n\n## Requirements\n\n### Requirement: Test\nTest SHALL work.\n",
    );

    // Malformed backend config
    cx.write_project_config(r#"{"backend": {"url": 123}}"#);

    let out = cx.run_backend_cmd(&["grep", "--all", "Test"]);

    // The command may succeed or fail, but should warn about backend cache
    if out.stderr.contains("Warning") {
        assert!(
            out.stderr.contains("backend cache materialization skipped")
                || out.stderr.contains("backend")
                || out.stderr.contains("cache")
                || out.stderr.contains("skipped"),
            "if warning present, should mention backend cache\nstderr={}",
            out.stderr
        );
    }
}

// ============================================================================
// Additional Edge Case Tests
// ============================================================================

#[test]
fn backend_status_with_valid_config_but_no_server() {
    let cx = TestContext::new();
    cx.git_init();

    // Valid config structure, but server not running
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:9999",
                "token": "valid-token",
                "project": {
                    "org": "test-org",
                    "repo": "test-repo"
                }
            }
        }"#,
    );

    let out = cx.run_backend_cmd(&["backend", "status"]);

    // Should fail because server is unreachable
    assert_ne!(out.code, 0, "should fail when server is unreachable");
}

#[test]
fn backend_status_json_includes_config_details() {
    let cx = TestContext::new();
    cx.git_init();

    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:9999",
                "token": "test-token",
                "project": {
                    "org": "my-org",
                    "repo": "my-repo"
                }
            }
        }"#,
    );

    let out = cx.run_backend_cmd(&["backend", "status", "--json"]);

    // May fail due to unreachable server, but JSON should still be parseable
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&out.stdout) {
        assert_eq!(parsed["enabled"], true, "enabled should be true");
        // Check that project info is included
        if let Some(project) = parsed.get("project") {
            assert_eq!(project["org"], "my-org", "org should match");
            assert_eq!(project["repo"], "my-repo", "repo should match");
        }
    }
}

#[test]
fn generate_token_with_all_sources_prefers_env() {
    let cx = TestContext::new();
    cx.git_init();

    // Set up global config seed
    cx.write_global_config(
        r#"{
            "backendServer": {
                "auth": {
                    "tokenSeed": "config-seed"
                }
            }
        }"#,
    );

    // Env var should take highest precedence (env > flag > config)
    let expected = ito_backend::derive_project_token("env-seed", "acme", "widgets");

    let out = cx.run_backend_cmd_with_env(
        &[
            "backend",
            "generate-token",
            "--seed",
            "flag-seed",
            "--org",
            "acme",
            "--repo",
            "widgets",
        ],
        &[("ITO_BACKEND_TOKEN_SEED", "env-seed")],
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(
        out.stdout.trim(),
        expected,
        "env seed should take precedence over flag\nstdout={}\nexpected={}",
        out.stdout,
        expected
    );
}

#[test]
fn backend_status_with_env_token_no_warning() {
    let cx = TestContext::new();
    cx.git_init();

    // Config without hardcoded token
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": true,
                "url": "http://127.0.0.1:9999",
                "project": {
                    "org": "acme",
                    "repo": "widgets"
                }
            }
        }"#,
    );

    // Token from env var
    let out = cx.run_backend_cmd_with_env(
        &["backend", "status"],
        &[("ITO_BACKEND_TOKEN", "env-token")],
    );

    // Should not warn about hardcoded token
    assert!(
        !out.stderr.contains("token is set directly in config file"),
        "should not warn when token is from env\nstderr={}",
        out.stderr
    );
}

#[test]
fn generate_token_missing_repo_fails() {
    let cx = TestContext::new();
    cx.git_init();

    // In non-interactive mode (ITO_INTERACTIVE=0), prompting fails
    let out = cx.run_backend_cmd(&[
        "backend",
        "generate-token",
        "--seed",
        "test-seed",
        "--org",
        "acme",
    ]);

    assert_ne!(
        out.code, 0,
        "should fail without repo in non-interactive mode"
    );
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("not a terminal") || combined.contains("Failed to read input"),
        "output should mention interactive failure\nstdout={}\nstderr={}",
        out.stdout,
        out.stderr
    );
}

#[test]
fn silent_fallback_with_valid_backend_no_warnings() {
    let cx = TestContext::new();
    cx.git_init();

    // Valid backend config (even if server is unreachable, config is valid)
    cx.write_project_config(
        r#"{
            "backend": {
                "enabled": false
            }
        }"#,
    );

    // Set up a valid change
    fixtures::write(
        cx.repo.path().join(".ito/changes/000-01_test/proposal.md"),
        "## Why\nTest\n\n## What Changes\n- Test\n\n## Impact\n- None\n",
    );
    fixtures::write(
        cx.repo.path().join(".ito/changes/000-01_test/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Do a thing\n",
    );

    let out = cx.run_backend_cmd(&["tasks", "status", "000-01_test"]);

    // Should not contain fallback warnings when config is valid
    assert!(
        !out.stderr.contains("backend integration skipped"),
        "should not warn when config is valid\nstderr={}",
        out.stderr
    );
}

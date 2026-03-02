use crate::cli::GrepArgs;
use crate::cli_error::{fail, to_cli_error, CliResult};
use crate::runtime::Runtime;
use ito_core::change_repository::FsChangeRepository;
use ito_core::grep::{grep, GrepInput, GrepScope};
use ito_core::module_repository::FsModuleRepository;

/// Handle the `ito grep` CLI command.
///
/// Resolves the grep scope from CLI arguments, optionally materialises
/// backend artifacts into the local cache, then delegates to
/// `ito_core::grep::grep` and prints results in a stable
/// `<path>:<line>:<text>` format suitable for piping.
pub(crate) fn handle_grep_clap(rt: &Runtime, args: &GrepArgs) -> CliResult<()> {
    let ito_path = rt.ito_path();
    let change_repo = FsChangeRepository::new(ito_path);
    let module_repo = FsModuleRepository::new(ito_path);

    let (scope, pattern) = parse_scope_and_pattern(args)?;

    // In backend mode, materialise artifacts before searching.
    materialize_backend_cache(rt, &scope, &change_repo)?;

    let input = GrepInput {
        pattern,
        scope,
        limit: args.limit,
    };

    let output = grep(ito_path, &input, &change_repo, &module_repo).map_err(to_cli_error)?;

    // Compute the project root once for relative-path display.
    let project_root = ito_path.parent().unwrap_or(ito_path);

    if args.json {
        let json_matches: Vec<serde_json::Value> = output
            .matches
            .iter()
            .map(|m| {
                let rel = m.path.strip_prefix(project_root).unwrap_or(&m.path);
                serde_json::json!({
                    "path": rel.display().to_string(),
                    "line_number": m.line_number,
                    "line": m.line,
                })
            })
            .collect();
        let envelope = serde_json::json!({
            "matches": json_matches,
            "truncated": output.truncated,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&envelope).map_err(to_cli_error)?
        );
    } else {
        for m in &output.matches {
            let rel = m.path.strip_prefix(project_root).unwrap_or(&m.path);
            println!("{}:{}:{}", rel.display(), m.line_number, m.line);
        }

        if output.truncated {
            eprintln!(
                "[ito grep] output limited to {} matches (use --limit 0 for unlimited)",
                args.limit
            );
        }
    }

    Ok(())
}

/// Parse the scope and pattern from the positional arguments.
///
/// With `--module` or `--all`, only one positional arg (the pattern) is expected.
/// Without flags, two positional args are expected: `<CHANGE_ID> <PATTERN>`.
fn parse_scope_and_pattern(args: &GrepArgs) -> CliResult<(GrepScope, String)> {
    if args.all {
        let pattern = single_pattern(&args.args, "--all")?;
        return Ok((GrepScope::All, pattern));
    }

    if let Some(module_id) = &args.module {
        let pattern = single_pattern(&args.args, "--module")?;
        return Ok((GrepScope::Module(module_id.clone()), pattern));
    }

    // No flags: expect <CHANGE_ID> <PATTERN>
    if args.args.len() != 2 {
        return fail("expected: ito grep <CHANGE_ID> <PATTERN>");
    }
    let target = args.args[0].clone();
    let pattern = args.args[1].clone();
    Ok((GrepScope::Change(target), pattern))
}

/// Extract a single pattern from the positional args when a scope flag is active.
fn single_pattern(positional: &[String], flag: &str) -> CliResult<String> {
    if positional.is_empty() {
        return fail(format!("expected: ito grep {flag} <PATTERN>"));
    }
    if positional.len() > 1 {
        return fail(format!(
            "too many positional arguments with {flag}; expected only <PATTERN>"
        ));
    }
    Ok(positional[0].clone())
}

/// When backend mode is enabled, pull artifacts for every change in the
/// grep scope so the local `.ito/` directory is up to date before
/// searching.
///
/// This is best-effort: if the backend is unreachable or the sync
/// endpoints are not yet implemented, the local cache is searched
/// as-is. When the backend supports conditional requests
/// (`ETag`/`If-None-Match`), unchanged artifacts are not
/// re-downloaded.
fn materialize_backend_cache(
    rt: &Runtime,
    scope: &GrepScope,
    change_repo: &FsChangeRepository<'_>,
) -> CliResult<()> {
    use ito_config::load_cascading_project_config;
    use ito_config::types::ItoConfig;
    use ito_core::backend_client::resolve_backend_runtime;
    use ito_core::backend_coordination::sync_pull;

    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let merged = load_cascading_project_config(project_root, ito_path, rt.ctx()).merged;
    let config: ItoConfig = match serde_json::from_value(merged) {
        Ok(c) => c,
        Err(e) => {
            tracing::debug!("skipping backend cache materialization: invalid config: {e}");
            return Ok(());
        }
    };

    if !config.backend.enabled {
        return Ok(());
    }

    let Ok(Some(runtime)) = resolve_backend_runtime(&config.backend) else {
        // Backend enabled but misconfigured — use local files.
        return Ok(());
    };

    // Use the stub sync client (matches the tasks command pattern).
    // The real HTTP client will be wired up when the backend adds
    // sync endpoints.
    let client = StubSyncClient;

    let change_ids: Vec<String> = match scope {
        GrepScope::Change(id) => {
            match change_repo.resolve_target(id) {
                ito_core::ChangeTargetResolution::Unique(resolved) => vec![resolved],
                _ => return Ok(()), // resolution handled later by core grep
            }
        }
        GrepScope::Module(module_id) => {
            let module_repo = FsModuleRepository::new(ito_path);
            let module = match module_repo.get(module_id) {
                Ok(m) => m,
                Err(_) => return Ok(()),
            };
            match change_repo.list_by_module(&module.id) {
                Ok(changes) => changes.into_iter().map(|c| c.id).collect(),
                Err(e) => {
                    tracing::warn!("failed to list changes for module {}: {e}", module.id);
                    vec![]
                }
            }
        }
        GrepScope::All => match change_repo.list() {
            Ok(changes) => changes.into_iter().map(|c| c.id).collect(),
            Err(e) => {
                tracing::warn!("failed to list all changes: {e}");
                vec![]
            }
        },
    };

    for change_id in &change_ids {
        // Best-effort: log and continue if one pull fails.
        if let Err(e) = sync_pull(&client, ito_path, change_id, &runtime.backup_dir) {
            tracing::debug!("backend cache pull for {change_id}: {e}");
        }
    }

    Ok(())
}

/// Stub backend sync client used until the backend adds sync endpoints.
struct StubSyncClient;

impl ito_core::BackendSyncClient for StubSyncClient {
    fn pull(&self, change_id: &str) -> Result<ito_core::ArtifactBundle, ito_core::BackendError> {
        Err(ito_core::BackendError::Other(format!(
            "Sync endpoints not yet available on backend for change '{change_id}'"
        )))
    }

    fn push(
        &self,
        change_id: &str,
        _bundle: &ito_core::ArtifactBundle,
    ) -> Result<ito_core::PushResult, ito_core::BackendError> {
        Err(ito_core::BackendError::Other(format!(
            "Sync endpoints not yet available on backend for change '{change_id}'"
        )))
    }
}

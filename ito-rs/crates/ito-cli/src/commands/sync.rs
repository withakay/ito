use crate::cli::SyncArgs;
use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::coordination_worktree::{CoordinationSyncOutcome, sync_coordination_worktree};

/// Blocking best-effort coordination sync.
///
/// Runs the sync synchronously and swallows errors (prints a warning to
/// stderr on failure). Use this when the caller needs coordination data to
/// be up-to-date before proceeding (e.g. before reading apply instructions).
pub(crate) fn best_effort_sync_coordination(rt: &Runtime, context: &str) {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    if let Err(err) = sync_coordination_worktree(project_root, ito_path, false) {
        eprintln!("Warning: failed to sync coordination state {context}: {err}");
    }
}

/// Non-blocking best-effort coordination sync.
///
/// Spawns the sync in a detached background thread so the primary CLI
/// command is not blocked by network operations (fetch/push). Use this for
/// "after write" hooks where eventual consistency is acceptable.
pub(crate) fn best_effort_sync_coordination_bg(rt: &Runtime, context: &str) {
    let ito_path = rt.ito_path().to_path_buf();
    let context = context.to_string();

    std::thread::Builder::new()
        .name("ito-sync-bg".to_string())
        .spawn(move || {
            let project_root_ref = ito_path.parent().unwrap_or(&ito_path);
            if let Err(err) = sync_coordination_worktree(project_root_ref, &ito_path, false) {
                eprintln!("Warning: failed to sync coordination state {context}: {err}");
            }
        })
        .ok();
}

/// Dispatch the top-level `ito sync` command.
pub(crate) fn handle_sync_clap(rt: &Runtime, args: &SyncArgs) -> CliResult<()> {
    let ito_path = rt.ito_path();
    let project_root = ito_path.parent().unwrap_or(ito_path);
    let outcome =
        sync_coordination_worktree(project_root, ito_path, args.force).map_err(to_cli_error)?;

    if args.json {
        let json = match outcome {
            CoordinationSyncOutcome::Embedded => serde_json::json!({
                "action": "sync",
                "mode": "embedded",
                "synced": false,
            }),
            CoordinationSyncOutcome::RateLimited => serde_json::json!({
                "action": "sync",
                "mode": "worktree",
                "synced": false,
                "rate_limited": true,
            }),
            CoordinationSyncOutcome::Synchronized => serde_json::json!({
                "action": "sync",
                "mode": "worktree",
                "synced": true,
            }),
        };
        println!(
            "{}",
            serde_json::to_string(&json).expect("JSON serialization should not fail")
        );
        return Ok(());
    }

    match outcome {
        CoordinationSyncOutcome::Embedded => {
            println!("Worktree sync is not active for the current project mode.");
        }
        CoordinationSyncOutcome::RateLimited => {
            println!("Coordination worktree already synchronized recently.");
        }
        CoordinationSyncOutcome::Synchronized => {
            println!("Coordination worktree synchronized.");
        }
    }

    Ok(())
}

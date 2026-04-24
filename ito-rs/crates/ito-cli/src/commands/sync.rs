use crate::cli::SyncArgs;
use crate::cli_error::{CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_core::coordination_worktree::{CoordinationSyncOutcome, sync_coordination_worktree};

/// Best-effort coordination sync that runs in a background thread.
///
/// Spawns the sync in a detached thread so the primary CLI command is not
/// blocked by network operations (fetch/push). If the sync fails, a warning
/// is printed to stderr. The thread is detached — the process will not wait
/// for it to complete on exit.
pub(crate) fn best_effort_sync_coordination(rt: &Runtime, context: &str) {
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
        .ok(); // Silently ignore thread spawn failures.
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

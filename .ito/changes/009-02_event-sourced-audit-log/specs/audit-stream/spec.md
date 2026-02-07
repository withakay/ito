# audit-stream (NEW)

## Purpose

Define the `ito audit stream` command that provides real-time monitoring of audit events across the current project, including events from multiple git worktrees. The stream is informative (not authoritative) and serves as a live activity feed for observability across concurrent sessions.

## Requirements

### REQ-STREAM-1: Live tail of audit events

`ito audit stream` SHALL continuously watch the audit event log file and print new events to stdout as they are appended.

- **Scenario: Single session streaming**
  - Given `ito audit stream` is running in terminal A
  - When `ito tasks complete 005-01 1.1` is run in terminal B
  - Then terminal A displays the new `TaskStatusChanged` event within 1 second
  - And the event is formatted with timestamp, operation, entity, and actor

- **Scenario: Stream starts with recent events**
  - Given an `events.jsonl` with 50 existing events
  - When `ito audit stream --last 5` is run
  - Then the 5 most recent events are printed first
  - And then new events are streamed as they arrive

- **Scenario: No existing events**
  - Given no `events.jsonl` exists
  - When `ito audit stream` is run
  - Then the command prints "Waiting for audit events..." and begins watching
  - And the file is created and monitored once the first event is written

### REQ-STREAM-2: Worktree-aware monitoring

`ito audit stream` SHALL discover all git worktrees for the current repository and monitor audit event files across all of them.

- **Scenario: Multi-worktree streaming**
  - Given the project has worktrees at `/project/main`, `/project/wt-feature-a`, `/project/wt-feature-b`
  - And each has its own `.ito/.state/audit/events.jsonl`
  - When `ito audit stream` is run from any worktree
  - Then events from all three worktrees are interleaved in chronological order
  - And each event is prefixed with the worktree name (e.g., `[main]`, `[wt-feature-a]`)

- **Scenario: New worktree appears during streaming**
  - Given `ito audit stream` is running
  - When a new worktree is created (e.g., via `git worktree add`)
  - Then the stream discovers and begins monitoring the new worktree's event file within the next poll cycle

- **Scenario: Worktree removed during streaming**
  - Given `ito audit stream` is monitoring worktree `wt-old`
  - When `wt-old` is removed
  - Then the stream stops monitoring that path without crashing
  - And prints an informational message: "[wt-old] worktree removed, no longer monitoring"

### REQ-STREAM-3: Worktree discovery via git

Worktree discovery SHALL use `git worktree list --porcelain` to enumerate all worktrees, then check each for an `.ito/.state/audit/events.jsonl` path.

- **Scenario: Discovery finds all worktrees**
  - Given `git worktree list --porcelain` returns 3 worktrees
  - When the stream initializes
  - Then all 3 paths are checked for `.ito/.state/audit/events.jsonl`
  - And only those with existing `.ito/` directories are monitored

- **Scenario: Periodic re-discovery**
  - The stream SHALL re-run worktree discovery every 30 seconds (configurable) to detect new or removed worktrees

### REQ-STREAM-4: Output formatting

Events SHALL be formatted for human readability by default, with a `--json` flag for structured output.

- **Scenario: Human-readable output**
  - Given a `TaskStatusChanged` event
  - Then it is displayed as: `[main] 14:32:05 task.status_changed  005-01/1.1  pending -> in-progress  (cli)`

- **Scenario: JSON output**
  - Given `ito audit stream --json` is running
  - Then each event is printed as a single JSON line with an additional `worktree` field

- **Scenario: Filter by change**
  - Given `ito audit stream --change 005-01` is running
  - Then only events with `change_id == "005-01"` are displayed
  - And events from all worktrees matching that change are included

### REQ-STREAM-5: Stream is informational only

The stream output SHALL NOT be treated as a source of truth. Events from discarded branches (never merged) may appear in the stream. The authoritative audit trail is the `events.jsonl` in the git history of the merged branch.

- **Scenario: Discarded branch events**
  - Given worktree `wt-experiment` emits 10 events
  - And the branch is later discarded (never merged)
  - Then those 10 events appeared in the stream during the session
  - But they do not exist in the merged `events.jsonl` on the main branch
  - And this is expected and acceptable behavior

### REQ-STREAM-6: File watching mechanism

The stream SHALL use filesystem polling (not inotify/kqueue) for maximum portability across macOS and Linux. Poll interval SHALL default to 500ms.

- **Scenario: Poll-based watching**
  - Given the stream is watching `events.jsonl`
  - When a new line is appended
  - Then the stream detects the change within 500ms (one poll cycle)

- **Scenario: Configurable poll interval**
  - Given `ito audit stream --poll-interval 200` is run
  - Then the file is polled every 200ms instead of the default 500ms

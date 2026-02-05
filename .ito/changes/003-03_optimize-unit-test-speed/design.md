# Design: Optimize Unit Test Speed

## Root Cause Analysis

The test suite hangs in `ito-harness` opencode tests. The root cause is the `monitor_timeout` function in `opencode.rs`:

```rust
fn monitor_timeout(...) {
    loop {
        thread::sleep(check_interval);  // 1 second
        // Check elapsed time...
        // NO EXIT CONDITION when process has already terminated!
    }
}
```

The monitor thread loops forever checking `last_activity`, even after the child process has exited. The streaming threads (`stdout_handle`, `stderr_handle`) finish when pipes close, but the monitor thread has no signal to stop.

## Solution

### Option 1: Use AtomicBool for process completion (Recommended)

Add a `process_done` flag that the main thread sets after `child.wait()` returns:

```rust
let process_done = Arc::new(AtomicBool::new(false));
let process_done_monitor = Arc::clone(&process_done);

let monitor_handle = thread::spawn(move || {
    monitor_timeout(..., &process_done_monitor)
});

// Wait for process
let status = child.wait()?;
process_done.store(true, Ordering::SeqCst);

// Now monitor thread will exit on next check
let _ = monitor_handle.join();
```

Update `monitor_timeout`:

```rust
fn monitor_timeout(..., process_done: &AtomicBool) {
    loop {
        thread::sleep(check_interval);

        // Exit if process is done
        if process_done.load(Ordering::SeqCst) {
            return;
        }

        // Check timeout...
    }
}
```

### Option 2: Use channel with timeout

Instead of `thread::sleep`, use a channel with `recv_timeout` that can receive a "stop" signal.

### Option 3: Use condvar

Use a condition variable to wake the monitor thread when the process exits.

## Recommendation

**Option 1** is simplest and sufficient. The 1-second polling delay is acceptable for cleanup.

## Implementation

1. Add `process_done: Arc<AtomicBool>` parameter to `monitor_timeout`
2. Check `process_done` at start of each loop iteration
3. Set `process_done = true` after `child.wait()` returns
4. Join monitor thread after setting the flag

## Testing

- Existing opencode tests should pass without hanging
- Add a test that verifies quick process exit doesn't hang
- Full test suite should complete in < 60 seconds

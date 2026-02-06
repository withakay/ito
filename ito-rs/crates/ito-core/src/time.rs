//! Clock helpers for the application layer.
//!
//! These live in `ito-core` rather than `ito-domain` because they depend on the
//! system clock (`Local::now()`), which is non-deterministic I/O.

use chrono::Local;

/// Current local time formatted as `HH:MM:SS`.
pub fn now_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

/// Current local date formatted as `YYYY-MM-DD`.
pub fn now_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

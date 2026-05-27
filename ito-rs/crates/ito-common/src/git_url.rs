//! Git remote URL parsing utilities.
//!
//! Pure parsing logic with no domain knowledge and no I/O. Handles the common
//! git remote URL formats used by GitHub, GitLab, and similar hosts.

/// Parse an `<org>/<repo>` pair from a git remote URL.
///
/// Handles the four common formats:
///
/// | Format | Example |
/// |---|---|
/// | SCP-style SSH | `git@github.com:withakay/ito.git` |
/// | HTTPS with `.git` | `https://github.com/withakay/ito.git` |
/// | HTTPS without `.git` | `https://github.com/withakay/ito` |
/// | SSH with explicit port | `ssh://git@github.com:22/withakay/ito.git` |
///
/// Returns `Some((org, repo))` when the URL contains a recognisable two-component
/// path, or `None` when the URL is empty, malformed, or has fewer than two path
/// components.
///
/// The `.git` suffix is stripped from the repository name when present.
///
/// # Examples
///
/// ```
/// use ito_common::git_url::parse_remote_url_org_repo;
///
/// assert_eq!(
///     parse_remote_url_org_repo("git@github.com:withakay/ito.git"),
///     Some(("withakay".to_string(), "ito".to_string())),
/// );
/// assert_eq!(
///     parse_remote_url_org_repo("https://github.com/withakay/ito.git"),
///     Some(("withakay".to_string(), "ito".to_string())),
/// );
/// assert_eq!(
///     parse_remote_url_org_repo("https://github.com/withakay/ito"),
///     Some(("withakay".to_string(), "ito".to_string())),
/// );
/// assert_eq!(
///     parse_remote_url_org_repo("ssh://git@github.com:22/withakay/ito.git"),
///     Some(("withakay".to_string(), "ito".to_string())),
/// );
/// assert_eq!(parse_remote_url_org_repo(""), None);
/// ```
pub fn parse_remote_url_org_repo(url: &str) -> Option<(String, String)> {
    let url = url.trim();
    if url.is_empty() {
        return None;
    }

    // Extract the path portion depending on URL format.
    let path = if let Some(rest) = url
        .strip_prefix("ssh://")
        .or_else(|| url.strip_prefix("git://"))
    {
        // ssh://[user@]host[:port]/path  or  git://host/path
        // Drop everything up to and including the first '/' after the authority.
        rest.split_once('/')?.1
    } else if url.contains("://") {
        // HTTPS (or any other scheme): https://host/path
        // Strip scheme + authority, keep the path.
        let after_scheme = url.split_once("://")?.1;
        after_scheme.split_once('/')?.1
    } else if let Some(colon_pos) = url.find(':') {
        // SCP-style SSH: git@github.com:org/repo.git
        // The colon separates host from path; there must be no '/' before the colon.
        let before_colon = &url[..colon_pos];
        if before_colon.contains('/') {
            // Looks like a Windows absolute path or something unexpected — bail.
            return None;
        }
        &url[colon_pos + 1..]
    } else {
        return None;
    };

    extract_org_repo_from_path(path)
}

/// Extract `(org, repo)` from the last two path components.
///
/// Strips a leading `/`, splits on `/`, and takes the last two non-empty
/// segments. The `.git` suffix is removed from the repo component.
fn extract_org_repo_from_path(path: &str) -> Option<(String, String)> {
    let path = path.trim_start_matches('/');
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if segments.len() < 2 {
        return None;
    }

    let org = segments[segments.len() - 2];
    let repo_raw = segments[segments.len() - 1];
    let repo = repo_raw.strip_suffix(".git").unwrap_or(repo_raw);

    if org.is_empty() || repo.is_empty() {
        return None;
    }

    Some((org.to_string(), repo.to_string()))
}

#[cfg(test)]
#[path = "git_url_tests.rs"]
mod git_url_tests;

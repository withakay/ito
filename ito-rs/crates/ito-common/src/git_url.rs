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
        rest.splitn(2, '/').nth(1)?
    } else if url.contains("://") {
        // HTTPS (or any other scheme): https://host/path
        // Strip scheme + authority, keep the path.
        let after_scheme = url.splitn(2, "://").nth(1)?;
        after_scheme.splitn(2, '/').nth(1)?
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
mod tests {
    use super::*;

    // ── Happy-path: all four canonical formats ────────────────────────────────

    #[test]
    fn parses_scp_ssh_url() {
        let result = parse_remote_url_org_repo("git@github.com:withakay/ito.git");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    #[test]
    fn parses_https_url_with_git_suffix() {
        let result = parse_remote_url_org_repo("https://github.com/withakay/ito.git");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    #[test]
    fn parses_https_url_without_git_suffix() {
        let result = parse_remote_url_org_repo("https://github.com/withakay/ito");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    #[test]
    fn parses_ssh_with_explicit_port() {
        let result = parse_remote_url_org_repo("ssh://git@github.com:22/withakay/ito.git");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    // ── Variations ────────────────────────────────────────────────────────────

    #[test]
    fn parses_gitlab_style_subgroup_takes_last_two_segments() {
        // GitLab allows nested groups; we take the last two path components.
        let result = parse_remote_url_org_repo("https://gitlab.com/group/subgroup/repo.git");
        assert_eq!(result, Some(("subgroup".to_string(), "repo".to_string())));
    }

    #[test]
    fn parses_http_scheme() {
        let result = parse_remote_url_org_repo("http://github.com/acme/widget.git");
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    #[test]
    fn parses_git_protocol_url() {
        let result = parse_remote_url_org_repo("git://github.com/acme/widget.git");
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    #[test]
    fn strips_git_suffix_only_once() {
        // repo name that ends with ".git.git" — only the trailing ".git" is stripped.
        let result = parse_remote_url_org_repo("https://github.com/org/repo.git.git");
        assert_eq!(result, Some(("org".to_string(), "repo.git".to_string())));
    }

    #[test]
    fn handles_trailing_slash_in_https_url() {
        let result = parse_remote_url_org_repo("https://github.com/withakay/ito/");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    #[test]
    fn handles_ssh_url_without_user() {
        let result = parse_remote_url_org_repo("ssh://github.com/withakay/ito.git");
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    // ── Edge cases / error paths ──────────────────────────────────────────────

    #[test]
    fn returns_none_for_empty_string() {
        assert_eq!(parse_remote_url_org_repo(""), None);
    }

    #[test]
    fn returns_none_for_whitespace_only() {
        assert_eq!(parse_remote_url_org_repo("   "), None);
    }

    #[test]
    fn returns_none_for_single_path_component() {
        assert_eq!(
            parse_remote_url_org_repo("https://github.com/onlyone"),
            None
        );
    }

    #[test]
    fn returns_none_for_no_path_after_host() {
        assert_eq!(parse_remote_url_org_repo("https://github.com"), None);
        assert_eq!(parse_remote_url_org_repo("https://github.com/"), None);
    }

    #[test]
    fn returns_none_for_scp_url_with_single_component() {
        assert_eq!(
            parse_remote_url_org_repo("git@github.com:onlyone.git"),
            None
        );
    }

    #[test]
    fn returns_none_for_bare_string_without_separator() {
        assert_eq!(parse_remote_url_org_repo("notaurl"), None);
    }
}

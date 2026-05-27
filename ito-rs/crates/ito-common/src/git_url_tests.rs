use super::*;

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

#[test]
fn parses_gitlab_style_subgroup_takes_last_two_segments() {
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

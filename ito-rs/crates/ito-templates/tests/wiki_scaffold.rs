use std::collections::HashSet;

use ito_templates::default_project_files;

#[test]
fn default_project_embeds_ito_wiki_scaffold() {
    let files: HashSet<_> = default_project_files()
        .into_iter()
        .map(|file| file.relative_path)
        .collect();

    for expected in [
        ".ito/wiki/index.md",
        ".ito/wiki/log.md",
        ".ito/wiki/overview.md",
        ".ito/wiki/_meta/config.yaml",
        ".ito/wiki/_meta/schema.md",
        ".ito/wiki/_meta/status.md",
    ] {
        assert!(
            files.contains(expected),
            "missing wiki scaffold file: {expected}"
        );
    }
}

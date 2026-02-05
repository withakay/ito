use std::fs;
use std::path::{Path, PathBuf};

fn is_markdown(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "md")
}

fn is_prompt_markdown(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".prompt.md"))
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read_dir should succeed") {
        let entry = entry.expect("dir entry should succeed");
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
            continue;
        }

        if is_markdown(&path) || is_prompt_markdown(&path) {
            out.push(path);
        }
    }
}

fn has_unterminated_frontmatter(contents: &str) -> bool {
    let mut lines = contents.lines();
    let Some(first) = lines.next() else {
        return false;
    };

    if first.trim() != "---" {
        return false;
    }

    lines.all(|line| line.trim() != "---")
}

fn has_trailing_whitespace(contents: &str) -> bool {
    contents
        .lines()
        .any(|line| line.ends_with(' ') || line.ends_with('\t'))
}

fn has_unbalanced_code_fences(contents: &str) -> bool {
    let fences = contents
        .lines()
        .filter(|line| line.starts_with("```"))
        .count();
    fences % 2 == 1
}

#[test]
fn template_markdown_is_well_formed() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/default/project");
    assert!(root.exists(), "template root missing: {root:?}");

    let mut files = Vec::new();
    walk(&root, &mut files);

    assert!(!files.is_empty(), "no markdown files found under {root:?}");

    let mut bad = Vec::new();
    for path in files {
        let bytes = fs::read(&path).expect("read should succeed");
        let contents = String::from_utf8_lossy(&bytes);

        if has_trailing_whitespace(&contents) {
            bad.push(format!("trailing whitespace: {}", path.display()));
        }

        if has_unterminated_frontmatter(&contents) {
            bad.push(format!("unterminated frontmatter: {}", path.display()));
        }

        if has_unbalanced_code_fences(&contents) {
            bad.push(format!("unbalanced code fences: {}", path.display()));
        }
    }

    if !bad.is_empty() {
        panic!("Invalid markdown templates:\n{}", bad.join("\n"));
    }
}

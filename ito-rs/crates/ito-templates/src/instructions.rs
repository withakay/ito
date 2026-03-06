//! Embedded instruction template loading and rendering.

use include_dir::{Dir, include_dir};
use minijinja::{Environment, UndefinedBehavior};
use serde::Serialize;

static INSTRUCTIONS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/instructions");

/// List all embedded instruction template paths.
pub fn list_instruction_templates() -> Vec<&'static str> {
    let mut out = Vec::new();
    collect_paths(&INSTRUCTIONS_DIR, &mut out);
    out.sort_unstable();
    out
}

/// Fetch an embedded instruction template as raw bytes.
pub fn get_instruction_template_bytes(path: &str) -> Option<&'static [u8]> {
    INSTRUCTIONS_DIR.get_file(path).map(|f| f.contents())
}

/// Fetch an embedded instruction template as UTF-8 text.
pub fn get_instruction_template(path: &str) -> Option<&'static str> {
    let bytes = get_instruction_template_bytes(path)?;
    std::str::from_utf8(bytes).ok()
}

/// Render an instruction template by path using a serializable context.
pub fn render_instruction_template<T: Serialize>(
    path: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let template = get_instruction_template(path).ok_or_else(|| {
        minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, path.to_string())
    })?;
    render_instruction_template_str(template, ctx)
}

fn render_instruction_template_str<T: Serialize>(
    template: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let mut env = template_env();

    // Instruction templates are markdown snippets shown directly in the CLI.
    // Trim block-only lines so conditional sections do not leave extra blank lines.
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);

    env.add_template("_inline", template)?;
    env.get_template("_inline")?.render(ctx)
}

/// Render an arbitrary template string using a serializable context.
pub fn render_template_str<T: Serialize>(
    template: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let mut env = template_env();

    env.add_template("_inline", template)?;
    env.get_template("_inline")?.render(ctx)
}

fn template_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Render text files as-is and preserve their trailing newline.
    env.set_auto_escape_callback(|_name| minijinja::AutoEscape::None);
    env.set_keep_trailing_newline(true);

    env
}

fn collect_paths(dir: &'static Dir<'static>, out: &mut Vec<&'static str>) {
    for f in dir.files() {
        if let Some(p) = f.path().to_str() {
            out.push(p);
        }
    }
    for d in dir.dirs() {
        collect_paths(d, out);
    }
}

#[cfg(test)]
#[path = "instructions_tests.rs"]
mod tests;

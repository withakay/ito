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
    render_template_str(template, ctx)
}

/// Render an arbitrary template string using a serializable context.
pub fn render_template_str<T: Serialize>(
    template: &str,
    ctx: &T,
) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Templates are markdown; we don't want any escaping.
    env.set_auto_escape_callback(|_name| minijinja::AutoEscape::None);

    env.add_template("_inline", template)?;
    env.get_template("_inline")?.render(ctx)
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
mod tests {
    use super::*;

    #[test]
    fn render_template_str_renders_from_serialize_ctx() {
        #[derive(Serialize)]
        struct Ctx {
            name: &'static str,
        }

        let out = render_template_str("hello {{ name }}", &Ctx { name: "world" }).unwrap();
        assert_eq!(out, "hello world");
    }

    #[test]
    fn render_template_str_is_strict_on_undefined() {
        #[derive(Serialize)]
        struct Ctx {}

        let err = render_template_str("hello {{ missing }}", &Ctx {}).unwrap_err();
        assert_eq!(err.kind(), minijinja::ErrorKind::UndefinedError);
    }
}

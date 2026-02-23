import pathlib
import re


def main() -> int:
    """
    Prepare a check configuration by copying `zensical.toml` to `.local/zensical.check.toml` with `site_dir` changed to "site-check".

    Reads `zensical.toml`, replaces exactly one line setting `site_dir = "site"` with `site_dir = "site-check"`, ensures the replacement count is exactly one, creates the `.local` directory if needed, and writes the modified content to `.local/zensical.check.toml`. Prints the path written.

    Returns:
        int: Exit code 0 on success.


    Raises:
        SystemExit: If the number of `site_dir` replacements is not exactly one.
    """
    local_dir = pathlib.Path(".local")
    local_dir.mkdir(parents=True, exist_ok=True)

    src_path = pathlib.Path("zensical.toml")
    text = src_path.read_text(encoding="utf-8")

    pattern = r'(?m)^site_dir\s*=\s*"site"\s*$'
    replacement = 'site_dir = "site-check"'
    text, count = re.subn(pattern, replacement, text, count=1)
    if count != 1:
        raise SystemExit(
            f"Expected exactly one site_dir assignment in {src_path}; found {count}"
        )

    out_path = local_dir / "zensical.check.toml"
    out_path.write_text(text, encoding="utf-8")
    print(f"Wrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

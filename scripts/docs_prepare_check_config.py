import pathlib


def main() -> int:
    local_dir = pathlib.Path(".local")
    local_dir.mkdir(parents=True, exist_ok=True)

    src_path = pathlib.Path("zensical.toml")
    text = src_path.read_text(encoding="utf-8")

    needle = 'site_dir = "site"'
    replacement = 'site_dir = "site-check"'
    if needle not in text:
        raise SystemExit(f"Expected {needle!r} in {src_path}")

    out_path = local_dir / "zensical.check.toml"
    out_path.write_text(text.replace(needle, replacement), encoding="utf-8")
    print(f"Wrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

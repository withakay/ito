import pathlib
import shutil
import importlib


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]

    zensical = importlib.import_module("zensical")
    zensical_file = getattr(zensical, "__file__", None)
    if not zensical_file:
        raise SystemExit("zensical module has no __file__")

    src = pathlib.Path(zensical_file).resolve().parent / "templates" / "assets"
    dst = repo_root / "docs" / "assets"

    if not src.exists():
        raise SystemExit(f"zensical assets dir not found: {src}")

    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(src, dst)
    print(f"Copied zensical assets: {src} -> {dst}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

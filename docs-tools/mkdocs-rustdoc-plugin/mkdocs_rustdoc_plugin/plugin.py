# pyright: reportMissingImports=false

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path
from urllib.parse import urlparse

from mkdocs.config import config_options
from mkdocs.exceptions import PluginError
from mkdocs.plugins import BasePlugin


class RustdocPlugin(BasePlugin):
    config_scheme = (
        ("workspace_root", config_options.Type(str, default=".")),
        ("crate_dir", config_options.Type(str, default="ito-rs")),
        (
            "cargo_doc_args",
            config_options.Type(list, default=["--workspace", "--no-deps"]),
        ),
        ("markdown_output", config_options.Type(str, default="docs/api/rustdoc.md")),
        ("site_subdir", config_options.Type(str, default="rustdoc")),
    )

    def on_pre_build(self, config):
        """
        Prepare Rust documentation before the MkDocs build by running `cargo doc`, generating a Markdown index of crates, and recording the rustdoc output directory.

        Parameters:
            config (mkdocs.config.defaults.Theme): MkDocs build configuration object; used to derive project paths and site URL.

        Raises:
            PluginError: If the configured crate directory does not exist.
            PluginError: If the `cargo` executable is not found.
            PluginError: If `cargo doc` exits with a non-zero status.
            PluginError: If the expected rustdoc output directory is missing.
        """
        project_root = Path(config.config_file_path).resolve().parent
        workspace_root = (project_root / self.config["workspace_root"]).resolve()
        crate_dir = (project_root / self.config["crate_dir"]).resolve()
        cargo_doc_args = list(self.config["cargo_doc_args"])

        if not crate_dir.exists():
            raise PluginError(f"crate_dir does not exist: {crate_dir}")

        cmd = ["cargo", "doc", *cargo_doc_args]
        try:
            subprocess.run(cmd, cwd=crate_dir, check=True)
        except FileNotFoundError as error:
            raise PluginError("cargo is required for rustdoc generation") from error
        except subprocess.CalledProcessError as error:
            raise PluginError(
                f"cargo doc failed with exit code {error.returncode}"
            ) from error

        rustdoc_dir = workspace_root / "target" / "doc"
        if not rustdoc_dir.exists():
            raise PluginError(f"rustdoc output missing: {rustdoc_dir}")

        crate_links = self._collect_crate_links(rustdoc_dir)
        site_subdir = self.config["site_subdir"].strip("/")
        site_path_prefix = self._site_path_prefix(config.get("site_url", ""))
        markdown_path = (project_root / self.config["markdown_output"]).resolve()
        markdown_path.parent.mkdir(parents=True, exist_ok=True)
        markdown_path.write_text(
            self._render_markdown(crate_links, site_subdir, site_path_prefix),
            encoding="utf-8",
        )

        self._rustdoc_dir = rustdoc_dir

    def on_post_build(self, config):
        """
        Copy the previously generated rustdoc output into the built site under the configured site subdirectory.

        Parameters:
            config (dict): The MkDocs build configuration mapping (contains "site_dir").

        Raises:
            PluginError: If rustdoc output was not initialized by the pre-build step.

        Description:
            Removes any existing destination directory at <site_dir>/<site_subdir> then copies the rustdoc directory stored on the plugin instance into that destination, preserving the rustdoc output tree.
        """
        rustdoc_dir: Path | None = getattr(self, "_rustdoc_dir", None)
        if rustdoc_dir is None:
            raise PluginError(
                "rustdoc directory not initialized; pre-build hook did not run"
            )

        site_dir = Path(config["site_dir"]).resolve()
        site_subdir = self.config["site_subdir"].strip("/")
        destination = site_dir / site_subdir

        if destination.exists():
            shutil.rmtree(destination)
        shutil.copytree(rustdoc_dir, destination)

    def _collect_crate_links(self, rustdoc_dir: Path) -> list[str]:
        """
        Collects crate directory names from a rustdoc output directory.

        Scans the immediate subdirectories of `rustdoc_dir` and returns the names of those
        that contain an `index.html` file, sorted alphabetically.

        Parameters:
            rustdoc_dir (Path): Path to the rustdoc output directory (typically `target/doc`).

        Returns:
            list[str]: Alphabetically sorted crate directory names that contain an `index.html`.
        """
        crate_names: list[str] = []
        for entry in rustdoc_dir.iterdir():
            if not entry.is_dir():
                continue
            if entry.name.startswith("."):
                continue
            index_file = entry / "index.html"
            if index_file.exists():
                crate_names.append(entry.name)

        crate_names.sort()
        return crate_names

    def _render_markdown(
        self,
        crate_names: list[str],
        site_subdir: str,
        site_path_prefix: str,
    ) -> str:
        """
        Builds a Markdown index page that lists available Rust crate documentation links.

        Parameters:
            crate_names (list[str]): Sorted crate names whose rustdoc index pages are present.
            site_subdir (str): Subdirectory under the site where rustdoc content will be served.
            site_path_prefix (str): Optional site URL path prefix to prepend to generated links (may be empty).

        Returns:
            str: A Markdown document containing a header, an optional "Crates" section with links to each crate's index.html, or a message indicating no crate indexes were found.
        """
        lines = [
            "# Rust API Reference",
            "",
            "This page is generated by `mkdocs-rustdoc-plugin` from Rust docstrings.",
            "",
        ]

        if not crate_names:
            lines.extend(["No crate rustdoc indexes were found.", ""])
            return "\n".join(lines)

        lines.append("## Crates")
        lines.append("")
        for crate in crate_names:
            link = self._build_site_link(site_path_prefix, site_subdir, crate)
            lines.append(f"- [{crate}]({link})")

        lines.append("")
        return "\n".join(lines)

    def _site_path_prefix(self, site_url: str) -> str:
        """
        Extract the path component from a site URL for use in constructing site-relative links.

        Parameters:
            site_url (str): The full site URL (from MkDocs config); may be empty.

        Returns:
            str: The URL path component with leading and trailing slashes removed (e.g., "sub/path"), or an empty string if no path is present or `site_url` is empty.
        """
        if not site_url:
            return ""
        parsed = urlparse(site_url)
        return parsed.path.strip("/")

    def _build_site_link(
        self, site_path_prefix: str, site_subdir: str, crate: str
    ) -> str:
        """
        Constructs a site-relative URL to a crate's documentation index.

        Parameters:
            site_path_prefix (str): Optional path component derived from the site URL (no leading/trailing slashes).
            site_subdir (str): Subdirectory under the site where rustdoc content is served; may be empty.
            crate (str): Crate directory name.

        Returns:
            str: A path starting with '/' that joins the non-empty parts (site_path_prefix, site_subdir, crate, "index.html") using '/'.
        """
        parts = [
            part
            for part in [site_path_prefix, site_subdir, crate, "index.html"]
            if part
        ]
        return "/" + "/".join(parts)

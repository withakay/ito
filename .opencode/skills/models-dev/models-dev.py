#!/usr/bin/env python3
"""Fetch and cache model identifiers from models.dev.

This is intentionally lightweight (stdlib only) so it works in most
agent harnesses. It caches the raw provider pages and emits a compact
JSON with extracted model IDs.

Usage:
  python .opencode/skills/models-dev/models-dev.py [--refresh] [--out <path>]

Cache:
  .opencode/skills/models-dev/temp/models.dev.json
"""

from __future__ import annotations

import argparse
import json
import os
import re
import time
import urllib.request
from typing import Any, Dict, List, Optional

DEFAULT_OUT = ".opencode/skills/models-dev/temp/models.dev.json"
DEFAULT_TTL_SECONDS = 60 * 60 * 24  # 24h

API_URL = "https://models.dev/api.json"

# The provider ids we care about for the plan. These are keys in api.json.
TARGET_PROVIDERS = [
    "github-copilot",
    "zai",
    "zhipuai",
    "zai-coding-plan",
    "zhipuai-coding-plan",
    "opencode",
]


def _read_json(path: str) -> Optional[Dict[str, Any]]:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    except FileNotFoundError:
        return None


def _write_json(path: str, payload: Dict[str, Any]) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(payload, f, indent=2, sort_keys=True)
        f.write("\n")
    os.replace(tmp, path)


def _fetch(url: str) -> str:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "models-dev-skill/1.0 (stdlib urllib)",
            "Accept": "text/html,application/json;q=0.9,*/*;q=0.8",
        },
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        raw = resp.read()
    # Best-effort decode
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError:
        return raw.decode("utf-8", errors="replace")


def _extract_model_ids_from_api(provider_payload: Any) -> List[str]:
    # api.json provider payloads are expected to contain a list of models (format may evolve).
    # We keep this tolerant and only extract plausible model id strings.
    ids: List[str] = []

    if not isinstance(provider_payload, dict):
        return ids

    models = provider_payload.get("models")

    # models.dev/api.json uses `models` as either:
    # - list[model]
    # - dict[modelId -> modelPayload]
    if isinstance(models, list):
        iterable = models
    elif isinstance(models, dict):
        iterable = list(models.values())
    else:
        iterable = []

    for m in iterable:
        if not isinstance(m, dict):
            continue

        # Most reliable is the dict key in the dict form; but in list form we'd expect `id`.
        # Pull a few known fields and accept the first plausible model id.
        for key in ("id", "name", "model"):
            v = m.get(key)
            if isinstance(v, str) and v:
                ids.append(v)
                break

    # De-dupe while preserving order
    seen = set()
    out: List[str] = []
    for x in ids:
        if x in seen:
            continue
        seen.add(x)
        out.append(x)
    return out


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--refresh", action="store_true", help="Ignore cache and refetch")
    p.add_argument("--out", default=DEFAULT_OUT, help="Output JSON path")
    p.add_argument(
        "--ttl", type=int, default=DEFAULT_TTL_SECONDS, help="Cache TTL seconds"
    )
    args = p.parse_args()

    existing = _read_json(args.out)
    now = int(time.time())

    if not args.refresh and existing:
        fetched_at = int(existing.get("fetchedAt", 0))
        if fetched_at and now - fetched_at < args.ttl:
            print(json.dumps(existing, indent=2, sort_keys=True))
            return 0

    providers_out: Dict[str, Any] = {}

    api_payload = json.loads(_fetch(API_URL))

    all_ids: List[str] = []
    for provider_id in TARGET_PROVIDERS:
        provider_payload = api_payload.get(provider_id)
        ids = _extract_model_ids_from_api(provider_payload)
        providers_out[provider_id] = {
            "modelIds": ids,
        }
        all_ids.extend(ids)

    # De-dupe global list
    seen = set()
    unique_all: List[str] = []
    for x in all_ids:
        if x in seen:
            continue
        seen.add(x)
        unique_all.append(x)

    payload = {
        "source": "models.dev",
        "fetchedAt": now,
        "providers": providers_out,
        "allModelIds": unique_all,
    }

    _write_json(args.out, payload)
    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

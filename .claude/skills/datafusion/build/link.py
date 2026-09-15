"""Derive symbol-to-example edges structurally, by running ast-grep over the corpus.

The naive way to cross-reference a symbol against an example corpus is to search for its name.
That conflates three different things: a type actually being implemented, a type being mentioned
in a comment, and an unrelated identifier that happens to share a word.

ast-grep separates them, because it matches syntax rather than text. `impl TableProvider for X`
is an implementation; `TableProvider` inside a `//` comment is not a node this rule can match at
all. So the edges produced here are evidence, not coincidence.

The division of labour matters: ast-grep emits one record per match and cannot aggregate across
matches, so grouping and deduplication happen here, over its `--json=stream` output.
"""

from __future__ import annotations

import json
import shutil
import subprocess
from collections import defaultdict
from pathlib import Path


class AstGrepMissing(RuntimeError):
    """The ast-grep binary is required for the structural stages."""


def available() -> bool:
    return shutil.which("ast-grep") is not None


def _run(args: list[str]) -> list[dict]:
    done = subprocess.run(args, capture_output=True, text=True, check=False)
    if done.returncode not in (0, 1):
        raise AstGrepMissing(f"ast-grep failed: {done.stderr.strip()[:300]}")
    records: list[dict] = []
    for line in done.stdout.splitlines():
        line = line.strip()
        if line:
            records.append(json.loads(line))
    return records


def trait_implementations(corpus: Path, content: Path) -> dict[str, list[str]]:
    """Map a trait name to the corpus files that implement it.

    Uses a pattern here rather than a rule because `impl $TRAIT for $TYPE` is a single unambiguous
    shape and the metavariable gives the trait name directly. Where a pattern would be
    formatting-sensitive -- anything matching a method declaration -- the shipped rules use
    `kind`-anchored matching instead.
    """
    if not available():
        raise AstGrepMissing("ast-grep is not on PATH")

    records = _run(
        [
            "ast-grep",
            "run",
            "--lang",
            "rust",
            "--pattern",
            "impl $TRAIT for $TYPE { $$$ }",
            "--json=stream",
            str(corpus),
        ]
    )

    edges: dict[str, set[str]] = defaultdict(set)
    for record in records:
        single = (record.get("metaVariables") or {}).get("single") or {}
        trait_text = (single.get("TRAIT") or {}).get("text", "")
        if not trait_text:
            continue
        # Strip generic arguments: `ScalarUDFImpl` and `ScalarUDFImpl<T>` are the same edge.
        name = trait_text.split("<", 1)[0].strip()
        try:
            relative = Path(record["file"]).resolve().relative_to(content.resolve()).as_posix()
        except ValueError:
            relative = record["file"]
        edges[name].add(relative)

    return {name: sorted(files) for name, files in sorted(edges.items())}


def registrations(corpus: Path) -> dict[str, int]:
    """Count the `register_*` / `add_*` calls the corpus demonstrates, by method name."""
    if not available():
        raise AstGrepMissing("ast-grep is not on PATH")

    records = _run(
        [
            "ast-grep",
            "run",
            "--lang",
            "rust",
            "--pattern",
            "$RECEIVER.$METHOD($$$)",
            "--json=stream",
            str(corpus),
        ]
    )

    counts: dict[str, int] = defaultdict(int)
    for record in records:
        single = (record.get("metaVariables") or {}).get("single") or {}
        method = (single.get("METHOD") or {}).get("text", "")
        if method.startswith(("register_", "add_", "enable_")):
            counts[method] += 1
    return dict(sorted(counts.items(), key=lambda pair: (-pair[1], pair[0])))

"""Emit one page per capability axis, mapping a capability to its entry points.

These pages are mostly generated. A topic definition names seeds -- types, extension points,
config prefixes, example directories -- and everything else is joined out of the model, so the
mapping stays exact across a rebuild and cannot drift from the API it describes.

Seeds are resolved by leaf name and an unresolved seed **fails the build**. A topic page that
quietly drops a type it can no longer find would be worse than no page: it would assert, by
omission, that a capability does not exist.

The curated sections are deliberately thin and factual. What an agent needs from a topic page is
the map -- which types, which traits, which knobs, which examples -- not an opinion about
architecture.
"""

from __future__ import annotations

import json
import re
from collections import defaultdict
from pathlib import Path

from emit import api_file, model_file
from model import Item

CONFIG_ROW = re.compile(r"^\|\s*(datafusion\.[a-z0-9_.]+)\s*\|(.*?)\|(.*)\|\s*$")


class UnresolvedSeed(RuntimeError):
    """A topic names something that is not in the pinned model."""


def _by_leaf(items: dict[str, Item]) -> dict[str, list[Item]]:
    index: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        index[item.name].append(item)
    return index


def _resolve(seed: str, by_leaf: dict[str, list[Item]], topic: str) -> Item:
    """Resolve a seed to exactly one item.

    A seed containing `::` is a path suffix and must match exactly one item. Leaf names are
    convenient but ambiguous across 60 crates -- `Statistics` alone resolves to Parquet's enum
    rather than DataFusion's struct -- so qualify a seed whenever the name is shared.
    """
    if "::" in seed:
        matches = [
            item
            for items in by_leaf.values()
            for item in items
            if item.path == seed or item.path.endswith(f"::{seed}")
        ]
        if not matches:
            raise UnresolvedSeed(f"topic {topic!r} names {seed!r}, which is not in the model")
        if len(matches) > 1:
            paths = sorted(item.path for item in matches)[:4]
            raise UnresolvedSeed(f"topic {topic!r} seed {seed!r} is ambiguous: {paths}")
        return matches[0]

    candidates = by_leaf.get(seed) or []
    if not candidates:
        raise UnresolvedSeed(f"topic {topic!r} names {seed!r}, which is not in the model")
    if len(candidates) > 1:
        paths = sorted(item.path for item in candidates)[:4]
        raise UnresolvedSeed(
            f"topic {topic!r} seed {seed!r} matches {len(candidates)} items; qualify it: {paths}"
        )
    return candidates[0]


def _config_rows(content: Path) -> list[tuple[str, str, str]]:
    source = content.joinpath("corpus/guides/user-guide/configs.md")
    if not source.exists():
        return []
    rows = []
    for line in source.read_text().splitlines():
        match = CONFIG_ROW.match(line)
        if match:
            rows.append((match.group(1).strip(), match.group(2).strip(), match.group(3).strip()))
    return rows


def _builder_methods(item: Item) -> list[tuple[str, str]]:
    seen: set[str] = set()
    found: list[tuple[str, str]] = []
    for method in sorted(item.methods, key=lambda m: m.name):
        if method.via_trait or not method.name.startswith("with_"):
            continue
        if method.name in seen:
            continue
        seen.add(method.name)
        found.append((method.name, method.summary))
    return found


def write_topics(items: dict[str, Item], content: Path, definitions: Path) -> int:
    payload = json.loads(definitions.read_text())
    by_leaf = _by_leaf(items)
    config_rows = _config_rows(content)
    topics_dir = content.joinpath("topics")
    topics_dir.mkdir(parents=True, exist_ok=True)

    written: list[dict] = []
    for topic in payload["topics"]:
        slug = topic["slug"]
        lines = [f"# {topic['title']}", "", topic["mental_model"].strip(), ""]

        entries = [_resolve(leaf, by_leaf, slug) for leaf in topic.get("entry_points", [])]
        if entries:
            lines += [
                "## Entry points",
                "",
                "| Type | Kind | Methods | Prose | Records |",
                "|---|---|---:|---|---|",
            ]
            for item in entries:
                prose = api_file(item.module)
                records = model_file(item.module)
                lines.append(
                    f"| `{item.path}` | {item.kind} | {len(item.methods)} | "
                    f"[prose](../{prose}#{item.name.lower()}) | [records](../{records}) |"
                )
            lines.append("")

        points = topic.get("extension_points", [])
        if points:
            lines += [
                "## Extension points",
                "",
                "| Trait | Required | Provided | Implementors | Page |",
                "|---|---:|---:|---:|---|",
            ]
            for leaf in points:
                item = _resolve(leaf, by_leaf, slug)
                page = f"../traits/{leaf}.md"
                exists = content.joinpath("traits", f"{leaf}.md").exists()
                link = f"[{leaf}]({page})" if exists else leaf
                lines.append(
                    f"| `{item.path}` | {len(set(item.required_methods))} | "
                    f"{len(set(item.provided_methods))} | {len(item.implementors)} | {link} |"
                )
            lines.append("")

        knobs = topic.get("builder_types", [])
        if knobs:
            lines += [
                "## Configuration methods",
                "",
                "Chainable `with_*` builders. These are invisible to anyone reading only the",
                "constructor, which is why they are the most consistently missed part of the API.",
                "",
            ]
            for leaf in knobs:
                item = _resolve(leaf, by_leaf, slug)
                methods = _builder_methods(item)
                if not methods:
                    continue
                lines.append(f"**`{item.name}`** — {len(methods)} builder methods")
                lines.append("")
                lines.append(", ".join(f"`{name}`" for name, _ in methods))
                lines.append("")

        prefixes = tuple(topic.get("config_prefixes", []))
        if prefixes and config_rows:
            matched = [row for row in config_rows if row[0].startswith(prefixes)]
            if matched:
                lines += [
                    f"## Settings ({len(matched)})",
                    "",
                    "Full table with Rust setters in "
                    "[`../catalogs/config-options.md`](../catalogs/config-options.md).",
                    "",
                    "| Setting | Default |",
                    "|---|---|",
                ]
                for key, default, _ in sorted(matched):
                    lines.append(f"| `{key}` | {default or '—'} |")
                lines.append("")

        example_dirs = topic.get("example_dirs", [])
        found_examples: list[str] = []
        for directory in example_dirs:
            base = content.joinpath("corpus/examples", directory)
            if base.is_dir():
                found_examples += [
                    path.relative_to(content).as_posix() for path in sorted(base.glob("*.rs"))
                ]
        if found_examples:
            lines += [f"## Runnable examples ({len(found_examples)})", ""]
            lines += [f"- [`{path}`](../{path})" for path in found_examples]
            lines.append("")

        guides = [g for g in topic.get("guides", []) if content.joinpath(g).exists()]
        if guides:
            lines += ["## Upstream guides", ""]
            lines += [f"- [`{guide}`](../{guide})" for guide in guides]
            lines.append("")

        for heading, key in (
            ("Decision rules", "decision_rules"),
            ("Anti-patterns", "anti_patterns"),
            ("Agent checklist", "checklist"),
        ):
            bullets = topic.get(key) or []
            if bullets:
                lines += [f"## {heading}", ""]
                lines += [f"- {bullet}" for bullet in bullets]
                lines.append("")

        topics_dir.joinpath(f"{slug}.md").write_text("\n".join(lines))
        written.append({"slug": slug, "title": topic["title"]})

    _write_map(written, payload, topics_dir)
    return len(written)


def _write_map(written: list[dict], payload: dict, topics_dir: Path) -> None:
    lines = [
        "# Capability map",
        "",
        payload.get("preamble", "").strip(),
        "",
        "| Topic | Covers |",
        "|---|---|",
    ]
    for topic in payload["topics"]:
        lines.append(f"| [{topic['title']}]({topic['slug']}.md) | {topic['covers']} |")
    lines.append("")
    topics_dir.joinpath("00-map.md").write_text("\n".join(lines))

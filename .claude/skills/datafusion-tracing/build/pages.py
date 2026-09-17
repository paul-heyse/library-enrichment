"""Render the seam pages and the map that routes to them.

A seam page is this repository's answer to the sibling repositories' topic page, and it differs
in one structural way: **every seam must say what it refuses to answer**, and the build fails a
seam whose `cannot` list has fewer than two entries. That section is the rung an agent should
reach before concluding a capability is missing, which is why it is required rather than
optional -- a seam nobody could name two limits for is a seam nobody thought hard enough about.

Seeds are resolved against the built model. An unresolved or ambiguous seed fails the build,
because a page that silently drops a type it can no longer find asserts by omission that the
capability does not exist.
"""

from __future__ import annotations

import json
from pathlib import Path

import emit

MINIMUM_CANNOT = 2


class SeedError(RuntimeError):
    """A seam names something the model does not contain, or names it ambiguously."""


def _resolve(seed: str, items: dict) -> object:
    """Resolve a leaf name or a qualified path to exactly one item."""
    if seed in items:
        return items[seed]
    matches = [item for path, item in items.items() if path.rsplit("::", 1)[-1] == seed]
    if not matches:
        matches = [item for path, item in items.items() if path.endswith(seed)]
    if not matches:
        raise SeedError(
            f"seed {seed!r} resolves to nothing. A page that quietly dropped it would assert "
            f"by omission that the capability does not exist, so this is a stop."
        )
    if len(matches) > 1:
        paths = sorted(item.path for item in matches)
        raise SeedError(f"seed {seed!r} is ambiguous across {paths}; qualify it")
    return matches[0]


def _span_rows(content: Path) -> dict[str, list[str]]:
    table: dict[str, list[str]] = {}
    path = content / "index" / "spans.tsv"
    if not path.exists():
        return table
    for line in path.read_text().splitlines():
        if line:
            columns = line.split("\t")
            table.setdefault(columns[0], []).append(line)
    return table


def _macro_arms(content: Path) -> dict[str, list[str]]:
    table: dict[str, list[str]] = {}
    path = content / "index" / "macros.tsv"
    if not path.exists():
        return table
    for line in path.read_text().splitlines():
        if line:
            columns = line.split("\t")
            table.setdefault(columns[0], []).append(columns[2])
    return table


def write_all(content: Path, items: dict, definitions: Path) -> dict[str, int]:
    spec = json.loads(definitions.read_text())
    seams = content / "seams"
    seams.mkdir(parents=True, exist_ok=True)
    spans = _span_rows(content)
    arms = _macro_arms(content)

    for topic in spec["topics"]:
        if len(topic.get("cannot", [])) < MINIMUM_CANNOT:
            raise SeedError(
                f"seam {topic['slug']!r} lists {len(topic.get('cannot', []))} limit(s). "
                f"At least {MINIMUM_CANNOT} are required: the section a reader is told to "
                f"consult before concluding a capability is absent cannot be the thin one."
            )
        _write_seam(seams, topic, items, spans, arms)

    _write_map(seams, spec)
    return {"seams": len(spec["topics"])}


def _write_seam(seams: Path, topic: dict, items: dict, spans: dict, arms: dict) -> None:
    lines = [
        f"# {topic['title']}",
        "",
        topic["mental_model"],
        "",
    ]

    entry_points = topic.get("entry_points") or []
    if entry_points:
        lines += ["## Entry points", "", "| Item | Visibility | Methods | Reached via |",
                  "|---|---|---:|---|"]
        for seed in entry_points:
            item = _resolve(seed, items)
            page = emit.api_file(item.module)
            lines.append(
                f"| [`{item.name}`]({'../' + page}) | {item.visibility} | "
                f"{len(item.methods)} | {item.reached_via or '—'} |"
            )
        lines.append("")

    if topic.get("macros"):
        lines += ["## Macros", "", "| Macro | Accepted forms |", "|---|---|"]
        for name in topic["macros"]:
            forms = arms.get(name, [])
            if not forms:
                raise SeedError(f"seam {topic['slug']!r} names macro {name!r}, which has no arms")
            lines.append(f"| `{name}!` | " + "<br>".join(f"`{f}`" for f in forms) + " |")
        lines += ["", "Full table: [`catalogs/macros.md`](../catalogs/macros.md)", ""]

    if topic.get("spans"):
        lines += ["## Spans it emits", "", "| Span | Target | Level | Fields | Evidence |",
                  "|---|---|---|---:|---|"]
        for name in topic["spans"]:
            rows = spans.get(name)
            if not rows:
                raise SeedError(
                    f"seam {topic['slug']!r} names span {name!r}, which no trace snapshot "
                    f"contains. A span page claiming an unobserved name would be the one kind "
                    f"of row this repository must never emit."
                )
            for row in rows:
                columns = row.split("\t")
                lines.append(
                    f"| `{columns[0]}` | `{columns[1]}` | {columns[2]} | {columns[3]} | "
                    f"{columns[9]} |"
                )
        lines.append("")

    lines += ["## What this seam cannot tell you", ""]
    for limit in topic["cannot"]:
        lines.append(f"- {limit}")
    lines.append("")

    if topic.get("catalogs") or topic.get("corpus"):
        lines += ["## Read next", ""]
        for reference in topic.get("catalogs", []):
            lines.append(f"- [`{reference}`](../{reference})")
        for reference in topic.get("corpus", []):
            lines.append(f"- `content/{reference}` — upstream, verbatim")
        lines.append("")

    if topic.get("checklist"):
        lines += ["## Before you call it done", ""]
        for check in topic["checklist"]:
            lines.append(f"- {check}")
        lines.append("")

    (seams / f"{topic['slug']}.md").write_text("\n".join(lines))


def _write_map(seams: Path, spec: dict) -> None:
    lines = [
        "# Seam map",
        "",
        spec["preamble"],
        "",
        "| Seam | Covers |",
        "|---|---|",
    ]
    for topic in spec["topics"]:
        lines.append(f"| [{topic['title']}]({topic['slug']}.md) | {topic['covers']} |")
    lines += [
        "",
        "Every page ends with **What this seam cannot tell you**. Read it before reporting that "
        "a capability is absent — most of what looks missing in this library is either "
        "undocumented-but-present, or genuinely DataFusion's rather than this library's.",
    ]
    (seams / "00-map.md").write_text("\n".join(lines) + "\n")

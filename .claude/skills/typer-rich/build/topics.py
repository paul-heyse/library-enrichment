"""Capability topics: one page per axis, joined to the model rather than written about it.

A topic definition names seeds -- canonical paths -- and everything else on the page is joined
out of the built model, so the mapping stays exact across a rebuild and cannot drift from the API
it describes.

**An unresolved seed fails the build.** A topic page that quietly dropped a type it could no
longer find would be worse than no page: it would assert, by omission, that a capability does not
exist. The same applies to a guide path that does not resolve into the corpus -- `SKILL.md` has
already shipped one dangling pointer, to a corpus that was never fetched, and a reader following
it could not tell "no corpus" from "no matches".

Seeds are canonical paths, not leaf names. The Rust skills resolve by leaf and raise on ambiguity
because their namespaces are flat; here `Tool` alone is `mcp_types._types.Tool`,
`fastmcp.tools.tool.Tool` and two dated era copies, so leaf resolution would be a coin flip
dressed as a lookup.
"""

from __future__ import annotations

import json
from pathlib import Path

from model import Model


class UnresolvedSeed(RuntimeError):
    """A topic names something the model does not contain."""


def _resolve(built: Model, topic: str, seed: str) -> str:
    """Return the canonical path for a seed, or raise.

    Accepts an access path as well as a canonical one, because the spelling a reader would
    write is often not the defining path -- that asymmetry is the whole point of this index.
    """
    if seed in built.items:
        return seed
    landed = built.access.get(seed)
    if landed and landed in built.items:
        return landed
    raise UnresolvedSeed(
        f"topic {topic!r} names {seed!r}, which is not in the model. "
        f"It was renamed, removed, or is a member rather than an item -- all three are re-pin "
        f"decisions, not rows to drop."
    )


def _guide_links(topic: str, guides: list[str], content: Path) -> list[str]:
    links: list[str] = []
    for guide in guides:
        target = content / "corpus" / guide
        if not target.exists():
            raise UnresolvedSeed(
                f"topic {topic!r} links {guide!r}, which is not in the corpus. "
                f"Check the manifest's corpora block and re-run acquire.py."
            )
        links.append(f"- [`corpus/{guide}`](../corpus/{guide})")
    return links


def write_all(built: Model, content: Path, definitions: Path) -> int:
    """Write `topics/00-map.md` and one page per topic. Returns the number written."""
    document = json.loads(definitions.read_text())
    topics = document["topics"]
    out = content / "topics"
    out.mkdir(parents=True, exist_ok=True)

    for topic in topics:
        slug = topic["slug"]
        lines = [f"# {topic['title']}", "", topic["mental_model"], "", "## Entry points", ""]
        lines += ["| Type | Kind | Import as | Prose | Records |", "|---|---|---|---|---|"]
        for seed in topic["entry_points"]:
            canonical = _resolve(built, slug, seed)
            item = built.items[canonical]
            spelling = f"`{item.preferred}`" if item.nameable else "*not importable*"
            lines.append(
                f"| `{canonical}` | {item.kind} | {spelling} | "
                f"[prose](../api/{item.module}.md) | "
                f"[records](../model/{item.module}.json) |"
            )
        lines.append("")

        guides = _guide_links(slug, topic.get("guides", []), content)
        if guides:
            lines += ["## Upstream guides", "", *guides, ""]
        for heading, key in (
            ("Decision rules", "decision_rules"),
            ("Anti-patterns", "anti_patterns"),
            ("Agent checklist", "checklist"),
        ):
            rows = topic.get(key) or []
            if rows:
                lines += [f"## {heading}", "", *[f"- {row}" for row in rows], ""]
        (out / f"{slug}.md").write_text("\n".join(lines))

    header = [
        "# Capability map",
        "",
        "Each page maps one capability to the types that implement it, the upstream guide that",
        "explains it, and the decisions worth making deliberately. Start here when the question",
        "is *which construct do I want*, rather than *how is this spelled*.",
        "",
        "| Topic | Covers |",
        "|---|---|",
    ]
    header += [
        f"| [{topic['title']}]({topic['slug']}.md) | {topic['covers']} |" for topic in topics
    ]
    (out / "00-map.md").write_text("\n".join(header) + "\n")
    return len(topics)

"""The router: which artifact answers which question, and which one you were about to misuse.

Every other index here is derived from something upstream publishes. This one is not, and it is
the reason a reader who knows nothing about this library can still get a right answer quickly.

`questions.tsv` carries a `rejected` column, and that column is the load-bearing field. A row
that only named the right artifact would leave the reader's first instinct untouched, and for
this subject the first instinct is usually *read the documentation*, which is exactly the move
that loses sixteen methods. Naming the plausible wrong source AND why it is wrong is what
intercepts it.

`verify.py` runs every recipe in the emitted table and fails on an empty result or a citation to
a probe that did not run. A curated table silently diverging from what it describes is the
failure this repository is built to prevent, so it stops the build rather than ageing quietly.
"""

from __future__ import annotations

import json
from pathlib import Path

COLUMNS = ("question", "area", "entry_point", "rejected", "why_rejected", "recipe", "probe")


class RouterError(RuntimeError):
    """A routed question names something that does not exist."""


def write_questions(content: Path, definitions: Path, probe_ids: set[str] | None = None) -> int:
    spec = json.loads(definitions.read_text())
    areas = set(spec["areas"])
    known_probes = probe_ids if probe_ids is not None else set()

    rows: list[str] = []
    for entry in spec["questions"]:
        if entry["area"] not in areas:
            raise RouterError(f"question {entry['question']!r} routes to unknown area {entry['area']!r}")
        probe = entry.get("probe", "-")
        if probe != "-" and known_probes and probe not in known_probes:
            raise RouterError(
                f"question {entry['question']!r} cites probe {probe}, which did not run. "
                f"A citation to a probe that does not exist is worse than none: it reads as "
                f"evidence."
            )
        rejected = entry.get("rejected") or []
        rows.append("\t".join((
            entry["question"],
            entry["area"],
            entry["entry_point"],
            ",".join(name for name, _ in rejected) or "-",
            " | ".join(f"{name}: {reason}" for name, reason in rejected) or "-",
            entry["recipe"],
            probe,
        )))

    index = content / "index"
    index.mkdir(parents=True, exist_ok=True)
    (index / "questions.tsv").write_text("".join(f"{row}\n" for row in sorted(set(rows))))
    _write_map(content, spec)
    return len(rows)


def _write_map(content: Path, spec: dict) -> None:
    by_area: dict[str, list[dict]] = {}
    for entry in spec["questions"]:
        by_area.setdefault(entry["area"], []).append(entry)

    lines = [
        "# By what you are trying to do",
        "",
        spec["preamble"],
        "",
    ]
    for area in spec["areas"]:
        entries = by_area.get(area, [])
        if not entries:
            continue
        lines += [f"## {area}", "", "| Question | Go to | Not — |", "|---|---|---|"]
        for entry in sorted(entries, key=lambda e: e["question"]):
            rejected = "; ".join(f"**{n}** — {r}" for n, r in (entry.get("rejected") or []))
            lines.append(
                f"| {entry['question']} | `{entry['entry_point']}` | {rejected or '—'} |"
            )
        lines.append("")
    lines += [
        "## Running a route",
        "",
        "`index/questions.tsv` carries an executable recipe per row, relative to the skill "
        "directory:",
        "",
        "```bash",
        "rg -P '^Which macro' content/index/questions.tsv | cut -f6",
        "```",
        "",
        "`verify.py` executes all of them. A recipe that returns nothing fails the build, "
        "because a confident pointer to an empty set is the failure this repository exists to "
        "prevent.",
    ]
    (content / "topics").mkdir(parents=True, exist_ok=True)
    (content / "topics" / "00-map.md").write_text("\n".join(lines) + "\n")

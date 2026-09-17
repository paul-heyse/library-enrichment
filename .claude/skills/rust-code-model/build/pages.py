"""Render the prose pages an agent reads once an index row has pointed it somewhere.

Three kinds, and the split is by what the reader is doing:

    topics/     use-case pages, reached when you know the task and not the library
    layers/     one page per subject, reached when you know the layer and need its terms
    catalogs/   direct-lookup tables, reached when you already know what you are looking for

`topics/00-map.md` is the entry point for all of it and the first rung of the ladder in
SKILL.md. It is the router in readable form: the same content as `questions.tsv` and
`layers.tsv`, arranged for a reader rather than for ripgrep.

Every page is generated. Nothing here is hand-maintained prose that could drift from the index
beside it -- the seeds in `topics.json` and `router.json` are the only written text, and they
are rendered, never copied.
"""

from __future__ import annotations

import json
from pathlib import Path

import model


def _table(headers: list[str], rows: list[list[str]]) -> list[str]:
    lines = ["| " + " | ".join(headers) + " |", "|" + "|".join("---" for _ in headers) + "|"]
    lines += ["| " + " | ".join(cell.replace("|", "\\|") for cell in row) + " |" for row in rows]
    return lines


def _load(build_dir: Path, name: str) -> dict:
    return json.loads((build_dir / name).read_text(encoding="utf-8"))


# --------------------------------------------------------------------------- the map


def write_map(router: dict, topics: dict, probe_results: list[dict], content: Path) -> None:
    """The router in readable form. Rung 0 of the ladder."""
    confirmed = sum(1 for row in probe_results if row["verdict"] == "confirmed")
    lines = [
        "# Which layer answers this?",
        "",
        "Seven ways to know something about Rust code. They overlap enough to look",
        "interchangeable and they answer different questions, so the first decision is which one",
        "to ask -- not which API to call.",
        "",
        "**The failure this page exists to prevent is a confident answer from the wrong layer.**",
        "Rustdoc JSON will tell you a function exists and has no opinion at all about what it",
        "does, because it contains no bodies. The syntax tree will parse anything and resolve",
        "nothing. MIR knows exactly which branch runs and has forgotten the names.",
        "",
        f"{confirmed} of the claims below were executed against the pinned toolchain, each",
        "paired with a control that had to come out the other way. The commands are in",
        "`content/index/behaviors.tsv`.",
        "",
        "## The layers",
        "",
    ]
    lines += _table(
        ["Layer", "Answers", "Cannot answer"],
        [[layer["id"], layer["answers"], layer["cannot_answer"]] for layer in router["layers"]],
    )
    lines += [
        "",
        "`content/index/layers.tsv` carries the same rows plus how each is obtained, whether it",
        "needs a build or a network, its stability, and the pins.",
        "",
        "## By what you are trying to do",
        "",
    ]
    lines += _table(
        ["Task", "Layer", "Start at"],
        [
            [entry["question"], entry["layer"], f"`{entry['entry_point']}`"]
            for entry in router["questions"]
        ],
    )
    lines += [
        "",
        "`content/index/questions.tsv` carries the same rows with two more columns: the layers a",
        "reader plausibly reaches for instead, and why each is wrong. That column is the point of",
        "the table.",
        "",
        "## Topic pages",
        "",
    ]
    lines += _table(
        ["Topic", "Covers"],
        [[f"[{t['title']}]({t['slug']}.md)", t["lead"]] for t in topics["topics"]],
    )
    lines += [
        "",
        "## Layer pages",
        "",
    ]
    lines += _table(
        ["Layer", "Subject"],
        [
            [f"[{layer['id']}](../layers/{layer['id']}.md)", layer["subject"]]
            for layer in router["layers"]
        ],
    )
    (content / "topics" / "00-map.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


# --------------------------------------------------------------------------- topics


def write_topics(router: dict, topics: dict, probe_results: list[dict], content: Path) -> int:
    for topic in topics["topics"]:
        wanted = set(topic["layers"])
        questions = [q for q in router["questions"] if q["layer"] in wanted]
        probes = [row for row in probe_results if row["layer"] in wanted]

        lines = [
            f"# {topic['title']}",
            "",
            topic["lead"],
            "",
            "## Mental model",
            "",
            topic["mental_model"],
            "",
        ]
        if questions:
            lines += ["## Questions this covers", ""]
            lines += _table(
                ["Question", "Start at", "Instead of"],
                [
                    [
                        q["question"],
                        f"`{q['entry_point']}`",
                        ", ".join(name for name, _ in q["rejected"]) or "-",
                    ]
                    for q in questions
                ],
            )
            lines += [""]
            rejections = [
                (q["question"], name, why) for q in questions for name, why in q["rejected"]
            ]
            if rejections:
                lines += ["## Why not the neighbouring layer", ""]
                for question, layer, why in rejections:
                    lines += [f"- **{question}** -- not `{layer}`: {why}"]
                lines += [""]
        if probes:
            lines += [
                "## What was executed",
                "",
                "Each row ran against the pinned toolchain. A `confirmed` verdict means the probe",
                "held *and* its control came out the other way; `recorded` means there was nothing",
                "for a control to falsify.",
                "",
            ]
            lines += _table(
                ["Probe", "Question", "Verdict"],
                [[row["id"], row["question"], row["verdict"]] for row in probes],
            )
            lines += [""]
            for row in probes:
                if row.get("note"):
                    lines += [f"**{row['id']}** — {row['note']}", ""]
        (content / "topics" / f"{topic['slug']}.md").write_text(
            "\n".join(lines) + "\n", encoding="utf-8"
        )
    return len(topics["topics"])


# --------------------------------------------------------------------------- layers


def write_layers(
    router: dict,
    items: dict[str, model.Item],
    facts: dict[str, dict],
    probe_results: list[dict],
    content: Path,
) -> int:
    directory = content / "layers"
    directory.mkdir(parents=True, exist_ok=True)
    for layer in router["layers"]:
        crates = sorted(p for p, e in facts.items() if e["layer"] == layer["id"])
        symbols = sum(1 for item in items.values() if item.crate in crates)
        probes = [row for row in probe_results if row["layer"] == layer["id"]]
        questions = [q for q in router["questions"] if q["layer"] == layer["id"]]

        pinned = (
            ", ".join(f"{c} {facts[c]['version']}" for c in crates)
            or "none -- this layer is the compiler itself"
        )
        lines = [
            f"# {layer['subject']}",
            "",
            f"**Reach for it when** {layer['reach_for_when']}.",
            "",
            "## What it is for",
            "",
            layer["answers"] + ".",
            "",
            "## What it cannot answer",
            "",
            layer["cannot_answer"] + ".",
            "",
            "## Getting it",
            "",
        ]
        lines += _table(
            ["", ""],
            [
                ["Obtained by", f"`{layer['obtained_by']}`"],
                ["Entry point", f"`{layer['entry_point']}`"],
                ["Needs a build", layer["needs_build"]],
                ["Needs a network", layer["needs_network"]],
                ["Stability", layer["stability"]],
                ["Crates pinned here", pinned],
                ["Indexed symbols", str(symbols)],
            ],
        )
        lines += [""]
        if questions:
            lines += ["## Questions routed here", ""]
            lines += [f"- {q['question']} — `{q['entry_point']}`" for q in questions]
            lines += [""]
        if probes:
            lines += ["## Executed evidence", ""]
            lines += _table(
                ["Probe", "Question", "Verdict", "Command"],
                [
                    [row["id"], row["question"], row["verdict"], f"`{row['command'][:90]}`"]
                    for row in probes
                ],
            )
            lines += [""]
        (directory / f"{layer['id']}.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    return len(router["layers"])


# --------------------------------------------------------------------------- catalogs


def write_catalogs(router: dict, probe_results: list[dict], index: Path, content: Path) -> int:
    directory = content / "catalogs"
    directory.mkdir(parents=True, exist_ok=True)
    written = 0

    lines = [
        "# Compiler views",
        "",
        "One flag walks every layer the compiler builds. The list is read from the compiler",
        "itself at build time, so it cannot drift from the toolchain that produced the",
        "observations here.",
        "",
    ]
    lines += _table(
        ["View", "Shows", "Loses", "Probe"],
        [[f"`{v['view']}`", v["shows"], v["loses"], v["probe"]] for v in router["views"]],
    )
    lines += [
        "",
        "`-Zdump-mir=all` writes every pass separately, and the filenames are the pass pipeline.",
        "Adding `-Zdump-mir-dataflow=yes` adds the dataflow results as graphviz; region inference",
        "comes from `-Zdump-mir=all` alone and does not need the second flag (DF004).",
    ]
    (directory / "compiler-views.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    written += 1

    lines = [
        "# Version oracles",
        "",
        "For each subject: how to ask what is actually running, what that answer covers, and the",
        "reading that looks authoritative and is not. Three of these answer a question next to",
        "the one you asked.",
        "",
    ]
    lines += _table(
        ["Subject", "Ask", "Answers for", "Trap"],
        [
            [o["subject"], f"`{o['command']}`", o["answers_for"], o["trap"]]
            for o in router["oracles"]
        ],
    )
    (directory / "version-oracles.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    written += 1

    lines = [
        "# Not reachable from here",
        "",
        "Things that are real -- in an API, in a rustup component, or in a reader's memory --",
        "and cannot be obtained in this setting. Silence in an index is not evidence of absence,",
        "so the absences worth knowing about are written down.",
        "",
    ]
    lines += _table(
        ["Capability", "Exists in", "Reachable", "Why", "Use instead"],
        [
            [u["capability"], u["exists_in"], u["reachable"], u["why"], u["instead"]]
            for u in router["unreachable"]
        ],
    )
    (directory / "unreachable.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    written += 1

    tally: dict[str, int] = {}
    for row in probe_results:
        tally[row["verdict"]] = tally.get(row["verdict"], 0) + 1
    lines = [
        "# Executed behaviour",
        "",
        "Every claim in this repository that could be executed, was. A `confirmed` verdict means",
        "the probe held and its control came out the other way; a probe without a control is",
        "`recorded`, which is weaker and kept distinct on purpose.",
        "",
        "Verdicts: " + ", ".join(f"**{k}** {v}" for k, v in sorted(tally.items())) + ".",
        "",
    ]
    lines += _table(
        ["Probe", "Layer", "Question", "Verdict", "Evidence"],
        [
            [row["id"], row["layer"], row["question"], row["verdict"], f"`{row['evidence'][:50]}`"]
            for row in probe_results
        ],
    )
    lines += ["", "`content/index/behaviors.tsv` carries the commands and the controls."]
    (directory / "behaviour.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    written += 1
    return written


# --------------------------------------------------------------------------- entry point


def write_all(
    build_dir: Path,
    content: Path,
    items: dict[str, model.Item],
    probe_results: list[dict],
    readings: dict,
    facts: dict[str, dict],
) -> dict[str, int]:
    router = _load(build_dir, "router.json")
    topics = _load(build_dir, "topics.json")
    (content / "topics").mkdir(parents=True, exist_ok=True)

    write_map(router, topics, probe_results, content)
    return {
        "topic_pages": write_topics(router, topics, probe_results, content) + 1,
        "layer_pages": write_layers(router, items, facts, probe_results, content),
        "catalog_pages": write_catalogs(router, probe_results, content / "index", content),
    }

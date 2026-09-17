"""The router: which layer answers which question, and which one you were about to misuse.

Every other index here is derived from something upstream publishes. This one is not, and it is
the reason the repository exists. Seven layers overlap enough to look interchangeable, so the
scarce knowledge is not "what can ra_ap_hir do" but "you are holding the wrong one".

`questions.tsv` therefore carries a `rejected` column, and it is the load-bearing field. A row
that only named the right layer would leave the reader's first instinct untouched; naming the
plausible wrong layer *and why it is wrong* is what intercepts it.

Two tables are checked against the world rather than merely written down:

    compiler-views   every `-Zunpretty` value the compiler admits must have a curated row,
                     and no row may name a view the compiler rejects
    questions        every `probe` reference must name a probe that actually ran

Both are assertions, not filters. A curated table silently diverging from the tool it describes
is the failure this whole repository is built to prevent, so it stops the build instead.
"""

from __future__ import annotations

import json
from pathlib import Path


class RouterError(RuntimeError):
    """The curated router disagrees with something that was executed."""


def _clean(text: str) -> str:
    return " ".join(str(text).replace("\t", " ").split())


def load(build_dir: Path) -> dict:
    return json.loads((build_dir / "router.json").read_text(encoding="utf-8"))


# --------------------------------------------------------------------------- tables


def write_layers(spec: dict, facts: dict[str, dict], out: Path) -> int:
    """One row per layer: what it answers, and -- the useful half -- what it cannot.

    `cannot_answer` is populated from probes and verified upstream facts, never from impressions.
    An agent that reads only this file should still be unable to ask MIR for a comment.
    """
    pins = {}
    for package, entry in facts.items():
        pins.setdefault(entry["layer"], []).append(f"{package} {entry['version']}")

    rows = []
    for layer in spec["layers"]:
        rows.append(
            "\t".join(
                (
                    layer["id"],
                    _clean(layer["subject"]),
                    _clean(layer["obtained_by"]),
                    _clean(layer["entry_point"]),
                    _clean(layer["answers"]),
                    _clean(layer["cannot_answer"]),
                    _clean(layer["needs_build"]),
                    _clean(layer["needs_network"]),
                    _clean(layer["stability"]),
                    _clean(", ".join(sorted(pins.get(layer["id"], []))) or "no crate -- compiler"),
                )
            )
        )
    (out / "layers.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")
    return len(rows)


def write_questions(spec: dict, probe_ids: set[str], out: Path) -> int:
    """The functional-use-case router."""
    known = {layer["id"] for layer in spec["layers"]} | {"cross-layer"}
    rows = []
    for entry in spec["questions"]:
        if entry["layer"] not in known:
            raise RouterError(f"question {entry['question']!r} routes to unknown {entry['layer']}")
        for rejected, _why in entry["rejected"]:
            if rejected not in known:
                raise RouterError(f"question {entry['question']!r} rejects unknown {rejected}")
        if entry["probe"] != "-" and entry["probe"] not in probe_ids:
            raise RouterError(
                f"question {entry['question']!r} cites probe {entry['probe']}, which did not run. "
                f"A citation to a probe that does not exist is worse than none: it reads as "
                f"evidence."
            )
        rows.append(
            "\t".join(
                (
                    _clean(entry["question"]),
                    entry["layer"],
                    _clean(entry["entry_point"]),
                    ",".join(layer for layer, _ in entry["rejected"]) or "-",
                    " | ".join(f"{layer}: {why}" for layer, why in entry["rejected"]) or "-",
                    _clean(entry["recipe"]),
                    entry["probe"],
                )
            )
        )
    (out / "questions.tsv").write_text("\n".join(sorted(rows)) + "\n", encoding="utf-8")
    return len(rows)


def write_views(spec: dict, admitted: list[str], probe_ids: set[str], out: Path) -> int:
    """The `-Zunpretty` ladder, cross-checked against what the compiler admits."""
    curated = {view["view"] for view in spec["views"]}
    if admitted:
        missing = sorted(set(admitted) - curated)
        invented = sorted(curated - set(admitted))
        if missing or invented:
            raise RouterError(
                f"the curated view table disagrees with the compiler. Undocumented: {missing}. "
                f"Named but rejected by rustc: {invented}."
            )
    rows = []
    for view in spec["views"]:
        if view["probe"] != "-" and view["probe"] not in probe_ids:
            raise RouterError(f"view {view['view']} cites probe {view['probe']}, which did not run")
        rows.append(
            "\t".join(
                (
                    view["view"],
                    f"-Zunpretty={view['view']}",
                    _clean(view["shows"]),
                    _clean(view["loses"]),
                    view["probe"],
                )
            )
        )
    (out / "compiler-views.tsv").write_text("\n".join(sorted(rows)) + "\n", encoding="utf-8")
    return len(rows)


def write_oracles(spec: dict, readings: dict, out: Path) -> int:
    """For each subject: how to ask what is actually running, and the reading that misleads.

    The `trap` column is the point. Three of these subjects answer a question next to the one
    being asked, and confidently.
    """
    observed = {
        "rust-analyzer": readings["rust_analyzer"].get("binary_version", ""),
        "rustc": readings["rustc"],
        "rustdoc": str(readings["rustdoc_format_emitted"]),
        "cargo": str(readings["cargo"].get("cargo", "")),
    }
    rows = []
    for entry in spec["oracles"]:
        rows.append(
            "\t".join(
                (
                    _clean(entry["subject"]),
                    _clean(entry["question"]),
                    _clean(entry["command"]),
                    _clean(entry["answers_for"]),
                    _clean(entry["trap"]),
                    _clean(observed.get(entry["subject"], "-")) or "-",
                )
            )
        )
    (out / "version-oracles.tsv").write_text("\n".join(sorted(rows)) + "\n", encoding="utf-8")
    return len(rows)


def write_unreachable(spec: dict, out: Path) -> int:
    """Real in the API or in someone's memory, not reachable from here -- and what to use."""
    rows = [
        "\t".join(
            (
                _clean(entry["capability"]),
                _clean(entry["exists_in"]),
                entry["reachable"],
                _clean(entry["why"]),
                _clean(entry["instead"]),
            )
        )
        for entry in spec["unreachable"]
    ]
    (out / "unreachable.tsv").write_text("\n".join(sorted(rows)) + "\n", encoding="utf-8")
    return len(rows)


# --------------------------------------------------------------------------- entry point


def write_all(
    build_dir: Path,
    out: Path,
    readings: dict,
    facts: dict[str, dict],
    probe_results: list[dict],
) -> dict[str, int]:
    spec = load(build_dir)
    probe_ids = {row["id"] for row in probe_results}
    admitted = list(readings.get("unpretty_views") or [])
    return {
        "layers": write_layers(spec, facts, out),
        "questions": write_questions(spec, probe_ids, out),
        "compiler_views": write_views(spec, admitted, probe_ids, out),
        "version_oracles": write_oracles(spec, readings, out),
        "unreachable": write_unreachable(spec, out),
    }

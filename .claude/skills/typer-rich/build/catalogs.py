"""Direct answers to fixed questions, generated from the model rather than written.

fastmcp's catalogs are mostly MCP-specific and do not transfer; two do, and are adapted here.
The rest answer questions these subjects raise and fastmcp's do not: what moved between releases,
which library's way to reach for, why the output is plain, and what this index refuses to claim.

`resolve` keeps fastmcp's rule verbatim: an unresolved seed fails the build. A catalog whose
subject was renamed upstream must stop the build rather than silently shipping a shorter table,
because a short table and a complete one look identical to a reader.
"""

from __future__ import annotations

import json
from pathlib import Path

import model

REACHABILITY_PROSE = (
    ("importable", "Typer re-exports it; write that spelling"),
    ("subclassed-via", "a class you *can* name inherits from it"),
    ("subtype-of", "you handle it through an ancestor you can name"),
    ("received", "it arrives in a parameter of something you can name"),
    ("declared", "what your annotation becomes; see the `param-type` seam"),
    ("registered-via", "reached only through a registrar function"),
    ("internal", "genuinely never met; the manifest allowlist is the claim"),
)


class CatalogError(RuntimeError):
    """A catalog seed does not match the model, which is a re-pin decision."""


def resolve(seed: str, built: model.Model) -> str:
    if seed in built.items:
        return seed
    for path, item in built.items.items():
        if item.preferred == seed:
            return path
    raise CatalogError(
        f"catalog seed {seed!r} is not in the model; it was renamed or removed upstream, which "
        "is a re-pin decision, not a row to drop silently"
    )


def _table(header: list[str], rows: list[list[str]]) -> str:
    out = ["| " + " | ".join(header) + " |", "|" + "|".join(["---"] * len(header)) + "|"]
    out += ["| " + " | ".join(cell for cell in row) + " |" for row in rows]
    return "\n".join(out)


def write_breaking_changes(data: dict, content: Path) -> int:
    """What is true at the pins, and -- for Rich only -- what changed on the way here.

    Typer's pre-0.27 history is deliberately absent: this repository describes Typer 0.27.2 and
    says nothing about earlier releases. The facts that history used to carry are stated in the
    present tense instead, which is also what the Typer `project-*` rules are generated from.
    """
    facts = [f for f in data.get("current_facts", []) if f["subject"] == "typer"]
    rows = data["breaking_changes"]
    body = [
        "# Facts at the pins",
        "",
        "The reason this repository exists. Both libraries carry behaviour a model answers wrong",
        "from memory, and it is invisible at the call site: the code reads correctly, the type",
        "checker passes, and the failure arrives at runtime or -- worse -- not at all, as",
        "silently wrong output.",
        "",
        "## Typer 0.27.2",
        "",
        "Stated in the present tense. This index describes the pinned release and does not",
        "catalogue what earlier versions did.",
        "",
        _table(
            ["Fact", "Why it matters", "Rule"],
            [
                [
                    fact["fact"],
                    fact.get("consequence", ""),
                    f"`{fact['rule']}`" if fact.get("rule", "-") != "-" else "—",
                ]
                for fact in facts
            ],
        ),
        "",
        "## Changes within a pinned line",
        "",
        "`after` is upstream's own replacement text wherever upstream supplied one.",
        "",
        _table(
            ["Version", "Date", "Subject", "What changed", "What to do instead", "Rule"],
            [
                [
                    f"`{r['version']}`",
                    r["date"],
                    r["subject"],
                    r["before"],
                    r["after"],
                    f"`{r['rule']}`" if r["rule"] != "-" else "—",
                ]
                for r in rows
            ],
        ),
        "",
        "## The two that catch people",
        "",
        "**Click is vendored.** `typer/_click/` *is* Click, adapted, and `click` is not a Typer",
        "dependency: `Requires-Dist` names `shellingham`, `rich`, `annotated-doc` and `colorama`",
        "on Windows, and nothing else. So `import click` in a Typer project resolves to whatever",
        "copy is installed for some other reason -- a different library from the one Typer runs,",
        "and it still type-checks. `typer.main.get_command(app)` returns a `Command`; probe",
        "`V003` shows whose.",
        "",
        "**`typer-slim` does not avoid Rich.** It is a shallow wrapper around `typer` that",
        "requires `rich` and `shellingham` unconditionally. Installing it to keep the dependency",
        "tree small accomplishes nothing.",
        "",
    ]
    (content / "catalogs" / "breaking-changes.md").write_text("\n".join(body))
    return len(rows) + len(facts)


def write_overlap(data: dict, content: Path) -> int:
    rows = data["overlap"]
    body = [
        "# Same job, different answer",
        "",
        "Typer depends on Rich, and `typer/rich_utils.py` is the seam where help, errors and",
        "`rich_markup_mode` render. That makes them one subject -- and it also means they present",
        "near-duplicate surfaces that look interchangeable and are not.",
        "",
        "Each row names the spelling to write, the one an agent reaches for instead, and what",
        "actually differs. Rows carrying a probe id were measured, not reasoned about.",
        "",
        _table(
            ["I want to…", "Write", "Not", "Because", "Probe"],
            [
                [
                    r["question"],
                    f"`{r['answer']}`",
                    ", ".join(f"`{n}`" for n in r["rejected"]),
                    r["why"],
                    f"`{r['probe']}`" if r.get("probe") else "—",
                ]
                for r in rows
            ],
        ),
        "",
        "## The rule underneath",
        "",
        "`typer.echo` is literal: no markup, no wrapping, no width. Everything on the Rich side",
        "parses markup and wraps to a Console width. That single difference explains most of the",
        "table, and it is why a help string containing `[path]` loses the word while the same",
        "string through `typer.echo` does not.",
        "",
    ]
    (content / "catalogs" / "same-job-different-answer.md").write_text("\n".join(body))
    return len(rows)


def write_env_vars(data: dict, content: Path) -> int:
    rows = data["env_vars"]
    body = [
        "# Environment variables that change the output",
        "",
        "Why the colour is gone, why the width is 80, why the table looks different in CI. Most",
        "of these are documented only in a changelog entry, and two of them beat settings you",
        "passed explicitly to the constructor.",
        "",
        "Ordered by the layer they act on, not alphabetically.",
        "",
        _table(
            ["Variable", "Read by", "Effect", "Empty value", "Since", "Probe"],
            [
                [
                    f"`{r['name']}`",
                    r["read_by"],
                    r["effect"],
                    r["empty"],
                    r["since"],
                    f"`{r['probe']}`" if r.get("probe") else "—",
                ]
                for r in rows
            ],
        ),
        "",
        "## Two that are not intuitive",
        "",
        "`NO_COLOR=1` strips the colour from a Console built with `force_terminal=True` and",
        '`color_system="truecolor"` -- both explicit, and neither wins. The bold survives; only',
        "the colour goes (probe `E001`). An **empty** `NO_COLOR` is ignored, which is the",
        "opposite of the usual presence-is-enough convention (probe `E002`).",
        "",
        "`Console(_environ=...)` is Rich's own injection point and isolates most of this. It does",
        "**not** cover `UNICODE_VERSION`, which `rich/_unicode_data` reads from `os.environ`",
        "directly and then caches -- so that one is order-dependent as well (probe `E005`).",
        "",
    ]
    (content / "catalogs" / "env-vars.md").write_text("\n".join(body))
    return len(rows)


def write_vendored_click(built: model.Model, content: Path) -> int:
    """The page that makes 5,548 lines behind a private module path legible."""
    from collections import Counter

    click_items = {p: i for p, i in built.items.items() if i.subject == "click"}
    tally = Counter(i.reachability for i in click_items.values())
    modules = Counter(i.module for i in click_items.values())

    reachable = [
        (p, i)
        for p, i in sorted(click_items.items())
        if i.reachability not in ("internal", "importable")
    ]
    importable = [(p, i) for p, i in sorted(click_items.items()) if i.reachability == "importable"]

    body = [
        "# Typer's vendored Click",
        "",
        "Typer 0.27.2 carries its own adapted copy of Click at `typer/_click/`. Fifteen",
        "modules and 5,548 lines -- `Context`, `Parameter`, `ParamType`, shell completion and the",
        "whole exception hierarchy -- now sit behind a private module path.",
        "",
        "**`typer._click` is not `click`.** It is Typer's adapted copy. Every row under this",
        "subject is a fact about that copy at this pin. Never cite one as a fact about the `click`",
        "package, and never write the import.",
        "",
        '## Why the index does not stop at "not importable"',
        "",
        "It is true that you cannot import any of this, and on its own that is a useless answer.",
        "You never import `typer._click.core.Context` -- you are *handed* one as `ctx`, and",
        "`typer.Context` subclasses it, so most of what you can call on `typer.Context` is",
        "declared in a module you cannot name. So nameability is recorded honestly and",
        "**reachability** is recorded beside it.",
        "",
        _table(
            ["Reachability", "Count", "What it means"],
            [[f"`{kind}`", str(tally.get(kind, 0)), prose] for kind, prose in REACHABILITY_PROSE],
        ),
        "",
        f"## Re-exported by Typer ({len(importable)})",
        "",
        "These you write as ordinary Typer API. The definition path is private; the import path",
        "is not.",
        "",
        _table(
            ["Write", "Defined at"],
            [[f"`{i.preferred}`", f"`{p}`"] for p, i in importable[:40]],
        ),
        "",
        f"## Reached without being importable ({len(reachable)})",
        "",
        _table(
            ["Class", "Reached", "Via"],
            [
                [f"`{p}`", i.reachability, ", ".join(f"`{v}`" for v in i.reached_via[:2])]
                for p, i in reachable[:40]
            ],
        ),
        "",
        "## Modules",
        "",
        _table(
            ["Module", "Items"],
            [[f"`{m}`", str(n)] for m, n in sorted(modules.items())],
        ),
        "",
    ]
    (content / "catalogs" / "vendored-click.md").write_text("\n".join(body))
    return len(click_items)


def write_refused(data: dict, content: Path) -> int:
    rows = data["refused"]
    body = [
        "# What this index refuses to claim",
        "",
        "Stated as boundaries rather than left as gaps, because silence in an index is not",
        "evidence and a reader cannot tell an omission from an absence.",
        "",
        "This page covers behaviour that was considered for a probe and deliberately not probed.",
        "`reference.md` carries the full known-limits list, including what is excluded from the",
        "API index and why.",
        "",
        _table(
            ["Behaviour", "Why no probe", "Where to look instead"],
            [[r["what"], r["why"], r["instead"]] for r in rows],
        ),
        "",
        "## The general rule",
        "",
        "A capture is a fact about a construction. Where no construction produces stable bytes --",
        "anything driven by elapsed time, by a real terminal's negotiation, or by a library this",
        "index does not pin -- a normalised capture would be a fiction dressed as evidence, and",
        "upstream's own tests are the better answer. They are in the corpus.",
        "",
    ]
    (content / "catalogs" / "refused.md").write_text("\n".join(body))
    return len(rows)


def write_subject_map(built: model.Model, content: Path) -> int:
    from collections import Counter

    tally = Counter(i.subject for i in built.items.values())
    modules = Counter((i.subject, i.module) for i in built.items.values())
    body = [
        "# Three subjects, two pins",
        "",
        "`typer._click` is indexed as its own subject although it ships inside the `typer`",
        "distribution, because a row that cannot say which library it describes is worse than no",
        "row -- `typer._click.Command` and `click.Command` are different classes with the same",
        "name, and the one an agent imports is the wrong one.",
        "",
        _table(
            ["Subject", "Distribution", "Items", "Modules"],
            [
                [
                    f"`{name}`",
                    where,
                    str(tally[name]),
                    str(sum(1 for subject, _ in modules if subject == name)),
                ]
                for name, where in (
                    ("typer", "typer 0.27.2"),
                    ("rich", "rich 15.0.0"),
                    ("click", "vendored in typer 0.27.2"),
                )
            ],
        ),
        "",
        "Column 2 of `content/index/symbols.tsv` is the spelling to import; column 1 is where it",
        "is defined. For Typer those differ constantly, and a definition path containing `_click`",
        "is a warning rather than an address.",
        "",
    ]
    (content / "catalogs" / "subject-map.md").write_text("\n".join(body))
    return len(tally)


def write_all(built: model.Model, content: Path, data_path: Path) -> dict[str, int]:
    (content / "catalogs").mkdir(parents=True, exist_ok=True)
    data = json.loads(data_path.read_text())
    return {
        "breaking_changes": write_breaking_changes(data, content),
        "overlap": write_overlap(data, content),
        "env_vars": write_env_vars(data, content),
        "vendored_click": write_vendored_click(built, content),
        "refused": write_refused(data, content),
        "subjects": write_subject_map(built, content),
    }

"""Generate `content/seams/` -- the places your code meets theirs.

This replaces fastmcp's `extension_points.py`, which cannot express these subjects. That module
has one algorithm: take a nominal base class, split its members into `@abstractmethod`
(required) and the rest (provided), and count descendants. Applied here it would produce a page
for `rich.abc.RichRenderable` claiming zero obligations and zero implementors -- which is true,
and which is the opposite of the answer, because the class exists for `isinstance` and its own
docstring says not to extend it. Meanwhile the two protocols that actually decide whether an
object can be printed, `ConsoleRenderable` and `RichCast`, declare nothing but a dunder each and
would come back empty from every member filter in the pipeline.

So a seam declares its *kind*, and the kind decides where implementors come from:

  runtime-protocol  adjudicated verdicts from the checker (satisfies.tsv), not the class tree
  abstract-base     the transitive descendant closure
  duck-protocol     source and corpus sites that define the dunder -- there is no class at all
  subclass          the descendant closure, but of a concrete class you are meant to extend
  parameter         a value you pass, so implementors are the wrong question entirely
  module-constant   extension by rebinding; the "implementors" are the constants themselves

And every page carries `supported`, which is the field that keeps this honest. A monkeypatch
target rendered in the same shape as a documented protocol would read as equally blessed.
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path

import model

KIND_PROSE = {
    "runtime-protocol": (
        "A runtime protocol. Implementors below are the checker's own verdicts, not a class "
        "tree -- nothing subclasses this, and nothing needs to."
    ),
    "abstract-base": (
        "An abstract base. The required members are the obligations; everything else is "
        "provided, and provided is where the capability hides."
    ),
    "duck-protocol": (
        "A duck protocol. No class declares it -- you add the method and rich finds it, so no "
        "hierarchy will ever list you."
    ),
    "subclass": "A concrete class you are meant to extend.",
    "parameter": "A value you pass in. There is nothing to implement.",
    "module-constant": (
        "Extension by rebinding a module-level name. It works; it is not a promise."
    ),
}

SUPPORTED_PROSE = {
    "documented": "Upstream documents this.",
    "observed-only": (
        "Indexed from source and not executed here. Real, and not demonstrated by any probe in "
        "this repository -- see the known limits in `reference.md`."
    ),
    "private": (
        "**Not a public interface.** It works at this pinned release and upstream has promised "
        "nothing. Prefer the supported alternative named below, and if you use this anyway, "
        "write down that you did."
    ),
}


class SeamError(RuntimeError):
    """A declared seam does not match the model, which is a re-pin decision."""


def _resolve(seed: str, built: model.Model) -> str:
    """Fail loudly on a seed the model does not carry.

    Copied deliberately from `extension_points.resolve`: a seam whose subject was renamed
    upstream must stop the build, not quietly produce a shorter map. That rule is the most
    valuable line in the module it came from, and it fired correctly the first time this build
    ran against fastmcp's seeds.
    """
    if seed in built.items:
        return seed
    for path, item in built.items.items():
        if item.preferred == seed:
            return path
    # A `module-constant` seam's seed is a module rather than an item, which is the point of that
    # kind: extension by rebinding a module-level name. `typer.rich_utils` is the only one, and
    # it is exactly the seam a reader most needs warned about.
    if seed in built.modules:
        return seed
    raise SeamError(
        f"seam seed {seed!r} is not in the model; it was renamed or removed upstream, which is "
        "a re-pin decision, not a page to drop silently"
    )


def _implementors(
    seam: dict, resolved: list[str], built: model.Model, satisfies: list[list]
) -> list[str]:
    kind = seam["kind"]
    if kind == "runtime-protocol":
        names = {row[0] for row in satisfies if row[1] in resolved and row[2] == "yes"}
        return sorted(names)
    if kind in ("abstract-base", "subclass"):
        names: set[str] = set()
        for seed in resolved:
            names.update(child for child, _depth in built.descendants.get(seed, ()))
        return sorted(names)
    if kind == "duck-protocol":
        names = set()
        for path, item in built.items.items():
            member_names = {member["name"] for member in item.members}
            if any(dunder in member_names for dunder in seam["dunders"]):
                names.add(path)
        return sorted(names)
    return []


def _required(seam: dict, resolved: list[str], built: model.Model) -> list[str]:
    if seam["kind"] == "runtime-protocol":
        return sorted(seam["dunders"])
    if seam["kind"] == "duck-protocol":
        return sorted(seam["dunders"])
    if seam["kind"] == "module-constant":
        # The "obligations" of a rebinding seam are the constants themselves, which live in the
        # module rather than on any class.
        return sorted(
            built.items[path].name
            for seed in resolved
            for path in built.modules.get(seed, ())
            if path in built.items and built.items[path].kind == "attribute"
        )
    required: set[str] = set()
    for seed in resolved:
        item = built.items.get(seed)
        if item is None:
            continue
        for member in item.members:
            if "abstractmethod" in (member.get("labels") or []):
                required.add(member["name"])
    return sorted(required)


def _page(
    seam: dict,
    resolved: list[str],
    built: model.Model,
    satisfies: list[list],
    api_file: Callable[[str], str],
) -> str:
    implementors = _implementors(seam, resolved, built, satisfies)
    required = _required(seam, resolved, built)

    lines = [
        f"# {seam['title']}",
        "",
        f"**kind** `{seam['kind']}` · **supported** `{seam['supported']}` · "
        f"**subject** `{seam['subject']}`",
        "",
        KIND_PROSE[seam["kind"]],
        "",
        SUPPORTED_PROSE[seam["supported"]],
        "",
        "## Entry points",
        "",
    ]
    for seed in resolved:
        item = built.items.get(seed)
        if item is None:
            count = len(built.modules.get(seed, ()))
            lines.append(f"- `{seed}` — a module, {count} indexed names · `{api_file(seed)}`")
            continue
        spelling = item.preferred or seed
        note = "" if item.nameable else f" — not importable; reached {item.reachability}"
        lines.append(f"- `{spelling}` · defined at `{seed}`{note} · `{api_file(item.module)}`")

    lines += ["", "## Mental model", "", seam["mental_model"], ""]

    if required:
        label = "Required" if seam["kind"] != "parameter" else "Members"
        lines += [f"## {label}", ""]
        lines += [f"- `{name}`" for name in required]
        lines.append("")

    if implementors:
        lines += [f"## Implementors ({len(implementors)})", ""]
        shown = implementors[:20]
        lines += [f"- `{built.items[p].preferred or p}`" for p in shown if p in built.items]
        if len(implementors) > len(shown):
            lines.append(f"- _…and {len(implementors) - len(shown)} more_")
        lines.append("")
    elif seam["kind"] in ("runtime-protocol", "abstract-base", "duck-protocol", "subclass"):
        lines += [
            "## Implementors (0)",
            "",
            "None in the indexed set. That is a fact about this release, not a hint that the "
            "seam is unused -- for `rich.abc.RichRenderable` it is the whole point.",
            "",
        ]

    lines += ["## Decision rules", ""]
    lines += [f"- {rule}" for rule in seam["decision_rules"]]
    lines += ["", "## Anti-patterns", ""]
    lines += [f"- {bad}" for bad in seam["anti_patterns"]]

    if seam.get("probe"):
        lines += [
            "",
            "## Observed",
            "",
            f"Probe `{seam['probe']}` — see `content/probes/00-index.md`.",
        ]
    return "\n".join(lines) + "\n"


def write_all(
    declarations: dict,
    built: model.Model,
    satisfies: list[list],
    content: Path,
    api_file: Callable[[str], str],
) -> int:
    out = content / "seams"
    out.mkdir(parents=True, exist_ok=True)

    rows = []
    for seam in declarations["seams"]:
        resolved = [_resolve(seed, built) for seed in seam["seeds"]]
        (out / f"{seam['name']}.md").write_text(_page(seam, resolved, built, satisfies, api_file))
        implementors = _implementors(seam, resolved, built, satisfies)
        rows.append(
            f"| [`{seam['name']}`]({seam['name']}.md) | {seam['kind']} | "
            f"`{seam['supported']}` | {seam['subject']} | {len(implementors)} | {seam['title']} |"
        )

    header = [
        "# Seams",
        "",
        "Where your code meets theirs. Not all of these are extension points, and the",
        "`supported` column is the one to read first: `private` means it works and upstream has",
        "promised nothing, which is a different thing from an API even though it is reached the",
        "same way.",
        "",
        "| Seam | Kind | Supported | Subject | Implementors | What it is |",
        "|---|---|---|---|---:|---|",
    ]
    (out / "00-map.md").write_text("\n".join(header + rows) + "\n")
    return len(rows)

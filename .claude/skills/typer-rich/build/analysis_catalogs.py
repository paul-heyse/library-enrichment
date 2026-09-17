"""Catalogs derived from the checker's batch reports.

Kept separate from `catalogs.py` for one reason: every catalog there is derived from the *model*,
which is always present. These are derived from the analysis stage, which can be `blocked`. A
blocked stage must produce a page that says so, naming the prerequisite, rather than a page that
is silently thin -- and mixing the two sources in one module makes that distinction easy to lose.

The ranking page is the one to read first. An index of 3,056 symbols with no usage signal makes
every symbol look equally likely; `most-used.md` is what turns it into something an agent can
triage, and the counts come from the checker rather than from a name search.
"""

from __future__ import annotations

from pathlib import Path

from model import Model

BLOCKED = """# {title}

**Not available at this build.** The batch analysis stage reported `{status}`.

{reason}

This page is written rather than omitted so the absence is legible: an index that simply lacked
the page would be indistinguishable from one whose question was never asked. Re-run
`python3 build/acquire.py` with the pinned checker available, then rebuild.
"""


def _blocked(content: Path, name: str, title: str, record: dict) -> int:
    reason = record.get("reason") or "No reason was recorded."
    content.joinpath("catalogs", name).write_text(
        BLOCKED.format(title=title, status=record.get("status", "not_run"), reason=reason)
    )
    return 0


def _rows(path: Path) -> list[list[str]]:
    if not path.is_file():
        return []
    return [line.split("\t") for line in path.read_text().splitlines() if line]


def write_cross_references(built: Model, content: Path, record: dict) -> int:
    """Who calls this, and where is it referenced -- the limit this whole stage exists to close."""
    if record.get("status") != "passed":
        return _blocked(content, "cross-references.md", "Cross-references and call graph", record)

    callers = _rows(content / "index" / "callers.tsv")
    references = _rows(content / "index" / "references.tsv")
    usage = _rows(content / "index" / "usage.tsv")
    external = _rows(content / "index" / "external-refs.tsv")

    ranked = sorted(usage, key=lambda row: -int(row[3]))[:25]
    lines = [
        "# Cross-references and the call graph",
        "",
        "`reference.md` used to say this was unanswerable. It is answerable, from the checker's",
        "batch reports rather than its language server -- one run over 388 files, not a loop over",
        "3,056 symbols.",
        "",
        f"{len(callers):,} call edges · {len(references):,} reference rows · "
        f"{len(external):,} external targets.",
        "",
        "## Who calls this?",
        "",
        "```bash",
        "rg -P '^fastmcp\\.server\\.server\\.FastMCP\\t' content/index/callers.tsv | cut -f2,3,4",
        "```",
        "",
        "Column 1 is the callee, column 2 the caller, then the file and line. A caller reading",
        "`… (module level)` is import-time code, `… (decorator)` a decorator expression, and",
        "`… (class body)` a class-level statement -- all three are real call sites that no",
        "function name would describe.",
        "",
        "A constructor call is recorded against the **class**, not `__init__`: the question is",
        "who builds a `FastMCP`, not who calls `object.__init__`.",
        "",
        "## Where is this referenced?",
        "",
        "```bash",
        "rg -P '^fastmcp\\.server\\.context\\.Context\\t' content/index/references.tsv",
        "```",
        "",
        "One row per (target, file) with a count. References into typeshed and third-party code",
        "are separated into `external-refs.tsv` by the target's **defining file**, which the",
        "report carries -- not by guessing from the name.",
        "",
        "## The most-called items",
        "",
        "| Item | Callers | Referencing files | References |",
        "|---|---:|---:|---:|",
    ]
    for path, refs, files, n_callers, _callees in ranked:
        lines.append(f"| `{path}` | {n_callers} | {files} | {refs} |")
    lines += [
        "",
        "Full ranking in `content/index/usage.tsv`, which carries every item the checker saw",
        "referenced or called. An item absent from it was never referenced *within these three",
        "distributions* -- which is not the same as unused, because callers outside them are",
        "outside this index's scope.",
        "",
    ]
    content.joinpath("catalogs", "cross-references.md").write_text("\n".join(lines))
    return len(callers)


def write_most_used(built: Model, content: Path, record: dict) -> int:
    """The triage page: which of 3,056 symbols an agent will actually meet."""
    if record.get("status") != "passed":
        return _blocked(content, "most-used.md", "Most-used API", record)

    usage = _rows(content / "index" / "usage.tsv")
    ranked = sorted(usage, key=lambda row: -(int(row[1]) + 4 * int(row[3])))[:40]
    lines = [
        "# The API you will actually meet",
        "",
        "3,056 symbols is a flat list until something says which ones matter. This ranks by",
        "how often the library itself references and calls each item, weighting callers above",
        "bare references because a call is a stronger signal of a contract than a mention.",
        "",
        "It is a measurement of *this library's own use of itself*, not of what callers do.",
        "Something rare here can still be exactly what your code needs -- `create_proxy` is",
        "called eight times upstream and is the answer to a common question.",
        "",
        "| Item | Import as | Callers | References |",
        "|---|---|---:|---:|",
    ]
    for path, refs, _files, n_callers, _callees in ranked:
        item = built.items.get(path)
        spelling = f"`{item.preferred}`" if item and item.nameable else "*not importable*"
        lines.append(f"| `{path}` | {spelling} | {n_callers} | {refs} |")
    lines.append("")
    content.joinpath("catalogs", "most-used.md").write_text("\n".join(lines))
    return len(ranked)


def write_import_graph(built: Model, content: Path, record: dict) -> int:
    """The module dependency structure, resolved by the checker rather than scanned."""
    if record.get("status") != "passed":
        return _blocked(content, "import-graph.md", "Import graph", record)

    imports = _rows(content / "index" / "imports.tsv")
    fan_out: dict[str, int] = {}
    fan_in: dict[str, int] = {}
    for importer, imported in imports:
        fan_out[importer] = fan_out.get(importer, 0) + 1
        fan_in[imported] = fan_in.get(imported, 0) + 1
    hubs = sorted(fan_in.items(), key=lambda pair: -pair[1])[:15]

    lines = [
        "# The import graph",
        "",
        f"{len(imports):,} edges between {len(set(fan_out) | set(fan_in))} files, resolved by the",
        "type checker. This is **file-level**: keys and values are paths relative to",
        "site-packages, and there is no module-name representation, so joining it to anything",
        "that speaks dotted names is your job.",
        "",
        "It is not a call graph. An import is not a call, and `cross-references.md` is where",
        "call edges live.",
        "",
        "## Most depended upon",
        "",
        "| File | Imported by |",
        "|---|---:|",
    ]
    lines += [f"| `{path}` | {count} |" for path, count in hubs]
    lines += [
        "",
        "## Recipes",
        "",
        "```bash",
        "# what does this file import?",
        "rg -P '^fastmcp/server/server\\.py\\t' content/index/imports.tsv | cut -f2",
        "# what imports it?",
        "rg -P '\\tfastmcp/server/server\\.py$' content/index/imports.tsv | cut -f1",
        "```",
        "",
    ]
    content.joinpath("catalogs", "import-graph.md").write_text("\n".join(lines))
    return len(imports)


def write_typing_health(built: Model, content: Path, record: dict) -> int:
    """How well typed the library is, per symbol, and where it suppressed."""
    if record.get("status") != "passed":
        return _blocked(content, "typing-health.md", "Typing health", record)

    coverage = _rows(content / "index" / "type-coverage.tsv")
    suppressions = _rows(content / "index" / "suppressions.tsv")
    summary = record.get("summary") or {}
    weakest = sorted(coverage, key=lambda row: -(int(row[5]) + int(row[6])))[:20]

    lines = [
        "# Typing health",
        "",
        "From the checker's own coverage report, over public symbols only -- names without a",
        "leading underscore, plus anything an `__all__` exports, followed through re-export",
        "chains. That last part matters here: this library re-exports heavily, and a report over",
        "raw modules would describe a different surface from the one a caller sees.",
        "",
        f"**{summary.get('coverage', 0):.1f}% covered**, {summary.get('strict_coverage', 0):.1f}%"
        f" strictly, over {summary.get('n_typable', 0)} typable positions in"
        f" {summary.get('n_modules', 0)} modules.",
        "",
        f"{summary.get('n_typed', 0)} typed · {summary.get('n_any', 0)} `Any` ·"
        f" {summary.get('n_untyped', 0)} untyped · {len(suppressions)} suppressions.",
        "",
        "`Any` and untyped are different failures. An `Any` is a type the checker resolved to",
        "the top type -- it will not catch a mistake there. An untyped position has no",
        "annotation at all, and `index/inferred.tsv` may still carry an inferred answer for it.",
        "",
        "## Where the types are weakest",
        "",
        "| Symbol | Kind | Any | Untyped |",
        "|---|---|---:|---:|",
    ]
    for name, kind, _line, _typable, _typed, any_count, untyped in weakest:
        lines.append(f"| `{name}` | {kind} | {any_count} | {untyped} |")
    lines += [
        "",
        "## Suppressions",
        "",
        f"{len(suppressions)} in the indexed distributions. Each one is a place upstream decided",
        "the checker was wrong or the cost was too high, which makes them the best available map",
        "of where this library's typing is genuinely hard.",
        "",
        "```bash",
        "cut -f4 content/index/suppressions.tsv | tr ',' '\\n' | sort | uniq -c | sort -rn",
        "```",
        "",
    ]
    content.joinpath("catalogs", "typing-health.md").write_text("\n".join(lines))
    return len(coverage)


def write_protocols(built: Model, content: Path, record: dict) -> int:
    """Which classes actually satisfy which Protocol -- adjudicated, not matched.

    `reference.md` limit 1 refused to ship this table, and was right to: built by comparing
    member names it would have been the most dangerous page here, confidently wrong wherever a
    signature differed. The names now only choose the questions; every verdict comes from the
    checker, and a rejection carries the member that caused it.
    """
    if record.get("status") != "passed":
        return _blocked(content, "protocols.md", "Protocol conformance", record)

    rows = _rows(content / "index" / "satisfies.tsv")
    by_protocol: dict[str, list[list[str]]] = {}
    for row in rows:
        by_protocol.setdefault(row[1], []).append(row)

    satisfied = sum(1 for row in rows if row[2] == "yes")
    lines = [
        "# Protocol conformance",
        "",
        "Structural conformance is not decidable by reading member names, which is why this",
        "index refused to guess at it. It *is* decidable by asking the type checker, so each row",
        "below is a probe it adjudicated:",
        "",
        "```python",
        "def _p(c: SomeClass) -> SomeProtocol:",
        "    return c",
        "```",
        "",
        "No diagnostic means assignable. A `bad-return` names the obligation that failed.",
        "",
        f"{satisfied} of {len(rows)} probed pairs are assignable, across",
        f"{len(by_protocol)} protocols.",
        "",
        "**What this does not say.** Only classes whose public members are a superset of the",
        "protocol's were probed -- a class missing a member cannot satisfy it, so asking would",
        "waste a probe, but it also means absence from this table is not a verdict. And the",
        "conformance suite records that this checker deviates on **variance**",
        "(`protocols_variance.py`, and the ParamSpec and TypeVarTuple variance tests), so a",
        "verdict that turns on variance is worth confirming rather than trusting.",
        "",
    ]
    for protocol in sorted(by_protocol):
        entries = sorted(by_protocol[protocol])
        yes = [row for row in entries if row[2] == "yes"]
        no = [row for row in entries if row[2] != "yes"]
        item = built.items.get(protocol)
        spelling = f"`{item.preferred}`" if item and item.nameable else f"`{protocol}`"
        lines += [f"## {protocol.rsplit('.', 1)[-1]}", "", f"Import as {spelling}.", ""]
        if yes:
            lines += [f"Satisfied by {len(yes)}:", ""]
            lines += [f"- `{row[0]}`" for row in yes[:12]]
            if len(yes) > 12:
                lines.append(f"- … and {len(yes) - 12} more")
            lines.append("")
        if no:
            lines += ["Near misses, with the obligation that failed:", ""]
            lines += [f"- `{row[0]}` — {row[3]}" for row in no[:6]]
            lines.append("")
    content.joinpath("catalogs", "protocols.md").write_text("\n".join(lines))
    return len(rows)


def annotate_with_usage(content: Path) -> int:
    """Append a usage line to the catalogs that list types without saying which matter.

    `middleware.md` names twelve hooks as equals. Eleven of the seventeen built-in middleware
    override `on_message` and one overrides `on_list_resource_templates`, and a reader choosing
    where to start cannot tell from the list. The counts already exist; this is the join.
    """
    usage = {}
    table = content / "index" / "usage.tsv"
    if table.is_file():
        for line in table.read_text().splitlines():
            row = line.split("\t")
            if len(row) >= 4:
                usage[row[0]] = (int(row[1]), int(row[3]))

    touched = 0
    for name in ("middleware.md", "extension-points.md", "transports.md", "auth-providers.md"):
        page = content / "catalogs" / name
        if not page.is_file():
            continue
        text = page.read_text()
        if "Referenced" in text:
            continue
        ranked = []
        for path, (references, callers) in usage.items():
            leaf = path.rsplit(".", 1)[-1]
            if f"`{leaf}`" in text or f".{leaf}`" in text:
                ranked.append((callers, references, path))
        if not ranked:
            continue
        ranked.sort(reverse=True)
        lines = [
            "",
            "## Referenced how often",
            "",
            "From `index/usage.tsv` — how much this library uses each of these itself. A low",
            "count is not a warning: it measures internal use, not what callers need.",
            "",
            "| Item | Callers | References |",
            "|---|---:|---:|",
        ]
        lines += [
            f"| `{path}` | {callers} | {references} |" for callers, references, path in ranked[:12]
        ]
        page.write_text(text.rstrip() + "\n" + "\n".join(lines) + "\n")
        touched += 1
    return touched


def write_all(built: Model, content: Path, record: dict) -> dict[str, int]:
    return {
        "cross_references": write_cross_references(built, content, record),
        "most_used": write_most_used(built, content, record),
        "import_graph": write_import_graph(built, content, record),
        "typing_health": write_typing_health(built, content, record),
        "protocols": write_protocols(built, content, record),
    }

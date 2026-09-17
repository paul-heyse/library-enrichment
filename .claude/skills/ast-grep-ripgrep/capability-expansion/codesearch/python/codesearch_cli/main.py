"""The Typer command surface.

Every command shells out to ``codesearch-query`` with ``--json`` and renders the result. The
``--json`` path prints the binary's bytes straight through and **never constructs a Console**,
because a Console is exactly what deletes ``[a-z]`` from a pattern (probe P01).

Rich owns all rendering; Typer owns only argument parsing. ``typer.progressbar`` is the vendored
Click one and fights a live Console, and Typer's own prompts bypass the Console, so neither is
used here.
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Annotated, Any

import typer

from codesearch_cli import render

app = typer.Typer(
    name="codesearch",
    help="Query a machine-readable catalog of code-search capabilities.",
    no_args_is_help=True,
    add_completion=False,
)

BINARY = "codesearch-query"

# Set by the app callback, read by `_run`. A module-level dict rather than a parameter on all
# twelve commands: `--include-partial` is a property of the REQUEST, not of any one question, and
# repeating it per command is how the twelfth one ends up spelling it differently.
_REQUEST: dict[str, bool | str] = {"include_partial": False, "snapshot": ""}


@app.callback()
def _options(
    include_partial: Annotated[
        bool,
        typer.Option(
            "--include-partial",
            help="Also serve rows written by runs that did not complete.",
        ),
    ] = False,
    snapshot: Annotated[
        str,
        typer.Option(
            "--snapshot",
            help="Which snapshot to answer about. Defaults to the newest with a visible run.",
        ),
    ] = "",
) -> None:
    """Options that describe the request rather than the question."""
    _REQUEST["include_partial"] = include_partial
    _REQUEST["snapshot"] = snapshot


def _binary() -> str:
    """Locate the query binary, preferring an explicit override."""
    explicit = os.environ.get("CODESEARCH_QUERY_BIN")
    if explicit:
        return explicit
    found = shutil.which(BINARY)
    if found:
        return found
    # The usual development location, relative to this package.
    local = Path(__file__).resolve().parents[2] / "target/debug" / BINARY
    if local.exists():
        return str(local)
    raise typer.BadParameter(
        f"cannot find `{BINARY}`. Build it with `just build`, or set CODESEARCH_QUERY_BIN."
    )


def _run(args: list[str], *, home: Path | None) -> list[dict[str, Any]]:
    """Invoke the query binary and parse its JSON.

    A non-zero exit is surfaced with the binary's own stderr rather than being reshaped: the Rust
    side has the context, and paraphrasing it here would lose it.
    """
    command = [_binary()]
    if home is not None:
        command += ["--home", str(home)]
    if _REQUEST["include_partial"]:
        command.append("--include-partial")
    if _REQUEST["snapshot"]:
        command += ["--snapshot", str(_REQUEST["snapshot"])]
    command += ["--json", *args]
    completed = subprocess.run(command, capture_output=True, text=True, check=False)
    if completed.returncode != 0:
        sys.stderr.write(completed.stderr)
        raise typer.Exit(code=completed.returncode)
    payload = completed.stdout.strip()
    if not payload:
        return []
    parsed = json.loads(payload)
    return parsed if isinstance(parsed, list) else [parsed]


def _emit(rows: list[dict[str, Any]], *, as_json: bool, title: str) -> None:
    """Render rows, or print them as JSON.

    The JSON branch writes bytes and returns. It must not touch a Console: machine output that has
    been through markup parsing is silently wrong, and a caller has no way to tell.
    """
    if as_json:
        sys.stdout.write(json.dumps(rows, indent=2) + "\n")
        return
    console = render.console()
    if not rows:
        # An empty result is a real answer and says so, rather than printing nothing and letting
        # the caller wonder whether the query ran.
        console.print(f"no rows for {render.safe(title)}", markup=False)
        return
    console.print(render.rows_table(rows, title=title))


HomeOption = Annotated[
    Path | None,
    typer.Option("--home", envvar="CODESEARCH_HOME", help="Where the catalog tables live."),
]
JsonOption = Annotated[bool, typer.Option("--json", help="Emit JSON instead of a table.")]


@app.command()
def discover(
    tool: Annotated[str | None, typer.Option(help="Restrict to one tool.")] = None,
    surface: Annotated[str | None, typer.Option(help="cli, rule or pattern.")] = None,
    capability: Annotated[str | None, typer.Option(help="Restrict to an ability.")] = None,
    min_observed: Annotated[
        str | None,
        typer.Option(help="Evidence floor: not-probed, unknown, recorded or confirmed."),
    ] = None,
    subject: Annotated[
        str | None, typer.Option(help="What it acts on: text, source-code, paths.")
    ] = None,
    relationship: Annotated[
        str | None,
        typer.Option(help="containment, equality, ordering, identity or exclusion."),
    ] = None,
    result: Annotated[
        str | None,
        typer.Option(help="nodes, matching-text, paths, structured-records, rewritten-text."),
    ] = None,
    semantic_layer: Annotated[
        str | None, typer.Option("--semantic-layer", help="lexical or syntactic.")
    ] = None,
    scope: Annotated[
        str | None, typer.Option(help="A file type the tool can be pointed at, e.g. rust.")
    ] = None,
    language: Annotated[
        str | None, typer.Option(help="A language the tool can parse, e.g. rust.")
    ] = None,
    budget: Annotated[
        int | None,
        typer.Option(help="How many CANDIDATES to return. Never fewer columns of one."),
    ] = None,
    limit: Annotated[int, typer.Option(help="Maximum candidates.")] = 20,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Find mechanisms matching a tool, surface, capability or retrieval facet.

    The six facets are §11's decision-packet surface: what the capability acts on, the relationship
    it expresses, what it returns, which layer it works at, and what the tool can be pointed at.
    """
    args = ["discover", "--limit", str(limit)]
    if budget is not None:
        args += ["--budget", str(budget)]
    for flag, value in (
        ("--tool", tool),
        ("--surface", surface),
        ("--capability", capability),
        ("--min-observed", min_observed),
        ("--subject", subject),
        ("--relationship", relationship),
        ("--result", result),
        ("--semantic-layer", semantic_layer),
        ("--scope", scope),
        ("--language", language),
    ):
        if value is not None:
            args += [flag, value]
    _emit(_run(args, home=home), as_json=as_json, title="mechanisms")


@app.command()
def describe(
    entity_key: Annotated[
        list[str], typer.Argument(help="One or more keys, e.g. mech:rg/cli/--pcre2.")
    ],
    field: Annotated[
        list[str] | None, typer.Option(help="Show only these columns. Repeatable.")
    ] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Show what is known about one or more mechanisms."""
    args = ["describe", *entity_key]
    for f in field or []:
        args += ["--field", f]
    rows = _run(args, home=home)
    title = ", ".join(entity_key)
    if as_json:
        _emit(rows, as_json=True, title=title)
        return
    _emit(rows, as_json=False, title=title)
    # The invocation form is catalog content and can contain brackets, so it goes through the
    # markup-safe renderable rather than through Console.print.
    for row in rows:
        form = row.get("invocation_form")
        if form:
            render.console().print(render.pattern(str(form)))


@app.command()
def compare(
    from_snapshot: Annotated[str, typer.Option("--from", help="The snapshot to compare from.")],
    to_snapshot: Annotated[str, typer.Option("--to", help="The snapshot to compare to.")],
    entity_key: Annotated[
        str | None,
        typer.Argument(help="Restrict to one entity. Omit to see every difference."),
    ] = None,
    limit: Annotated[int, typer.Option(help="Most rows to show.")] = 100,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """What changed between two snapshots.

    Differences, not a compatibility verdict. Three kinds -- added, removed, changed -- and no
    score over them, for the same reason coverage reports three numbers and never a total.
    """
    args = ["compare", "--from", from_snapshot, "--to", to_snapshot, "--limit", str(limit)]
    if entity_key is not None:
        args.append(entity_key)
    rows = _run(args, home=home)
    _emit(rows, as_json=as_json, title=f"{from_snapshot} -> {to_snapshot}")


@app.command()
def capabilities(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """List the abstract abilities and how many mechanisms each binds."""
    _emit(_run(["capabilities"], home=home), as_json=as_json, title="capabilities")


@app.command()
def coverage(
    tool: Annotated[str | None, typer.Option(help="Report only this tool.")] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Report three independent completeness numbers. Never a composite score.

    All three are shown, as three tables, because they answer different questions: how much of the
    tool is catalogued at all, how much of that rests on something actually run, and how much of
    the tool's own code a mechanism can point at. A single number would have to weigh them against
    one another, and there is no honest weight.
    """
    scope = ["--tool", tool] if tool else []
    surface = _run(["coverage", "--dimension", "surface", *scope], home=home)
    evidence = _run(["coverage", "--dimension", "evidence", *scope], home=home)
    # `api` has no tool dimension: it counts program rows against bound mechanisms, and the
    # program model is not partitioned by tool. Passing --tool there would silently do nothing.
    api = _run(["coverage", "--dimension", "api"], home=home)
    if as_json:
        # Keyed rather than concatenated: three lists of differently-shaped rows in one array would
        # invite a consumer to sum them.
        sys.stdout.write(
            json.dumps({"surface": surface, "evidence": evidence, "api": api}, indent=2) + "\n"
        )
        return
    _emit(surface, as_json=False, title="surface coverage -- how much is catalogued")
    _emit(evidence, as_json=False, title="evidence coverage -- how much was run")
    _emit(api, as_json=False, title="api coverage -- how much code a mechanism can name")


@app.command()
def validate(
    detail: Annotated[bool, typer.Option(help="List offending rows instead of counts.")] = False,
    invariant: Annotated[
        list[str] | None, typer.Option(help="Report only these invariants. Repeatable.")
    ] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Run the cross-row invariants, reporting what each one inspected.

    ``checked`` is not decoration. An invariant that examined no rows proved nothing, and printing
    it as ``0 violations`` beside one that examined a thousand would make the two look alike.
    """
    args = ["validate"]
    if detail:
        args.append("--detail")
    for name in invariant or []:
        args += ["--invariant", name]
    _emit(_run(args, home=home), as_json=as_json, title="invariants")


@app.command()
def projections(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """List the retrieval surface, read out of the catalog's own information_schema."""
    _emit(_run(["projections"], home=home), as_json=as_json, title="projections")


@app.command()
def negative_space(
    tool: Annotated[str | None, typer.Option(help="Restrict to one tool.")] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """What is known NOT to be reachable, why, and what to do instead.

    An empty search result and a known absence are different answers. This command returns the
    second, so "ripgrep cannot rewrite files in place, use ``ast-grep -U``" arrives as a fact
    rather than as nothing found.
    """
    args = ["negative-space"]
    if tool is not None:
        args += ["--tool", tool]
    _emit(_run(args, home=home), as_json=as_json, title="known absences")


@app.command()
def interactions(
    bind: Annotated[
        list[str] | None,
        typer.Option(help="Bind a request facet, e.g. --bind engine=pcre2. Repeatable."),
    ] = None,
    affecting: Annotated[str | None, typer.Option(help="Only this mechanism.")] = None,
    relevant: Annotated[bool, typer.Option(help="Hide the ones that evaluate to false.")] = False,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """How one option changes what another does, evaluated against what you have bound.

    A facet you have not bound evaluates to ``unknown``, not ``false``. That is the answer, not a
    gap in it: the interaction depends on a choice still open, and reporting it as inapplicable
    would hide the choice. Bind the facet to collapse it either way.
    """
    args = ["interactions"]
    for value in bind or []:
        args += ["--bind", value]
    if affecting is not None:
        args += ["--affecting", affecting]
    if relevant:
        args.append("--relevant")
    _emit(_run(args, home=home), as_json=as_json, title="interactions")


@app.command()
def fragments(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """The composable steps: what each accepts, produces, and still owes.

    ``unresolved_obligations`` is the column to read. A fragment marked ``heuristic`` says there
    what would have to be true for it to be recall-preserving, so the gap is a sentence you can
    check rather than a word you have to take on trust.
    """
    _emit(_run(["fragments"], home=home), as_json=as_json, title="plan fragments")


@app.command()
def chains(
    start_unit: Annotated[str, typer.Option(help="file_set, line_set, node_set ...")] = "file_set",
    max_depth: Annotated[int, typer.Option(help="Chain length bound.")] = 8,
    min_recall: Annotated[
        str | None, typer.Option(help="unknown, heuristic or preserving.")
    ] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Every chain from a starting unit, with its recall folded and its path shown.

    ``--max-depth`` is visible rather than hidden because the bound is real: fragment composition
    is cyclic in general -- ``rg -r`` takes a line set and produces one -- and probe PB09 measured
    an unbounded recursion over a cyclic graph failing to terminate.

    A chain is only as strong as its weakest step. ``rg -l`` narrowing before ``ast-grep -p``
    reports ``heuristic``, not ``preserving``, because the narrowing may drop files the pattern
    would have matched.
    """
    args = ["chains", "--start-unit", start_unit, "--max-depth", str(max_depth)]
    if min_recall is not None:
        args += ["--min-recall", min_recall]
    _emit(_run(args, home=home), as_json=as_json, title=f"chains from {start_unit}")


@app.command()
def implements(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """A mechanism and the Rust code that implements it, with the evidence for the claim.

    These rows are authored, not derived: nothing the skill ships links a flag to a crate. Read
    `precision` and `observed` on every row -- a crate the skill's own prose names is not the same
    kind of fact as a builder type inferred from its name.
    """
    _emit(_run(["implements"], home=home), as_json=as_json, title="mechanism implementations")


@app.command()
def closures(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """Each closure, its edge count, and what limits it.

    Read this before reading an empty `containment`. A closure that traversed nothing looks exactly
    like a closure that found nothing to traverse, and the difference matters: `containment` is
    empty because the index holds items and not modules, not because nothing contains anything.
    """
    _emit(_run(["closures"], home=home), as_json=as_json, title="closure queries")


@app.command()
def containment(
    root: Annotated[str | None, typer.Option(help="Restrict to one root canonical path.")] = None,
    limit: Annotated[int, typer.Option(help="Maximum rows.")] = 20,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """What a definition contains, transitively, with the path that reaches it.

    Empty against the shipped index. Run `closures` for why.
    """
    args = ["containment", "--limit", str(limit)]
    if root is not None:
        args += ["--root", root]
    _emit(_run(args, home=home), as_json=as_json, title="containment closure")


@app.command()
def re_exports(
    access_path: Annotated[str | None, typer.Option(help="Restrict to one access path.")] = None,
    limit: Annotated[int, typer.Option(help="Maximum rows.")] = 20,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Where an item can be named from, following re-exports to their origin."""
    args = ["re-exports", "--limit", str(limit)]
    if access_path is not None:
        args += ["--access-path", access_path]
    _emit(_run(args, home=home), as_json=as_json, title="re-export chains")


@app.command()
def runs(
    detail: Annotated[
        bool, typer.Option(help="Also show the interpretive scope of each run.")
    ] = False,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """Every extraction run, finished or not.

    This is the view that explains an absence: a projection that looks empty is usually a run that
    did not complete, and this is where that shows up.
    """
    args = ["runs", "--detail"] if detail else ["runs"]
    _emit(_run(args, home=home), as_json=as_json, title="extraction runs")


@app.command()
def result_contracts(home: HomeOption = None, as_json: JsonOption = False) -> None:
    """What each command's exit status means, and the trap in reading it across tools."""
    _emit(_run(["result-contracts"], home=home), as_json=as_json, title="exit statuses")


@app.command()
def domains(
    kind: Annotated[str | None, typer.Option(help="file_type or language.")] = None,
    home: HomeOption = None,
    as_json: JsonOption = False,
) -> None:
    """What each tool can be pointed at, by name."""
    args = ["domains"]
    if kind is not None:
        args += ["--kind", kind]
    _emit(_run(args, home=home), as_json=as_json, title="search domains")


def main() -> None:
    app()

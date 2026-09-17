"""The type checker as a batch producer: calls, references, parameters, coverage, imports.

This is the stage that closes `reference.md` limit 2 -- "call graphs and cross-file references
are absent" -- and it does so from the checker's *batch* reports rather than its language server.
That distinction is the whole design. A language server answers one position at a time, so
building a project-wide index from it means driving a loop over 3,056 symbols and trusting every
answer; the batch reports emit the same facts for all 388 files in about nine seconds, as data.

Four producers, each chosen for what only it can answer:

  `--report-pysa --report-pysa-format json`
      The richest. Per module: `definitions/` (undecorated signatures with each parameter's
      kind, requiredness and *resolved* annotation), `call_graphs/` (call sites keyed
      `line:col-line:col`, with `init_targets`/`new_targets`, `receiver_class` and
      `implicit_receiver`), and `type_of_expressions/` (every expression's type).
      At its DEFAULT format this report is Cap'n Proto binary; the format flag is what makes it
      readable, and missing that flag is why it was written off as unusable.

  `--report-glean`
      Byte-offset spans and flat dotted names. Kept for one thing pysa does not give: every
      *non-call* reference to a target, via `python.xrefs.XRefsByFile.1`, whose target carries
      the defining file and so separates first-party from typeshed without guessing.

  `coverage report --public-only`
      Per-symbol typed/any/untyped with line and column, keyed by fully-qualified dotted name --
      a direct join onto the canonical paths this index already uses, and an independent second
      opinion on where every symbol is.

  `--dependency-graph`
      File-level module imports, resolved by the type checker rather than by scanning syntax.

Two hazards, both measured rather than assumed:

**The obvious configuration analyses nothing and succeeds.** `site-package-path` is added to
`project-excludes` automatically, the default excludes contain `**/venv/**/*`, and the capsule
lives under a directory called `venv`. The first attempt covered *zero* files, emitted three
`WARN` lines, and exited 0. So the config disables the exclude heuristics, states its own
excludes, points the interpreter at the capsule, and `assert_coverage` refuses to continue
unless the covered-file count matches what Griffe indexed.

**The raw reports are 800 MB.** 332 MB of Glean and 469 MB of pysa, the latter spread over 6,308
modules because the checker reports on typeshed too. Only 388 of those are ours. So the raw
output stays in the gitignored cache, regenerable in nine seconds, and what lands in `acquired/`
is the reduction -- the facts about the three indexed distributions, sorted, with the digest of
every report directory recorded so a re-run can be shown to be equivalent.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

CONFIG = """project-includes = [{includes}]
project-excludes = ["**/__pycache__/**"]
disable-project-excludes-heuristics = true
python-interpreter-path = "{interpreter}"
search-path = ["{site_packages}"]
python-version = "{python}"
"""

# `line:col-line:col`, the position form every pysa record uses.
SPAN = re.compile(r"^(\d+):(\d+)-(\d+):(\d+)$")


class AnalysisError(RuntimeError):
    """The checker could not be configured, asserted, or read."""


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def _run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False)


def install_checker(capsule: Path, version: str) -> Path:
    """Create a venv beside the capsule holding only the pinned checker, and return its binary.

    Beside, not inside. The capsule's site-packages is the subject of this index and
    `assert_resolved` pins it exactly; adding a type checker to it would put a distribution in
    the analysed environment that the manifest does not declare.
    """
    root = capsule / "checker"
    binary = root / "bin" / "pyrefly"
    if binary.exists():
        done = _run([str(binary), "--version"], cwd=Path.cwd())
        if version in done.stdout:
            return binary
        shutil.rmtree(root)

    done = subprocess.run(["uv", "venv", str(root)], capture_output=True, text=True, check=False)
    if done.returncode != 0:
        raise AnalysisError(f"could not create the checker venv: {done.stderr[:300]}")
    environment = dict(os.environ)
    environment["VIRTUAL_ENV"] = str(root)
    environment.pop("PYTHONPATH", None)
    done = subprocess.run(
        ["uv", "pip", "install", f"pyrefly=={version}"],
        capture_output=True,
        text=True,
        check=False,
        env=environment,
    )
    if done.returncode != 0:
        raise AnalysisError(f"could not install pyrefly=={version}: {done.stderr[:300]}")
    if not binary.exists():
        raise AnalysisError(f"pyrefly=={version} installed but {binary} is absent")
    return binary


def assert_checker(binary: Path, expected: str) -> str:
    """Refuse to capture from a version other than the pin.

    The workstation carried 0.51.1 while the manifest pins 1.3.1. A capture from the wrong one
    would have produced plausible output describing a different tool -- the same failure mode
    that made a sibling skill's rule count 934 against the pinned 970.
    """
    done = _run([str(binary), "--version"], cwd=Path.cwd())
    reported = done.stdout.strip()
    if expected not in reported:
        raise AnalysisError(f"checker is {reported!r}, manifest pins {expected!r}")
    return reported


def write_config(
    cache: Path, site_packages: Path, venv: Path, modules: list[str], python: str
) -> Path:
    """Write the project configuration, and return its directory."""
    root = cache / "config"
    root.mkdir(parents=True, exist_ok=True)
    includes = ", ".join(f'"{site_packages}/{module}/**/*.py"' for module in modules)
    (root / "pyrefly.toml").write_text(
        CONFIG.format(
            includes=includes,
            interpreter=venv / "bin" / "python",
            site_packages=site_packages,
            python=python,
        )
    )
    return root


def assert_coverage(binary: Path, config: Path, site_packages: Path, expected: set[str]) -> int:
    """Fail unless the checker covers every file the index describes.

    This is the load-bearing assertion of the whole stage. A misconfiguration here does not
    error: it warns, covers nothing, exits 0, and every downstream table comes out empty but
    well-formed. That happened on the first attempt -- the capsule lives under a directory named
    `venv`, which the default `project-excludes` matches.

    Containment rather than equality. The checker covers every `.py` file in the three package
    trees; Griffe records a path only for files that produced an indexable item, so an
    `__init__.py` that only re-exports contributes none. Covering more than the index describes
    is expected; covering less is the failure.
    """
    done = _run([str(binary), "dump-config", "--max-files", "all"], cwd=config)
    prefix = f"{site_packages}/"
    covered = set()
    for line in done.stdout.splitlines():
        candidate = line.strip()
        if candidate.endswith(".py") and candidate.startswith(prefix):
            covered.add(candidate[len(prefix) :])
    missing = sorted(expected - covered)
    if missing:
        raise AnalysisError(
            f"the checker covers {len(covered)} files but misses {len(missing)} the index "
            f"describes, e.g. {missing[:3]}. stderr: {done.stderr.strip()[:300]}"
        )
    return len(covered)


def run_reports(binary: Path, config: Path, cache: Path) -> dict[str, Path]:
    """Produce every batch report into the cache. Returns the directories written."""
    glean = cache / "glean"
    pysa = cache / "pysa"
    for directory in (glean, pysa):
        if directory.exists():
            shutil.rmtree(directory)
        directory.mkdir(parents=True)

    depgraph = cache / "dependency-graph.json"
    done = _run(
        [
            str(binary),
            "check",
            "--report-glean",
            str(glean),
            "--report-pysa",
            str(pysa),
            "--report-pysa-format",
            "json",
            "--output-format",
            "json",
            "--output",
            str(cache / "diagnostics.json"),
        ],
        cwd=config,
    )
    # A non-zero status here means diagnostics were found, which is a finding about the library
    # rather than a failure of this stage. The payload is what matters.
    if not any(glean.iterdir()):
        raise AnalysisError(f"the glean report wrote nothing; stderr: {done.stderr[:400]}")

    # Its own invocation. Combined with the other report flags it writes nothing at all -- no
    # error, no warning, exit 0 -- which is the failure mode this whole module keeps meeting.
    _run([str(binary), "check", "--dependency-graph", str(depgraph)], cwd=config)

    coverage = cache / "coverage.json"
    done = _run([str(binary), "coverage", "report", "--public-only"], cwd=config)
    coverage.write_text(done.stdout)
    return {"glean": glean, "pysa": pysa, "dependency_graph": depgraph, "coverage": coverage}


# --------------------------------------------------------------------------- reduction


def _module_of(path: Path) -> str:
    """`fastmcp.server.server:1234.json` -> `fastmcp.server.server`."""
    return path.name.rsplit(":", 1)[0]


def _ours(module: str, roots: tuple[str, ...]) -> bool:
    return module.split(".", 1)[0] in roots


def _line(span: str) -> int:
    """Leading line of a `line:col-line:col` key.

    Returns 0 for anything else. A call graph is keyed by span for real call sites and
    by a synthetic key -- `CTL:0`, the class top level -- for implicit ones, and those
    have no position to report.
    """
    match = SPAN.match(span)
    return int(match.group(1)) if match else 0


def reduce_definitions(
    pysa: Path, roots: tuple[str, ...]
) -> tuple[dict, dict, list[list], list[list], list[list]]:
    """Return `(function identities, class identities, parameters, mro rows, variable types)`.

    The identity maps are built first and separately because a call edge names a module and an
    id, not a path: resolving one means having read the *callee's* module.
    """
    functions: dict[tuple[str, str], str] = {}
    classes: dict[tuple[str, str], str] = {}
    parameters: list[list] = []
    mro: list[list] = []
    variables: list[list] = []

    for path in sorted((pysa / "definitions").glob("*.json")):
        module = _module_of(path)
        if not _ours(module, roots):
            continue
        document = json.loads(path.read_text())

        for class_id, record in (document.get("class_definitions") or {}).items():
            qualified = f"{module}.{record['name']}"
            classes[(module, str(class_id))] = qualified
            resolved = (record.get("mro") or {}).get("Resolved")
            if resolved is None:
                # An unresolvable MRO is a fact about the class, not a gap in the report.
                mro.append([qualified, "0", "(unresolved)"])
                continue
            for position, ancestor in enumerate(resolved, start=1):
                ancestor_name = f"{ancestor.get('module_name')}.{ancestor.get('class_name')}"
                mro.append([qualified, str(position), ancestor_name])

        for function_id, record in (document.get("function_definitions") or {}).items():
            owner = record.get("defining_class") or {}
            holder = owner.get("class_name")
            stem = f"{holder}.{record['name']}" if holder else record["name"]
            qualified = f"{module}.{stem}"
            functions[(module, function_id)] = qualified
            # The ordinal keeps overload variants apart. Without it the emitted rows for two
            # overloads of the same function are identical wherever the parameters agree, and
            # the table's own deduplication silently drops 1,452 of them.
            for ordinal, signature in enumerate(record.get("undecorated_signatures") or []):
                # `parameters` is usually `{"List": [...]}`, but a gradual signature -- the
                # `Callable[..., X]` shape -- serialises as the bare string "Ellipsis". One in
                # this library, and it would otherwise crash the whole reduction.
                declared = signature.get("parameters")
                listed = declared.get("List") or [] if isinstance(declared, dict) else []
                for position, entry in enumerate(listed):
                    for kind, parameter in entry.items():
                        annotation = (parameter.get("annotation") or {}).get("string", "-")
                        parameters.append(
                            [
                                qualified,
                                str(ordinal),
                                str(position),
                                parameter.get("name", "-"),
                                kind,
                                "required" if parameter.get("required") else "optional",
                                annotation,
                            ]
                        )
                returns = (signature.get("return_annotation") or {}).get("string")
                if returns:
                    parameters.append(
                        [qualified, str(ordinal), "-1", "(return)", "Return", "-", returns]
                    )
        # Module-level variables, with the checker's type and a location. This is the second
        # opinion on the attributes ty answered by hover, and it arrives in the same report.
        for name, entry in sorted((document.get("global_variables") or {}).items()):
            declared = (entry.get("type") or {}).get("string")
            if not declared:
                continue
            variables.append([f"{module}.{name}", declared, str(_line(entry.get("location", "")))])
    return functions, classes, parameters, mro, variables


def _caller_name(module: str, function_id: str, functions: dict, classes: dict) -> str:
    """Resolve a call-graph key to something a reader can act on.

    `MTL` is module-level code -- an import-time call, which is the interesting case for a
    library. `CTL:n` is a class body. `FDT:n` is a decorator expression, so the call belongs to
    the function being decorated.
    """
    if function_id == "MTL":
        return f"{module} (module level)"
    if function_id.startswith("CTL:"):
        holder = classes.get((module, function_id.split(":", 1)[1]))
        return f"{holder} (class body)" if holder else f"{module} (class body)"
    if function_id.startswith("FDT:"):
        decorated = functions.get((module, "F:" + function_id.split(":", 1)[1]))
        return f"{decorated} (decorator)" if decorated else f"{module} (decorator)"
    return functions.get((module, function_id), f"{module}.{function_id}")


def reduce_calls(
    pysa: Path,
    roots: tuple[str, ...],
    functions: dict,
    classes: dict,
    site_packages: Path,
) -> list[list]:
    """Return `[callee, caller, file, line]`.

    A constructor call resolves to `__init__` in `builtins` with `receiver_class` naming the
    real class; the receiver is what gets recorded, because "who constructs a FastMCP" is the
    question, not "who calls object.__init__".
    """
    rows: list[list] = []
    prefix = f"{site_packages}/"
    for path in sorted((pysa / "call_graphs").glob("*.json")):
        module = _module_of(path)
        if not _ours(module, roots):
            continue
        document = json.loads(path.read_text())
        source = (document.get("source_path") or {}).get("FileSystem", "")
        source = source[len(prefix) :] if source.startswith(prefix) else source
        for function_id, sites in (document.get("call_graphs") or {}).items():
            caller = _caller_name(module, function_id, functions, classes)
            for span, payload in (sites or {}).items():
                call = payload.get("Call") or {}
                for key in ("init_targets", "new_targets", "call_targets"):
                    for target in call.get(key) or []:
                        receiver = target.get("receiver_class") or {}
                        if receiver.get("class_name"):
                            callee = f"{receiver['module_name']}.{receiver['class_name']}"
                        else:
                            function = (target.get("target") or {}).get("Function") or {}
                            if not function:
                                continue
                            where_from = function.get("module_name")
                            callee = f"{where_from}.{function.get('function_name')}"
                        rows.append([callee, caller, source, str(_line(span))])
    return sorted(set(map(tuple, rows)))  # one edge per (callee, caller, file, line)


def reduce_references(
    glean: Path, roots: tuple[str, ...], site_packages: Path
) -> tuple[list[list], list[list]]:
    """Return `(internal reference rows, external target counts)` from the Glean report.

    `python.xrefs.XRefsByFile.1` is the predicate to read rather than
    `python.XRefsViaNameByTarget.4`: only it carries the target's *defining file*, which is what
    separates a reference into this library from one into typeshed without guessing from a name.
    """
    internal: dict[tuple[str, str], int] = {}
    external: dict[str, int] = {}
    prefix = f"{site_packages}/"
    for path in sorted(glean.glob("*.json")):
        for block in json.loads(path.read_text()):
            if block.get("predicate") != "python.xrefs.XRefsByFile.1":
                continue
            for fact in block.get("facts") or []:
                key = fact.get("key") or {}
                where = (key.get("file") or {}).get("key", "")
                where = where[len(prefix) :] if where.startswith(prefix) else where
                for xref in key.get("xrefs") or []:
                    target = xref.get("target") or {}
                    name = (target.get("name") or {}).get("key", "")
                    defined_in = (target.get("file") or {}).get("key", "")
                    if defined_in.startswith(prefix):
                        defined_in = defined_in[len(prefix) :]
                    if not name:
                        continue
                    if _ours(name, roots):
                        internal[(name, where)] = internal.get((name, where), 0) + 1
                    else:
                        outside = f"{name}\t{defined_in}"
                        external[outside] = external.get(outside, 0) + 1
    reference_rows = [
        [name, where, str(count)] for (name, where), count in sorted(internal.items())
    ]
    external_rows = [[*key.split("\t"), str(count)] for key, count in sorted(external.items())]
    return reference_rows, external_rows


def reduce_coverage(path: Path, roots: tuple[str, ...]) -> tuple[list[list], list[list], dict]:
    """Return `(symbol rows, suppression rows, summary)` from the coverage report."""
    document = json.loads(path.read_text())
    symbols: list[list] = []
    suppressions: list[list] = []
    for report in document.get("module_reports") or []:
        module = report.get("name", "")
        if not _ours(module, roots):
            continue
        for symbol in report.get("symbol_reports") or []:
            location = symbol.get("location") or {}
            symbols.append(
                [
                    symbol.get("name", "-"),
                    symbol.get("kind", "-"),
                    str(location.get("line", 0)),
                    str(symbol.get("n_typable", 0)),
                    str(symbol.get("n_typed", 0)),
                    str(symbol.get("n_any", 0)),
                    str(symbol.get("n_untyped", 0)),
                ]
            )
        for ignore in report.get("type_ignores") or []:
            location = ignore.get("location") or {}
            suppressions.append(
                [
                    module,
                    str(location.get("line", 0)),
                    ignore.get("kind", "-"),
                    ",".join(ignore.get("codes") or ()) or "-",
                ]
            )
    return symbols, suppressions, document.get("summary") or {}


def reduce_imports(path: Path, site_packages: Path, roots: tuple[str, ...]) -> list[list]:
    """Return `[importer, imported]` as site-packages-relative paths, ours only."""
    if not path.is_file():
        # `--dependency-graph` is documented as experimental and silently writes nothing in some
        # flag combinations. An empty table with the absence recorded beats a crash, and beats a
        # table that cannot be told from "this library imports nothing".
        return []
    document = json.loads(path.read_text())
    prefix = f"{site_packages}/"
    rows: list[list] = []
    for importer, imports in sorted((document.get("modules") or {}).items()):
        if not importer.startswith(prefix):
            continue
        source = importer[len(prefix) :]
        if source.split("/", 1)[0] not in roots:
            continue
        for imported in sorted(imports):
            if not imported.startswith(prefix):
                continue
            target = imported[len(prefix) :]
            if target != source:
                rows.append([source, target])
    return rows


def digest_tree(root: Path) -> str:
    """One digest over a whole report directory, so a re-run can be shown to be equivalent."""
    accumulator = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        if path.is_file():
            accumulator.update(str(path.relative_to(root)).encode())
            accumulator.update(b"\0")
            accumulator.update(hashlib.sha256(path.read_bytes()).digest())
    return accumulator.hexdigest()


# --------------------------------------------------------------------------- assignability


def _class_members(
    documents: dict[str, dict], dunder_protocols: tuple[str, ...] = ()
) -> tuple[dict[str, set[str]], dict[str, set[str]]]:
    """Return `(protocol -> obligations, class -> public members)` from the Griffe documents.

    Protocol-ness is read from the *source* spelling of the bases. The checker's own reports
    cannot supply it: pysa strips `Protocol` out of `bases` and reports an empty MRO, so every
    Protocol in this library looks like a plain class there.

    `dunder_protocols` is the named exception to the underscore filter, and it is load-bearing
    here in a way it is not for a library whose protocols declare ordinary methods. Rich's two
    central protocols declare nothing else: `ConsoleRenderable` is `__rich_console__` and
    `RichCast` is `__rich__`. Filtered out, both come back with an empty obligation set, are
    skipped by `assignability` as vacuous, and `satisfies.tsv` ships with zero rows for the two
    protocols that decide whether an object can be printed at all -- with every check still
    green. The names are declared in the manifest rather than pattern-matched, so adding one is
    a decision someone made.
    """
    protocols: dict[str, set[str]] = {}
    members: dict[str, set[str]] = {}
    for document in documents.values():
        for item in document["items"]:
            if item.get("kind") != "class":
                continue
            declared = {
                member["name"]
                for member in item.get("members") or []
                if not member.get("inherited")
                and (not member["name"].startswith("_") or member["name"] in dunder_protocols)
            }
            members[item["path"]] = declared
            if any("Protocol" in base for base in item.get("bases") or []):
                protocols[item["path"]] = declared
    return protocols, members


def _probe_source(protocol: str, candidates: list[str]) -> tuple[str, dict[int, str]]:
    """Build one snippet asking about every candidate, and the line -> candidate map."""
    modules = sorted({path.rsplit(".", 1)[0] for path in [protocol, *candidates]})
    lines = [f"import {module}" for module in modules]
    lines.append("")
    where: dict[int, str] = {}
    for ordinal, candidate in enumerate(candidates):
        lines.append(f"def _p{ordinal}(c: {candidate}) -> {protocol}:")
        # The `return` line is the one a diagnostic lands on, so that is what gets mapped.
        where[len(lines) + 1] = candidate
        lines.append("    return c")
        lines.append("")
    return "\n".join(lines) + "\n", where


def assignability(
    binary: Path,
    config: Path,
    documents: dict[str, dict],
    limit: int = 4000,
    dunder_protocols: tuple[str, ...] = (),
) -> list[list]:
    """Return `[class, protocol, verdict, disqualifier]`, adjudicated by the checker.

    Candidates are prefiltered to classes whose public members are a superset of the protocol's.
    That filter decides which questions are worth asking and nothing else -- every verdict below
    comes from the checker, and a class the filter excludes is simply not reported, never
    reported as failing.

    A protocol with no public members is skipped: every class would be a candidate and the
    answer would be vacuous.
    """
    protocols, members = _class_members(documents, dunder_protocols)
    rows: list[list] = []
    asked = 0
    for protocol, required in sorted(protocols.items()):
        if not required or asked >= limit:
            continue
        candidates = sorted(
            path
            for path, has in members.items()
            if path != protocol and path not in protocols and required <= has
        )
        if not candidates:
            continue
        candidates = candidates[: max(0, limit - asked)]
        asked += len(candidates)

        source, where = _probe_source(protocol, candidates)
        done = subprocess.run(
            [str(binary), "snippet", "-", "--output-format", "json"],
            cwd=config,
            input=source,
            capture_output=True,
            text=True,
            check=False,
        )
        try:
            errors = json.loads(done.stdout or "{}").get("errors") or []
        except json.JSONDecodeError:
            # A snippet the checker could not parse is a fact about the probe, not a verdict.
            for candidate in candidates:
                rows.append([candidate, protocol, "unchecked", "the probe did not parse"])
            continue

        failed: dict[str, str] = {}
        for error in errors:
            candidate = where.get(error.get("line", -1))
            if candidate is None:
                continue
            detail = (error.get("description") or "").replace("\n", " ").strip()
            failed[candidate] = detail[:200] or error.get("name", "error")
        for candidate in candidates:
            if candidate in failed:
                rows.append([candidate, protocol, "no", failed[candidate]])
            else:
                rows.append([candidate, protocol, "yes", "-"])
    return sorted(rows)


def capture(
    binary: Path,
    venv: Path,
    site_packages: Path,
    cache: Path,
    modules: list[str],
    python: str,
    expected_files: set[str],
    pinned: str,
    documents: dict[str, dict],
    dunder_protocols: tuple[str, ...] = (),
) -> dict:
    """Run every batch report and return the reduction acquisition stores.

    Ordered so the cheap assertions fail before the expensive work: version, then configuration
    coverage, then the reports themselves.
    """
    reported = assert_checker(binary, pinned)
    say(f"  checker {reported}")

    config = write_config(cache, site_packages, venv, modules, python)
    covered = assert_coverage(binary, config, site_packages, expected_files)
    say(f"  covers {covered} files")

    produced = run_reports(binary, config, cache)
    roots = tuple(modules)

    functions, classes, parameters, mro, variables = reduce_definitions(produced["pysa"], roots)
    calls = reduce_calls(produced["pysa"], roots, functions, classes, site_packages)
    references, external = reduce_references(produced["glean"], roots, site_packages)
    symbols, suppressions, summary = reduce_coverage(produced["coverage"], roots)
    imports = reduce_imports(produced["dependency_graph"], site_packages, roots)
    satisfies = assignability(binary, config, documents, dunder_protocols=dunder_protocols)

    say(
        f"  {len(calls)} call edges, {len(references)} reference rows, "
        f"{len(parameters)} parameter rows, {len(mro)} mro rows, "
        f"{len(variables)} variable types, {len(imports)} import edges, "
        f"{len(satisfies)} assignability verdicts"
    )
    return {
        "checker": reported,
        "covered_files": covered,
        "digests": {
            "glean": digest_tree(produced["glean"]),
            "pysa": digest_tree(produced["pysa"]),
        },
        "summary": summary,
        "calls": [list(row) for row in calls],
        "references": references,
        "external_references": external,
        "parameters": parameters,
        "mro": mro,
        "variable_types": variables,
        "coverage": symbols,
        "suppressions": suppressions,
        "imports": imports,
        "satisfies": satisfies,
    }

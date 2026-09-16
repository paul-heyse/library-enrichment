"""Build the FastMCP capability repository from the acquired documents.

Offline by construction: the standard library plus the `ast-grep` binary, nothing else. Every
byte it reads comes from `acquired/`, which `acquire.py` produced once. Keeping this stage free
of Griffe, `uv` and the network is what makes `verify.py`'s determinism check meaningful -- it
re-runs this file and demands the same bytes.

Usage:
    python3 build.py [--manifest manifests/fastmcp.json] [--content ../content] [--stage ...]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import shutil
import subprocess
import sys
from pathlib import Path

# Sibling modules. Python puts the script's own directory on `sys.path`, so these resolve
# whatever directory the build is invoked from.
import analysis_catalogs
import catalogs
import emit
import extension_points
import link
import model
import queries
import topics

HERE = Path(__file__).resolve().parent
SKILL_ROOT = HERE.parent
ACQUIRED = HERE / "acquired"


GENERATED = ("index", "model", "api", "catalogs", "corpus", "extension-points", "topics")


def _reset(content: Path) -> None:
    """Remove the generated subtrees before rebuilding.

    Without this the build is not idempotent from a dirty tree, and the failure is invisible:
    `digest_tree` hashes whatever is on disk, so pages left behind by an earlier schema get
    recorded into PROVENANCE as though they were this run's output, and the determinism check
    then reproduces them happily forever. A schema change left 1,047 stale pages behind exactly
    this way.
    """
    for name in GENERATED:
        target = content / name
        if target.is_dir():
            shutil.rmtree(target)
    content.mkdir(parents=True, exist_ok=True)


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def _clean(text: str) -> str:
    """TSV has exactly two forbidden characters and no escape for either."""
    return text.replace("\t", " ").replace("\n", " ").replace("\r", " ").strip()


def _write_table(path: pathlib.Path, rows: list[str]) -> int:
    """Write a sorted table. An empty table is an empty file, not a blank line.

    `wc -l` on a file holding one newline reports 1, which reads as "one finding" for tables
    whose whole purpose is to be empty -- `not-indexed.tsv` above all.
    """
    ordered = sorted(set(rows))
    path.write_text("\n".join(ordered) + "\n" if ordered else "")
    return len(ordered)


def load_analysis(acquired: Path, acquisition: dict) -> dict:
    """Return the analysis payload, verified against the digest acquisition recorded.

    A locally produced payload has no published pin behind it -- nothing stops a truncated write
    or a stale file from an earlier pin -- so it is checked rather than trusted, the same way the
    Rust skills check locally built rustdoc.
    """
    summary = acquisition.get("analysis") or {"status": "not_run"}
    if summary.get("status") != "passed":
        return summary
    entry = (acquisition.get("files") or {}).get("ANALYSIS.json")
    path = acquired / "ANALYSIS.json"
    if entry is None or not path.is_file():
        raise SystemExit(
            "the analysis stage reported `passed` but ANALYSIS.json is absent; re-run acquire.py"
        )
    blob = path.read_bytes()
    actual = hashlib.sha256(blob).hexdigest()
    if actual != entry["sha256"]:
        raise SystemExit(
            f"ANALYSIS.json does not match its recorded digest "
            f"({actual[:12]} vs {entry['sha256'][:12]}); re-run acquire.py"
        )
    return json.loads(blob)


def latest_acquisition(manifest: dict) -> Path:
    name = manifest["repository"]["name"]
    candidates = sorted(ACQUIRED.glob(f"{name}@*"))
    if not candidates:
        raise FileNotFoundError(
            f"no acquired documents under {ACQUIRED}. Run `python3 acquire.py` once first."
        )
    return candidates[-1]


def ast_grep_version() -> str:
    try:
        done = subprocess.run(
            ["ast-grep", "--version"], capture_output=True, text=True, check=False
        )
        return done.stdout.strip() or "unavailable"
    except FileNotFoundError:
        return "unavailable"


# --------------------------------------------------------------------------- index tables


def write_indexes(built: model.Model, out: Path) -> dict[str, int]:
    out.mkdir(parents=True, exist_ok=True)
    counts: dict[str, int] = {}

    symbols = [
        "\t".join(
            (
                item.path,
                item.preferred,
                item.kind,
                item.dist,
                emit.api_file(item.module) if not item.boundary else "-",
                "yes" if item.nameable else "no",
                "yes" if item.in_all else "no",
                str(len(item.aliases)),
                item.protocol_era or "-",
                _clean(item.summary),
            )
        )
        for item in built.items.values()
    ]
    (out / "symbols.tsv").write_text("\n".join(sorted(symbols)) + "\n")
    counts["symbols"] = len(symbols)

    members = [
        "\t".join(
            (
                item.path,
                member["name"],
                member["kind"],
                member.get("declared_on", item.path),
                ",".join(member.get("labels", [])) or "-",
                _clean(member["signature"]),
                _clean(member.get("summary", "")),
            )
        )
        for item in built.items.values()
        for member in emit._interesting_members(item)
    ]
    (out / "members.tsv").write_text("\n".join(sorted(set(members))) + "\n")
    counts["members"] = len(set(members))

    aliases = [
        "\t".join(
            (
                alias,
                item.path,
                item.kind,
                "export" if built.exported(alias, item.name) else "import-site",
            )
        )
        for item in built.items.values()
        for alias in item.aliases
    ]
    (out / "aliases.tsv").write_text("\n".join(sorted(set(aliases))) + "\n")
    counts["aliases"] = len(set(aliases))

    bases = [
        "\t".join((item.path, base, item.dist))
        for item in built.items.values()
        for base in item.base_paths
    ]
    (out / "bases.tsv").write_text("\n".join(sorted(set(bases))) + "\n")
    counts["bases"] = len(set(bases))

    descendants = [
        "\t".join((base, child, str(depth), "yes" if built.items[child].nameable else "no"))
        for base, children in built.descendants.items()
        for child, depth in children
        if child in built.items
    ]
    (out / "descendants.tsv").write_text("\n".join(sorted(set(descendants))) + "\n")
    counts["descendants"] = len(set(descendants))

    overrides = ["\t".join(row) for row in built.overrides]
    (out / "overrides.tsv").write_text("\n".join(sorted(set(overrides))) + "\n")
    counts["overrides"] = len(set(overrides))

    # Keyed by owner and member, because the overloads that matter are mostly methods'.
    # `FastMCP.tool` is the case the registration catalog turns on, and an item-only table
    # would not carry it at all.
    overloads = [
        "\t".join((item.path, "-", str(ordinal), _clean(signature)))
        for item in built.items.values()
        for ordinal, signature in enumerate(item.overloads)
    ]
    overloads += [
        "\t".join((item.path, member["name"], str(ordinal), _clean(signature)))
        for item in built.items.values()
        for member in item.members
        for ordinal, signature in enumerate(member.get("overloads", []))
    ]
    (out / "overloads.tsv").write_text("\n".join(sorted(overloads)) + "\n")
    counts["overloads"] = len(overloads)

    # Position, as a range. `lineno` alone cannot answer "which definition encloses this
    # offset", and that is the shape every cross-reference fact arrives in.
    locations = []
    for item in built.items.values():
        if not item.file or item.lineno is None:
            continue
        locations.append(
            "\t".join(
                (
                    item.path,
                    item.kind,
                    item.file,
                    str(item.lineno),
                    str(item.endlineno if item.endlineno is not None else item.lineno),
                )
            )
        )
        for member in item.members:
            if member.get("inherited") or not member.get("file") or member.get("lineno") is None:
                continue
            end = member.get("endlineno") or member["lineno"]
            locations.append(
                "\t".join(
                    (
                        f"{item.path}.{member['name']}",
                        member["kind"],
                        member["file"],
                        str(member["lineno"]),
                        str(end),
                    )
                )
            )
    (out / "locations.tsv").write_text("\n".join(sorted(set(locations))) + "\n")
    counts["locations"] = len(set(locations))

    # `@abstractmethod` on a member is the difference between "you must write this" and "a
    # default exists". Item-level decorators alone could never express it.
    member_decorators = [
        "\t".join((f"{item.path}.{member['name']}", decorator))
        for item in built.items.values()
        for member in item.members
        if not member.get("inherited")
        for decorator in member.get("decorators", ())
    ]
    decorators = [
        "\t".join((item.path, decorator))
        for item in built.items.values()
        for decorator in item.decorators
    ] + member_decorators
    (out / "decorators.tsv").write_text("\n".join(sorted(set(decorators))) + "\n")
    counts["decorators"] = len(set(decorators))

    inferred = [
        "\t".join((item.path, item.kind, _clean(item.inferred), "ty"))
        for item in built.items.values()
        if item.inferred
    ]
    (out / "inferred.tsv").write_text("\n".join(sorted(inferred)) + "\n")
    counts["inferred"] = len(inferred)

    unannotated = [
        "\t".join((item.path, item.kind, "no annotation in source, and ty did not answer"))
        for item in built.items.values()
        if item.unannotated and not item.inferred
    ]
    (out / "unannotated.tsv").write_text("\n".join(sorted(unannotated)) + "\n")
    counts["unannotated"] = len(unannotated)

    # Member signatures carry most of the type surface, now that a
    # method is a member rather than an item. Keyed like `overloads.tsv`:
    # owner, then member, with "-" when the edge belongs to the item.
    types_used = [
        "\t".join((item.path, "-", used))
        for item in built.items.values()
        for used in item.types_used
    ]
    types_used += [
        "\t".join((item.path, member["name"], used))
        for item in built.items.values()
        for member in item.members
        for used in member.get("types_used", [])
    ]
    (out / "types-used.tsv").write_text("\n".join(sorted(set(types_used))) + "\n")
    counts["types_used"] = len(set(types_used))

    conditional = [
        "\t".join(
            (
                row["module"],
                row["name"],
                row["guard"],
                "fallback-defined" if row["fallback_defines"] else "import-only",
            )
        )
        for row in built.conditional
    ]
    (out / "conditional.tsv").write_text("\n".join(sorted(set(conditional))) + "\n")
    counts["conditional"] = len(set(conditional))

    unresolved = ["\t".join(row) for row in built.unresolved]
    (out / "unresolved.tsv").write_text("\n".join(sorted(unresolved)) + "\n")
    counts["unresolved"] = len(unresolved)

    not_indexed = [
        "\t".join((row["module"], _clean(row.get("reason", "")))) for row in built.not_indexed
    ]
    counts["not_indexed"] = _write_table(out / "not-indexed.tsv", not_indexed)

    return counts


def write_extras(manifest: dict, acquisition: dict, out: Path) -> int:
    """Which optional install unlocks which capability cluster.

    Read from `fastmcp-slim`, not `fastmcp`: the latter is a metapackage whose own metadata is
    six lines, so asking it about extras yields seven rows that each say "install fastmcp-slim".

    This read `extras_documented` from the manifest, where no such key existed, so the loop never
    ran -- leaving 101 rows that all said `_resolved` under a heading promising a capability map.
    The key exists now, and `verify.py` asserts every module it names is in the index, so the
    table cannot drift back into decoration.
    """
    rows: list[str] = []
    resolved = acquisition["resolved"]
    documented = manifest["environment"].get("extras_documented", [])
    for extra in sorted(documented, key=lambda entry: entry["extra"]):
        rows.append(
            "\t".join(
                (
                    "_extra",
                    extra["extra"],
                    ",".join(sorted(extra.get("distributions") or ())) or "-",
                    ",".join(sorted(extra.get("modules") or ())) or "-",
                    _clean(extra["unlocks"]),
                )
            )
        )
    for dist, version in sorted(resolved.items()):
        rows.append("\t".join(("_resolved", dist, version)))
    (out / "extras.tsv").write_text("\n".join(rows) + "\n")
    return len(rows)


def write_corpora(manifest: dict, acquired: Path, content: Path) -> dict[str, int]:
    """Copy the vendored upstream corpora out of `acquired/`, verbatim.

    No network here: `acquire.py` fetched and digested these. The sibling skills fetch inside
    `build.py`, which means their "offline by construction" build reaches for a network on a
    cold cache; keeping it in acquisition is what makes the claim in this module's docstring
    literally true.

    An empty result is a build failure rather than an empty directory, for the reason
    `fetch.repo_files` states: a reader cannot tell a corpus that was never fetched from one
    with no matches, and the second answers "this capability does not exist".
    """
    counts: dict[str, int] = {}
    for entry in manifest.get("corpora") or []:
        source = acquired.joinpath("corpus", entry["name"])
        if not source.is_dir():
            raise SystemExit(
                f"corpus {entry['name']!r} absent from {acquired.name}; re-run acquire.py"
            )
        files = sorted(path for path in source.rglob("*") if path.is_file())
        if not files:
            raise SystemExit(f"corpus {entry['name']!r} is empty in {acquired.name}")
        target = content.joinpath(entry["dest"])
        for path in files:
            destination = target.joinpath(path.relative_to(source))
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, destination)
        counts[entry["name"]] = len(files)
    return counts


def write_analysis_indexes(record: dict, built: model.Model, out: Path) -> dict[str, int]:
    """Project the checker's batch reports into index tables.

    These are the tables that close `reference.md` limit 2. Everything here is keyed by a
    fully-qualified dotted path, which is the same identity the rest of the index uses, so a row
    joins to `symbols.tsv` by string equality rather than by position arithmetic.

    When the analysis stage is `blocked` the tables are written EMPTY rather than skipped. An
    absent file and an empty one say different things to a reader: the second says "this question
    was asked and produced nothing", and `check_honesty` asserts that the emptiness agrees with
    the recorded status.
    """
    rows = {
        "callers": record.get("calls") or [],
        "references": record.get("references") or [],
        "external-refs": record.get("external_references") or [],
        "parameters": record.get("parameters") or [],
        "mro": record.get("mro") or [],
        "type-coverage": record.get("coverage") or [],
        "suppressions": record.get("suppressions") or [],
        "imports": record.get("imports") or [],
        "satisfies": record.get("satisfies") or [],
        "checker-types": record.get("variable_types") or [],
    }
    counts: dict[str, int] = {}
    for name, table in rows.items():
        lines = sorted({"\t".join(_clean(cell) for cell in row) for row in table})
        (out / f"{name}.tsv").write_text("\n".join(lines) + ("\n" if lines else ""))
        counts[name.replace("-", "_")] = len(lines)

    # `usage.tsv` is the ranking, and the single most useful thing here: it turns a flat index of
    # 3,056 symbols into one an agent can triage. Only rows that resolve to a real item are
    # ranked -- a count against a name this index does not carry would be unactionable.
    references: dict[str, list[int]] = {}
    for target, _file, count in rows["references"]:
        entry = references.setdefault(target, [0, 0])
        entry[0] += int(count)
        entry[1] += 1
    callers: dict[str, set[str]] = {}
    callees: dict[str, set[str]] = {}
    for callee, caller, _file, _line in rows["callers"]:
        callers.setdefault(callee, set()).add(caller)
        callees.setdefault(caller.split(" (", 1)[0], set()).add(callee)

    usage = []
    for path in sorted(set(references) | set(callers) | set(callees)):
        if path not in built.items:
            continue
        refs, files = references.get(path, [0, 0])
        usage.append(
            "\t".join(
                (
                    path,
                    str(refs),
                    str(files),
                    str(len(callers.get(path, ()))),
                    str(len(callees.get(path, ()))),
                )
            )
        )
    (out / "usage.tsv").write_text("\n".join(sorted(usage)) + ("\n" if usage else ""))
    counts["usage"] = len(usage)
    return counts


def _comparable(text: str) -> str:
    """Reduce a rendered type to something two engines can be compared on.

    They do not spell types the same way and never will. ty's hover returns markdown -- 96 of
    its 445 answers arrive wrapped in code fences -- and renders a gradual type as `float*`. The
    checker returns a fully-qualified string. Neither is wrong; they are different renderings,
    so the comparison strips to the leaf of every dotted name and drops the decoration.
    """
    cleaned = text.replace("```", " ").replace("`", " ").replace("*", "")
    cleaned = cleaned.replace("xml", " ") if cleaned.strip().startswith("xml") else cleaned
    # ty's hover wraps a type in a describing form and elides Annotated's metadata. Neither is
    # a claim about the type, and leaving them in reported 96 differences that were not.
    for noise in ("<special-form", "<class", "<metadata>", "'", ">"):
        cleaned = cleaned.replace(noise, " ")
    cleaned = cleaned.replace(", ]", "]").replace(",]", "]")
    out: list[str] = []
    token = ""
    for character in cleaned:
        if character.isalnum() or character in "._":
            token += character
        else:
            if token:
                out.append(token.rsplit(".", 1)[-1])
                token = ""
            if not character.isspace():
                out.append(character)
    if token:
        out.append(token.rsplit(".", 1)[-1])
    return "".join(out)


def write_inference_agreement(record: dict, content: Path) -> int:
    """Add an `agrees` column to inferred.tsv, and write the disagreement catalog.

    Two independent engines answering the same question is worth more than either answer alone,
    and the cases where they differ are the ones a caller should not rely on. ty was asked 452
    positions by hover; the checker answers the module-level ones in the same batch report it
    produces for everything else, so the second opinion costs nothing extra.
    """
    index = content / "index"
    (content / "catalogs").mkdir(parents=True, exist_ok=True)
    inferred = [
        line.split("\t") for line in (index / "inferred.tsv").read_text().splitlines() if line
    ]
    checker = {row[0]: row[1] for row in (record.get("variable_types") or [])}

    rows: list[str] = []
    tally = {"agree": 0, "narrower": 0, "differ": 0, "unanswered": 0}
    differences: list[tuple[str, str, str]] = []
    narrower: list[tuple[str, str, str]] = []
    for path, kind, value, source in inferred:
        other = checker.get(path)
        if other is None:
            verdict = "unanswered"
        elif _comparable(other) == _comparable(value):
            verdict = "agree"
        elif "Literal" in value and "Literal" not in other:
            # Answers at different precision, not in conflict. ty narrows a constant to its
            # literal type; the checker gives the base. Code relying on the narrow one will not
            # type-check under the other engine, which is the fact worth surfacing.
            verdict = "narrower"
            narrower.append((path, value, other))
        else:
            verdict = "differ"
            differences.append((path, value, other))
        tally[verdict] += 1
        rows.append("\t".join((path, kind, value, source, verdict)))
    (index / "inferred.tsv").write_text("\n".join(sorted(rows)) + "\n")

    lines = [
        "# Where the two engines disagree",
        "",
        "`inferred.tsv` holds types the source does not state. They come from **ty**, asked by",
        "hover at each definition site. The pinned checker answers the module-level ones too, in",
        "the batch report it produces anyway, so every row can carry a second opinion.",
        "",
        f"{tally['agree']} agree · {tally['narrower']} where ty is narrower · "
        f"{tally['differ']} differ · {tally['unanswered']} the checker did not answer.",
        "",
        f"**{tally['narrower']} of these are precision, not conflict.** ty narrows a constant to "
        "its literal type",
        "where the checker gives the widened base. Both are right. Code",
        "written against the narrow answer will not type-check under the other engine, which is "
        "why they are",
        "counted separately rather than folded into either bucket.",
        "",
        "`unanswered` is not disagreement. ty was asked about class attributes and function",
        "returns as well as module-level names; the checker's `global_variables` covers only the",
        "last of those, so most rows have nothing to compare against.",
        "",
        "Comparison strips each dotted name to its leaf and drops rendering noise, because the",
        "two engines spell types differently by design -- `Logger` against `logging.Logger` is",
        "agreement, not conflict. A row below is a real difference of opinion about the type.",
        "",
    ]
    if differences:
        lines += ["| Item | ty | checker |", "|---|---|---|"]
        lines += [
            f"| `{path}` | `{_clean(left)[:60]}` | `{_clean(right)[:60]}` |"
            for path, left, right in sorted(differences)[:40]
        ]
        if len(differences) > 40:
            lines.append(f"\n… and {len(differences) - 40} more; see column 5 of `inferred.tsv`.")
    else:
        lines.append("No row where both engines answered shows a difference.")
    lines += [
        "",
        "Neither answer is a promise upstream made. An inferred type is correct at this pin and",
        "can change without a release note, which is why these rows are separated from the",
        "declared ones rather than merged into them.",
        "",
    ]
    (content / "catalogs" / "inference-agreement.md").write_text("\n".join(lines))
    return tally["differ"]


def module_page(built: model.Model, content: Path) -> int:
    lines = [
        "# Modules",
        "",
        "Every module that contributed an item, with its distribution and item count.",
        "",
        "| Module | Distribution | Items | Page |",
        "|---|---|---:|---|",
    ]
    for module, paths in sorted(built.modules.items()):
        if not paths:
            continue
        dist = built.items[paths[0]].dist
        lines.append(f"| `{module}` | {dist} | {len(paths)} | `{emit.api_file(module)}` |")
    content.joinpath("modules.md").write_text("\n".join(lines) + "\n")
    return sum(1 for paths in built.modules.values() if paths)


# --------------------------------------------------------------------------- entry point


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Build the FastMCP capability repository.")
    parser.add_argument("--manifest", type=Path, default=HERE / "manifests" / "fastmcp.json")
    parser.add_argument("--content", type=Path, default=SKILL_ROOT / "content")
    parser.add_argument("--stage", choices=("model", "content", "all"), default="all")
    args = parser.parse_args(argv)

    manifest = json.loads(args.manifest.read_text())
    acquired = latest_acquisition(manifest)
    acquisition = json.loads((acquired / "ACQUISITION.json").read_text())
    say(f"acquisition {acquired.name}")

    documents = model.load_documents(acquired)
    built = model.stitch(documents, acquisition.get("semantic"))
    say(
        f"stitched: {len(built.items)} items, {len(built.access)} access paths, "
        f"{len(built.unresolved)} unresolved"
    )

    content = args.content
    _reset(content)

    counts = write_indexes(built, content / "index")
    counts["extras"] = write_extras(manifest, acquisition, content / "index")
    analysed = load_analysis(acquired, acquisition)
    counts.update(write_analysis_indexes(analysed, built, content / "index"))
    say("index rows: " + ", ".join(f"{k}={v}" for k, v in sorted(counts.items())))

    if args.stage == "model":
        sys.stdout.write(json.dumps({"items": len(built.items), "index": counts}) + "\n")
        return 0

    corpus_counts = write_corpora(manifest, acquired, content)
    counts.update({f"corpus_{k}": v for k, v in corpus_counts.items()})
    say("corpus: " + ", ".join(f"{k}={v}" for k, v in sorted(corpus_counts.items())))

    # After the catalogs directory exists, and after inferred.tsv has been written.
    counts["inference_disagreements"] = write_inference_agreement(analysed, content)

    grouped = emit.group_by_module(built.items)
    counts["model_files"] = emit.write_model(grouped, content)
    counts["api_pages"] = emit.write_api(grouped, content, built.exports)
    counts["modules"] = module_page(built, content)
    say(f"wrote {counts['api_pages']} api pages")

    # Structural edges from the corpus, then the pages that consume them. A page reporting
    # "no example demonstrates this" is indistinguishable from one whose search broke, so the
    # build says so out loud rather than emitting a confident empty section.
    demonstrated = link.subclass_sites(content.joinpath("corpus"), content)
    if len(demonstrated) < 10:
        say(f"! only {len(demonstrated)} bases have corpus edges -- check the corpus paths")
    counts["corpus_edges"] = len(demonstrated)
    counts["extension_points"] = extension_points.write_all(built, content, demonstrated)
    say(f"extension points: {counts['extension_points']} pages")

    counts["topics"] = topics.write_all(built, content, HERE.joinpath("topics.json"))
    say(f"topics: {counts['topics']} pages")

    catalog_counts = catalogs.write_all(built, manifest, content)
    catalog_counts.update(analysis_catalogs.write_all(built, content, analysed))
    catalog_counts["usage_annotated"] = analysis_catalogs.annotate_with_usage(content)
    counts.update({f"catalog_{k}": v for k, v in catalog_counts.items()})
    say("catalogs: " + ", ".join(f"{k}={v}" for k, v in sorted(catalog_counts.items())))

    # Generated rules come last: they read the catalogs this build just wrote, and the
    # hand-written rule they replace was exact only on the day it was typed.
    counts["generated_rules"] = queries.generate(content, SKILL_ROOT.joinpath("queries"))
    say(f"generated rules: {counts['generated_rules']}")

    emit.write_provenance(
        content,
        manifest,
        acquisition,
        counts,
        {"ast_grep": ast_grep_version(), "python": sys.version.split()[0]},
    )
    sys.stdout.write(json.dumps(counts, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

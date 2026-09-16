"""Verify the built repository.

Seven checks, in increasing order of what they can catch:

1. determinism     -- a rebuild reproduces every file byte for byte
2. integrity       -- every pointer between index, model, prose and catalog resolves
3. rule tests      -- every shipped rule still matches what it claims to
4. navigation      -- the recipes documented in reference.md actually find the answers
5. transferability -- nothing reaches outside the skill directory, content included
    5b. corpus -- the vendored upstream text is byte-for-byte what acquisition fetched
6. honesty         -- the tables that record limits cannot quietly disagree with each other
7. tripwires       -- the hand-written claims fail loudly when upstream invalidates them
8. closure        -- every recorded count matches the table it describes
9. join quality   -- Griffe and the checker agree about where things are
10. assignability  -- no conformance verdict is stated more confidently than measured
11. recipes       -- every command the documentation shows still returns something

Check 4 is the one that matters most: the others prove the repository is internally consistent,
only the probes prove it is *findable*. Check 7 is the one that would decay without help, since
it guards the few rows no static analysis could have produced.

Usage:
    python3 verify.py [--content ../content] [--skip-rebuild]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL_ROOT = HERE.parent

# `ast-grep test` exit codes, measured rather than assumed: 0 success, 3 the filter selected
# nothing, 4 an assertion or snapshot failed. It never returns 1.
TEST_OK = 0
TEST_FILTER_EMPTY = 3

HOST_PATTERNS = (
    (r"/home/[a-z][a-z0-9_-]*/", "hardcodes an absolute home path"),
    (r"\blibrary_enrichment\b", "references the host project"),
    (r"\bfrom\s+enrichment\b", "imports from the host project"),
)


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


class Failure(Exception):
    """A verification check did not hold."""


def _column(path: Path, index: int) -> set[str]:
    values: set[str] = set()
    for line in path.read_text().splitlines():
        if not line:
            continue
        fields = line.split("\t")
        if len(fields) > index:
            values.add(fields[index])
    return values


def check_determinism(content: Path, skip_rebuild: bool) -> str:
    provenance = json.loads(content.joinpath("PROVENANCE.json").read_text())
    recorded: dict[str, str] = provenance["files"]

    if not skip_rebuild:
        done = subprocess.run(
            [sys.executable, str(HERE / "build.py"), "--content", str(content)],
            capture_output=True,
            text=True,
            check=False,
        )
        if done.returncode != 0:
            raise Failure(f"rebuild failed: {done.stderr.strip()[-400:]}")

    missing: list[str] = []
    changed: list[str] = []
    for relative, expected in recorded.items():
        target = content / relative
        if not target.exists():
            missing.append(relative)
        elif hashlib.sha256(target.read_bytes()).hexdigest() != expected:
            changed.append(relative)
    if missing or changed:
        sample = (missing + changed)[:5]
        raise Failure(
            f"rebuild was not reproducible: {len(missing)} missing, "
            f"{len(changed)} changed; e.g. {sample}"
        )
    return f"{len(recorded)} files reproduced byte for byte"


def check_integrity(content: Path) -> str:
    index = content / "index"
    symbols = _column(index / "symbols.tsv", 0)
    problems: list[str] = []

    for name, column in (
        ("aliases.tsv", 1),
        ("members.tsv", 0),
        ("descendants.tsv", 1),
        ("inferred.tsv", 0),
        ("overloads.tsv", 0),
    ):
        unknown = _column(index / name, column) - symbols
        if unknown:
            sample = sorted(unknown)[:2]
            problems.append(f"{name}: {len(unknown)} rows name an unknown item, e.g. {sample}")

    pages = {value for value in _column(index / "symbols.tsv", 4) if value != "-"}
    absent = [page for page in sorted(pages) if not (content / page).exists()]
    if absent:
        problems.append(f"{len(absent)} api pages named by symbols.tsv do not exist")

    dangling = 0
    for document in sorted(content.joinpath("model").glob("*.json")):
        payload = json.loads(document.read_text())
        for item in payload.get("items", []):
            pointer = item.get("doc")
            if pointer and not (content / pointer.split("#", 1)[0]).exists():
                dangling += 1
    if dangling:
        problems.append(f"{dangling} model doc pointers do not resolve")

    if problems:
        raise Failure("; ".join(problems))
    return f"{len(symbols)} symbols, {len(pages)} pages, all cross-references resolve"


def check_rule_tests(queries: Path) -> str:
    """Run the fixtures, guarding the two ways this check lies about passing.

    A test naming a rule id that does not exist exits 0 with `0 passed; 0 failed`, so a typo
    deletes the test rather than failing it. And `--update-all` rewrites snapshots and then
    always exits 0, so it can never be part of a gate. Assert on the number of cases executed.
    """
    done = subprocess.run(
        ["ast-grep", "test", "-c", str(queries / "sgconfig.yml"), "--color=never"],
        capture_output=True,
        text=True,
        check=False,
    )
    output = done.stdout + done.stderr
    if done.returncode == TEST_FILTER_EMPTY:
        raise Failure("ast-grep test selected no cases (exit 3)")
    if done.returncode != TEST_OK:
        raise Failure(f"ast-grep test failed (exit {done.returncode}): {output.strip()[-400:]}")

    match = re.search(r"(\d+) passed; (\d+) failed", output)
    if not match:
        raise Failure(f"could not read a tally from: {output.strip()[-200:]}")
    passed, failed = int(match.group(1)), int(match.group(2))
    rule_files = list((queries / "rules").rglob("*.yml"))
    if passed == 0:
        raise Failure("ast-grep test exited 0 having run no cases; check the test `id` fields")
    if passed < len(rule_files):
        raise Failure(f"{len(rule_files)} rules but {passed} cases ran; each rule needs fixtures")
    if failed:
        raise Failure(f"{failed} rule tests failed")
    return f"{passed} rule cases passed, covering {len(rule_files)} rules"


def check_probes(content: Path, probes_path: Path) -> str:
    probes = json.loads(probes_path.read_text())
    failures: list[str] = []
    for probe in probes["probes"]:
        question = probe["question"]
        target = content / probe["file"]
        if not target.exists():
            failures.append(f"{question}: {probe['file']} absent")
            continue
        haystack = target.read_text()
        if "expect" in probe and not re.search(probe["expect"], haystack, re.MULTILINE):
            failures.append(f"{question}: no match for the expected pattern")
        # Some of what this repository must get right is an absence. A "must match" probe cannot
        # express that -- under MULTILINE a negative lookahead passes on the first unrelated
        # line -- so absence gets its own assertion.
        if "reject" in probe:
            hit = re.search(probe["reject"], haystack, re.MULTILINE)
            if hit:
                failures.append(f"{question}: rejected pattern matched {hit.group(0)[:50]!r}")
    if failures:
        raise Failure(f"{len(failures)} of {len(probes['probes'])} probes failed: {failures[:4]}")
    return f"{len(probes['probes'])} capability probes located their answers"


def check_transferability(skill_root: Path) -> str:
    """Nothing may reach outside the skill directory -- and that includes what it emits.

    The Rust version greps only the builder sources, which is a check that cannot fire on the
    failure it exists to prevent: Griffe records absolute file paths and a language server
    speaks absolute `file://` URIs, so a leak lands in `content/` and `acquired/`, never in
    `build/*.py`.

    Vendored upstream corpora are excluded and verified by digest in `check_corpus` instead;
    see the comment at the exclusion for why a pattern scan is the weaker check there.
    """
    offences: list[str] = []
    scanned = 0
    for source in sorted(HERE.glob("*.py")):
        scanned += 1
        text = source.read_text()
        offences.extend(
            f"build/{source.name}: {reason}"
            for pattern, reason in HOST_PATTERNS
            if re.search(pattern, text)
        )

    for tree in ("content", "build/acquired"):
        root = skill_root / tree
        if not root.exists():
            continue
        for path in sorted(root.rglob("*")):
            if not path.is_file():
                continue
            # Vendored upstream text is exempt. Upstream's own filesystem examples use a
            # placeholder home directory, which this scan cannot tell from a real leak, so
            # four legitimate documentation pages failed the check the first time a corpus
            # existed. `check_corpus` proves the stronger property for these bytes -- that
            # they hash to what acquisition fetched, and so are upstream's rather than this
            # machine's. Do not "restore" the scan here without reading it.
            if any(part == "corpus" for part in path.relative_to(skill_root).parts):
                continue
            scanned += 1
            try:
                text = path.read_text(encoding="utf-8")
            except (UnicodeDecodeError, OSError):
                continue
            for pattern, reason in HOST_PATTERNS:
                hit = re.search(pattern, text)
                if hit:
                    where = path.relative_to(skill_root)
                    offences.append(f"{where}: {reason} ({hit.group(0)!r})")
                    break
    if offences:
        raise Failure(f"{len(offences)} offences, e.g. {offences[:4]}")
    return f"{scanned} files carry no host path"


def check_corpus(skill_root: Path, content: Path) -> str:
    """The vendored corpora are upstream's bytes, unmodified.

    This is the check that lets `check_transferability` skip these trees. A host-path pattern
    cannot distinguish upstream's placeholder home directory from a real leak; a digest can
    distinguish upstream's bytes from anything else, which is the property that actually
    matters. It also catches the failure the pattern scan never could: a corpus silently
    truncated, half-copied, or left over from a different pin.

    An empty corpus is a failure rather than an empty directory, for the reason
    `fetch.repo_files` states -- a reader cannot tell a corpus that was never fetched from one
    with no matches, and the second answers "this capability does not exist".
    """
    acquired = sorted((skill_root / "build" / "acquired").glob("*/ACQUISITION.json"))
    if not acquired:
        raise Failure("no ACQUISITION.json; run acquire.py")
    record = json.loads(acquired[-1].read_text())
    declared = record.get("corpora") or {}
    if not declared:
        raise Failure("acquisition recorded no corpora")

    total = 0
    for name, entry in sorted(declared.items()):
        root = content / entry["dest"]
        if not root.is_dir():
            raise Failure(f"corpus {name!r} is absent from content/{entry['dest']}")
        blobs = {
            str(path.relative_to(root)): path.read_bytes()
            for path in sorted(root.rglob("*"))
            if path.is_file()
        }
        if not blobs:
            raise Failure(f"corpus {name!r} is empty")
        actual = hashlib.sha256(
            b"".join(name.encode() + b"\0" + blob for name, blob in sorted(blobs.items()))
        ).hexdigest()
        if actual != entry["sha256"]:
            raise Failure(
                f"corpus {name!r} does not match the acquired bytes "
                f"({actual[:12]} vs {entry['sha256'][:12]}); rebuild or re-acquire"
            )
        if len(blobs) != entry["files"]:
            raise Failure(f"corpus {name!r}: {len(blobs)} files against {entry['files']} acquired")
        total += len(blobs)
    return f"{total} corpus files across {len(declared)} corpora match the acquired digests"


def _lines(path: Path) -> list[str]:
    return path.read_text().splitlines() if path.is_file() else []


def _rows(path: Path) -> list[list[str]]:
    return [line.split("\t") for line in _lines(path)]


def check_honesty(content: Path) -> str:
    """The tables that record limits must not disagree with one another."""
    index = content / "index"
    problems: list[str] = []

    inferred = _column(index / "inferred.tsv", 0)
    unannotated = _column(index / "unannotated.tsv", 0)
    both = inferred & unannotated
    if both:
        problems.append(f"{len(both)} items are both inferred and unannotated: {sorted(both)[:2]}")

    provenance = json.loads((content / "PROVENANCE.json").read_text())
    semantic = provenance["semantic"]
    answered = semantic.get("answered", 0)
    if semantic["status"] == "passed" and len(inferred) != answered:
        problems.append(f"PROVENANCE says ty answered {answered}, inferred.tsv has {len(inferred)}")
    if semantic["status"] != "passed" and inferred:
        problems.append(f"semantic stage is {semantic['status']} yet inferred.tsv is populated")

    # A nameable item must have somewhere to be imported from. If `preferred` still points at a
    # private path while the item claims to be nameable, the preferred-spelling ranking broke.
    for line in (index / "symbols.tsv").read_text().splitlines():
        fields = line.split("\t")
        private = any(part.startswith("_") for part in fields[1].split(".")[1:])
        if len(fields) > 5 and fields[5] == "yes" and private:
            problems.append(f"{fields[0]} is marked nameable but prefers a private path")
            break

    if problems:
        raise Failure("; ".join(problems))

    # The analysis tables must agree with the status acquisition recorded. Written-but-empty is
    # a legitimate state (the stage was blocked); written-and-populated while the record says
    # blocked, or empty while it says passed, is a lie the rest of the repository would repeat.
    acquired = sorted((SKILL_ROOT / "build" / "acquired").glob("*/ACQUISITION.json"))
    if acquired:
        record = json.loads(acquired[-1].read_text()).get("analysis") or {}
        status = record.get("status", "not_run")
        callers = _rows(content / "index" / "callers.tsv")
        if status == "passed" and not callers:
            raise Failure("the analysis stage reports `passed` but callers.tsv is empty")
        if status != "passed" and callers:
            raise Failure(f"callers.tsv has {len(callers)} rows but the stage reports `{status}`")
        if status == "passed":
            declared = (record.get("rows") or {}).get("calls")
            if declared is not None and declared != len(callers):
                raise Failure(
                    f"callers.tsv has {len(callers)} rows against {declared} recorded at "
                    f"acquisition; the build and the capture disagree"
                )
            # Every ranked path must be an item this index actually carries, or the ranking
            # points at names a reader cannot look up.
            symbols = {line.split("\t")[0] for line in _lines(content / "index" / "symbols.tsv")}
            unknown = [
                row[0] for row in _rows(content / "index" / "usage.tsv") if row[0] not in symbols
            ]
            if unknown:
                raise Failure(f"{len(unknown)} usage rows name unknown items, e.g. {unknown[:3]}")

    return (
        f"{len(inferred)} inferred and {len(unannotated)} unannotated rows are disjoint "
        "and agree with PROVENANCE"
    )


def check_tripwires(content: Path) -> str:
    """Guard the hand-written claims that no static analysis produced.

    `catalogs/registration.md` states that `@mcp.tool` returns the decorated function, against
    an implementation annotation that says `FunctionTool`. That claim is only worth making while
    the two disagree. If upstream ever makes them agree, the page is stale in a way nothing else
    would notice, so the disagreement itself is the assertion.
    """
    index = content / "index"
    rows = [
        line.split("\t")
        for line in (index / "overloads.tsv").read_text().splitlines()
        if line.startswith("fastmcp.server.server.FastMCP\ttool\t")
    ]
    if not rows:
        raise Failure(
            "no overloads recorded for FastMCP.tool; the registration catalog's claim about "
            "what the decorator returns can no longer be checked"
        )
    if not any("-> F" in row[3] for row in rows):
        raise Failure(
            "no `FastMCP.tool` overload returns `F` any more. Either the decorator contract "
            "changed or the overloads moved; re-read catalogs/registration.md before shipping"
        )
    implementation = [
        line
        for line in (index / "members.tsv").read_text().splitlines()
        if line.startswith("fastmcp.server.server.FastMCP\ttool\t")
    ]
    if implementation and "FunctionTool" not in implementation[0]:
        raise Failure(
            "the `FastMCP.tool` implementation no longer annotates a `FunctionTool` return, so "
            "the three-way disagreement documented in catalogs/registration.md is stale"
        )
    return f"{len(rows)} overloads still contradict the implementation annotation, as documented"


def check_closure(content: Path) -> str:
    """Every recorded count matches the table it describes.

    A count in PROVENANCE is what a reader trusts without opening the file, and it is written by
    the same pass that writes the tables -- so on its own it proves nothing. Recomputing from
    disk is what makes it evidence. This also caught a real loss: 1,452 parameter rows collapsed
    because two overloads of one function emit identical rows wherever their parameters agree.
    """
    provenance = json.loads((content / "PROVENANCE.json").read_text())
    counts = provenance["counts"]
    checked = 0
    for name, recorded in sorted(counts.items()):
        if name.startswith(("catalog_", "corpus_")) or name in {
            "api_pages",
            "model_files",
            "modules",
            "inference_disagreements",
            "corpus_edges",
            "extension_points",
            "topics",
            "generated_rules",
        }:
            continue
        table = content / "index" / f"{name.replace('_', '-')}.tsv"
        if not table.is_file():
            table = content / "index" / f"{name}.tsv"
        if not table.is_file():
            continue
        actual = len([line for line in table.read_text().splitlines() if line])
        if actual != recorded:
            raise Failure(f"{table.name}: {actual} rows against {recorded} recorded")
        checked += 1
    if checked < 10:
        raise Failure(f"only {checked} tables could be matched to a recorded count")
    return f"{checked} tables match their recorded counts"


def check_join_quality(content: Path) -> str:
    """The two producers agree about where things are.

    Griffe reads the source and reports a 1-based line. The checker reports a `line:col` span in
    its own report, independently. Nothing else in this repository compares them, and if they
    disagreed every location-bearing row would be quietly pointing at the wrong place.

    Agreement is asserted as a rate rather than absolutely: the checker locates a decorated
    function at its `def`, Griffe at its first decorator, so a small disagreement is expected
    and a large one is a broken pipeline.
    """
    locations = {}
    for line in _lines(content / "index" / "locations.tsv"):
        row = line.split("\t")
        if len(row) >= 5:
            locations[row[0]] = (row[2], int(row[3]))

    coverage = _rows(content / "index" / "type-coverage.tsv")
    if not coverage:
        return "no checker locations to compare (the analysis stage did not run)"

    compared = agreed = 0
    examples: list[str] = []
    for row in coverage:
        known = locations.get(row[0])
        if known is None:
            continue
        compared += 1
        # Within a few lines: a decorated definition is located at the `def` by one and at the
        # decorator by the other, and both are correct answers to different questions.
        if abs(known[1] - int(row[2])) <= 3:
            agreed += 1
        elif len(examples) < 3:
            examples.append(f"{row[0]} griffe={known[1]} checker={row[2]}")
    if compared < 100:
        raise Failure(f"only {compared} items carry a location from both producers")
    rate = agreed / compared
    if rate < 0.95:
        raise Failure(
            f"the two producers agree on only {rate:.1%} of {compared} locations, e.g. {examples}"
        )
    return f"{agreed}/{compared} locations agree between Griffe and the checker ({rate:.1%})"


def check_assignability_honesty(content: Path) -> str:
    """No conformance verdict is stated more confidently than it was measured."""
    page = content / "catalogs" / "protocols.md"
    if not page.is_file():
        raise Failure("catalogs/protocols.md is absent")
    text = page.read_text()
    rows = _rows(content / "index" / "satisfies.tsv")
    if not rows:
        if "Not available at this build" not in text:
            raise Failure("satisfies.tsv is empty but protocols.md does not say the stage failed")
        return "no verdicts; the page says so"
    if "variance" not in text:
        raise Failure(
            "protocols.md states verdicts without the variance caveat the conformance "
            "suite records for this checker"
        )
    verdicts = {row[2] for row in rows}
    if not verdicts <= {"yes", "no", "unchecked"}:
        raise Failure(f"unexpected verdicts in satisfies.tsv: {sorted(verdicts - {'yes', 'no'})}")
    return f"{len(rows)} verdicts, each adjudicated, with the variance caveat stated"


def check_recipes(skill_root: Path) -> str:
    """Every documented recipe still returns something.

    A recipe that silently finds nothing is worse than a missing one: the reader runs it, sees
    an empty result, and concludes the capability is absent. Two shipped this way -- one whose
    anchor stopped matching when a table gained a column, one naming a Protocol that was never
    probed -- and neither probes nor integrity could catch them, because both assert things
    about file *contents* and a recipe is a command.

    Placeholders are skipped: a line containing `<...>` is an instruction to substitute, not a
    command to run.
    """
    fence = re.compile(r"```(?:bash)?\n(.*?)```", re.DOTALL)
    empty: list[str] = []
    ran = 0
    for name in ("reference.md", "SKILL.md"):
        page = skill_root / name
        if not page.is_file():
            continue
        for block in fence.findall(page.read_text()):
            for line in block.splitlines():
                command = line.strip()
                if not command or command.startswith("#") or "<" in command:
                    continue
                if not command.startswith(("rg", "cut", "sort", "ast-grep")):
                    continue
                ran += 1
                done = subprocess.run(
                    ["bash", "-c", command],
                    cwd=skill_root,
                    capture_output=True,
                    text=True,
                    check=False,
                )
                if not done.stdout.strip():
                    empty.append(f"{name}: {command}")
    if empty:
        raise Failure(f"{len(empty)} documented recipes return nothing: {empty[:3]}")
    if ran < 20:
        raise Failure(f"only {ran} recipes were found to run; the fence parsing has drifted")
    return f"{ran} documented recipes all return output"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Verify the built FastMCP repository.")
    parser.add_argument("--content", type=Path, default=SKILL_ROOT / "content")
    parser.add_argument("--queries", type=Path, default=SKILL_ROOT / "queries")
    parser.add_argument("--probes", type=Path, default=HERE / "probes.json")
    parser.add_argument("--skip-rebuild", action="store_true")
    args = parser.parse_args(argv)

    checks = (
        ("determinism", lambda: check_determinism(args.content, args.skip_rebuild)),
        ("integrity", lambda: check_integrity(args.content)),
        ("rule tests", lambda: check_rule_tests(args.queries)),
        ("navigation probes", lambda: check_probes(args.content, args.probes)),
        ("transferability", lambda: check_transferability(SKILL_ROOT)),
        ("corpus", lambda: check_corpus(SKILL_ROOT, args.content)),
        ("honesty", lambda: check_honesty(args.content)),
        ("closure", lambda: check_closure(args.content)),
        ("join quality", lambda: check_join_quality(args.content)),
        ("assignability", lambda: check_assignability_honesty(args.content)),
        ("tripwires", lambda: check_tripwires(args.content)),
        ("recipes", lambda: check_recipes(SKILL_ROOT)),
    )

    results: dict[str, str] = {}
    failed = 0
    for name, run in checks:
        try:
            results[name] = run()
            say(f"  ok    {name}: {results[name]}")
        except Failure as error:
            results[name] = f"FAILED: {error}"
            failed += 1
            say(f"  FAIL  {name}: {error}")
        except FileNotFoundError as error:
            results[name] = f"BLOCKED: {error}"
            failed += 1
            say(f"  BLOCK {name}: {error}")

    sys.stdout.write(json.dumps(results, indent=2) + "\n")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())

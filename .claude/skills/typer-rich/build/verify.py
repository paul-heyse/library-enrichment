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
import os
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

# Applied to shipped content and to the acquisition record, never to `build/*.py`: the builder
# legitimately names the capsule directory it creates, and `capsule_root()` derives the rest from
# the environment.
CONTENT_ONLY_PATTERNS = ((r"python-skill-acquire", "leaks the acquisition capsule path"),)

HOST_PATTERNS = (
    (r"/home/[a-z][a-z0-9_-]*/", "hardcodes an absolute home path"),
    (r"\blibrary_enrichment\b", "references the host project"),
    (r"\bfrom\s+enrichment\b", "imports from the host project"),
    (r"0x[0-9a-f]{8,}", "carries a raw memory address; normalise it or drop the probe"),
)

# Probe captures are NOT exempt from the scan the way the corpus is. `rich.traceback` renders
# absolute source paths straight into its output, so a capture is the most likely place for a
# home path to reach `content/` -- which is why probe R002 raises inside a synthetic filename
# rather than a real one, and why the address pattern above exists at all.


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
            for pattern, reason in HOST_PATTERNS + CONTENT_ONLY_PATTERNS:
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
    """Guard the claims that would go quietly stale rather than loudly wrong.

    Each of these is true today, load-bearing somewhere in the prose, and invisible to every
    other check if it stopped being true.
    """
    index = content / "index"
    symbols = (index / "symbols.tsv").read_text()
    checked = []

    # 1. The MRO still crosses the vendoring boundary. `catalogs/vendored-click.md` and the
    #    whole reachability model rest on `typer.Context` inheriting from the vendored Context.
    #    If Typer ever stops subclassing it, `subclassed-via` silently empties.
    if not re.search(r"^typer\._click\.core\.Context\t.*\tsubclassed-via\t", symbols, re.M):
        raise Failure(
            "typer._click.core.Context is no longer reached by subclassing; the vendored-Click "
            "page's central claim -- that you are handed one rather than importing it -- is stale"
        )
    checked.append("vendoring boundary")

    # 2. `rich.abc.RichRenderable` still has zero descendants. The seam page says outright that
    #    there is no need to extend it; if upstream ever adds a subclass the page becomes wrong
    #    in a way no count and no recipe would flag.
    descendants = (index / "descendants.tsv").read_text()
    if re.search(r"^rich\.abc\.RichRenderable\t", descendants, re.M):
        raise Failure(
            "rich.abc.RichRenderable has gained descendants; seams/renderable-protocol.md says "
            "there is no need to extend it, which is now a claim about a different class"
        )
    checked.append("RichRenderable is still empty")

    # 3. `rich.Console` is still NOT offered as the importable spelling. This is the finding the
    #    repository opens with, and the fix is one ranking rule deep -- a regression would put a
    #    line both type checkers accept and Python rejects back at the top of the page.
    if re.search(r"^rich\.console\.Console\trich\.Console\t", symbols, re.M):
        raise Failure(
            "rich.Console is being offered as the preferred spelling again; it exists only under "
            "`if TYPE_CHECKING:` and `from rich import Console` raises ImportError at runtime"
        )
    checked.append("no TYPE_CHECKING-only spelling preferred")

    # 4. The excluded data modules are still excluded. Re-including them would roughly double
    #    the symbol count with rows nobody queries, and every count in the prose would drift.
    if re.search(r"^rich\._unicode_data\.|^rich\._emoji_codes\.", symbols, re.M):
        raise Failure(
            "the generated Unicode or emoji data modules are back in the index; reference.md "
            "limit 8 states they are excluded and every count in SKILL.md assumes it"
        )
    checked.append("data modules excluded")

    return f"{len(checked)} tripwires hold: {', '.join(checked)}"


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
    """No conformance verdict is stated more confidently than it was measured.

    The floor below is the point of this check here. Rich's two central protocols declare nothing
    but `__rich_console__` and `__rich__`, and three separate underscore filters in the pipeline
    would drop them -- after which `assignability` skips them as "a protocol with no public
    members", `satisfies.tsv` ships with zero rows for the two protocols that decide whether an
    object can be printed at all, and every other check stays green. An empty result must fail.
    """
    rows = _rows(content / "index" / "satisfies.tsv")
    if not rows:
        raise Failure("satisfies.tsv is empty; the assignability stage produced nothing")

    verdicts = {row[2] for row in rows}
    if not verdicts <= {"yes", "no", "unchecked"}:
        raise Failure(f"unexpected verdicts in satisfies.tsv: {sorted(verdicts)}")

    required = ("rich.console.ConsoleRenderable", "rich.console.RichCast")
    for protocol in required:
        hits = [row for row in rows if row[1] == protocol and row[2] == "yes"]
        if not hits:
            raise Failure(
                f"no class satisfies {protocol}; that is the silent-emptiness failure this "
                "repository exists to prevent -- check the dunder_protocols allowlist reaches "
                "analysis._class_members"
            )
    return f"{len(rows)} verdicts, each adjudicated; both dunder-only protocols carry some"


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


def _acquired() -> Path:
    """The acquisition directory this content was built from."""
    candidates = sorted((HERE / "acquired").glob("typer-rich@*"))
    if not candidates:
        raise Failure("no acquired documents; run acquire.py once first")
    return candidates[-1]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Verify the built Typer/Rich capability repository."
    )
    parser.add_argument("--content", type=Path, default=SKILL_ROOT / "content")
    parser.add_argument("--queries", type=Path, default=SKILL_ROOT / "queries")
    # Two registers, kept apart: `navigation.json` asserts that the documented route leads to a
    # file and the answer is in it; `probes.json` executes the subjects. fastmcp overloads one
    # name for both concepts.
    parser.add_argument("--navigation", type=Path, default=HERE / "navigation.json")
    parser.add_argument("--skip-rebuild", action="store_true")
    args = parser.parse_args(argv)

    checks = (
        ("determinism", lambda: check_determinism(args.content, args.skip_rebuild)),
        ("integrity", lambda: check_integrity(args.content)),
        ("rule tests", lambda: check_rule_tests(args.queries)),
        ("navigation", lambda: check_probes(args.content, args.navigation)),
        ("transferability", lambda: check_transferability(SKILL_ROOT)),
        ("corpus", lambda: check_corpus(SKILL_ROOT, args.content)),
        ("honesty", lambda: check_honesty(args.content)),
        ("closure", lambda: check_closure(args.content)),
        ("join quality", lambda: check_join_quality(args.content)),
        ("assignability", lambda: check_assignability_honesty(args.content)),
        ("behaviours", lambda: check_behaviours(args.content, _acquired())),
        ("probe hermeticity", check_probe_hermeticity),
        ("reachability", lambda: check_reachability(args.content)),
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


def check_behaviours(content: Path, acquired: Path) -> str:
    """Compare every shipped capture against the digest `behaviors.tsv` recorded for it.

    A drifted verdict fails. A drifted capture fails louder -- capturing bytes is the whole point
    of executing the subjects, and a capture that quietly changed while its verdict stayed
    `confirmed` would be the most convincing wrong answer this repository could give.
    """
    import render_probes

    payload = json.loads((acquired / "PROBES.json").read_text())
    recorded = {row["id"]: row for row in payload["results"]}

    rows = _rows(content / "index" / "behaviors.tsv")
    if not rows:
        raise Failure("behaviors.tsv is empty; no behaviour was observed at all")

    inconclusive = [row[0] for row in rows if row[3] == "inconclusive"]
    if inconclusive:
        raise Failure(
            f"{len(inconclusive)} probes are inconclusive ({inconclusive[:4]}); an inconclusive "
            "probe demonstrates nothing and must not ship as evidence"
        )

    disagree = sorted(set(recorded) ^ {row[0] for row in rows})
    if disagree:
        raise Failure(
            f"behaviors.tsv and PROBES.json disagree about which probes exist: {disagree}"
        )

    drifted = []
    for row in rows:
        probe_id, digest, capture = row[0], row[10], row[11]
        if capture == "-":
            continue
        path = content / capture
        if not path.is_file():
            raise Failure(f"{probe_id} names capture {capture}, which is absent")
        # The digest covers exactly the bytes render_probes wrote, so the file is hashed as it
        # stands. Re-deriving the termination here was a second implementation of the same rule
        # and disagreed with the first for the ANSI channel, whose repr never ends in a newline.
        if render_probes._digest(path.read_text()) != digest:
            drifted.append(probe_id)
    if drifted:
        raise Failure(f"capture digests do not match behaviors.tsv for {drifted}")

    confirmed = sum(1 for row in rows if row[3] == "confirmed")
    return f"{len(rows)} probes, {confirmed} confirmed, every capture matches its digest"


def check_probe_hermeticity() -> str:
    """The probe environment is built from the manifest alone and carries nothing forbidden.

    A hermeticity regression is invisible to every other check on a machine where the offending
    variable happens to be unset -- which is every developer machine, until it is not. So this
    asserts the construction rather than the outcome.
    """
    import tempfile

    import probes as probe_runner

    manifest = json.loads((HERE / "manifests" / "typer-rich.json").read_text())
    spec = manifest["probe_environment"]

    with tempfile.TemporaryDirectory() as scratch:
        env = probe_runner.probe_environment(manifest, Path(scratch), os.environ.get("PATH", ""))
    leaked = sorted(name for name in spec["must_be_absent"] if name in env)
    if leaked:
        raise Failure(f"the probe environment carries {leaked}, which must be absent")

    unexpected = sorted(set(env) - set(spec["allow"]))
    if unexpected:
        raise Failure(f"the probe environment carries undeclared names {unexpected}")

    # Every Rich probe must reach its Console through the prelude, which is what passes
    # `_environ`. E003 builds a bare one deliberately -- it is testing the absence of the
    # isolation -- and its question says so, which is the only exemption.
    register = json.loads((HERE / "probes.json").read_text())["probes"]
    bare = [
        row["id"]
        for row in register
        if row["subject"] in ("rich", "both")
        and "Console(" in row["source"]
        and "_environ" not in row["source"]
        and "override" not in row["question"].lower()
        and "isolate" not in row["question"].lower()
    ]
    if bare:
        raise Failure(
            f"{bare} construct a Console without going through the prelude and without saying "
            "why; layer two of the isolation is bypassed silently"
        )
    return f"{len(env)} names, none forbidden; {len(register)} probes honour the isolation"


def check_reachability(content: Path) -> str:
    """No vendored-Click item with public members is `internal` without the manifest saying so.

    This is what stops 5,548 lines quietly falling off the map. `nameable: no` is honest and
    useless on its own; the allowlist is the *claim* that a particular class is genuinely never
    met, and a claim someone made is reviewable in a way a silent default is not.
    """
    manifest = json.loads((HERE / "manifests" / "typer-rich.json").read_text())
    subject = next(s for s in manifest["subjects"] if s["name"] == "click")
    allow = set(subject.get("internal_allowlist", ()))

    rows = _rows(content / "index" / "symbols.tsv")
    valid = {
        "importable",
        "received",
        "subclassed-via",
        "subtype-of",
        "registered-via",
        "declared",
        "internal",
    }
    seen = {row[6] for row in rows}
    if not seen <= valid:
        raise Failure(f"unknown reachability values {sorted(seen - valid)}")

    for row in rows:
        if (row[6] == "internal") != (row[7] == "-"):
            raise Failure(
                f"{row[0]} has reachability {row[6]!r} with reached_via {row[7]!r}; the two must "
                "agree -- anything not internal has to say what reaches it"
            )

    members = {line.split("\t")[0] for line in _lines(content / "index" / "members.tsv")}
    orphans = sorted(
        row[0]
        for row in rows
        if row[3] == "click" and row[6] == "internal" and row[0] in members and row[0] not in allow
    )
    if orphans:
        raise Failure(
            f"{len(orphans)} vendored-Click items with public members are internal and not in "
            f"the manifest allowlist: {orphans[:5]}"
        )

    stale = sorted(
        path for path in allow if not any(row[0] == path and row[6] == "internal" for row in rows)
    )
    if stale:
        raise Failure(f"allowlist entries are no longer internal and should be removed: {stale}")
    return f"{len(rows)} rows carry a valid reachability; allowlist of {len(allow)} is exact"


if __name__ == "__main__":
    raise SystemExit(main())

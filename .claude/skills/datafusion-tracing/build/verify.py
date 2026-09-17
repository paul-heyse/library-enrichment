"""Verify the built repository. Nine checks; exit 1 if any fails.

Five are the ones every capability repository in this family carries. Three exist because this
repository's subject can be *executed* rather than only read, and because its most valuable
table is curated rather than derived. The ninth exists because the sibling repositories' prose
has drifted from their own artifacts -- one says 42 where 44 files exist, another cites a count
of 116 that is 3 in its own generated rule -- and nothing in either of them would ever notice.

`acquire.py` is deliberately unreachable from here. This re-runs `build.py` to prove
determinism, and if acquisition could run inside that, a rebuild could fetch different bytes and
still call itself reproducible.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL_DIR = HERE.parent
CONTENT = SKILL_DIR / "content"
QUERIES = SKILL_DIR / "queries"

# Nothing in this directory may reach outside it. The whole point of the repository is that it
# can be copied into any project and rebuilt there.
HOST_COUPLING = (
    (r"\bfrom\s+enrichment", "imports a host package"),
    (r"/home/[a-z]+/", "hardcodes an absolute home path"),
    (r"\blibrary_enrichment\b", "references the host project"),
    (r"\blibrary-enrichment\b", "references the host project"),
)


class Failure(RuntimeError):
    """One check did not hold."""


def _run(command: list[str], cwd: Path) -> tuple[int, str, str]:
    done = subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False)
    return done.returncode, done.stdout, done.stderr


def _provenance() -> dict:
    return json.loads((CONTENT / "PROVENANCE.json").read_text())


# --------------------------------------------------------------------------- 1

def check_determinism(skip_rebuild: bool) -> dict:
    """A rebuild reproduces every generated file byte for byte."""
    if not skip_rebuild:
        code, _, stderr = _run([sys.executable, str(HERE / "build.py")], SKILL_DIR)
        if code != 0:
            raise Failure(f"the rebuild failed: {stderr.strip()[-400:]}")
    provenance = _provenance()
    missing, changed = [], []
    import hashlib

    for relative, expected in provenance["files"].items():
        path = CONTENT / relative
        if not path.exists():
            missing.append(relative)
        elif hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            changed.append(relative)
    if missing or changed:
        raise Failure(
            f"{len(missing)} missing, {len(changed)} changed; "
            f"sample {(missing + changed)[:5]}"
        )
    return {"files": len(provenance["files"])}


# --------------------------------------------------------------------------- 2

def check_integrity() -> dict:
    """Every pointer between index, model, prose, corpus and captures resolves."""
    index = CONTENT / "index"
    symbols = {
        line.split("\t")[0] for line in (index / "symbols.tsv").read_text().splitlines() if line
    }
    problems: list[str] = []

    for row in (index / "methods.tsv").read_text().splitlines():
        if row and row.split("\t")[0] not in symbols:
            problems.append(f"methods.tsv owner {row.split(chr(9))[0]} is not in symbols.tsv")
    for row in (index / "aliases.tsv").read_text().splitlines():
        if row and row.split("\t")[1] not in symbols:
            problems.append(f"aliases.tsv target {row.split(chr(9))[1]} is not in symbols.tsv")
    for row in (index / "symbols.tsv").read_text().splitlines():
        if row:
            page = row.split("\t")[6]
            if not (CONTENT / page).exists():
                problems.append(f"symbols.tsv names api page {page}, which does not exist")
    for record in sorted((CONTENT / "model").glob("*.json")):
        document = json.loads(record.read_text())
        for item in document["items"]:
            pointer = item.get("doc")
            if pointer and not (CONTENT / pointer.split("#", 1)[0]).exists():
                problems.append(f"{record.name} points at {pointer}, which does not exist")
    for row in (index / "previews.tsv").read_text().splitlines():
        if row:
            capture = row.split("\t")[6]
            if not (CONTENT / capture).exists():
                problems.append(f"previews.tsv names {capture}, which does not exist")

    if problems:
        raise Failure(f"{len(problems)} dangling pointer(s): {problems[:4]}")
    return {"symbols": len(symbols)}


# --------------------------------------------------------------------------- 3

def check_rule_tests() -> dict:
    """Every shipped rule still matches what it claims to, and still parses.

    The exit status of `ast-grep test` cannot be trusted on its own: it returns 0 while
    reporting `0 passed; 4 failed`, and `--filter` matching nothing also exits 0, so a typo in a
    rule id would delete the test rather than fail it. Exit 8 is the sharp one -- a single
    malformed rule file stops the WHOLE corpus from loading, which was observed while writing
    these rules: one duplicate `has:` key disabled all ten.
    """
    code, stdout, stderr = _run(
        ["ast-grep", "test", "-c", str(QUERIES / "sgconfig.yml"), "--color=never"], QUERIES
    )
    if code == 8:
        raise Failure(f"a rule file does not parse, so no rule loaded at all: {stderr[:300]}")
    if code == 3:
        raise Failure("`--filter` selected no rule; the corpus did not run")
    if code == 6:
        raise Failure("the test directory is missing")
    match = re.search(r"(\d+) passed; (\d+) failed", stdout + stderr)
    if not match:
        raise Failure(f"could not read a result from ast-grep test: {(stdout + stderr)[:300]}")
    passed, failed = int(match.group(1)), int(match.group(2))
    rules = list((QUERIES / "rules").rglob("*.yml"))
    if failed:
        raise Failure(f"{failed} rule test(s) failed")
    if passed < len(rules):
        raise Failure(
            f"{passed} test(s) ran for {len(rules)} rule(s). A test file naming a rule id that "
            f"does not exist exits 0 with `0 passed; 0 failed`, so a typo would silently "
            f"delete the test rather than fail it."
        )
    return {"rules": len(rules), "passed": passed}


# --------------------------------------------------------------------------- 4

def check_navigation(specification: Path) -> dict:
    """The routes the documentation describes still lead to the answer.

    `reject` is as important as `expect`. Some of what this repository must get right is an
    absence -- that the hosted document does NOT carry a builder method, that a name is NOT in
    the nameable set -- and a positive-only probe cannot check it.
    """
    probes = json.loads(specification.read_text())["probes"]
    failures = []
    for probe in probes:
        target = CONTENT / probe["file"]
        if not target.exists():
            failures.append(f"{probe['question']}: {probe['file']} does not exist")
            continue
        text = target.read_text()
        if "expect" in probe and not re.search(probe["expect"], text, re.MULTILINE):
            failures.append(f"{probe['question']}: {probe['expect']!r} not found")
        if "reject" in probe and re.search(probe["reject"], text, re.MULTILINE):
            failures.append(f"{probe['question']}: {probe['reject']!r} is present and must not be")
    if failures:
        raise Failure(f"{len(failures)} navigation probe(s) failed: {failures[:4]}")
    return {"probes": len(probes)}


# --------------------------------------------------------------------------- 5

def check_transferability() -> dict:
    """Nothing in the skill directory reaches outside it.

    Scans the generators, the rules and the prose -- not just `build/*.py` as the earliest
    sibling does. A hardcoded path in a rule or in SKILL.md breaks the copy-out guarantee just
    as completely as one in a builder.
    """
    scanned, offences = 0, []
    roots = [HERE.glob("*.py"), QUERIES.rglob("*.yml"), SKILL_DIR.glob("*.md"),
             QUERIES.glob("*.md"), HERE.glob("*.md"), HERE.glob("*.json")]
    for group in roots:
        for path in group:
            scanned += 1
            text = path.read_text(errors="replace")
            for pattern, why in HOST_COUPLING:
                for match in re.finditer(pattern, text):
                    # This file names the patterns it forbids, which is not an offence.
                    if path.name == "verify.py" and "HOST_COUPLING" in text[:match.start()]:
                        continue
                    offences.append(f"{path.name}: {why} ({match.group(0)!r})")
    if offences:
        raise Failure(f"{len(offences)} host coupling(s): {offences[:4]}")
    return {"scanned": scanned}


# --------------------------------------------------------------------------- 6

def check_behaviours(execute: bool) -> dict:
    """Every recorded observation still holds, control included.

    The library repositories in this family cannot do this: a recorded fact about DataFusion is
    a reading of its documentation. Here every row of `behaviors.tsv` is an executed command, so
    the index can be checked against the world rather than against itself.

    Re-execution is opt-in because it compiles DataFusion. Without `--execute-probes` the check
    still refuses a `divergent` row, because a probe whose control did not come out the other
    way demonstrates nothing and must never ship as evidence.
    """
    path = CONTENT / "index" / "behaviors.tsv"
    if not path.exists():
        raise Failure("behaviors.tsv is absent; probes have never run")
    recorded = {}
    for line in path.read_text().splitlines():
        if line:
            columns = line.split("\t")
            recorded[columns[0]] = columns[2]

    divergent = [i for i, v in recorded.items() if v == "divergent"]
    if divergent:
        raise Failure(
            f"{len(divergent)} probe(s) are divergent: {divergent[:5]}. A divergent probe is "
            f"one whose control did not come out the other way, so it demonstrates nothing."
        )
    if not execute:
        return {"probes": len(recorded), "re_executed": False,
                "blocked": sum(1 for v in recorded.values() if v == "blocked")}

    sys.path.insert(0, str(HERE))
    import probes as probe_module

    results = probe_module.run_all(HERE)
    drifted = [
        f"{r['id']}: recorded {recorded.get(r['id'])!r}, now {r['verdict']!r}"
        for r in results
        if recorded.get(r["id"]) != r["verdict"] and r["verdict"] != "blocked"
    ]
    if drifted:
        raise Failure(f"{len(drifted)} behaviour(s) changed: {drifted[:3]}")
    return {"probes": len(results), "re_executed": True}


# --------------------------------------------------------------------------- 7

def check_recipes() -> dict:
    """Every fenced command in the pages runs and finds something.

    Documentation that shows a command nobody ran is a liability rather than a reference.
    """
    sources = list((CONTENT / "seams").glob("*.md")) + list((CONTENT / "catalogs").glob("*.md")) \
        + list((CONTENT / "topics").glob("*.md"))
    failures, ran = [], 0
    for page in sources:
        for block in re.findall(r"```bash\n(.*?)```", page.read_text(), re.DOTALL):
            for command in block.strip().splitlines():
                command = command.strip()
                if not command or command.startswith("#") or "<" in command:
                    continue
                ran += 1
                code, stdout, _ = _run(["sh", "-c", command], SKILL_DIR)
                if code in (2, 8):
                    failures.append(f"{page.name}: `{command}` exited {code}")
                elif not stdout.strip():
                    failures.append(f"{page.name}: `{command}` returned nothing")
    if failures:
        raise Failure(f"{len(failures)} documented recipe(s) failed: {failures[:4]}")
    return {"recipes": ran}


# --------------------------------------------------------------------------- 8

def check_router() -> dict:
    """The curated router still points at rows that exist.

    `questions.tsv` is written, not extracted, so nothing else here would notice it going
    stale. A recipe that returns nothing is a confident pointer to an empty set, which is the
    precise failure this repository is built to prevent.
    """
    rows = [
        line for line in (CONTENT / "index" / "questions.tsv").read_text().splitlines() if line
    ]
    behaviours = CONTENT / "index" / "behaviors.tsv"
    probe_ids = {
        line.split("\t")[0] for line in behaviours.read_text().splitlines() if line
    } if behaviours.exists() else set()

    failures = []
    for row in rows:
        question, _area, _entry, _rejected, _why, recipe, probe = row.split("\t")
        code, stdout, _ = _run(["sh", "-c", recipe], SKILL_DIR)
        if code in (2, 8) or not stdout.strip():
            failures.append(f"{question!r}: `{recipe}` returned nothing (exit {code})")
        if probe != "-" and probe not in probe_ids:
            failures.append(f"{question!r}: cites probe {probe}, which did not run")
    if failures:
        raise Failure(f"{len(failures)} router row(s) are stale: {failures[:3]}")
    return {"questions": len(rows)}


# --------------------------------------------------------------------------- 9

def check_counts() -> dict:
    """Every number the prose states is read back from the artifact that produced it.

    This check exists because of the siblings. One of them says `42 regex constructs` where 44
    files exist; another repeats a figure of 116 deprecated identifiers whose own generated rule
    carries 3. Both numbers were hand-written, both drifted, and nothing in either repository
    would ever have noticed. Here a claim is registered in `counts.json` against the artifact
    that settles it, and stating a number the artifact does not support fails the build.
    """
    registry = HERE / "counts.json"
    if not registry.exists():
        return {"claims": 0, "note": "no registry"}
    provenance = _provenance()
    claims = json.loads(registry.read_text())["claims"]
    failures = []
    for claim in claims:
        source = claim["source"]
        if source.startswith("counts."):
            actual = provenance["counts"].get(source.split(".", 1)[1])
        elif source.startswith("wc:"):
            target = CONTENT / source.split(":", 1)[1]
            actual = len([line for line in target.read_text().splitlines() if line]) \
                if target.exists() else None
        elif source.startswith("rg:"):
            pattern, _, relative = source.split(":", 2)[1].partition("@")
            target = CONTENT / relative
            actual = len(re.findall(pattern, target.read_text(), re.MULTILINE)) \
                if target.exists() else None
        elif source.startswith("glob:"):
            actual = len(list(CONTENT.glob(source.split(":", 1)[1])))
        else:
            failures.append(f"{claim['name']}: unknown source {source!r}")
            continue
        if actual != claim["value"]:
            failures.append(
                f"{claim['name']}: prose says {claim['value']}, {source} says {actual}"
            )
            continue
        for page in claim.get("stated_in", []):
            target = SKILL_DIR / page
            if target.exists() and str(claim["value"]) not in target.read_text():
                failures.append(f"{claim['name']}: {page} no longer states {claim['value']}")
    if failures:
        raise Failure(f"{len(failures)} count(s) have drifted: {failures[:4]}")
    return {"claims": len(claims)}


# --------------------------------------------------------------------------- entry point

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--skip-rebuild", action="store_true")
    parser.add_argument(
        "--execute-probes",
        action="store_true",
        help="re-run the executed probes, which compiles DataFusion in the capsule.",
    )
    parser.add_argument("--navigation", default=str(HERE / "navigation.json"))
    arguments = parser.parse_args()

    checks = (
        ("determinism", lambda: check_determinism(arguments.skip_rebuild)),
        ("integrity", check_integrity),
        ("rule_tests", check_rule_tests),
        ("navigation", lambda: check_navigation(Path(arguments.navigation))),
        ("transferability", check_transferability),
        ("behaviours", lambda: check_behaviours(arguments.execute_probes)),
        ("recipes", check_recipes),
        ("router", check_router),
        ("counts", check_counts),
    )
    results: dict[str, object] = {}
    failed = 0
    for name, check in checks:
        try:
            results[name] = check()
            sys.stderr.write(f"  {name}: ok\n")
        except (Failure, FileNotFoundError, KeyError) as error:
            results[name] = {"failed": str(error)}
            failed += 1
            sys.stderr.write(f"  {name}: FAILED {error}\n")
    print(json.dumps(results, indent=2, default=str))
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())

"""Verify the built repository.

Five checks, in increasing order of what they can catch:

1. determinism    -- a rebuild reproduces every file byte for byte
2. integrity      -- every pointer between index, model, prose and corpus resolves
3. rule tests     -- every shipped rule still matches what it claims to
4. navigation     -- the recipes documented in reference.md actually find the answers
5. transferability -- nothing reaches outside the skill directory

These are integrity and navigation checks, not proof of semantic decision quality.
Contract/runtime checks and independent task evaluation provide separate evidence.

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

# `ast-grep test` exit codes, measured rather than assumed. 0 is success, 3 means the filter
# selected nothing, 4 means an assertion or snapshot failed, 6 a missing test directory, 8 an
# unparsable rule. It never returns 1.
TEST_OK = 0
TEST_FILTER_EMPTY = 3


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


class Failure(Exception):
    """A verification check did not hold."""


def check_determinism(content: Path, skip_rebuild: bool) -> str:
    provenance = json.loads(content.joinpath("PROVENANCE.json").read_text())
    recorded: dict[str, str] = provenance["files"]

    if not skip_rebuild:
        done = subprocess.run(
            [sys.executable, str(HERE.joinpath("build.py"))],
            capture_output=True,
            text=True,
            check=False,
        )
        if done.returncode != 0:
            raise Failure(f"rebuild failed: {done.stderr.strip()[-400:]}")
        regenerated = json.loads(content.joinpath("PROVENANCE.json").read_text())["files"]
        if set(regenerated) != set(recorded):
            raise Failure("rebuild changed the generated-file inventory")

    mismatched: list[str] = []
    missing: list[str] = []
    for relative, expected in recorded.items():
        target = content.joinpath(relative)
        if not target.exists():
            missing.append(relative)
        elif hashlib.sha256(target.read_bytes()).hexdigest() != expected:
            mismatched.append(relative)

    if missing or mismatched:
        detail = f"{len(missing)} missing, {len(mismatched)} changed"
        sample = (missing + mismatched)[:5]
        raise Failure(f"rebuild was not reproducible: {detail}; e.g. {sample}")
    action = (
        "match recorded digests (rebuild not run)" if skip_rebuild else "reproduced byte for byte"
    )
    return f"{len(recorded)} files {action}"


def check_integrity(content: Path) -> str:
    index = content.joinpath("index")
    symbols = {
        line.split("\t", 1)[0]
        for line in index.joinpath("symbols.tsv").read_text().splitlines()
        if line
    }
    problems: list[str] = []

    for name, column in (("aliases.tsv", 1), ("impls.tsv", 1), ("methods.tsv", 0)):
        unknown = 0
        for line in index.joinpath(name).read_text().splitlines():
            if not line:
                continue
            fields = line.split("\t")
            if len(fields) > column and fields[column] not in symbols:
                unknown += 1
        if unknown:
            problems.append(f"{name}: {unknown} rows referencing an unknown canonical path")

    # Every api_page named by symbols.tsv must exist, and every model `doc` pointer must resolve.
    pages = {
        line.split("\t")[3]
        for line in index.joinpath("symbols.tsv").read_text().splitlines()
        if line and len(line.split("\t")) > 3
    }
    absent = [page for page in sorted(pages) if not content.joinpath(page).exists()]
    if absent:
        problems.append(f"{len(absent)} api pages named by symbols.tsv do not exist")

    dangling = 0
    for document in content.joinpath("model").glob("*.json"):
        payload = json.loads(document.read_text())
        for item in payload.get("items", []):
            pointer = item.get("doc")
            if pointer and not content.joinpath(pointer.split("#", 1)[0]).exists():
                dangling += 1
    if dangling:
        problems.append(f"{dangling} model doc pointers do not resolve")

    if problems:
        raise Failure("; ".join(problems))
    return f"{len(symbols)} symbols, {len(pages)} pages, all cross-references resolve"


def check_rule_tests(queries: Path) -> str:
    """Run the rule fixtures, guarding the two ways this check lies about passing.

    A test file naming a rule id that does not exist exits 0 with `0 passed; 0 failed` -- a typo
    deletes the test rather than failing it. And `--update-all` rewrites snapshots and then always
    exits 0, so it can never be part of a gate. Assert on the number of cases executed.
    """
    done = subprocess.run(
        ["ast-grep", "test", "-c", str(queries.joinpath("sgconfig.yml")), "--color=never"],
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
        raise Failure(f"could not read a test tally from: {output.strip()[-200:]}")
    passed, failed = int(match.group(1)), int(match.group(2))

    rule_files = list(queries.joinpath("rules").rglob("*.yml"))
    if passed == 0:
        raise Failure("ast-grep test exited 0 having executed no cases; check the test `id` fields")
    if passed < len(rule_files):
        raise Failure(
            f"{len(rule_files)} rules but only {passed} cases ran; every rule needs fixtures"
        )
    if failed:
        raise Failure(f"{failed} rule tests failed")
    return f"{passed} rule cases passed, covering {len(rule_files)} rules"


def check_probes(content: Path, probes_path: Path) -> str:
    """Run each capability question through the documented recipe and confirm it lands."""
    probes = json.loads(probes_path.read_text())
    failures: list[str] = []
    for probe in probes["probes"]:
        target = content.joinpath(probe["file"])
        if not target.exists():
            failures.append(f"{probe['question']}: {probe['file']} absent")
            continue
        haystack = target.read_text()
        if not re.search(probe["expect"], haystack, re.MULTILINE):
            failures.append(f"{probe['question']}: no match for /{probe['expect']}/")
    if failures:
        raise Failure(f"{len(failures)} of {len(probes['probes'])} probes failed: {failures[:4]}")
    return f"{len(probes['probes'])} capability probes located their answers"


def check_transferability(skill_root: Path) -> str:
    """The builder must not import from, or hardcode a path into, any host repository."""
    offences: list[str] = []
    for source in sorted(HERE.glob("*.py")):
        text = source.read_text()
        for pattern, reason in (
            (r"\bfrom\s+enrichment", "imports a host crate"),
            (r"/home/[a-z]+/", "hardcodes an absolute home path"),
            (r"\blibrary_enrichment\b", "references the host project"),
        ):
            if re.search(pattern, text):
                offences.append(f"{source.name}: {reason}")
    if offences:
        raise Failure("; ".join(offences))
    return f"{len(list(HERE.glob('*.py')))} builder modules are host-independent"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Verify the built DataFusion repository.")
    parser.add_argument("--content", type=Path, default=SKILL_ROOT.joinpath("content"))
    parser.add_argument("--queries", type=Path, default=SKILL_ROOT.joinpath("queries"))
    parser.add_argument("--probes", type=Path, default=HERE.joinpath("probes.json"))
    parser.add_argument(
        "--skip-rebuild",
        action="store_true",
        help="check recorded digests without rebuilding first",
    )
    args = parser.parse_args(argv)

    checks = (
        ("determinism", lambda: check_determinism(args.content, args.skip_rebuild)),
        ("integrity", lambda: check_integrity(args.content)),
        ("rule tests", lambda: check_rule_tests(args.queries)),
        ("navigation probes", lambda: check_probes(args.content, args.probes)),
        ("transferability", lambda: check_transferability(SKILL_ROOT)),
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

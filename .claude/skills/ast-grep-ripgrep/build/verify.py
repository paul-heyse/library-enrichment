"""Verify the capability repository. Exit 1 on any failure.

Seven checks. Five are the ones every capability repository in this family carries; two exist
only because this repository's subjects are programs rather than libraries, and can therefore be
re-run rather than merely re-read.

    determinism     rebuild and compare byte-for-byte
    integrity       every file matches its recorded digest
    rule tests      the ast-grep fixtures, asserting how many cases actually ran
    probes          the documented navigation routes still lead somewhere
    behaviours      every recorded observation reproduces, control included
    recipes         every command printed in the documentation runs as documented
    transferable    the builder imports nothing from a host repository

The rule-test check deserves a note, because two of ast-grep's exit conventions make a naive
gate useless. `ast-grep test` returns 0 even when it reports `0 passed; 4 failed`, so the status
cannot be trusted; and `--filter` matching no rule exits 3, so a typo in a filter reads as a
clean run. Both are asserted on parsed output instead, and both are recorded in
`content/index/exit-codes.tsv` for the same reason.
"""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

BUILD_DIR = Path(__file__).resolve().parent
SKILL_DIR = BUILD_DIR.parent
CONTENT = SKILL_DIR / "content"
QUERIES = SKILL_DIR / "queries"

# Patterns that would mean the builder had grown a dependency on the repository it happens to
# live in. The whole point of a capability repository is that it can be copied out intact.
HOST_COUPLING = (
    (r"\bfrom\s+enrichment", "imports a host package"),
    (r"/home/[a-z]+/", "hardcodes an absolute home path"),
    (r"\blibrary_enrichment\b", "references the host project"),
    (r"\blibrary-enrichment\b", "references the host project"),
)


class Failure(RuntimeError):
    """A check did not pass."""


def _run(command: list[str], cwd: Path) -> tuple[int, str, str]:
    done = subprocess.run(
        command, cwd=cwd, capture_output=True, text=True, errors="replace", check=False
    )
    return done.returncode, done.stdout, done.stderr


# --------------------------------------------------------------------------- checks


def check_integrity() -> dict:
    """Every file in `content/` matches the digest recorded when it was built."""
    provenance = json.loads((CONTENT / "PROVENANCE.json").read_text(encoding="utf-8"))
    recorded = provenance["files"]
    mismatched, missing = [], []

    for relative, digest in recorded.items():
        path = CONTENT / relative
        if not path.exists():
            missing.append(relative)
        elif hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            mismatched.append(relative)

    present = {
        str(p.relative_to(CONTENT))
        for p in CONTENT.rglob("*")
        if p.is_file() and p.name != "PROVENANCE.json"
    }
    untracked = sorted(present - set(recorded))

    if mismatched or missing or untracked:
        raise Failure(
            f"{len(mismatched)} modified, {len(missing)} missing, {len(untracked)} untracked. "
            f"First few: {(mismatched + missing + untracked)[:5]}"
        )
    return {"files": len(recorded)}


def check_determinism() -> dict:
    """Rebuilding from the same acquired inputs reproduces the same bytes.

    This is the check that makes every other claim in the repository meaningful. If a rebuild
    can differ, a digest proves only that nothing changed since the last build -- not that the
    content follows from the pinned inputs.
    """
    before = json.loads((CONTENT / "PROVENANCE.json").read_text(encoding="utf-8"))["files"]
    code, _, stderr = _run([sys.executable, "build/build.py"], SKILL_DIR)
    if code != 0:
        raise Failure(f"rebuild failed: {stderr[-400:]}")
    after = json.loads((CONTENT / "PROVENANCE.json").read_text(encoding="utf-8"))["files"]

    changed = sorted(name for name in set(before) & set(after) if before[name] != after[name])
    added = sorted(set(after) - set(before))
    removed = sorted(set(before) - set(after))
    if changed or added or removed:
        raise Failure(
            f"rebuild was not reproducible: {len(changed)} changed, {len(added)} added, "
            f"{len(removed)} removed. First few: {(changed + added + removed)[:5]}"
        )
    return {"files": len(after)}


def check_rule_tests() -> dict:
    """Run the ast-grep fixtures, asserting on parsed output rather than exit status."""
    rule_files = sorted((QUERIES / "rules").rglob("*.yml"))
    code, stdout, stderr = _run(["ast-grep", "test", "-c", "sgconfig.yml"], QUERIES)
    report = stdout + stderr

    match = re.search(r"(\d+) passed; (\d+) failed", report)
    if not match:
        raise Failure(f"could not parse a test summary from: {report[-300:]}")
    passed, failed = int(match.group(1)), int(match.group(2))

    if failed:
        raise Failure(f"{failed} rule tests failed (exit status was {code}, which proves nothing)")
    if passed < len(rule_files) - 2:
        # model-* and corpus-* rules query generated content rather than fixtures.
        raise Failure(f"only {passed} tests ran for {len(rule_files)} rules; fixtures are missing")
    return {"rules": len(rule_files), "passed": passed}


def check_probes() -> dict:
    """Every documented navigation route still leads somewhere.

    A probe that keeps passing while the thing it describes moves underneath is worse than no
    probe: it certifies a route that no longer leads anywhere.
    """
    spec = json.loads((BUILD_DIR / "navigation.json").read_text(encoding="utf-8"))
    failures = []
    for probe in spec["probes"]:
        path = CONTENT / probe["file"]
        if not path.exists():
            failures.append(f"{probe['question']}: {probe['file']} does not exist")
            continue
        if not re.search(probe["expect"], path.read_text(encoding="utf-8"), re.MULTILINE):
            failures.append(f"{probe['question']}: expected {probe['expect']!r} in {probe['file']}")
    if failures:
        raise Failure(f"{len(failures)} navigation probes failed: {failures[:3]}")
    return {"probes": len(spec["probes"])}


def check_behaviours() -> dict:
    """Re-run every recorded observation, control included.

    The library capability repositories cannot do this: a recorded fact about DataFusion is a
    reading of its documentation. Here every row of `behaviors.tsv` is an executed command, so
    the index can be checked against the world rather than against itself.
    """
    import probes as probe_module

    recorded = {}
    for line in (CONTENT / "index" / "behaviors.tsv").read_text(encoding="utf-8").splitlines():
        if line:
            parts = line.split("\t")
            recorded[parts[0]] = parts[3]

    results = probe_module.run_all(BUILD_DIR)
    drifted = [
        f"{r['id']}: recorded {recorded.get(r['id'])!r}, now {r['verdict']!r}"
        for r in results
        if recorded.get(r["id"]) != r["verdict"]
    ]
    if drifted:
        raise Failure(f"{len(drifted)} behaviours changed: {drifted[:3]}")

    # The baseline is asserted, not inferred, so the check is an equality against the library
    # rather than a comparison of floors. A PCRE2 swapped underneath a built repository is
    # exactly the drift this is here to catch.
    import oracles

    provenance = json.loads((CONTENT / "PROVENANCE.json").read_text(encoding="utf-8"))
    baseline = provenance["pcre2"]["baseline"]
    if not oracles.pcre2_asserts(baseline):
        raise Failure(
            f"the linked PCRE2 no longer asserts as {baseline}. The repository was built against "
            f"that baseline, so every behavioural row in it now describes a different library."
        )
    return {"behaviours": len(results), "pcre2_baseline": baseline}


FENCE_RE = re.compile(r"^```[a-z]*\n(.*?)^```", re.MULTILINE | re.DOTALL)
# Non-capturing: `findall` returns the group rather than the whole match when one is captured,
# which would truncate every command to its first word and make the check vacuously pass.
COMMAND_RE = re.compile(r"^(?:rg|ast-grep) .+$", re.MULTILINE)


def _fenced_commands(text: str) -> list[str]:
    """Extract runnable commands from fenced blocks only.

    Prose is not a command. `ast-grep establishes syntax, not semantics` begins with a tool name
    and is a sentence; scraping it and running it produces a usage error that says nothing about
    the documentation being wrong. Only fenced blocks are executable claims.
    """
    commands = []
    for block in FENCE_RE.findall(text):
        for line in COMMAND_RE.findall(block):
            command = line.strip()
            # Templates and pipelines are not literals we can run as written.
            if any(token in command for token in ("PATH", "<", "|", "$(", "xargs")):
                continue
            commands.append(command)
    return commands


def check_recipes() -> dict:
    """Every command printed in a fenced block runs, and does not error.

    Documentation that shows a command nobody ran is a liability rather than a reference. Exit 1
    (a clean no-match) is fine here; exit 2 from ripgrep or exit 8 from ast-grep is not, because
    both mean the command itself was malformed.
    """
    sources = [SKILL_DIR / "SKILL.md", SKILL_DIR / "reference.md"]
    sources.extend(sorted((CONTENT / "topics").glob("*.md")))

    checked, failures = 0, []
    for source in sources:
        if not source.exists():
            continue
        for command in _fenced_commands(source.read_text(encoding="utf-8")):
            code, _, stderr = _run(["sh", "-c", command], SKILL_DIR)
            checked += 1
            if code in (2, 8):
                failures.append(f"{source.name}: `{command}` exited {code}: {stderr[:120]}")
    if failures:
        raise Failure(f"{len(failures)} documented commands failed: {failures[:3]}")
    return {"recipes": checked}


def check_transferability() -> dict:
    """The builder depends on nothing in whatever repository it happens to live in."""
    offenders = []
    for path in sorted(BUILD_DIR.glob("*.py")):
        text = path.read_text(encoding="utf-8")
        for pattern, reason in HOST_COUPLING:
            if re.search(pattern, text):
                offenders.append(f"{path.name}: {reason}")
    if offenders:
        raise Failure(f"builder is coupled to its host: {offenders}")
    return {"modules": len(list(BUILD_DIR.glob("*.py")))}


# --------------------------------------------------------------------------- entry point


CHECKS = (
    ("integrity", check_integrity),
    ("rule_tests", check_rule_tests),
    ("probes", check_probes),
    ("behaviours", check_behaviours),
    ("recipes", check_recipes),
    ("transferability", check_transferability),
    ("determinism", check_determinism),
)


def main(argv: list[str] | None = None) -> int:
    argv = sys.argv[1:] if argv is None else argv
    skip_rebuild = "--skip-rebuild" in argv
    only = [a for a in argv if not a.startswith("--")]

    results: dict[str, object] = {}
    failed = False
    for name, check in CHECKS:
        if only and name not in only:
            continue
        if name == "determinism" and skip_rebuild:
            results[name] = "skipped"
            continue
        try:
            results[name] = check()
        except Failure as exc:
            results[name] = {"failed": str(exc)}
            failed = True
        except Exception as exc:  # a check that crashes is a check that failed
            results[name] = {"errored": f"{type(exc).__name__}: {exc}"}
            failed = True

    sys.stdout.write(json.dumps(results, indent=2) + "\n")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())

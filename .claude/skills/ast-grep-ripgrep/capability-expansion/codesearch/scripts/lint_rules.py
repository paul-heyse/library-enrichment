"""Run the ast-grep rule corpus and its fixtures, asserting on OUTPUT rather than on exit status.

Two of ast-grep's exit conventions make a naive gate useless, and both are recorded in the
sibling skill's own `content/index/exit-codes.tsv` because they were measured rather than assumed:

    ast-grep test   returns 0 even when it reports `0 passed; 4 failed`
    ast-grep scan   returns 0 when a rule file fails to PARSE, reporting the error on stderr
    --filter        matching no rule exits 3, so a typo in a filter reads as a clean run

Every one of those was observed while writing this corpus: the first run of `ast-grep test` printed
`0 passed; 4 failed` and exited 0, and a `constraints:` block in the wrong place printed
`Cannot parse rule` and exited 0. A gate that trusted the status would have passed both.

So this parses the counts, requires them to be non-zero, and requires the rule count it sees to
match the number of rule files on disk -- because a corpus that silently stopped being discovered
is the failure mode a green run cannot distinguish from a clean one.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CONFIG = ROOT / "queries" / "sgconfig.yml"
RULE_DIR = ROOT / "queries" / "rules"

#: What the corpus is scanned against. Explicit for the reason given at the call site below.
SCAN_PATHS = ["crates", "python", "scripts"]


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["ast-grep", *args], capture_output=True, text=True, check=False, cwd=ROOT
    )


def main() -> int:
    write = sys.stdout.write
    rules = sorted(RULE_DIR.rglob("*.yml"))
    if not rules:
        write("failed: no rule files found -- an empty corpus checks nothing\n")
        return 1

    # ---- the fixtures ----------------------------------------------------------------------
    test = run(["test", "-c", str(CONFIG)])
    combined = test.stdout + test.stderr
    match = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", combined)
    if not match:
        write("failed: could not parse a test result from ast-grep\n")
        write(combined[-2000:])
        return 1
    passed, failed = int(match.group(1)), int(match.group(2))
    if failed:
        write(f"failed: {failed} rule fixture(s) failed\n")
        write(combined[-4000:])
        return 1
    if passed != len(rules):
        write(
            f"failed: {passed} rule(s) were tested but {len(rules)} exist on disk. A rule with "
            "no fixture is a rule nobody has seen fail.\n"
        )
        return 1

    # ---- the corpus, against real source ---------------------------------------------------
    #
    # `--json` so a parse failure is a missing/!=0 document rather than prose this has to read.
    # Paths are explicit because ast-grep takes the config's own directory as the project root --
    # `--inspect summary` reports `projectDir=queries`. Without them the scan walks `queries/` and
    # finds nothing, reports zero violations, and exits 0. That is how this gate first passed
    # against a deliberately planted violation.
    scan = run(["scan", "-c", str(CONFIG), "--json=compact", *SCAN_PATHS])
    if scan.returncode not in (0, 1):
        write(f"failed: ast-grep scan exited {scan.returncode}\n{scan.stderr[-2000:]}\n")
        return 1
    try:
        findings = json.loads(scan.stdout or "[]")
    except json.JSONDecodeError:
        # This is the parse-failure path: no JSON on stdout, the reason on stderr, status 0.
        write("failed: ast-grep produced no JSON -- a rule file probably does not parse\n")
        write(scan.stderr[-2000:])
        return 1
    if findings:
        write(f"failed: {len(findings)} rule violation(s) in the working tree\n")
        for f in findings[:10]:
            line = f.get("range", {}).get("start", {}).get("line")
            write(f"  {f.get('file')}:{line}  {f.get('ruleId')}\n")
        return 1

    write(f"rules  {len(rules)} rule(s), {passed} fixture set(s), 0 violations\n")
    for rule in rules:
        write(f"       {rule.relative_to(ROOT)}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

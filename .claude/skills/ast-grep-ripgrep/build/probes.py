"""Execute the pinned binaries against a fixture tree and record what actually happened.

This is the stage the sibling capability repositories could not have. DataFusion and delta-rs are
libraries: the only thing a builder can do with them is read their documented surface. ast-grep
and ripgrep are *programs*, so the surface can be executed, and a claim about behaviour can be an
observation rather than a reading.

Two rules make the observations worth trusting.

**A probe without a control is not evidence.** A pattern that matches proves only that something
matched. It cannot separate "the feature works" from "the pattern matched for an unrelated
reason". So a feature probe carries a second command, identical except for the feature under
test, which must come out the other way. If either half surprises us the probe records
`inconclusive`, never a pass.

**A version string is not a capability.** `rg --pcre2-version` prints a constant compiled in from
the PCRE2 headers at ripgrep's build time, so it describes the build and not the runtime. On the
machine this repository targets it reports 10.45 while the linked library is 10.48. PCRE2 10.48
is the asserted baseline -- `build.py` refuses to run otherwise -- so these probes establish what
a construct does, not whether you have it. The version string is recorded beside them as the
discrepancy it is.
"""

from __future__ import annotations

import json
import shutil
import subprocess
import tempfile
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

TIMEOUT_SECONDS = 30

# A probe's verdict. `inconclusive` is a first-class outcome, not a failure to be smoothed over:
# it is what an honest index says when the evidence did not come out cleanly.
CONFIRMED = "confirmed"
REFUTED = "refuted"
INCONCLUSIVE = "inconclusive"
RECORDED = "recorded"


class ProbeError(RuntimeError):
    """A probe could not be executed at all, as distinct from coming out negative."""


@contextmanager
def materialised_tree(fixtures: Path) -> Iterator[Path]:
    """Write the fixture manifest into a scratch directory, yield it, then remove it.

    The tree is a manifest rather than a committed directory because two of its files must be
    named `.gitignore` and `.ignore`. Committed under those names they would be obeyed by the
    host repository's git and by any ripgrep run that walked this skill, changing what unrelated
    searches see. Hidden fixtures have the same problem in reverse: several probes exist to prove
    hidden files are skipped by default, which cannot be shown if the checkout already hid them.
    """
    spec = json.loads((fixtures / "tree.json").read_text())
    renames = spec.get("renames", {})
    root = Path(tempfile.mkdtemp(prefix="ast-grep-ripgrep-probe-"))
    try:
        for name, content in spec["files"].items():
            relative = renames.get(name, name)
            target = root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content, encoding="utf-8")
        for name, hexdata in spec.get("binary_files", {}).items():
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(bytes.fromhex(hexdata))
        yield root
    finally:
        shutil.rmtree(root, ignore_errors=True)


def _run(command: list[str], cwd: Path) -> tuple[int, str, str]:
    if not shutil.which(command[0]):
        raise ProbeError(f"{command[0]!r} is not on PATH")
    try:
        proc = subprocess.run(
            command,
            cwd=cwd,
            capture_output=True,
            text=True,
            errors="replace",
            timeout=TIMEOUT_SECONDS,
            check=False,
        )
    except subprocess.TimeoutExpired:
        return 124, "", f"timed out after {TIMEOUT_SECONDS}s"
    return proc.returncode, proc.stdout, proc.stderr


def _satisfies(expect: str, code: int, stdout: str) -> bool:
    """Decide whether one half of a probe came out as expected.

    Exit codes carry meaning here and are not collapsed: for ripgrep 0 is a match, 1 is a clean
    no-match, and 2 is a real error. A probe that expects `no-match` must see 1, not merely
    "something nonzero".
    """
    if expect == "any":
        return True
    if expect == "match":
        return code == 0
    if expect == "no-match":
        return code == 1
    if expect == "error":
        return code == 2
    if expect.startswith("exit:"):
        return code == int(expect.split(":", 1)[1])
    if expect == "no-match-or-binary-notice":
        return code != 0 or "binary file" in stdout.lower()
    if expect.startswith("no-match-on:"):
        return expect.split(":", 1)[1] not in stdout
    return expect in stdout


def evaluate(probe: dict, root: Path) -> dict:
    """Run one probe and its control, and record the observation verbatim."""
    code, stdout, stderr = _run(probe["command"], root)
    primary_ok = _satisfies(probe.get("expect", "any"), code, stdout)

    record = {
        "id": probe["id"],
        "tool": probe["tool"],
        "topic": probe.get("topic", ""),
        "construct": probe.get("construct", ""),
        "since_pcre2": probe.get("since_pcre2", ""),
        "question": probe["question"],
        "command": " ".join(probe["command"]),
        "exit": code,
        "stdout": stdout.strip(),
        "stderr": stderr.strip(),
        "expect": probe.get("expect", "any"),
        "note": probe.get("note", ""),
    }

    control = probe.get("control")
    mode = probe.get("control_mode", "opposite")

    if control is None:
        # A shape probe. There is nothing to falsify, so the observation stands on its own and
        # is labelled as recorded rather than confirmed.
        record["verdict"] = RECORDED if primary_ok else INCONCLUSIVE
        record["control"] = ""
        record["control_exit"] = ""
        return record

    ccode, cstdout, cstderr = _run(control, root)
    record["control"] = " ".join(control)
    record["control_exit"] = ccode
    record["control_stdout"] = cstdout.strip()
    record["control_stderr"] = cstderr.strip()

    if mode == "record-both":
        record["verdict"] = RECORDED if primary_ok else INCONCLUSIVE
    elif mode == "differs":
        # Both halves run; the finding is that their output is not the same.
        control_ok = _satisfies(probe.get("control_expect", "any"), ccode, cstdout)
        differs = stdout.strip() != cstdout.strip()
        record["verdict"] = CONFIRMED if (primary_ok and control_ok and differs) else INCONCLUSIVE
        record["differs"] = differs
    else:
        control_ok = _satisfies(probe.get("control_expect", "no-match"), ccode, cstdout)
        if primary_ok and control_ok:
            record["verdict"] = CONFIRMED
        elif not primary_ok and control_ok:
            record["verdict"] = REFUTED
        else:
            record["verdict"] = INCONCLUSIVE
    return record


def run_all(build_dir: Path) -> list[dict]:
    """Execute every probe against a freshly materialised fixture tree."""
    spec = json.loads((build_dir / "probes.json").read_text())
    results = []
    with materialised_tree(build_dir / "fixtures") as root:
        for probe in spec["probes"]:
            results.append(evaluate(probe, root))
    return results


def summarise(results: list[dict]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for record in results:
        counts[record["verdict"]] = counts.get(record["verdict"], 0) + 1
    return counts

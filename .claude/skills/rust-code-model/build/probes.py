"""Execute the subjects and record what they actually did.

These layers are reached through programs, so a claim about them can be an observation rather
than a reading. That is the one thing this repository can do that a library index cannot, and
it is worth the machinery: the version of a fact that has been executed does not rot quietly.

**Every behavioural probe carries a control that must come out the other way.** A probe alone
cannot tell "the feature works" from "the pattern matched for an unrelated reason". `switchInt`
appearing in a MIR dump proves nothing until a straight-line function is shown not to produce
one. A probe whose control also passes is recorded as `divergent`, never as a confirmation.

Determinism. `behaviors.tsv` is part of `content/`, which must rebuild byte-for-byte, so raw
output cannot be stored: it carries temporary paths, timings and pass numbering that differ per
run. What is stored instead is the *matched evidence* -- the exact text the expectation found --
with the fixture root rewritten to a placeholder. That is deterministic when the claim is true
and changes when it stops being true, which is precisely the sensitivity a drift check wants.

Standard library only.
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

FIXTURE_PLACEHOLDER = "<FIXTURES>"
EVIDENCE_LIMIT = 160

VERDICT_CONFIRMED = "confirmed"
VERDICT_RECORDED = "recorded"
VERDICT_DIVERGENT = "divergent"
VERDICT_BLOCKED = "blocked"


class ProbeError(RuntimeError):
    """A probe is malformed. Not the same as a probe that failed to confirm."""


# --------------------------------------------------------------------------- fixtures


def materialise(tree: dict[str, str], root: Path) -> None:
    """Write the fixture tree to a scratch directory.

    The fixtures are a manifest rather than real files on purpose. A checked-in directory of
    deliberately broken Rust, a `Cargo.toml` and a `.rs` file with no trailing newline is a
    hazard to whatever repository this skill is copied into: editors, formatters and the host
    project's own lint corpus would all find it. Written to a temporary directory at probe time
    it cannot be found by anything.
    """
    for relative, contents in tree.items():
        path = root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")


# --------------------------------------------------------------------------- expectations


def _normalise(text: str, root: Path) -> str:
    return text.replace(str(root), FIXTURE_PLACEHOLDER)


def _satisfies(expectation: str, code: int, output: str) -> tuple[bool, str]:
    """Evaluate one expectation, returning (held, evidence).

    Four forms, and the prefix is required for all but the commonest:

        some text        the output contains it
        absent:X         the output does not contain X
        exit:N           the process exited N
        re:PATTERN       the output matches the regular expression
    """
    if expectation.startswith("exit:"):
        wanted = int(expectation[5:])
        return code == wanted, f"exit {code}"
    if expectation.startswith("absent:"):
        needle = expectation[7:]
        return needle not in output, "-"
    if expectation.startswith("re:"):
        found = re.search(expectation[3:], output, re.MULTILINE)
        return bool(found), found.group(0) if found else "-"
    return expectation in output, expectation if expectation in output else "-"


# --------------------------------------------------------------------------- execution


def _render(argv: list[str], root: Path, toolchain: str) -> list[str]:
    return [token.format(toolchain=toolchain, fixtures=root) for token in argv]


def _execute(argv: list[str], root: Path, toolchain: str, spec: dict) -> tuple[int, str]:
    rendered = _render(argv, root, toolchain)
    if shutil.which(rendered[0]) is None:
        raise FileNotFoundError(rendered[0])
    # Several rust-analyzer subcommands read a source file on stdin and take no path at all,
    # and two of them (`ssr`, `search`) operate on the current directory implicitly. Both are
    # expressible here so that a probe records the invocation that actually works.
    stdin = None
    if spec.get("stdin"):
        stdin = (root / spec["stdin"]).read_text(encoding="utf-8")
    working = root / spec["cwd"] if spec.get("cwd") else root
    done = subprocess.run(
        rendered,
        cwd=working,
        input=stdin,
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
        timeout=300,
    )
    return done.returncode, _normalise(done.stdout + done.stderr, root)


def run_probe(spec: dict, root: Path, toolchain: str) -> dict:
    """Run one probe and its control, and decide what was actually shown."""
    for required in ("id", "layer", "question", "command", "expect"):
        if required not in spec:
            raise ProbeError(f"probe {spec.get('id', '?')} has no {required!r}")

    try:
        code, output = _execute(spec["command"], root, toolchain, spec)
    except FileNotFoundError as missing:
        return {
            **_base(spec, toolchain, root),
            "verdict": VERDICT_BLOCKED,
            "exit": "-",
            "evidence": f"{missing} is not on PATH",
            "control_exit": "-",
        }
    except subprocess.TimeoutExpired:
        return {
            **_base(spec, toolchain, root),
            "verdict": VERDICT_BLOCKED,
            "exit": "-",
            "evidence": "timed out",
            "control_exit": "-",
        }

    held, evidence = _satisfies(spec["expect"], code, output)

    control = spec.get("control")
    if control is None:
        # A shape probe: there is nothing for a control to falsify, so the row records what was
        # seen without claiming it was discriminating. `recorded` is weaker than `confirmed`
        # and the index keeps them apart rather than flattering the weaker one.
        return {
            **_base(spec, toolchain, root),
            "verdict": VERDICT_RECORDED if held else VERDICT_DIVERGENT,
            "exit": str(code),
            "evidence": evidence[:EVIDENCE_LIMIT],
            "control_exit": "-",
        }

    try:
        control_code, control_output = _execute(
            control, root, toolchain, {**spec, "stdin": spec.get("control_stdin")}
        )
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return {
            **_base(spec, toolchain, root),
            "verdict": VERDICT_BLOCKED,
            "exit": str(code),
            "evidence": "control could not run",
            "control_exit": "-",
        }

    control_held, _ = _satisfies(spec["control_expect"], control_code, control_output)
    return {
        **_base(spec, toolchain, root),
        "verdict": VERDICT_CONFIRMED if held and control_held else VERDICT_DIVERGENT,
        "exit": str(code),
        "evidence": evidence[:EVIDENCE_LIMIT],
        "control_exit": str(control_code),
    }


def _base(spec: dict, toolchain: str, root: Path) -> dict:
    return {
        "id": spec["id"],
        "layer": spec["layer"],
        "topic": spec.get("topic", "-"),
        "question": spec["question"],
        "command": " ".join(_render(spec["command"], root, toolchain)).replace(
            str(root), FIXTURE_PLACEHOLDER
        ),
        "expect": spec["expect"],
        "control": (
            " ".join(_render(spec["control"], root, toolchain)).replace(
                str(root), FIXTURE_PLACEHOLDER
            )
            if spec.get("control")
            else "-"
        ),
        "note": spec.get("note", ""),
    }


# --------------------------------------------------------------------------- entry points


def run_all(build_dir: Path) -> list[dict]:
    """Materialise the fixtures once and run every probe against them."""
    specs = json.loads((build_dir / "probes.json").read_text(encoding="utf-8"))
    tree = json.loads((build_dir / "fixtures" / "tree.json").read_text(encoding="utf-8"))
    manifest = json.loads(
        (build_dir / "manifests" / "rust-code-model.json").read_text(encoding="utf-8")
    )
    toolchain = manifest["tools"]["toolchain"]

    identifiers = [spec["id"] for spec in specs["probes"]]
    if len(set(identifiers)) != len(identifiers):
        raise ProbeError("probe ids are not unique")

    results: list[dict] = []
    with tempfile.TemporaryDirectory(prefix="rust-code-model-probes-") as workspace:
        root = Path(workspace).resolve()
        materialise(tree, root)
        for spec in specs["probes"]:
            results.append(run_probe(spec, root, toolchain))
    return sorted(results, key=lambda row: row["id"])


def _clean(text: str) -> str:
    return " ".join(str(text).replace("\t", " ").split())


def write_index(results: list[dict], out: Path) -> int:
    """`probe · layer · topic · verdict · exit · question · command · evidence · control ...`"""
    rows = [
        "\t".join(
            (
                row["id"],
                row["layer"],
                row["topic"],
                row["verdict"],
                row["exit"],
                _clean(row["question"]),
                _clean(row["command"]),
                _clean(row["evidence"]),
                _clean(row["control"]),
                row["control_exit"],
            )
        )
        for row in results
    ]
    (out / "behaviors.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")
    return len(rows)


def summarise(results: list[dict]) -> str:
    tally: dict[str, int] = {}
    for row in results:
        tally[row["verdict"]] = tally.get(row["verdict"], 0) + 1
    return ", ".join(f"{verdict}={count}" for verdict, count in sorted(tally.items()))

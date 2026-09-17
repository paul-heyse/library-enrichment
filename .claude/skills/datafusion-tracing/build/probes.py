"""Execute the probe capsule and record what the span stream actually contained.

The sibling library repositories cannot do this. `ast-grep-ripgrep/build/probes.py` says so
outright -- "DataFusion and delta-rs are libraries: the only thing a builder can do with them is
read their documented surface" -- and for those subjects it is true. This library is different:
it *emits*, into a buffer, deterministically, so every behavioural claim here can be an
observation rather than a reading.

The protocol is the merge of the two that exist in this family:

    from ast-grep-ripgrep   the control, and the three control modes, and `refuted` as a real
                            verdict rather than a failure to confirm
    from rust-code-model    the expectation DSL, `blocked` for a missing prerequisite, and
                            matched-evidence storage so the captures survive the byte-for-byte
                            determinism gate

Four verdicts and no fifth:

    confirmed   the probe held and the control came out the other way
    refuted     the control discriminated and the probe did not hold -- the claim is disproved
    recorded    a shape probe with nothing to falsify; weaker, and labelled as such
    divergent   the control did not discriminate, so nothing was demonstrated. Fails verify.py
    blocked     cargo, the toolchain or the capsule was unavailable. Never "the behaviour is
                absent"
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

import fetch

HERE = Path(__file__).resolve().parent
ACQUIRED = HERE / "acquired"
FIXTURES = HERE / "fixtures" / "probe-crate"

CONFIRMED = "confirmed"
REFUTED = "refuted"
RECORDED = "recorded"
DIVERGENT = "divergent"
BLOCKED = "blocked"

EVIDENCE_LIMIT = 160


class ProbeError(RuntimeError):
    """The probe harness itself could not run, which is never a verdict about the subject."""


def _capsule() -> Path:
    for variable in ("DATAFUSION_TRACING_CAPSULE", "RUST_SKILL_ACQUIRE_CAPSULE"):
        chosen = os.environ.get(variable)
        if chosen:
            return Path(chosen).expanduser()
    cache_home = os.environ.get("XDG_CACHE_HOME")
    base = Path(cache_home).expanduser() if cache_home else Path.home() / ".cache"
    return base / "rust-skill-acquire" / "datafusion-tracing"


def _satisfies(expectation: str, output: str, code: int) -> tuple[bool, str]:
    """Decide one half of a probe, and return the evidence that decided it.

    What is returned is the *matched text*, not the capture: a capture carries paths, ordering
    and lengths that differ per run, and `content/` has to rebuild byte for byte. Matched
    evidence is deterministic while the claim holds and changes when it stops holding.
    """
    if expectation.startswith("exit:"):
        wanted = int(expectation.split(":", 1)[1])
        return code == wanted, f"exit {code}"
    if expectation.startswith("absent:"):
        needle = expectation.split(":", 1)[1]
        return needle not in output, f"absent {needle!r}"
    if expectation.startswith("re:"):
        pattern = expectation.split(":", 1)[1]
        match = re.search(pattern, output)
        return bool(match), (match.group(0)[:EVIDENCE_LIMIT] if match else "")
    found = expectation in output
    if not found:
        return False, ""
    start = output.index(expectation)
    window = output[max(0, start - 40): start + len(expectation) + 40]
    return True, window.replace("\n", " ")[:EVIDENCE_LIMIT]


# --------------------------------------------------------------------------- capsule

def _toolchain() -> dict:
    manifest = json.loads((HERE / "manifests" / "datafusion-tracing.json").read_text())
    return manifest["tools"]["probe_toolchain"]


def toolchain_reading() -> str:
    done = subprocess.run(
        ["rustc", f"+{_toolchain()['toolchain']}", "--version"],
        capture_output=True, text=True, check=False,
    )
    return done.stdout.strip() or "unavailable"


def prepare(capsule: Path) -> Path:
    """Copy the probe crate into the capsule and build it on the pinned STABLE toolchain.

    Pinned, not inherited. A probe is a claim about what a consumer will observe, and a consumer
    builds on stable -- but this workstation's rustup default is nightly, so an unpinned build
    quietly reports what a nightly compiler produced and labels it the behaviour of the library.
    The first run of this module did exactly that, which is why the assertion below exists.
    """
    if shutil.which("cargo") is None:
        raise ProbeError("cargo is not on PATH")
    pinned = _toolchain()
    reading = subprocess.run(
        ["rustc", f"+{pinned['toolchain']}", "--version"],
        capture_output=True, text=True, check=False,
    )
    if reading.returncode != 0:
        raise ProbeError(
            f"rustc +{pinned['toolchain']} is not installed; install it or re-pin "
            f"tools.probe_toolchain deliberately"
        )
    missing = [token for token in pinned["rustc_expected"] if token not in reading.stdout]
    if missing:
        raise ProbeError(
            f"rustc +{pinned['toolchain']} reports {reading.stdout.strip()!r}, missing {missing}"
        )
    target = capsule / "probe-crate"
    target.mkdir(parents=True, exist_ok=True)
    for source in FIXTURES.rglob("*"):
        if source.is_file():
            destination = target / source.relative_to(FIXTURES)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
    environment = dict(os.environ)
    for variable in ("RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_BUILD_TARGET_DIR"):
        environment.pop(variable, None)
    environment["CARGO_TARGET_DIR"] = str(capsule / "target")
    environment["RUSTUP_TOOLCHAIN"] = pinned["toolchain"]
    # --locked, because a probe is a pinned observation and an unpinned resolution is not one.
    # The capsule's Cargo.lock is committed beside its Cargo.toml: without it, cargo resolves
    # fresh on every run and a later patch release of any of ~300 dependencies would silently
    # change what the probes observed while every verdict still read `confirmed`.
    result = subprocess.run(
        ["cargo", "build", "--release", "--locked", "--bin", "probe"],
        cwd=target, env=environment, capture_output=True, text=True, check=False,
    )
    if result.returncode != 0:
        tail = "\n".join(result.stderr.strip().splitlines()[-25:])
        raise ProbeError(f"the probe capsule does not build:\n{tail}")
    binary = capsule / "target" / "release" / "probe"
    if not binary.exists():
        raise ProbeError(f"{binary} was not produced")
    return binary


def _run_scenario(binary: Path, scenario: str) -> tuple[int, str]:
    result = subprocess.run(
        [str(binary), scenario], capture_output=True, text=True, check=False, timeout=300
    )
    return result.returncode, result.stdout


# --------------------------------------------------------------------------- documents probe

def _documents_probe(probe: dict) -> dict:
    """The tripwire: compare the hosted document against the private capture.

    Not a span observation. It exists because this repository's central claim -- that no
    published artifact carries the option builders -- would become vacuous the moment upstream
    re-exported them, and a repository that kept asserting a vacuous claim would be worse than
    one that never made it.
    """
    record = json.loads((ACQUIRED / "ACQUISITION.json").read_text())["crates"]
    subject = next((p for p, r in record.items() if "private" in r), None)
    if subject is None:
        return {**_base(probe), "verdict": BLOCKED, "evidence": "no private capture acquired"}
    version = record[subject]["version"]
    root = ACQUIRED / "rustdoc"
    hosted = json.loads(fetch.decompress_zstd((root / f"{subject}@{version}.json.zst").read_bytes()))
    private = json.loads(
        fetch.decompress_zstd((root / f"{subject}@{version}.private.json.zst").read_bytes())
    )
    hosted_code, hosted_text = _interrogate(hosted, probe["query"])
    private_code, private_text = _interrogate(private, probe["query"])
    primary_ok, evidence = _satisfies(probe["expect"], hosted_text, hosted_code)
    control_ok, control_evidence = _satisfies(
        probe["control_expect"], private_text, private_code
    )
    verdict = CONFIRMED if (primary_ok and control_ok) else (
        REFUTED if (not primary_ok and control_ok) else DIVERGENT
    )
    return {
        **_base(probe),
        "verdict": verdict,
        "construction": f"{probe['query']} in docs.rs {subject}@{version}",
        # An `exit:N` expectation decides the probe but carries no evidence, so the
        # interrogation's own answer is preferred when there is one. A row whose evidence reads
        # `exit 0` is technically true and tells a reader nothing.
        "evidence": hosted_text or evidence or control_evidence,
        "control": "the private capture of the same version",
    }


def _interrogate(document: dict, query: str) -> tuple[int, str]:
    """Ask one rustdoc document a question. Exit 0 means found, 1 means absent.

    Two query forms, and the difference between them is the whole finding. `item:` asks whether
    any INDEX ENTRY carries the name -- whether a reader could look the method up. `prose:` asks
    whether the name appears in documentation TEXT -- whether a reader would see it being used.
    A name that is prose-present and item-absent is a method the documentation demonstrates and
    does not document.
    """
    kind, _, needle = query.partition(":")
    if kind == "item":
        for entry in document["index"].values():
            if entry.get("name") == needle:
                return 0, f"index entry {needle!r}"
        return 1, f"no index entry named {needle!r}"
    if kind == "prose":
        for entry in document["index"].values():
            docs = entry.get("docs") or ""
            if needle in docs:
                start = docs.index(needle)
                window = docs[max(0, start - 60): start + len(needle) + 20]
                return 0, window.replace("\n", " ")[:EVIDENCE_LIMIT]
        return 1, f"{needle!r} appears in no doc comment"
    raise ProbeError(f"unknown document query {query!r}")


def _base(probe: dict) -> dict:
    return {
        "id": probe["id"],
        "topic": probe["topic"],
        "question": probe["question"],
        "expect": probe["expect"],
        "control_expect": probe.get("control_expect", "-"),
        "mode": probe.get("control_mode", "opposite"),
        "note": probe.get("note", ""),
    }


# --------------------------------------------------------------------------- evaluation

def evaluate(probe: dict, binary: Path | None) -> dict:
    if probe.get("kind") == "documents":
        return _documents_probe(probe)
    if binary is None:
        return {**_base(probe), "verdict": BLOCKED, "evidence": "the probe capsule did not build"}

    code, output = _run_scenario(binary, probe["scenario"])
    primary_ok, evidence = _satisfies(probe["expect"], output, code)
    mode = probe.get("control_mode", "opposite")
    control = probe.get("control")
    record = {
        **_base(probe),
        "construction": probe["scenario"],
        "control": control or "-",
        "evidence": evidence,
    }
    if control is None:
        record["verdict"] = RECORDED if primary_ok else DIVERGENT
        return record

    control_code, control_output = _run_scenario(binary, control)
    control_ok, control_evidence = _satisfies(
        probe.get("control_expect", "-"), control_output, control_code
    )
    if mode == "differs":
        differs = output.strip() != control_output.strip()
        record["verdict"] = CONFIRMED if (primary_ok and control_ok and differs) else DIVERGENT
        record["evidence"] = evidence or control_evidence
        if not differs:
            record["evidence"] = "the two captures were identical"
    elif mode == "record-both":
        record["verdict"] = RECORDED if (primary_ok and control_ok) else DIVERGENT
        record["evidence"] = evidence or control_evidence
    else:
        if primary_ok and control_ok:
            record["verdict"] = CONFIRMED
        elif not primary_ok and control_ok:
            record["verdict"] = REFUTED
        else:
            record["verdict"] = DIVERGENT
    return record


def run_all(build_dir: Path) -> list[dict]:
    specification = json.loads((build_dir / "probes.json").read_text())
    try:
        binary = prepare(_capsule())
    except ProbeError as error:
        sys.stderr.write(f"  probe capsule unavailable: {error}\n")
        binary = None
    return [evaluate(probe, binary) for probe in specification["probes"]]


# --------------------------------------------------------------------------- emission

def write_all(content: Path, build_dir: Path, results: list[dict] | None = None) -> dict:
    results = run_all(build_dir) if results is None else results
    index = content / "index"
    index.mkdir(parents=True, exist_ok=True)
    rows = [
        "\t".join((
            record["id"],
            record["topic"],
            record["verdict"],
            record["question"],
            record.get("construction", "-"),
            record["expect"],
            record.get("control", "-"),
            record["control_expect"],
            record["mode"],
            (record.get("evidence") or "-").replace("\t", " ").replace("\n", " "),
        ))
        for record in results
    ]
    (index / "behaviors.tsv").write_text("".join(f"{row}\n" for row in sorted(rows)))
    _write_page(content, results)
    tally: dict[str, int] = {}
    for record in results:
        tally[record["verdict"]] = tally.get(record["verdict"], 0) + 1
    return {"probes": len(results), **{f"probe_{k}": v for k, v in tally.items()}}


def _write_page(content: Path, results: list[dict]) -> None:
    tally: dict[str, int] = {}
    for record in results:
        tally[record["verdict"]] = tally.get(record["verdict"], 0) + 1
    probes = content / "probes"
    probes.mkdir(parents=True, exist_ok=True)

    lines = [
        "# Observed behaviour",
        "",
        f"{len(results)} probes executed against the pinned capsule: "
        + ", ".join(f"{count} {name}" for name, count in sorted(tally.items()))
        + ".",
        "",
        "`confirmed` means the probe and its control both came out as expected and the control "
        "came out the other way. `recorded` is a shape probe with nothing to falsify -- weaker, "
        "and labelled as such. `refuted` means the control discriminated and the claim did not "
        "hold, which is a result rather than a failure. A `divergent` probe demonstrates "
        "nothing and fails `verify.py`; a `blocked` one means a prerequisite was missing and "
        "never that the behaviour is absent.",
        "",
        "Every capture is a fact about a **construction**, not about this library in general. "
        "The query is the same three-row `VALUES` scan throughout, and the subscriber is "
        "upstream's own: an `fmt` layer formatting JSON with `flatten_event(true)` and "
        "`without_time()`, writing into a buffer. Timings and metric timestamps are replaced "
        "with `<VARIES>` before anything is compared.",
        "",
        "---",
    ]
    for record in sorted(results, key=lambda r: r["id"]):
        lines += [
            f"## {record['id']} — {record['question']}",
            "",
            f"`{record['id']} · {record.get('construction', '-')} · {record['verdict']}`",
            "",
            f"**Expect** `{record['expect']}`",
            "",
            f"**Control** (`{record['control_expect']}`, mode `{record['mode']}`): "
            f"`{record.get('control', '-')}`",
            "",
        ]
        if record.get("evidence"):
            lines += ["```text", record["evidence"], "```", ""]
        if record.get("note"):
            lines += [record["note"], ""]
        lines.append("---")
    (probes / "00-index.md").write_text("\n".join(lines) + "\n")


def main() -> int:
    """Run the probes. A separate entry point, like acquisition, and for the same reason.

    `build.py` must be offline and must reproduce byte for byte; these compile DataFusion and
    execute it. What `build.py` consumes is the emitted `behaviors.tsv`, which is an input to it
    exactly as the acquired documents are -- so a rebuild reads a file in the tree rather than
    re-running a compiler and hoping it agrees with itself.
    """
    import argparse

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--content", default=str(HERE.parent / "content"))
    arguments = parser.parse_args()
    sys.stderr.write(f"toolchain: {toolchain_reading()}\n")
    report = write_all(Path(arguments.content), HERE)
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

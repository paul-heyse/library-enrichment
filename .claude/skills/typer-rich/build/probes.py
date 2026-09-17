"""Execute Typer and Rich against a pinned capsule and record what they actually rendered.

`ast-grep-ripgrep/build/probes.py` opens by saying the sibling repositories cannot do this:
"DataFusion and delta-rs are libraries: the only thing a builder can do with them is read their
documented surface." Typer and Rich are libraries too, and that claim does not hold for them.
They *render*, deterministically, into a `StringIO` -- so a claim about their behaviour can be an
observation rather than a reading, and "what does this actually look like, and how wide" becomes
answerable instead of guessable. That question is the one no type signature carries and the one
an agent gets wrong most.

Three rules make the observations worth trusting.

**A probe without a control is not evidence.** A rendering that looks right proves only that
something rendered. So a behaviour probe carries a second snippet, identical except for the thing
under test, and the two must not come out the same. If either half surprises us the probe records
`inconclusive`, never a pass.

**`differs` is the default control mode, not `opposite`.** For a program you can demand an error;
for a renderer "must fail" is almost never an exception. `NO_COLOR=1` does not raise -- it quietly
drops the colour and keeps the bold. The finding is the difference, so the difference is what is
asserted.

**Determinism is built, not assumed.** Four layers, because no single one covers the surface --
see `probe_environment.layers_note` in the manifest. Measured failures each layer prevents:
`TERM=dumb` overrode an explicitly passed `Console(width=40)` and reported 80; `UNICODE_VERSION`
is read from `os.environ` directly and never reaches `Console(_environ=...)`; typer's
`rich_utils` freezes `MAX_WIDTH` at first import, so setting `TERMINAL_WIDTH` afterwards does
nothing; and `rich.traceback` renders absolute source paths straight into its output, which is a
transferability leak as well as a determinism one.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent

CONFIRMED = "confirmed"
REFUTED = "refuted"
INCONCLUSIVE = "inconclusive"
RECORDED = "recorded"

TIMEOUT = 60

# Normalisations, applied only where a probe row names them, and recorded on the row so a reader
# knows the bytes were touched. Nothing is normalised silently.
NORMALISERS = {
    "address": (re.compile(r"0x[0-9a-fA-F]{6,}"), "0xADDR"),
    # Any absolute home path, however truncated. `Traceback(show_locals=True)` renders the
    # frame's locals, and at the top of a probe process those include the environment and the
    # interpreter path -- so the locals panel prints this machine's filesystem into the capture.
    # That is not incidental: it is why Typer leaves locals off by default,
    # and it is worth capturing as structure with the paths elided.
    "home": (re.compile(r"/home/[\w.-]+[^\s'\"\u2502\u2026]*"), "<path>"),
    "path": (re.compile(r"(?:[\w.+-]*/)+(?=[\w.+-]+\.py)"), ""),
    "duration": (re.compile(r"\d+\.\d+s"), "N.NNs"),
}

PRELUDE = '''
import io, json, os, sys
from datetime import datetime, timezone

_ENV = json.loads(os.environ["PROBE_RICH_ENVIRON"])
_FROZEN = datetime(2000, 1, 1, tzinfo=timezone.utc)


def console(env=None, **kw):
    """A Console that renders the same bytes on every machine.

    `_environ` is rich's own injection point on `Console.__init__`, and passing a literal dict is
    what stops `COLUMNS` reaching it. It does not cover `UNICODE_VERSION`, which
    `rich/_unicode_data` reads from `os.environ` directly -- that one is handled by replacing the
    process environment, one layer up.

    `env=` adds names to that literal dict, and is how a probe ABOUT an environment variable asks
    its question. Setting `os.environ` inside a probe does nothing here, which is the isolation
    working: the first draft of the NO_COLOR probes did exactly that and came out inconclusive
    because layer two had already shut the door. Declaring the variable is also the better test,
    since it asks rich's documented precedence rather than the process's.
    """
    from rich.console import Console

    environ = dict(_ENV)
    environ.update(env or {})
    kw["_environ"] = environ
    kw.setdefault("width", 60)
    kw.setdefault("force_terminal", False)
    kw.setdefault("color_system", None)
    kw.setdefault("legacy_windows", False)
    kw.setdefault("get_time", lambda: 0.0)
    kw.setdefault("get_datetime", lambda: _FROZEN)
    buffer = io.StringIO()
    made = Console(file=buffer, **kw)
    made._probe_buffer = buffer
    return made


def render(renderable, env=None, **kw):
    """Render one thing and return the text."""
    made = console(env=env, **kw)
    made.print(renderable)
    return made._probe_buffer.getvalue()


def cli(app, args):
    """Invoke a Typer app and return its output.

    `CliRunner` pins `typer._click.formatting.FORCED_WIDTH = 80`, which governs the PLAIN help
    formatter only. The rich path reads `typer.rich_utils.MAX_WIDTH`, which is None unless
    TERMINAL_WIDTH was set BEFORE that module was first imported -- so the familiar 80 columns is
    an accident of an empty environment, not a guarantee.
    """
    from typer.testing import CliRunner

    return CliRunner().invoke(app, args).output


def emit(value):
    sys.stdout.write("\\x00PROBE\\x00" + json.dumps(value))
'''


class ProbeError(RuntimeError):
    """A probe could not be executed at all, as distinct from coming out negative."""


def probe_environment(manifest: dict, home: Path, binaries: str) -> dict[str, str]:
    """Build the subprocess environment from the manifest, inheriting nothing.

    A replaced environment rather than an unset list. Unsetting names is a claim about what the
    surface is, and this one grew twice while the probes were being written; replacing it wholly
    means a variable nobody thought of cannot reach the subject.
    """
    spec = manifest["probe_environment"]
    env: dict[str, str] = {}
    for name in spec["allow"]:
        if name in spec["values"]:
            env[name] = spec["values"][name]
    env["HOME"] = str(home)
    env["PATH"] = binaries
    leaked = [name for name in spec["must_be_absent"] if name in env]
    if leaked:
        raise ProbeError(f"probe environment would carry {leaked}; the scrub is not hermetic")
    return env


def _normalise(text: str, names: list[str]) -> str:
    for name in names:
        if name not in NORMALISERS:
            raise ProbeError(f"unknown normaliser {name!r}")
        pattern, replacement = NORMALISERS[name]
        text = pattern.sub(replacement, text)
    return text


def _run(source: str, python: Path, env: dict[str, str], cwd: Path) -> tuple[int, str, str]:
    body = PRELUDE + "\n" + source + "\n"
    try:
        done = subprocess.run(
            [str(python), "-c", body],
            cwd=cwd,
            env=env,
            capture_output=True,
            text=True,
            timeout=TIMEOUT,
            check=False,
        )
    except subprocess.TimeoutExpired:
        return 124, "", "timeout"
    return done.returncode, done.stdout, done.stderr


def _payload(stdout: str) -> str:
    """Pull the emitted value out, so a probe's own prints cannot be mistaken for its answer."""
    marker = "\x00PROBE\x00"
    if marker not in stdout:
        return stdout
    return json.loads(stdout.split(marker, 1)[1])


def _satisfies(expect: str, code: int, output: str) -> bool:
    if expect == "any":
        return True
    if expect == "ok":
        return code == 0
    if expect == "raises":
        return code != 0
    if expect == "no-ansi":
        return code == 0 and "\x1b[" not in output
    if expect == "ansi":
        return code == 0 and "\x1b[" in output
    if expect.startswith("absent:"):
        return code == 0 and expect.split(":", 1)[1] not in output
    if expect.startswith("contains:"):
        return code == 0 and expect.split(":", 1)[1] in output
    if expect.startswith("equals:"):
        return code == 0 and output == expect.split(":", 1)[1]
    if expect.startswith("exit:"):
        return code == int(expect.split(":", 1)[1])
    return expect in output


def evaluate(probe: dict, python: Path, env: dict[str, str], cwd: Path) -> dict:
    """Run one probe and its control, and record the observation verbatim."""
    normalise = probe.get("normalise", [])
    code, stdout, stderr = _run(probe["source"], python, env, cwd)
    output = _normalise(str(_payload(stdout)), normalise)
    primary_ok = _satisfies(probe.get("expect", "any"), code, output)

    record = {
        "id": probe["id"],
        "subject": probe["subject"],
        "topic": probe.get("topic", ""),
        "question": probe["question"],
        "source": probe["source"],
        "expect": probe.get("expect", "any"),
        "exit": code,
        "output": output,
        "stderr": stderr.strip()[-400:],
        "normalise": normalise,
        "capture": probe.get("capture", ""),
        "note": probe.get("note", ""),
    }

    control = probe.get("control")
    if control is None:
        # A shape probe: nothing to falsify, so it is `recorded`, never `confirmed`.
        record["verdict"] = RECORDED if primary_ok else INCONCLUSIVE
        record["control"] = ""
        return record

    ccode, cstdout, cstderr = _run(control, python, env, cwd)
    coutput = _normalise(str(_payload(cstdout)), normalise)
    control_ok = _satisfies(probe.get("control_expect", "any"), ccode, coutput)
    record["control"] = control
    record["control_expect"] = probe.get("control_expect", "any")
    record["control_exit"] = ccode
    record["control_output"] = coutput
    record["control_stderr"] = cstderr.strip()[-400:]

    mode = probe.get("control_mode", "differs")
    if mode == "differs":
        differs = output != coutput
        record["differs"] = differs
        record["verdict"] = CONFIRMED if (primary_ok and control_ok and differs) else INCONCLUSIVE
    elif mode == "record-both":
        record["verdict"] = RECORDED if primary_ok else INCONCLUSIVE
    else:
        if primary_ok and control_ok:
            record["verdict"] = CONFIRMED
        elif not primary_ok and control_ok:
            record["verdict"] = REFUTED
        else:
            record["verdict"] = INCONCLUSIVE
    return record


def run_all(
    manifest: dict,
    python: Path,
    cwd: Path,
    binaries: str,
    register: Path | None = None,
) -> dict:
    """Execute the whole register and return the payload acquisition stores."""
    path = register or HERE / "probes.json"
    probes = json.loads(path.read_text())["probes"]
    home = cwd / "home"
    home.mkdir(parents=True, exist_ok=True)
    env = probe_environment(manifest, home, binaries)
    env["PROBE_RICH_ENVIRON"] = json.dumps(dict(sorted(env.items())))

    results = [evaluate(probe, python, env, cwd) for probe in probes]
    broken = [
        row
        for row in results
        if row["exit"] not in (0,) and row["expect"] not in ("raises",) and not row["output"]
    ]
    if broken:
        raise ProbeError(
            "probes failed to execute at all, as distinct from coming out negative: "
            + "; ".join(f"{row['id']}: {row['stderr'][-160:]}" for row in broken[:4])
        )
    return {
        "schema": 1,
        # The record says what was *declared*, not where it ran. `HOME` points into the capsule
        # and `PATH` is inherited from whoever invoked acquisition, so recording either verbatim
        # writes this machine's filesystem into a shipped artefact -- which the transferability
        # scan then reports, correctly, as a leak.
        "environment": {
            key: ("<scratch>" if key in ("HOME", "PATH") else value)
            for key, value in sorted(env.items())
            if key != "PROBE_RICH_ENVIRON"
        },
        "results": results,
        "summary": summarise(results),
    }


def summarise(results: list[dict]) -> dict[str, int]:
    tally: dict[str, int] = {}
    for row in results:
        tally[row["verdict"]] = tally.get(row["verdict"], 0) + 1
    return dict(sorted(tally.items()))


if __name__ == "__main__":
    payload = run_all(
        json.loads((HERE / "manifests" / "typer-rich.json").read_text()),
        Path(sys.argv[1]),
        Path(sys.argv[2]),
        sys.argv[3] if len(sys.argv) > 3 else "/usr/bin:/bin",
    )
    sys.stderr.write(json.dumps(payload["summary"], indent=2) + "\n")

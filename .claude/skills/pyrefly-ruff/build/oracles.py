"""Capture what each tool says about itself, from the tool, at the pinned version.

Both projects publish catalogs generated from their own source: ruff emits its complete rule
inventory, configuration schema and linter list as JSON, and both emit a full `--help` tree.
Transcribing those into prose is how a capability index acquires errors that look authoritative,
so nothing here is written by hand -- every catalog below is the tool's own output, stored
verbatim, and `catalogs.py` joins against it.

Three properties make this trustworthy rather than merely convenient:

  1. **The version is asserted, not assumed.** This workstation carries ruff 0.14.4 on PATH (pinned
     by the host repository) and a much older standalone checker, against manifest pins of 0.16.7
     and 1.3.1. A catalog captured from PATH would describe a different tool than the rustdoc index
     describes, and nothing downstream could tell. So each tool is installed into the capsule at
     its exact pin and `assert_version` aborts on any mismatch.
  2. **The fixture is a constant, not a file in the tree.** The sample project the checker runs
     against is materialised from the strings below. It is deliberately under-annotated -- that is
     what makes inference, import resolution and call edges observable -- and a file like that
     cannot live in a repository whose ruff configuration selects ANN.
  3. **Negative results are captured too.** `ruff analyze graph` against a path that does not exist
     returns `{}` with exit status 0. That is a trap worth documenting with evidence rather than an
     assertion, so it is measured here and stored beside the successful runs.

Standard library only, as everywhere else in this builder.
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ACQUIRED = HERE / "acquired"

# A deliberately small project exercising exactly the facts the catalogs claim are observable:
# a base class and an override (nominal hierarchy), a cross-module import (resolution), a call
# through an instance (a call-graph edge), and one unannotated function (inference).
FIXTURE: dict[str, str] = {
    "pkg/__init__.py": '"""A fixture package, materialised from oracles.FIXTURE."""\n',
    "pkg/core.py": '''"""Base and derived, so a subtype edge and an override both exist."""


class Greeter:
    """A base with one overridable method."""

    def greet(self, name: str) -> str:
        return f"hello {name}"


class LoudGreeter(Greeter):
    """Overrides the base method, so an override edge exists."""

    def greet(self, name: str) -> str:
        return super().greet(name).upper()


def build(loud: bool) -> Greeter:
    """Declared as the base while returning the subtype, so declared and inferred differ."""
    return LoudGreeter() if loud else Greeter()
''',
    "pkg/app.py": '''"""Imports across a module boundary and calls through an instance."""

from pkg.core import Greeter, build


def run(names: list[str]) -> list[str]:
    greeter = build(loud=True)
    return [greeter.greet(name) for name in names]


def untyped(values):
    """No annotations at all -- this is what makes inference observable."""
    return [v for v in values if v]


def widen(greeter: Greeter) -> str:
    return greeter.greet("world")
''',
    "pyrefly.toml": 'project-includes = ["pkg"]\npython-version = "3.13"\n',
}

RUFF_SUBCOMMANDS = (
    "check",
    "rule",
    "config",
    "linter",
    "clean",
    "format",
    "server",
    "analyze",
    "version",
)
# All twelve the binary reports, verified against `--help` at the pin rather than recalled.
# The first capture listed nine, which left `coverage` and `stubgen` -- the two that answer
# "how well typed is this project" and "what is its API surface" -- absent from cli.tsv.
CHECKER_SUBCOMMANDS = (
    "check",
    "snippet",
    "dump-config",
    "buck-check",
    "bazel-check",
    "init",
    "lsp",
    "tsp",
    "infer",
    "coverage",
    "report",
    "suppress",
    "stubgen",
)


class OracleError(RuntimeError):
    """A pinned tool could not be installed, asserted or interrogated."""


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def run(
    command: list[str], *, cwd: Path | None = None, timeout: int = 600
) -> subprocess.CompletedProcess:
    """Run a tool and return the result. Never `check=True`: a nonzero status is often the datum."""
    return subprocess.run(
        command,
        cwd=cwd,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
        env=tool_env(),
    )


def tool_env() -> dict:
    """Drop the host's configuration, so a captured catalog is the tool's defaults not this box's.

    `RUFF_OUTPUT_FORMAT` and the checker's `PYREFLY_*` variables would otherwise silently change
    what is recorded, and a catalog that depends on an ambient variable is not reproducible.
    """
    env = {k: v for k, v in os.environ.items() if not k.startswith(("RUFF_", "PYREFLY_"))}
    env["NO_COLOR"] = "1"
    return env


def install(capsule: Path, package: str, version: str) -> Path:
    """Install one pinned tool into its own capsule venv and return its executable."""
    venv = capsule / "tools" / f"{package}-{version}"
    binary = venv / "bin" / package
    if binary.exists():
        say(f"  {package} {version}: capsule venv present")
        return binary
    if shutil.which("uv") is None:
        raise OracleError("uv is not on PATH; the pinned tools cannot be installed")
    venv.parent.mkdir(parents=True, exist_ok=True)
    say(f"  creating capsule venv for {package}=={version}")
    done = run(["uv", "venv", "--quiet", str(venv)])
    if done.returncode != 0:
        raise OracleError(f"uv venv failed: {done.stderr[-400:]}")
    done = run(
        [
            "uv",
            "pip",
            "install",
            "--quiet",
            "--python",
            str(venv / "bin" / "python"),
            f"{package}=={version}",
        ]
    )
    if done.returncode != 0:
        raise OracleError(f"uv pip install {package}=={version} failed: {done.stderr[-400:]}")
    if not binary.exists():
        raise OracleError(f"{package}=={version} installed but {binary} is absent")
    return binary


def assert_version(binary: Path, expected: str) -> str:
    """Refuse to record a catalog from a tool that is not the pinned one."""
    done = run([str(binary), "--version"])
    reported = (done.stdout or done.stderr).strip()
    if expected not in reported:
        raise OracleError(
            f"{binary.name} reports {reported!r}, which does not contain the pinned {expected!r}. "
            "Refusing to capture a catalog that would describe a different tool than the index."
        )
    say(f"  {binary.name}: {reported}")
    return reported


def materialise(root: Path) -> Path:
    """Write the constant fixture into the capsule."""
    project = root / "fixture"
    if project.exists():
        shutil.rmtree(project)
    for relative, body in sorted(FIXTURE.items()):
        target = project / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(body)
    return project


def capture(out: Path, name: str, done: subprocess.CompletedProcess, *, as_json: bool) -> dict:
    """Store one invocation's result, including its status, and parse it when it claims JSON."""
    record: dict = {"exit": done.returncode, "stderr": done.stderr.strip()[:4000]}
    if as_json and done.returncode == 0 and done.stdout.strip():
        try:
            parsed = json.loads(done.stdout)
        except json.JSONDecodeError as exc:
            record["parse_error"] = str(exc)
        else:
            body = json.dumps(parsed, indent=1, sort_keys=True) + "\n"
            (out / f"{name}.json").write_text(body)
            record["file"] = f"{name}.json"
            record["sha256"] = hashlib.sha256(body.encode()).hexdigest()
            record["entries"] = len(parsed)
            return record
    (out / f"{name}.txt").write_text(done.stdout)
    record["file"] = f"{name}.txt"
    record["sha256"] = hashlib.sha256(done.stdout.encode()).hexdigest()
    return record


def help_tree(binary: Path, out: Path, subcommands: tuple[str, ...], prefix: str) -> dict:
    """The full `--help` surface. This is the only complete source for flags and their values."""
    results: dict[str, dict] = {}
    results["help"] = capture(out, f"{prefix}-help", run([str(binary), "--help"]), as_json=False)
    for sub in subcommands:
        done = run([str(binary), sub, "--help"])
        results[sub] = capture(out, f"{prefix}-help-{sub}", done, as_json=False)
    nested = run([str(binary), "analyze", "graph", "--help"])
    if nested.returncode == 0:
        name = f"{prefix}-help-analyze-graph"
        results["analyze graph"] = capture(out, name, nested, as_json=False)
    return results


def ruff_oracles(binary: Path, out: Path, fixture: Path) -> dict:
    """ruff's four JSON catalogs, its help tree, and the import graph measured three ways."""
    results: dict[str, dict] = {}
    for name, args in (
        ("rules", ["rule", "--all", "--output-format", "json"]),
        ("config", ["config", "--output-format", "json"]),
        ("linters", ["linter", "--output-format", "json"]),
        ("version", ["version", "--output-format", "json"]),
    ):
        results[name] = capture(out, f"ruff-{name}", run([str(binary), *args]), as_json=True)
    results["help"] = help_tree(binary, out, RUFF_SUBCOMMANDS, "ruff")

    # The import graph, measured rather than described. The third run is the trap: a path that
    # does not exist yields `{}` and exit 0, which reads as "no imports" to anything that only
    # checks the status.
    graph = ["analyze", "graph"]
    results["graph_dependencies"] = capture(
        out, "ruff-graph-dependencies", run([str(binary), *graph, "pkg"], cwd=fixture), as_json=True
    )
    results["graph_dependents"] = capture(
        out,
        "ruff-graph-dependents",
        run([str(binary), *graph, "pkg", "--direction", "dependents"], cwd=fixture),
        as_json=True,
    )
    results["graph_missing_path"] = capture(
        out,
        "ruff-graph-missing-path",
        run([str(binary), *graph, "no-such-directory"], cwd=fixture),
        as_json=True,
    )
    return results


def checker_oracles(binary: Path, out: Path, fixture: Path) -> dict:
    """The checker's help tree, its JSON diagnostics, and the two cross-reference reports.

    `--report-glean` and `--report-pysa` are the reason this skill exists: they are the only
    routes either toolchain offers to definitions, references and call edges, and neither is
    documented by a schema the tool prints. Capturing one real payload per report is what lets
    the catalog describe their shape without guessing.
    """
    results: dict[str, dict] = {}
    results["help"] = help_tree(binary, out, CHECKER_SUBCOMMANDS, "checker")
    results["dump_config"] = capture(
        out,
        "checker-dump-config",
        run([str(binary), "dump-config"], cwd=fixture),
        as_json=False,
    )
    results["check_json"] = capture(
        out,
        "checker-check",
        run([str(binary), "check", "--output-format", "json"], cwd=fixture),
        as_json=True,
    )
    # pysa is measured at BOTH formats. Measuring only the default is what produced the
    # catalog's claim that the report is binary -- true of `capnp`, and wrong as a description
    # of what the report can emit.
    for report, fmt in (("glean", None), ("pysa", "capnp"), ("pysa", "json")):
        key = f"report_{report}" if fmt is None else f"report_{report}_{fmt}"
        results[key] = cross_reference_report(binary, out, fixture, report, fmt)
    return results


def cross_reference_report(
    binary: Path, out: Path, fixture: Path, report: str, fmt: str | None = None
) -> dict:
    """Run one report at one format and describe what it actually wrote.

    Deliberately suffix-driven rather than trusting the help text. `--report-pysa` is documented
    as "a Pysa-compatible JSON file for each module" and at its DEFAULT format writes Cap'n Proto
    binary (`errors.capnp.bin`, `pyrefly.pysa.capnp.bin`). Decoding that as text yields a
    plausible-looking but meaningless sample, so a binary is measured by size and never sampled
    as text.

    `--report-pysa-format json` is a different matter: it writes `definitions/`, `call_graphs/`
    and `type_of_expressions/` as JSON keyed by `line:col-line:col`, which is strictly richer
    than the Glean report for calls, parameters and expression types. Both formats are captured
    so the catalogs can state the difference rather than the default.
    """
    suffix = report if fmt is None else f"{report}-{fmt}"
    directory = fixture / f"report-{suffix}"
    if directory.exists():
        shutil.rmtree(directory)
    directory.mkdir(parents=True)
    command = [str(binary), "check", f"--report-{report}", str(directory)]
    if fmt is not None:
        command += [f"--report-{report}-format", fmt]
    command += ["--output-format", "json"]
    done = run(command, cwd=fixture)
    produced = sorted(p for p in directory.rglob("*") if p.is_file())
    record: dict = {
        "exit": done.returncode,
        "stderr": done.stderr.strip()[:2000],
        "file_count": len(produced),
        "files": [p.name for p in produced[:40]],
        "formats": {},
        "sample": None,
        "predicates": [],
    }
    for path in produced:
        suffix = path.suffix or "(none)"
        record["formats"][suffix] = record["formats"].get(suffix, 0) + 1

    documents = [p for p in produced if p.suffix == ".json"]
    if not documents:
        record["note"] = (
            "no JSON written; this report is binary at the pinned version despite its help text"
        )
        return record

    # The filenames are content digests, so "first alphabetically" is deterministic but arbitrary.
    # The largest document is the representative one, with the name as a stable tiebreak.
    documents.sort(key=lambda p: (-p.stat().st_size, p.name))
    body = documents[0].read_text()[:400000]
    name = f"checker-report-{suffix}-sample.json"
    (out / name).write_text(body)
    record["sample"] = name

    predicates: dict[str, int] = {}
    for path in documents:
        try:
            parsed = json.loads(path.read_text())
        except (json.JSONDecodeError, UnicodeDecodeError):
            continue
        for entry in parsed if isinstance(parsed, list) else []:
            if isinstance(entry, dict) and entry.get("predicate"):
                predicates[entry["predicate"]] = predicates.get(entry["predicate"], 0) + len(
                    entry.get("facts") or []
                )
    record["predicates"] = sorted(predicates.items())
    return record


def language_server(binary: Path, out: Path, subcommand: str, root: Path) -> dict:
    """Ask a server what it can do, by speaking its own handshake.

    A server's advertised capabilities are the contract a client can rely on, and they are not
    discoverable from `--help`. LSP and TSP both begin with an `initialize` request over framed
    JSON-RPC on stdio, so one client reaches both.
    """
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": None,
            "rootUri": root.as_uri(),
            "capabilities": {},
            "clientInfo": {"name": "capability-skill-builder", "version": "1"},
        },
    }
    body = json.dumps(request).encode()
    frame = b"Content-Length: " + str(len(body)).encode() + b"\r\n\r\n" + body
    try:
        process = subprocess.Popen(
            [str(binary), subcommand],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=root,
            env=tool_env(),
        )
    except OSError as exc:
        return {"status": "blocked", "reason": str(exc)}
    try:
        stdout, stderr = process.communicate(input=frame, timeout=90)
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, stderr = process.communicate()
    finally:
        if process.poll() is None:
            process.kill()

    head, _, payload = stdout.partition(b"\r\n\r\n")
    record: dict = {"status": "ok", "header": head.decode("utf-8", "replace")[:200]}
    try:
        length = int(head.decode().split("Content-Length:")[1].split()[0])
        parsed = json.loads(payload[:length])
    except (ValueError, IndexError, json.JSONDecodeError) as exc:
        record["status"] = "unparsed"
        record["reason"] = str(exc)
        record["stderr"] = stderr.decode("utf-8", "replace")[:2000]
        return record
    text = json.dumps(parsed, indent=1, sort_keys=True) + "\n"
    (out / f"checker-{subcommand}-initialize.json").write_text(text)
    record["file"] = f"checker-{subcommand}-initialize.json"
    record["sha256"] = hashlib.sha256(text.encode()).hexdigest()
    capabilities = (parsed.get("result") or {}).get("capabilities") or {}
    record["capabilities"] = sorted(capabilities)
    return record


def collect(manifest: dict, capsule: Path) -> Path:
    """Install both pinned tools, interrogate them, and record everything with digests."""
    oracle_pins = manifest["oracles"]
    out = ACQUIRED / "oracles"
    out.mkdir(parents=True, exist_ok=True)
    fixture = materialise(capsule)

    record: dict = {"fixture_files": sorted(FIXTURE), "tools": {}}

    ruff_pin = oracle_pins["ruff"]
    say(f"\nruff {ruff_pin}")
    ruff = install(capsule, "ruff", ruff_pin)
    record["tools"]["ruff"] = {
        "version": ruff_pin,
        "reported": assert_version(ruff, ruff_pin),
        "captured": ruff_oracles(ruff, out, fixture),
    }

    checker_pin = oracle_pins["checker"]
    checker_package = oracle_pins["checker_package"]
    say(f"\n{checker_package} {checker_pin}")
    try:
        checker = install(capsule, checker_package, checker_pin)
        reported = assert_version(checker, checker_pin)
    except OracleError as exc:
        say(f"  blocked: {exc}")
        record["tools"][checker_package] = {
            "version": checker_pin,
            "status": "blocked",
            "reason": str(exc),
        }
    else:
        captured = checker_oracles(checker, out, fixture)
        captured["lsp"] = language_server(checker, out, "lsp", fixture)
        captured["tsp"] = language_server(checker, out, "tsp", fixture)
        record["tools"][checker_package] = {
            "version": checker_pin,
            "reported": reported,
            "captured": captured,
        }

    files = {}
    for path in sorted(out.glob("*")):
        if path.name == "ORACLES.json" or not path.is_file():
            continue
        files[path.name] = {
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            "bytes": path.stat().st_size,
        }
    record["files"] = files
    (out / "ORACLES.json").write_text(json.dumps(record, indent=1, sort_keys=True) + "\n")
    say(f"\nwrote {len(files)} oracle documents to {out}")
    return out

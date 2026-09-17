"""Produce the normalized API documents for a pinned Python environment, locally.

This is the one stage that needs a network, `uv`, Griffe and (optionally) `ty`. It is a separate
entry point on purpose: `verify.py` re-runs `build.py` to prove determinism, so acquisition must
never be reachable from there. After one run, `build.py` and `verify.py` need only the standard
library and `ast-grep`.

Usage:
    python3 acquire.py [--manifest manifests/fastmcp.json] [--check] [--clean] [--no-semantic]

Why this exists at all: PyPI serves no equivalent of docs.rs's rustdoc JSON, so the bytes have
to be made here. And they cannot be made from whatever happens to be installed in the calling
repository -- that would make the index a description of one developer's virtualenv.

Six phases, each of which fails loudly rather than degrading:

  0. assert the tools are the ones the manifest names
  1. build a capsule virtualenv outside every repository and install the pinned set
  2. assert the resolved graph matches the manifest's lock, because the pins are not a pin
  3. extract each distribution with Griffe, then optionally supplement with the ty language server
  3c. vendor the pinned upstream corpora, because prose is a captured input like any other
  4. collect, digest, and record what was actually resolved

Phase 2 is the one that is easy to skip and expensive to omit. `fastmcp==4.0.3` is a metapackage
shipping no code; it requires `fastmcp-slim[client,server]==4.0.3`, which requires
`mcp>=2.0.0,<3.0.0` -- a range. Two builds a month apart would silently index different
libraries.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

# Sibling modules. Python puts the script's own directory on `sys.path`, so these resolve
# whatever directory acquisition is invoked from.
import analysis
import fetch

HERE = Path(__file__).resolve().parent
ACQUIRED = HERE / "acquired"
SCHEMA = 1


class AcquireError(RuntimeError):
    """Acquisition could not produce a trustworthy artefact."""


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def capsule_root() -> Path:
    """Locate the scratch capsule. Never a literal path: the transferability check forbids it."""
    for name in ("PY_SKILL_ACQUIRE_CAPSULE", "PYTHON_SKILL_ACQUIRE_CAPSULE"):
        value = os.environ.get(name)
        if value:
            return Path(value)
    cache = os.environ.get("XDG_CACHE_HOME")
    base = Path(cache) if cache else Path.home() / ".cache"
    return base / "python-skill-acquire"


def envelope_key(manifest: dict, resolved: dict[str, str]) -> str:
    """A cache key over everything that changes the bytes, so a miss is the staleness signal.

    The *resolved* graph is hashed, not the declared install line. `mcp>=2.0.0,<3.0.0` resolving
    differently is precisely the drift this key has to notice.
    """
    environment = manifest["environment"]
    payload = {
        "schema": SCHEMA,
        "python": environment["python"],
        "install": sorted(environment["install"]),
        "resolved": dict(sorted(resolved.items())),
        "distributions": sorted(entry["dist"] for entry in manifest["distributions"]),
        "griffe": sorted(manifest["tools"]["griffe_versions"]),
        "docstring_parser": manifest["tools"]["docstring_parser"],
    }
    blob = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(blob).hexdigest()[:12]


def acquired_dir(manifest: dict, resolved: dict[str, str]) -> Path:
    """Label the acquisition by every pinned subject, not by the repository name.

    fastmcp looks the *repository* name up in the resolved graph, which works there only because
    "fastmcp" happens to also be a distribution. This repository is called `typer-rich`, which is
    not a distribution, so that reading yields `typer-rich@unknown-<key>` and every acquisition
    ever taken collides under one uninformative name.
    """
    name = manifest["repository"]["name"]
    pinned = [s for s in manifest["subjects"] if s.get("pinned")]
    if not pinned:
        raise AcquireError("no subject is marked pinned; the acquisition cannot be labelled")
    missing = [s["dist"] for s in pinned if s["dist"] not in resolved]
    if missing:
        raise AcquireError(f"pinned subjects absent from the resolved graph: {missing}")
    label = "+".join(f"{s['name']}-{resolved[s['dist']]}" for s in pinned)
    return ACQUIRED / f"{name}@{label}-{envelope_key(manifest, resolved)}"


def load_manifest(path: Path) -> tuple[dict, dict[str, str]]:
    manifest = json.loads(path.read_text())
    lock_path = path.parent / manifest["environment"]["resolved_file"]
    if not lock_path.exists():
        raise AcquireError(f"manifest lock {lock_path} is missing")
    return manifest, json.loads(lock_path.read_text())


# --------------------------------------------------------------------------- phases


def run(command: list[str], *, cwd: Path | None = None, env: dict | None = None) -> str:
    done = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, check=False)
    if done.returncode != 0:
        tail = (done.stderr or done.stdout).strip()[-600:]
        raise AcquireError(f"{command[0]} failed ({done.returncode}): {tail}")
    return done.stdout


def assert_tools(manifest: dict) -> dict[str, str]:
    """Phase 0. A tool outside the accepted set aborts before anything expensive happens."""
    tools = manifest["tools"]
    found: dict[str, str] = {}

    for binary in ("uv", "ast-grep"):
        if shutil.which(binary) is None:
            raise AcquireError(f"{binary} is not on PATH")
    found["uv"] = run(["uv", "--version"]).strip()
    found["ast_grep"] = run(["ast-grep", "--version"]).strip()

    # Imported here, not at module scope: these are acquisition-only dependencies, and the
    # offline half of the build must never acquire an import path to them.
    from importlib.metadata import PackageNotFoundError
    from importlib.metadata import version as dist_version

    for name in ("griffe", "griffelib"):
        try:
            found[name] = dist_version(name)
        except PackageNotFoundError as error:
            raise AcquireError(f"{name} is not installed; acquisition needs it") from error
    found["python"] = sys.version.split()[0]

    if found["griffe"] not in tools["griffe_versions"]:
        raise AcquireError(
            f"griffe {found['griffe']} is not in the accepted set {tools['griffe_versions']}; "
            "canonical-path inference changes between versions, so the manifest must be updated "
            "deliberately rather than drifted into"
        )
    # griffelib carries the models griffe's canonical-path inference is expressed over, so a
    # split between the two is the same decision as a griffe bump. fastmcp asserts griffelib is
    # *installed* and then never compares it; the manifest key was dead config. Compare it.
    if found["griffelib"] not in tools["griffelib_versions"]:
        raise AcquireError(
            f"griffelib {found['griffelib']} is not in the accepted set "
            f"{tools['griffelib_versions']}; it carries the models griffe resolves paths over"
        )
    # A hard gate, unlike fastmcp, where a mismatch only says(). link.dunder_sites reads the
    # corpus with this parser and check_rule_tests runs against it, so a silently different
    # ast-grep changes what the index contains without changing anything that would fail.
    expected = tools["ast_grep_version"]
    if expected not in found["ast_grep"]:
        raise AcquireError(
            f"ast-grep is {found['ast_grep']}, manifest expects {expected}; the corpus link pass "
            "and the rule tests both parse with it, so this is a decision, not a drift"
        )
    return found


def build_capsule(manifest: dict, capsule: Path, *, reuse: bool) -> Path:
    """Phase 1. A virtualenv outside every repository, never the caller's own."""
    environment = manifest["environment"]
    venv = capsule / "venv"
    if venv.exists() and not reuse:
        shutil.rmtree(venv)
    if not venv.exists():
        capsule.mkdir(parents=True, exist_ok=True)
        say(f"  creating capsule venv at {venv}")
        run(["uv", "venv", "--python", environment["python"], str(venv)])

    env = dict(os.environ)
    env["VIRTUAL_ENV"] = str(venv)
    env.pop("PYTHONPATH", None)
    say(f"  installing {', '.join(environment['install'])}")
    run(["uv", "pip", "install", *environment["install"]], env=env)
    return venv


def capsule_site_packages(venv: Path) -> Path:
    for candidate in sorted(venv.glob("lib/python*/site-packages")):
        return candidate
    raise AcquireError(f"no site-packages under {venv}")


def freeze(venv: Path) -> dict[str, str]:
    env = dict(os.environ)
    env["VIRTUAL_ENV"] = str(venv)
    output = run(["uv", "pip", "freeze"], env=env)
    pins: dict[str, str] = {}
    for line in output.splitlines():
        line = line.strip()
        if "==" in line and not line.startswith("#"):
            name, _, version = line.partition("==")
            pins[name.strip().lower().replace("_", "-")] = version.strip()
    return pins


def assert_resolved(manifest: dict, declared: dict[str, str], actual: dict[str, str]) -> None:
    """Phase 2. The declared pins are not a pin; the resolved graph is."""
    normalised = {k.lower().replace("_", "-"): v for k, v in declared.items()}
    drift: list[str] = []
    for name, version in sorted(normalised.items()):
        seen = actual.get(name)
        if seen is None:
            drift.append(f"{name}: locked {version}, absent from the capsule")
        elif seen != version:
            drift.append(f"{name}: locked {version}, resolved {seen}")
    extra = sorted(set(actual) - set(normalised))
    if extra:
        drift.extend(f"{name}: resolved {actual[name]}, absent from the lock" for name in extra)
    if drift:
        head = "\n    ".join(drift[:12])
        raise AcquireError(
            f"the resolved environment does not match {manifest['environment']['resolved_file']}:"
            f"\n    {head}\n"
            "Upstream moved. Re-lock deliberately rather than indexing a different library."
        )


def extract_all(manifest: dict, site_packages: Path) -> dict[str, dict]:
    """Phase 3a. One Griffe pass per distribution, into our own schema."""
    sys.path.insert(0, str(HERE))
    import extract

    resolved = freeze_cache["pins"]
    documents: dict[str, dict] = {}
    for entry in manifest["distributions"]:
        dist = entry["dist"]
        version = resolved.get(dist.lower().replace("_", "-"), "unknown")
        say(f"  griffe {dist} {version}")
        document = extract.extract(
            entry["module"],
            dist,
            version,
            [str(site_packages)],
            namespace_packages=tuple(_namespace_children(entry, site_packages)),
            export_signals=manifest["export_signals"],
            exclude_modules=tuple(entry.get("exclude_modules", ())),
        )
        counts = (
            f"{len(document['items'])} items, {len(document['modules'])} modules, "
            f"{len(document['access'])} access paths"
        )
        say(f"    {counts}")
        if document["not_indexed"]:
            say(f"    ! not indexed: {document['not_indexed']}")
        documents[dist] = document
    return documents


freeze_cache: dict[str, dict[str, str]] = {"pins": {}}


def _namespace_children(entry: dict, site_packages: Path) -> list[str]:
    """Expand each declared namespace package into its importable children."""
    children: list[str] = []
    for dotted in entry.get("namespace_packages", []):
        directory = site_packages.joinpath(*dotted.split("."))
        if not directory.is_dir():
            say(f"    ! declared namespace package {dotted} is not a directory")
            continue
        for child in sorted(directory.iterdir()):
            if child.is_dir() and (child / "__init__.py").exists():
                children.append(f"{dotted}.{child.name}")
    return children


def collect(
    manifest: dict,
    documents: dict[str, dict],
    destination: Path,
    facts: dict,
) -> None:
    """Phase 4. Write nothing until everything is present, so a short index cannot look complete."""
    expected = {entry["dist"] for entry in manifest["distributions"]}
    missing = sorted(expected - set(documents))
    if missing:
        raise AcquireError(f"no document produced for: {', '.join(missing)}")

    declared = {entry["name"] for entry in (manifest.get("corpora") or [])}
    absent = sorted(declared - set(facts.get("corpora") or {}))
    if absent:
        raise AcquireError(f"no corpus captured for: {', '.join(absent)}")

    destination.mkdir(parents=True, exist_ok=True)
    files: dict[str, dict] = {}
    for dist, document in sorted(documents.items()):
        # Sorting happens here, at serialisation, not only downstream: Griffe's submodule order
        # comes from an unsorted `iterdir()`, so it varies by filesystem. A determinism check
        # that re-runs on the same machine would never catch that -- the failure would surface
        # only when somebody else rebuilt, which is exactly when these committed bytes matter.
        blob = json.dumps(document, sort_keys=True, separators=(",", ":")).encode()
        name = f"{dist.replace('-', '_')}.json"
        (destination / name).write_bytes(blob)
        files[name] = {
            "dist": dist,
            "sha256": hashlib.sha256(blob).hexdigest(),
            "bytes": len(blob),
            "items": len(document["items"]),
            "modules": len(document["modules"]),
        }

    # The analysis payload is ~95,000 rows. It lives beside the manifest rather than inside it,
    # so ACQUISITION.json stays a readable integrity record and a re-pin produces a diff a human
    # can review. The payload's own digest is recorded in `files`, so nothing is lost.
    analysis_record = facts.get("analysis") or {"status": "not_run"}
    summary = {key: value for key, value in analysis_record.items() if not isinstance(value, list)}
    summary["rows"] = {
        key: len(value) for key, value in sorted(analysis_record.items()) if isinstance(value, list)
    }
    if analysis_record.get("status") == "passed":
        blob = json.dumps(analysis_record, sort_keys=True, separators=(",", ":")).encode()
        (destination / "ANALYSIS.json").write_bytes(blob)
        files["ANALYSIS.json"] = {
            "dist": "-",
            "sha256": hashlib.sha256(blob).hexdigest(),
            "bytes": len(blob),
            "items": sum(summary["rows"].values()),
            "modules": 0,
        }

    # PROBES.json is written by phase 3e and digested here for the same reason ANALYSIS.json is:
    # it holds bytes that exist only because something was executed against a specific capsule,
    # so `build.py` must be able to prove the file it replays is the one acquisition produced.
    probes_path = destination / "PROBES.json"
    if probes_path.is_file():
        blob = probes_path.read_bytes()
        files["PROBES.json"] = {
            "dist": "-",
            "sha256": hashlib.sha256(blob).hexdigest(),
            "bytes": len(blob),
            "items": len(json.loads(blob)["results"]),
            "modules": 0,
        }

    record = {
        "schema": SCHEMA,
        "envelope_key": destination.name.rsplit("-", 1)[-1],
        "tools": facts["tools"],
        "environment": {
            "python": manifest["environment"]["python"],
            "install": manifest["environment"]["install"],
        },
        "resolved": dict(sorted(facts["resolved"].items())),
        "distribution_records": facts["records"],
        "semantic": facts["semantic"],
        "corpora": facts.get("corpora") or {},
        "analysis": summary,
        "files": dict(sorted(files.items())),
    }
    (destination / "ACQUISITION.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n"
    )
    say(f"  wrote {len(files)} documents to {destination.name}")


def fetch_corpora(manifest: dict, destination: Path) -> dict[str, dict]:
    """Phase 3c. Vendor the pinned upstream corpora.

    This lives in acquisition rather than in `build.py`, which is where the sibling skills put
    it. They pay for that: on a cold cache their "offline by construction" build reaches for the
    network. Here the corpus is a captured input like any other -- fetched once, digested, and
    read back by an offline build from `acquired/`.

    A corpus that fetched nothing is indistinguishable downstream from a corpus with nothing to
    match, so `fetch.repo_files` raises rather than returning empty, and this aborts with it.
    """
    corpora = manifest.get("corpora") or []
    if not corpora:
        return {}

    cache = fetch.Cache(HERE / ".cache")
    captured: dict[str, dict] = {}
    root = destination / "corpus"
    for entry in corpora:
        files = fetch.repo_files(
            cache,
            entry["repo"],
            entry["ref"],
            entry["source_prefix"],
            tuple(entry.get("suffixes") or ()),
            entry.get("ref_kind", "tag"),
            tuple(entry.get("excludes") or ()),
        )
        target = root / entry["name"]
        if target.exists():
            shutil.rmtree(target)
        payload = 0
        for relative, blob in sorted(files.items()):
            path = target / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(blob)
            payload += len(blob)
        captured[entry["name"]] = {
            "repo": entry["repo"],
            "ref": entry["ref"],
            "ref_kind": entry.get("ref_kind", "tag"),
            "source_prefix": entry["source_prefix"],
            "dest": entry["dest"],
            "files": len(files),
            "bytes": payload,
            "sha256": hashlib.sha256(
                b"".join(
                    relative.encode() + b"\0" + blob for relative, blob in sorted(files.items())
                )
            ).hexdigest(),
        }
        say(f"  {entry['name']}: {len(files)} files, {payload // 1024} KiB")
    return captured


def record_hashes(site_packages: Path, resolved: dict[str, str]) -> dict[str, str]:
    """Identity of the installed code, from each distribution's RECORD.

    Hashing the installed tree would never be stable: every package directory carries
    `__pycache__` whose bytes depend on mtime and the hash seed. RECORD carries the publisher's
    own digests, which is the real content identity.
    """
    digests: dict[str, str] = {}
    for dist_info in sorted(site_packages.glob("*.dist-info")):
        record = dist_info / "RECORD"
        if not record.exists():
            continue
        name = dist_info.name.rsplit("-", 2)[0].lower().replace("_", "-")
        if name not in resolved:
            continue
        digests[name] = hashlib.sha256(record.read_bytes()).hexdigest()
    return dict(sorted(digests.items()))


# --------------------------------------------------------------------------- entry point


def check_drift(manifest: dict, declared: dict[str, str]) -> int:
    """Report whether PyPI now offers something newer. Never re-pins: the pin is the point."""
    import urllib.request

    moved = 0
    for entry in manifest["distributions"]:
        dist = entry["dist"]
        pinned = declared.get(dist)
        url = f"https://pypi.org/pypi/{dist}/json"
        try:
            with urllib.request.urlopen(url, timeout=30) as response:
                latest = json.loads(response.read())["info"]["version"]
        except Exception as error:
            say(f"  ? {dist}: could not check ({type(error).__name__})")
            continue
        if pinned and latest != pinned:
            say(f"  ! {dist}: pinned {pinned}, PyPI now offers {latest}")
            moved += 1
        else:
            say(f"  ok {dist}: {pinned}")
    return 1 if moved else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Acquire normalized API documents for Typer, Rich and Typer's vendored Click."
    )
    parser.add_argument("--manifest", type=Path, default=HERE / "manifests" / "typer-rich.json")
    parser.add_argument("--check", action="store_true", help="report upstream drift and exit")
    parser.add_argument("--clean", action="store_true", help="discard the capsule venv first")
    parser.add_argument(
        "--no-semantic", action="store_true", help="skip the ty language-server supplement"
    )
    parser.add_argument("--no-analysis", action="store_true", help="skip the checker batch reports")
    args = parser.parse_args(argv)

    manifest, declared = load_manifest(args.manifest)
    if args.check:
        return check_drift(manifest, declared)

    say("phase 0: tools")
    tools = assert_tools(manifest)
    for name, value in sorted(tools.items()):
        say(f"  {name}: {value}")

    capsule = capsule_root() / manifest["repository"]["name"]
    say(f"\nphase 1: capsule at {capsule}")
    venv = build_capsule(manifest, capsule, reuse=not args.clean)
    site_packages = capsule_site_packages(venv)

    say("\nphase 2: assert the resolved graph")
    resolved = freeze(venv)
    assert_resolved(manifest, declared, resolved)
    say(f"  {len(resolved)} distributions match the lock")
    freeze_cache["pins"] = resolved

    say("\nphase 3: extract")
    documents = extract_all(manifest, site_packages)

    semantic: dict = {"status": "not_run", "reason": "not requested"}
    if args.no_semantic:
        semantic = {"status": "not_run", "reason": "--no-semantic"}
    else:
        semantic = run_semantic(manifest, documents, site_packages)

    destination = acquired_dir(manifest, resolved)
    say("\nphase 3d: batch analysis")
    checker_analysis: dict = {"status": "not_run", "reason": "--no-analysis"}
    if not args.no_analysis:
        try:
            checker_analysis = analysis.capture(
                binary=analysis.install_checker(capsule, manifest["tools"]["checker_version"]),
                venv=venv,
                site_packages=site_packages,
                cache=HERE / ".cache" / "analysis",
                modules=[entry["module"] for entry in manifest["distributions"]],
                python=manifest["environment"]["python"],
                expected_files={
                    item["file"]
                    for document in documents.values()
                    for item in document["items"]
                    if item.get("file")
                },
                pinned=manifest["tools"]["checker_version"],
                documents=documents,
                dunder_protocols=tuple(manifest.get("dunder_protocols", ())),
            )
            checker_analysis["status"] = "passed"
        except analysis.AnalysisError as error:
            # Blocked, with the prerequisite named -- never a silently empty set of tables.
            checker_analysis = {"status": "blocked", "reason": str(error)}
            say(f"  BLOCKED: {error}")

    say("\nphase 3c: corpora")
    corpora = fetch_corpora(manifest, destination)

    say("\nphase 3e: probes")
    probe_payload = run_probes(manifest, venv, destination)

    say("\nphase 4: collect")
    facts = {
        "tools": tools,
        "resolved": resolved,
        "records": record_hashes(site_packages, resolved),
        "semantic": semantic,
        "corpora": corpora,
        "analysis": checker_analysis,
        "probes": probe_payload["summary"],
    }
    collect(manifest, documents, destination, facts)
    return 0


def run_probes(manifest: dict, venv: Path, destination: Path) -> dict:
    """Phase 3e. Execute the subjects and record what they rendered.

    This belongs to acquisition rather than to the build for the same reason the corpus does:
    `build.py` must stay on the standard library plus ast-grep so `verify.py` can re-run it and
    demand the same bytes. Probes spawn the capsule interpreter, which is emphatically not that.
    The build replays `PROBES.json`; it never executes anything.

    The scratch tree lives under the capsule, never under any repository -- a probe is handed a
    `HOME` of its own so `typer.get_app_dir()` is deterministic, and that must not land in a
    working tree.
    """
    sys.path.insert(0, str(HERE))
    import probes

    python = venv / "bin" / "python"
    if not python.is_file():
        python = venv / "Scripts" / "python.exe"
    scratch = venv.parent / "probe-scratch"
    if scratch.exists():
        shutil.rmtree(scratch)
    scratch.mkdir(parents=True)

    payload = probes.run_all(manifest, python, scratch, os.environ.get("PATH", ""))
    (destination / "PROBES.json").write_text(json.dumps(payload, indent=1, sort_keys=True) + "\n")
    summary = ", ".join(f"{count} {name}" for name, count in payload["summary"].items())
    say(f"  {len(payload['results'])} probes: {summary}")

    stalled = [row["id"] for row in payload["results"] if row["verdict"] == "inconclusive"]
    if stalled:
        # Not fatal, but never silent. An inconclusive probe is an honest outcome and a standing
        # question; a build that hid them would certify evidence it does not have.
        say(f"  ! inconclusive: {', '.join(stalled)}")
    return payload


def run_semantic(manifest: dict, documents: dict[str, dict], site_packages: Path) -> dict:
    """Phase 3b. The ty supplement, narrowed to the one question Griffe cannot answer.

    Everything broader was measured and lost: `typeHierarchy/subtypes` is direct-only (10
    children of `Provider` against a `bases` closure of 26, and it omits `FastMCP` itself),
    `textDocument/references` returned one hit for a heavily-used class, and whole-file inlay
    hints are mostly parameter-name hints and method-body locals. What is left is hover on the
    definition sites where Griffe genuinely has nothing.
    """
    sys.path.insert(0, str(HERE))
    try:
        import semantic
    except ImportError as error:
        return {"status": "blocked", "reason": f"semantic.py unavailable: {error}"}
    return semantic.supplement(manifest, documents, site_packages, say=say)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AcquireError as failure:
        say(f"\nacquisition failed: {failure}")
        raise SystemExit(2) from failure

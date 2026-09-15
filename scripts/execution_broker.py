"""The private-root container broker, shared by the two execution setup scripts.

Deliberately the same shape as `Runner::broker` in the daemon: the same private `--root`,
`--runroot`, `--tmpdir`, `--volumepath`, `--network-config-dir` and `--hooks-dir`, the same
cleared environment, and the same rule that only the local identity and bus settings reach the
broker while nothing reaches a capsule.

If these drift apart, setup would qualify one storage location and the service would use
another -- and a qualification receipt for an execution root nobody runs in is worse than none,
because it reads as readiness.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


def execution_root(explicit: str | None) -> Path:
    """Where the service keeps its private container storage.

    Mirrors `Runner::new`: an explicit `storage_root`, else `<cache>/podman`. Never a path
    inside a repository under study (blueprint §2.3).
    """
    if explicit:
        return Path(explicit).resolve()
    if env := os.environ.get("LIBENR_EXECUTION_ROOT"):
        return Path(env).resolve()
    cache = os.environ.get("LIBENR_CACHE_HOME")
    if not cache:
        sys.exit(
            "no execution root: set LIBENR_EXECUTION_ROOT or LIBENR_CACHE_HOME, or pass --root"
        )
    return Path(cache).resolve() / "podman"


def daemon_binary(profile: str = "debug") -> Path:
    """Select the same Cargo output directory and profile used by qualification builds."""
    if profile not in {"debug", "release"}:
        raise ValueError("qualification profile must be debug or release")
    project = Path(__file__).resolve().parent.parent
    target = Path(os.environ.get("CARGO_TARGET_DIR", project / "target"))
    if not target.is_absolute():
        target = project / target
    return target / profile / "library-enrichmentd"


def description(root: Path, *, profile: str = "debug") -> dict:
    """Read the current Rust-owned setup/runtime contract; never reconstruct its flags."""
    import json

    daemon = daemon_binary(profile)
    result = subprocess.run(
        [str(daemon), "execution-describe", str(root)],
        capture_output=True,
        text=True,
        check=True,
        timeout=30,
    )
    return json.loads(result.stdout)


def broker(root: Path) -> list[str]:
    value = description(root)["broker"]
    return [value["program"], *value["args"]]


def broker_env(root: Path) -> dict[str, str]:
    return description(root)["broker"]["env"]


def subdirectories(root: Path) -> list[str]:
    return description(root)["subdirectories"]


def probe(root: Path, ecosystem: str, image: str, *, profile: str = "debug") -> dict:
    """Execute the Rust-owned producer probes through the production capsule protocol."""
    import json

    daemon = daemon_binary(profile)
    result = subprocess.run(
        [str(daemon), "execution-probe", str(root), ecosystem, image],
        capture_output=True,
        text=True,
        check=False,
        timeout=900,
    )
    if result.returncode:
        raise RuntimeError(f"{ecosystem} qualification failed: {result.stderr.strip()}")
    return json.loads(result.stdout)

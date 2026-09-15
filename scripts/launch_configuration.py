"""Print absolute, locked launch configuration without installing or starting anything."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
from pathlib import Path

from cli import say, warn

ROOT = Path(__file__).resolve().parent.parent


def describe(state: Path, config: Path, profile: str, resources: str = "workstation") -> dict:
    """Resolve one explicit service state/configuration and the complete executable set."""
    if profile not in {"debug", "release"}:
        raise ValueError("build profile must be debug or release")
    if resources not in {"portable", "workstation"}:
        raise ValueError("resource configuration must be portable or workstation")
    if not state.is_absolute() or not config.is_absolute():
        raise ValueError("state and configuration paths must be absolute")
    state, config = state.resolve(), config.resolve()
    python = ROOT / ".venv/bin/python"
    uv = shutil.which("uv")
    if not uv:
        raise ValueError("uv is required to launch the locked service environment")
    binaries = {
        name: ROOT / "target" / profile / name
        for name in (
            "library-enrichmentd",
            "library-enrichment-executor",
            "library-enrichment-native-worker",
        )
    }
    for path in (python, *binaries.values()):
        if not path.is_file() or not os.access(path, os.X_OK):
            raise ValueError(f"required executable is missing: {path}")
    socket = state / "run/d.sock"
    if len(os.fsencode(socket)) >= 108:
        raise ValueError("service state makes the Unix socket path too long")
    environment = {
        "LIBENR_HOME": str(state),
        "LIBENR_CACHE_HOME": str(state / "cache"),
        "LIBENR_DATA_HOME": str(state / "data"),
        "LIBENR_SOCKET": str(socket),
        "LIBENR_CONFIG": str(config),
        "UV_PROJECT_ENVIRONMENT": str(ROOT / ".venv"),
        "UV_NO_SYNC": "1",
    }
    adapter = {
        "command": str(Path(uv).absolute()),
        "args": [
            "run",
            "--frozen",
            "--no-sync",
            "--project",
            str(ROOT),
            "--python",
            str(python),
            "python",
            "-I",
            "-B",
            "-m",
            "enrichment_mcp",
        ],
        "env": environment,
    }
    base = (
        (ROOT / "config/service.workstation.toml").read_text()
        if resources == "workstation"
        else 'config_version = "1.0"\n\n[policy]\nenabled_profiles = ["static"]\n'
    )
    config_text = base + f"\n[producers.python]\nworker_python = {json.dumps(str(python))}\n"
    identities = {}
    for name, path in {
        **binaries,
        "Cargo.lock": ROOT / "Cargo.lock",
        "uv.lock": ROOT / "uv.lock",
    }.items():
        with path.open("rb") as stream:
            identities[name] = hashlib.file_digest(stream, "sha256").hexdigest()
    return {
        "format": "library-enrichment-launch/1",
        "profile": profile,
        "resources": resources,
        "repository": str(ROOT),
        "configuration_path": str(config),
        "configuration_toml": config_text,
        "configuration_sha256": hashlib.sha256(config_text.encode()).hexdigest(),
        "mcpServers": {"library-enrichment": adapter},
        "daemon": {
            "command": str(binaries["library-enrichmentd"]),
            "args": ["start"],
            "env": environment,
        },
        "worker_python": str(python),
        "native_executables": {name: str(path) for name, path in binaries.items()},
        "sha256": identities,
        "setup_required": (
            "uv sync --locked; write/review configuration_toml at configuration_path "
            "before daemon start"
        ),
        "context7": "register separately in the calling client; no service proxy",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--state", type=Path, required=True)
    parser.add_argument("--config", type=Path, required=True)
    parser.add_argument("--profile", choices=("debug", "release"), default="release")
    parser.add_argument("--resources", choices=("portable", "workstation"), default="workstation")
    args = parser.parse_args()
    try:
        say(json.dumps(describe(args.state, args.config, args.profile, args.resources), indent=2))
    except (OSError, ValueError) as error:
        warn(f"launch-configuration: {error}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

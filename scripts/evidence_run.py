#!/usr/bin/env python3
"""Run a test command and bind its log to the actual source and dependency inputs."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
from datetime import UTC, datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INPUT_DIRS = (
    "crates",
    "python",
    "tests",
    "schemas",
    "skills",
    "contracts",
    "config",
    "scripts",
    "rules",
    "rule-tests",
)
INPUT_FILES = (
    "Cargo.toml",
    "Cargo.lock",
    "uv.lock",
    "pyproject.toml",
    "rust-toolchain.toml",
    "justfile",
    "deny.toml",
    ".python-version",
)

#: Environment variables a recorded command needs in order to run the same way again.
#:
#: `argv` alone is not reproduction instructions. `cargo nextest --message-format
#: libtest-json-plus` exits 95 without `NEXTEST_EXPERIMENTAL_LIBTEST_JSON`, and the sandbox tier
#: silently skips without its image variables -- so a receipt that recorded only the command
#: would send a reader to a run that cannot produce the log it is attached to.
#:
#: Values, not just names: these are paths, image digests and feature flags, none of them
#: secret. Nothing matching a credential pattern is collected, because this file is evidence
#: that gets read and copied around.
RECORDED_ENVIRONMENT_PREFIXES = ("LIBENR_", "NEXTEST_", "CARGO_", "RUST", "UV_", "PYTHON")
#: Never recorded, whatever prefix they carry.
SECRET_MARKERS = ("TOKEN", "SECRET", "PASSWORD", "KEY", "CREDENTIAL", "AUTH")


def reproducing_environment() -> dict[str, str]:
    """The environment settings that change whether this command works."""
    values = {
        name: value
        for name, value in sorted(os.environ.items())
        if name.startswith(RECORDED_ENVIRONMENT_PREFIXES)
        and not any(marker in name.upper() for marker in SECRET_MARKERS)
    }
    # This is an explicit boolean setup choice, not a credential. Recording it lets the
    # independent auditor reproduce authorized isolated existing-login client acceptance.
    if os.environ.get("LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS") == "1":
        values["LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS"] = "1"
    return values


def source_digest(root: Path = ROOT) -> str:
    paths = set(INPUT_FILES)
    for directory in INPUT_DIRS:
        base = root / directory
        if base.exists():
            paths.update(
                str(p.relative_to(root))
                for p in base.rglob("*")
                if p.is_file() and "__pycache__" not in p.parts and p.suffix != ".pyc"
            )
    digest = hashlib.sha256()
    for relative in sorted(paths):
        path = root / relative
        if path.is_file():
            digest.update(relative.encode() + b"\0")
            digest.update(hashlib.sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def log_digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_receipt(path: Path) -> dict:
    return json.loads(path.with_suffix(path.suffix + ".execution.json").read_text())


def client_outputs_match(path: Path) -> bool:
    """A client result is evidence only while its exact retained traces remain intact."""
    report = json.loads(path.read_text())
    manifests = [test.get("metadata", {}).get("client_evidence") for test in report["tests"]]
    if not manifests or not manifests[0] or any(m != manifests[0] for m in manifests):
        return False
    manifest = manifests[0]
    root = Path(manifest["root"])
    if not root.is_absolute() or not root.is_dir() or "summary.json" not in manifest["files"]:
        return False
    for relative, expected in manifest["files"].items():
        file = root / relative
        if not file.resolve().is_relative_to(root.resolve()) or log_digest(file) != expected:
            return False
    return True


def valid_receipt(path: Path, expected_source: str | None = None) -> bool:
    try:
        receipt = read_receipt(path)
        return (
            receipt["source_before"]
            == receipt["source_after"]
            == (expected_source or source_digest())
            and receipt["log_sha256"] == log_digest(path)
            and isinstance(receipt["exit_code"], int)
            and bool(receipt["argv"])
            and bool(receipt["started_at"])
            and bool(receipt["finished_at"])
            and bool(receipt["executable_sha256"])
            and bool(receipt["tool_versions"])
            and bool(receipt["native_before"])
            and receipt["native_before"] == receipt["native_after"]
            and (
                receipt["exit_code"] != 0
                or not any(
                    receipt["argv"][i : i + 2] == ["-m", "client"]
                    for i in range(len(receipt["argv"]) - 1)
                )
                or client_outputs_match(path)
            )
        )
    except (OSError, ValueError, KeyError, TypeError):
        return False


def execution_tools() -> dict[str, str]:
    versions = {}
    for name, argv in {
        "rustc": ["rustc", "--version"],
        "cargo": ["cargo", "--version"],
        "nextest": ["cargo", "nextest", "--version"],
        "uv": ["uv", "--version"],
        "python": ["uv", "run", "--frozen", "python", "--version"],
        "ty": ["uv", "run", "--frozen", "ty", "--version"],
        "ruff": ["uv", "run", "--frozen", "ruff", "--version"],
        "codex": ["codex", "--version"],
        "claude": ["claude", "--version"],
    }.items():
        try:
            result = subprocess.run(argv, capture_output=True, text=True, timeout=20, check=False)
            versions[name] = (result.stdout + result.stderr).strip()
        except (OSError, subprocess.TimeoutExpired) as error:
            versions[name] = str(error)
    return versions


def native_executables() -> dict[str, str | None]:
    """Bind the native executables used by Python fixtures, including the contained helper."""
    identities = {}
    for name in (
        "library-enrichmentd",
        "library-enrichment-executor",
        "library-enrichment-native-worker",
    ):
        path = ROOT / "target/debug" / name
        if path.is_file():
            with path.open("rb") as stream:
                identities[str(path)] = hashlib.file_digest(stream, "sha256").hexdigest()
        else:
            identities[str(path)] = None
    return identities


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--log", type=Path, required=True)
    parser.add_argument("--stdout", action="store_true")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    argv = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not argv:
        parser.error("a command is required")
    args.log.parent.mkdir(parents=True, exist_ok=True)
    args.log.unlink(missing_ok=True)
    receipt = {
        "argv": argv,
        "cwd": str(Path.cwd()),
        "executable": shutil.which(argv[0]),
        "executable_sha256": log_digest(Path(shutil.which(argv[0]) or argv[0])),
        "environment": reproducing_environment(),
        "tool_versions": execution_tools(),
        "native_before": native_executables(),
        "source_before": source_digest(),
        "started_at": datetime.now(UTC).isoformat(),
        "commit": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip(),
    }
    if args.stdout:
        with args.log.open("wb") as out:
            result = subprocess.run(argv, stdout=out, check=False)
    else:
        result = subprocess.run(argv, check=False)
    receipt.update(
        source_after=source_digest(),
        native_after=native_executables(),
        finished_at=datetime.now(UTC).isoformat(),
        exit_code=result.returncode,
        log_sha256=log_digest(args.log) if args.log.exists() else None,
    )
    args.log.with_suffix(args.log.suffix + ".execution.json").write_text(
        json.dumps(receipt, indent=2) + "\n"
    )
    unchanged = (
        receipt["source_before"] == receipt["source_after"]
        and receipt["native_before"] == receipt["native_after"]
    )
    return result.returncode if unchanged else 1


if __name__ == "__main__":
    raise SystemExit(main())

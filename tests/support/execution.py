"""Independent live-container observations for scratch and cleanup fixtures."""

import json
import os
import subprocess
import sys
import tempfile
import tomllib
from contextlib import contextmanager
from pathlib import Path

import anyio
from execution_broker import description


def container_name(root: Path, capsule: Path) -> str | None:
    for path in (root / "owned").glob("*.json"):
        try:
            record = json.loads(path.read_text())
        except FileNotFoundError:
            continue
        if record.get("capsule") == str(capsule):
            return record["name"]
    return None


async def broker(root: Path, *args: str):
    contract = await anyio.to_thread.run_sync(description, root)
    native = contract["broker"]
    with anyio.fail_after(10):
        return await anyio.run_process(
            [native["program"], *native["args"], *args],
            env=native["env"],
            cwd=root,
            check=False,
        )


async def heartbeat(root: Path, capsule: Path) -> bytes | None:
    name = container_name(root, capsule)
    if name is None:
        return None
    result = await broker(root, "exec", name, "/bin/cat", "/capsule/heartbeat")
    return result.stdout if result.returncode == 0 and result.stdout else None


def complete_fixture_configuration(path: Path) -> None:
    """Fill omitted execution settings from the actual selected qualification configuration.

    Explicit fixture overrides still apply and must qualify independently when they change
    containment. A receipt supplies configuration, never bypasses the daemon's identity checks.
    """
    selected = os.environ.get("LIBENR_EXECUTION_TEST_ROOT")
    if not selected:
        return
    raw = path.read_text()
    config = tomllib.loads(raw)
    execution = config.get("execution", {})
    if execution.get("storage_root") != selected:
        return
    receipt = Path(selected) / "admitted-images.json"
    if not receipt.is_file():
        raise ValueError("selected execution root has no qualification receipt")
    qualified = json.loads(receipt.read_text())
    effective = qualified.get("configuration")
    if not isinstance(effective, dict):
        raise ValueError("requalify execution: receipt lacks the effective configuration")
    inherited = {
        name: value
        for name, value in effective.items()
        if name not in execution
        and value is not None
        and name not in {"storage_root", "python_image", "rust_image"}
    }
    additions = "".join(f"{name}={json.dumps(value)}\n" for name, value in inherited.items())
    if additions:
        path.write_text(raw.replace("[execution]", "[execution]\n" + additions, 1))


@contextmanager
def qualified_override(config: Path, root: Path):
    """Actually qualify a changed broker; restore readiness with another actual run.

    Used only by the fixture that deliberately refuses removal after it is armed. A copied
    receipt cannot establish its new broker identity. Run this fixture serially.
    """
    original = json.loads((root / "admitted-images.json").read_text())
    effective = original["configuration"]
    repository = Path(__file__).resolve().parents[2]
    command = [
        sys.executable,
        str(repository / "scripts/execution-qualify.py"),
        "--root",
        str(root),
        "--apply",
    ]
    for ecosystem, image in original["images"].items():
        command.extend([f"--{ecosystem}-image", image])
    with tempfile.TemporaryDirectory(prefix=".qualification-restore-", dir=root.parent) as folder:
        restore = Path(folder) / "service.toml"
        restore.write_text(
            "[execution]\n"
            + "".join(
                f"{key}={json.dumps(value)}\n"
                for key, value in effective.items()
                if value is not None
            )
        )
        try:
            with (config.parent / "wrapper-qualification.log").open("w") as log:
                subprocess.run(
                    command,
                    cwd=repository,
                    env={**os.environ, "LIBENR_CONFIG": str(config), "CARGO_INCREMENTAL": "0"},
                    stdout=log,
                    stderr=subprocess.STDOUT,
                    check=True,
                    timeout=600,
                )
            yield
        finally:
            with (config.parent / "restored-qualification.log").open("w") as log:
                subprocess.run(
                    command,
                    cwd=repository,
                    env={**os.environ, "LIBENR_CONFIG": str(restore), "CARGO_INCREMENTAL": "0"},
                    stdout=log,
                    stderr=subprocess.STDOUT,
                    check=True,
                    timeout=600,
                )

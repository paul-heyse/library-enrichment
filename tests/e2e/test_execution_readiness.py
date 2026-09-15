"""Configured is not qualified: `service_status` reports execution readiness truthfully.

An image ID in a configuration file is a statement of intent. It says nothing about whether
that image exists, whether its tools are the ones we pinned, or whether this host can contain
it. Only `just execution-qualify` establishes that, by running the real containment tier and
writing a receipt naming the exact image IDs it ran against.

These drive a real daemon over real MCP. The receipt they read is the one an actual containment
run wrote; nothing here fabricates one.
"""

import json
import os
from pathlib import Path

import pytest
from fastmcp import Client

from support import daemon

ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT", "")
IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_PYTHON", "")
pytestmark = [
    pytest.mark.sandbox,
    daemon.requires_daemon_binary,
    pytest.mark.skipif(
        not ROOT or not IMAGE,
        reason=(
            "set LIBENR_EXECUTION_TEST_ROOT and LIBENR_EXECUTION_TEST_PYTHON "
            "after execution-image setup"
        ),
    ),
]

#: A well-formed image ID that was never built, let alone qualified. Format validity is not
#: admission: `Runner::valid_image` only checks the shape.
UNQUALIFIED = "sha256:" + "b" * 64


def configuration(path: Path, image: str) -> None:
    path.write_text(f"""[policy]
enabled_profiles=["static","build","runtime"]
[limits]
inline_wait_seconds=0
[execution]
storage_root={json.dumps(ROOT)}
python_image={json.dumps(image)}
""")


async def status(client) -> dict:
    return await daemon.read_complete_answer(
        client, (await client.call_tool("service_status", {})).structured_content
    )


def component(payload: dict, name: str) -> dict:
    for entry in payload["data"]["producers"] + payload["data"]["features"]:
        if entry["name"] == name:
            return entry
    raise AssertionError(f"{name} is not reported at all, which is its own failure")


async def test_a_qualified_image_is_reported_as_qualified_with_its_receipt(tmp_path: Path):
    config = tmp_path / "service.toml"
    configuration(config, IMAGE)
    receipt = json.loads((Path(ROOT) / "admitted-images.json").read_text())
    if receipt["images"].get("python") != IMAGE:
        pytest.skip("the admitted-images receipt covers a different Python image")

    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            answer = await status(client)
            sandbox = answer["data"]["sandbox"]
            assert sandbox["execution_qualified"] is True, sandbox
            assert receipt["qualified_at"] in sandbox["execution_readiness"]
            assert sandbox["admitted_images"]["python"] == IMAGE
            # Enabled, present and qualified are three separate facts; all three hold here.
            assert "build" in sandbox["enabled_profiles"]
            assert "podman" in sandbox["available_runtimes"]
            assert component(answer, "usage-verification")["available"] is True
            assert component(answer, "ty")["available"] is True
            # This configuration has a Python image and no Rust one. Qualification is per-image,
            # so rust-analyzer is unavailable for want of an image to run in -- and the detail
            # says which of the two reasons it is.
            rust_analyzer = component(answer, "rust-analyzer")
            assert rust_analyzer["available"] is False
            assert "Rust producer image" in rust_analyzer["detail"], rust_analyzer


async def test_an_unqualified_image_is_not_borrowed_readiness_from_a_neighbour(tmp_path: Path):
    """A receipt for one image must not make a different configured image look ready.

    The failure this guards: qualify an image, then point the configuration somewhere else and
    keep reporting readiness on the old evidence.
    """
    config = tmp_path / "service.toml"
    configuration(config, UNQUALIFIED)

    env = daemon.daemon_env(tmp_path / "state", config)
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            answer = await status(client)
            sandbox = answer["data"]["sandbox"]
            assert sandbox["execution_qualified"] is False, sandbox
            assert UNQUALIFIED in sandbox["execution_readiness"]
            assert sandbox["admitted_images"] == {}
            # A profile is still enabled and a runtime is still present. Neither is readiness.
            assert "build" in sandbox["enabled_profiles"]
            assert "podman" in sandbox["available_runtimes"]

            verification = component(answer, "usage-verification")
            assert verification["available"] is False
            assert "not qualified" in verification["detail"], verification
            assert "execution-qualify" in verification["detail"], verification

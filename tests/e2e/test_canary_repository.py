"""Gate C20: core operations never touch a working repository.

The blueprint's first binding boundary (§2.3): service state, environments, caches and outputs
live outside every working repository, and "a repository under study is never a subprocess
working directory, an extraction destination, or an install target."

The oracle is a **content-and-path digest** over a canary tree -- every path, its mode, and a
hash of its bytes -- taken before and after, under **every enabled profile**. Filenames and
sizes are not enough: a service that rewrote a file in place with the same length, or flipped an
executable bit, or added a `.gitignore`d directory, would pass a weaker check while having done
exactly the thing this boundary forbids.

Two things sharpen it beyond a passive comparison. The daemon is started **inside** the canary,
so any relative path anywhere in the service lands there. And the canary is also passed to a
runtime probe as a string, so a probe that took a caller-supplied path seriously would write to
it.
"""

from __future__ import annotations

import hashlib
import json
import os
import zipfile
from pathlib import Path
from typing import Any

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

ROOT = os.environ.get("LIBENR_EXECUTION_TEST_ROOT", "")
IMAGE = os.environ.get("LIBENR_EXECUTION_TEST_PYTHON", "")
pytestmark = [daemon.requires_daemon_binary]

#: Which profile sets to sweep. Each is a separate run of the whole operation set, because the
#: boundary has to hold under the profiles that actually execute code, not only under `static`.
PROFILE_SETS = (["static"], ["static", "build"], ["static", "build", "runtime"])


def tree_digest(root: Path) -> str:
    """Path, mode and content, for every entry. Symlinks digest their target, not its bytes."""
    entries = []
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root).as_posix()
        mode = oct(path.lstat().st_mode)
        if path.is_symlink():
            entries.append(f"{relative}\tlink\t{mode}\t{os.readlink(path)}")
        elif path.is_dir():
            entries.append(f"{relative}\tdir\t{mode}")
        else:
            entries.append(
                f"{relative}\tfile\t{mode}\t{hashlib.sha256(path.read_bytes()).hexdigest()}"
            )
    return hashlib.sha256("\n".join(entries).encode()).hexdigest()


def build_canary(root: Path) -> None:
    """A small but awkward working repository: nested, executable, linked, non-ASCII, git-like."""
    (root / "src/deep/nested").mkdir(parents=True)
    (root / "src/deep/nested/module.py").write_text("VALUE = 1\n")
    (root / "src/Grüße.txt").write_text("non-ascii name\n")
    (root / "README.md").write_text("# canary\n")
    script = root / "run.sh"
    script.write_text("#!/bin/sh\necho canary\n")
    script.chmod(0o755)
    (root / "link.py").symlink_to("src/deep/nested/module.py")
    # A .git directory makes it look like a repository to anything that sniffs for one.
    (root / ".git").mkdir()
    (root / ".git/HEAD").write_text("ref: refs/heads/main\n")
    (root / "Cargo.toml").write_text('[package]\nname = "canary"\nversion = "0.0.0"\n')
    (root / "pyproject.toml").write_text('[project]\nname = "canary"\nversion = "0.0.0"\n')


def upstream_fixture(root: Path) -> None:
    (root / "static").mkdir(parents=True)
    (root / "pypi/canary-demo").mkdir(parents=True)
    name = "canary_demo-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr("canary_demo/__init__.py", "def twice(v: int) -> int:\n    return v * 2\n")
        archive.writestr("canary_demo/py.typed", "")
        archive.writestr(
            "canary_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: canary-demo\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            "canary_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr("canary_demo-1.0.dist-info/RECORD", "")
    (root / "pypi/canary-demo/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": "canary-demo", "version": "1.0"},
                "urls": [
                    {
                        "filename": name,
                        "packagetype": "bdist_wheel",
                        "url": "{{BASE_URL}}/static/" + name,
                        "digests": {"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
                        "requires_python": ">=3.10",
                        "yanked": False,
                    }
                ],
            }
        )
    )
    (root / "pypi/canary-demo/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str, profiles: list[str]) -> None:
    execution = ""
    if ROOT and IMAGE:
        execution = (
            f"[execution]\nstorage_root={json.dumps(ROOT)}\n"
            f"python_image={json.dumps(IMAGE)}\ndeadline_seconds=60\n"
        )
    path.write_text(
        f"""[policy]
enabled_profiles={json.dumps(profiles)}
[limits]
inline_wait_seconds=0
{execution}[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
"""
    )


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def finish(client, pending: dict[str, Any]) -> dict[str, Any]:
    return await daemon.wait_for_answer(client, pending)


@pytest.mark.parametrize("profiles", PROFILE_SETS, ids=lambda p: "+".join(p))
async def test_core_operations_leave_a_canary_repository_byte_identical(
    tmp_path: Path, profiles: list[str]
):
    executes = profiles != ["static"]
    if executes and not (ROOT and IMAGE):
        pytest.skip(
            "the build and runtime profiles need LIBENR_EXECUTION_TEST_ROOT and "
            "LIBENR_EXECUTION_TEST_PYTHON; run just execution-qualify"
        )

    canary = tmp_path / "studied-repository"
    canary.mkdir()
    build_canary(canary)
    before = tree_digest(canary)

    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url, profiles)
        env = daemon.daemon_env(tmp_path / "state", config)
        # Both processes live inside the canary. A relative path anywhere in the service, or a
        # producer that used its own working directory as scratch, lands here.
        with daemon.running(env, cwd=canary):
            async with Client(daemon.transport(env, cwd=canary)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="canary-demo",
                    version="1.0",
                    python_version="3.14",
                )
                assert resolved["status"] != "error", resolved
                context = resolved["context_id"]

                # One operation of every kind this profile set permits.
                assert (await call(client, "library_overview", context_id=context))[
                    "status"
                ] != "error"
                assert (await call(client, "search_evidence", context_id=context, query="twice"))[
                    "status"
                ] != "error"
                await call(
                    client, "inspect_symbol", context_id=context, symbol_path="canary_demo.twice"
                )
                await call(client, "service_status")
                if executes:
                    semantic = await finish(
                        client,
                        await call(
                            client,
                            "inspect_symbol",
                            context_id=context,
                            symbol_path="canary_demo.twice",
                            selection={
                                "mode": "explicit",
                                "aspects": [{"aspect": name} for name in ["semantics"]],
                            },
                            execution={
                                "intent": "execute_on_miss",
                                "profile": "build",
                                "methods": ["definition"],
                            },
                        ),
                    )
                    assert semantic["status"] in {"ok", "partial"}, semantic
                    observation = semantic["data"]["execution_observations"][0]
                    assert observation["payload"]["value"]["outcome"] == "results", observation
                for artifact in resolved["artifacts"]:
                    await call(client, "read_artifact", artifact_id=artifact["artifact_id"])

                if executes:
                    # A probe whose snippet names the canary. The service passes the snippet to a
                    # container that cannot see the host at all; a service that ran it anywhere
                    # else would rewrite the file named here.
                    target = canary / "README.md"
                    verification = await finish(
                        client,
                        await call(
                            client,
                            "verify_usage",
                            context_id=context,
                            snippet=(
                                "from canary_demo import twice\nvalue: int = twice(2)\n"
                                f"# {target}\n"
                            ),
                            mode="typecheck",
                            profile="build",
                        ),
                    )
                    assert verification["status"] in {"ok", "partial"}, verification
                    assert verification["data"]["observations"][-1]["exit_code"] == 0
                    assert verification["data"]["observations"][-1]["cleanup_confirmed"]
                if profiles == ["static", "build", "runtime"]:
                    verification = await finish(
                        client,
                        await call(
                            client,
                            "verify_usage",
                            context_id=context,
                            snippet=(
                                "import pathlib\n"
                                f"target = pathlib.Path({str(canary / 'README.md')!r})\n"
                                "assert not target.exists(), 'the canary is visible from a probe'\n"
                                "from canary_demo import twice\n"
                                "assert twice(2) == 4\n"
                            ),
                            mode="runtime",
                            profile="runtime",
                            test_intent="the canary is not reachable from inside a probe",
                        ),
                    )

                    assert verification["status"] in {"ok", "partial"}, verification
                    assert verification["data"]["observations"][-1]["exit_code"] == 0
                    assert verification["data"]["observations"][-1]["cleanup_confirmed"]

    after = tree_digest(canary)
    assert after == before, (
        f"the canary tree changed under profiles {profiles}. This is the boundary in §2.3: a "
        "repository under study is never a working directory, an extraction destination or an "
        "install target."
    )
    # Nothing new appeared at the top level either -- a digest over an unchanged tree would not
    # notice a file the service created and then removed, but a leftover is exactly what would
    # survive here.
    assert sorted(p.name for p in canary.iterdir()) == [
        ".git",
        "Cargo.toml",
        "README.md",
        "link.py",
        "pyproject.toml",
        "run.sh",
        "src",
    ]


@pytest.mark.parametrize("profiles", PROFILE_SETS, ids=lambda p: "+".join(p))
async def test_completed_rust_operations_leave_canary_unchanged(
    tmp_path: Path, profiles: list[str]
):
    rust_image = os.environ.get("LIBENR_EXECUTION_TEST_RUST")
    executes = "build" in profiles
    if executes and not (ROOT and rust_image):
        pytest.skip("Rust canary execution requires the qualified Rust image and execution root")
    canary = tmp_path / "studied-repository"
    canary.mkdir()
    build_canary(canary)
    before = tree_digest(canary)
    config = tmp_path / "service.toml"
    with serve(daemon.ROOT / "tests/fixtures/upstream") as upstream:
        config.write_text(
            f"[policy]\nenabled_profiles={json.dumps(profiles)}\n"
            "[limits]\ninline_wait_seconds=0\n"
            + (
                f"[execution]\nstorage_root={json.dumps(ROOT)}\n"
                f"rust_image={json.dumps(rust_image)}\n"
                if executes
                else ""
            )
            + f'[producers.rust]\ncrates_io_index_url="{upstream.base_url}/index"\n'
            f'crates_io_api_url="{upstream.base_url}/api/v1"\ndocs_rs_url="{upstream.base_url}"\n'
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env, cwd=canary):
            async with Client(daemon.transport(env, cwd=canary)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="rust",
                    name="enr-fixture",
                    version="0.1.0" if executes else "0.2.0",
                    allow_local_build=executes,
                )
                assert resolved["status"] in {"ok", "partial"}, resolved
                assert resolved["data"]["snapshot"]["counts"]["symbols"] > 0, resolved
                context = resolved["context_id"]
                inspected = await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    symbol_path="enr_fixture::Shape",
                    selection={
                        "mode": "explicit",
                        "aspects": [{"aspect": name} for name in ["signature"]],
                    },
                )
                assert inspected["status"] in {"ok", "partial"}, inspected
                if executes:
                    semantic = await finish(
                        client,
                        await call(
                            client,
                            "inspect_symbol",
                            context_id=context,
                            symbol_path="enr_fixture::Widget",
                            selection={
                                "mode": "explicit",
                                "aspects": [{"aspect": name} for name in ["semantics"]],
                            },
                            execution={
                                "intent": "execute_on_miss",
                                "profile": "build",
                                "methods": ["definition"],
                            },
                        ),
                    )
                    assert semantic["status"] in {"ok", "partial"}, semantic
                    value = semantic["data"]["execution_observations"][0]["payload"]["value"]
                    assert value["outcome"] == "results" and value["locations"], value
                    snippet = "fn main() { let _ = enr_fixture::Widget::new(2); }"
                    verified = await finish(
                        client,
                        await call(
                            client,
                            "verify_usage",
                            context_id=context,
                            snippet=snippet,
                            mode="compile",
                            profile="build",
                        ),
                    )
                    assert verified["status"] in {"ok", "partial"}, verified
                    assert verified["data"]["observations"][-1]["exit_code"] == 0
                    assert verified["data"]["observations"][-1]["cleanup_confirmed"]
                if "runtime" in profiles:
                    target = json.dumps(str(canary / "README.md"))
                    snippet = (
                        "fn main() { assert!(!std::path::Path::new(" + target + ").exists()); "
                        "let _ = enr_fixture::Widget::new(2); }"
                    )
                    verified = await finish(
                        client,
                        await call(
                            client,
                            "verify_usage",
                            context_id=context,
                            snippet=snippet,
                            mode="runtime",
                            profile="runtime",
                            test_intent="Rust cannot reach the working repository",
                        ),
                    )
                    assert verified["status"] in {"ok", "partial"}, verified
                    assert verified["data"]["observations"][-1]["exit_code"] == 0
                    assert verified["data"]["observations"][-1]["cleanup_confirmed"]
    assert tree_digest(canary) == before

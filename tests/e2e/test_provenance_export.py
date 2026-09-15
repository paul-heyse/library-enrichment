"""Provenance export: a bundle a recipient can check without this service (ADR-0018).

Phase 6 asks for provenance export; ADR-0018 decides its shape. This drives the operator path
the way an operator would: resolve a real (fixture-served) library through MCP, export the
context it produced, verify the bundle, and then damage it and require the verifier to notice.

The unit tests in `enrichment_daemon::export::tests` cover the verifier's cases directly. What
they cannot cover is whether `export` produces a bundle that actually verifies against evidence
the service really published -- which is the point of doing it end to end here.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import zipfile
from pathlib import Path
from typing import Any

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary

PACKAGE = "export-demo"
MODULE = "export_demo"


def upstream_fixture(root: Path) -> None:
    """A one-module wheel, enough to publish a real snapshot with real artifacts."""
    (root / "static").mkdir(parents=True, exist_ok=True)
    (root / f"pypi/{PACKAGE}").mkdir(parents=True, exist_ok=True)
    name = f"{MODULE}-1.0-py3-none-any.whl"
    wheel = root / "static" / name
    with zipfile.ZipFile(wheel, "w") as archive:
        archive.writestr(
            f"{MODULE}/__init__.py",
            '"""A module whose evidence has to survive a round trip."""\n\n\n'
            "def halve(value: int) -> float:\n"
            '    """Return half of *value*."""\n'
            "    return value / 2\n",
        )
        archive.writestr(f"{MODULE}/py.typed", "")
        archive.writestr(
            f"{MODULE}-1.0.dist-info/METADATA",
            f"Metadata-Version: 2.4\nName: {PACKAGE}\nVersion: 1.0\nRequires-Python: >=3.10\n\n",
        )
        archive.writestr(
            f"{MODULE}-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
        )
        archive.writestr(f"{MODULE}-1.0.dist-info/RECORD", "")
    (root / f"pypi/{PACKAGE}/1.0.json").write_text(
        json.dumps(
            {
                "info": {"name": PACKAGE, "version": "1.0"},
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
    (root / f"pypi/{PACKAGE}/index.json").write_text(json.dumps({"versions": ["1.0"]}))


def configuration(path: Path, base: str) -> None:
    path.write_text(f"""[policy]
enabled_profiles=["static"]
[limits]
inline_wait_seconds=30
[freshness]
registry_ttl_seconds=0
[producers.python]
pypi_url="{base}/pypi"
simple_url="{base}/simple"
""")


async def call(client, tool: str, **params: Any) -> dict[str, Any]:
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, params, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


def cli(env: dict[str, str], *args: str) -> subprocess.CompletedProcess[str]:
    """Run the daemon binary as an operator would, with no daemon running."""
    return subprocess.run(
        [str(daemon.DAEMON_BIN), *args],
        env=env,
        capture_output=True,
        text=True,
        timeout=120,
        check=False,
    )


async def test_an_exported_bundle_verifies_offline_and_notices_a_damaged_byte(tmp_path: Path):
    """Export a real context, verify the bundle, then break it and require a failure."""
    upstream_fixture(tmp_path / "upstream")
    config = tmp_path / "service.toml"
    with serve(tmp_path / "upstream") as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                answer = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name=PACKAGE,
                    version="1.0",
                    python_version="3.14",
                )
            assert answer["status"] != "error", answer
            context_id = answer["context_id"]
            snapshot_id = answer["snapshot_id"]

    # The service is stopped. Export still works -- that is the point of reading the store
    # directly, because a wedged service is when an operator most wants a bundle.
    out = tmp_path / "bundle"
    exported = cli(env, "export", context_id, str(out))
    assert exported.returncode == 0, exported.stderr

    manifest = (out / "MANIFEST.sha256").read_text()
    assert manifest.strip(), "a bundle with an empty manifest vouches for nothing"

    described = json.loads((out / "bundle.json").read_text())
    # Identities are the service's own, not re-minted ones (ADR-0018 rule 1). Without this a
    # bundle could not be related back to the store that produced it.
    assert described["context_id"] == context_id
    assert described["snapshot_id"] == snapshot_id
    assert described["bundle_version"] == "typed-evidence-bundle/5"
    assert described["source_catalog_generation"] > 0

    # The snapshot travelled, file for file.
    assert (out / "data/snapshots" / snapshot_id).is_dir()
    snapshot_files = sorted(p for p in (out / "data/snapshots").rglob("*") if p.is_file())
    assert snapshot_files, "the context had a snapshot, so the bundle must carry it"
    for file in snapshot_files:
        relative = file.relative_to(out).as_posix()
        assert f"  {relative}\n" in manifest, f"{relative} travelled unlisted"

    verified = cli(env, "verify-bundle", str(out))
    assert verified.returncode == 0, verified.stderr
    assert "verifies against its manifest" in verified.stderr

    # A recipient with no copy of this service reaches the same verdict with coreutils.
    checked = subprocess.run(
        ["sha256sum", "--check", "--quiet", "MANIFEST.sha256"],
        cwd=out,
        capture_output=True,
        text=True,
        check=False,
    )
    assert checked.returncode == 0, checked.stdout + checked.stderr

    # Now damage one byte of one snapshot file, keeping its length. A size or mtime check would
    # miss this; a digest does not.
    victim = snapshot_files[0]
    original = victim.read_bytes()
    victim.write_bytes(bytes((original[0] ^ 0xFF,)) + original[1:])
    damaged = cli(env, "verify-bundle", str(out))
    assert damaged.returncode != 0, damaged.stderr
    assert "does NOT verify" in damaged.stderr
    assert victim.relative_to(out).as_posix() in damaged.stderr
    victim.write_bytes(original)

    # A file smuggled in afterwards is a problem too, even though every listed digest still
    # checks out. Verification is about the directory, not only about the list.
    (out / "extra.json").write_text('{"unvouched": true}')
    smuggled = cli(env, "verify-bundle", str(out))
    assert smuggled.returncode != 0, smuggled.stderr
    assert "not listed" in smuggled.stderr
    (out / "extra.json").unlink()

    # Exporting again into the same directory is refused rather than merged: a manifest that
    # does not describe everything present is not verifiable.
    again = cli(env, "export", context_id, str(out))
    assert again.returncode != 0, again.stderr
    assert "empty directory" in again.stderr


@pytest.mark.parametrize(
    ("args", "expected"),
    [
        (("export", "not-a-context-id", "{out}"), "ctx_<16 hex>"),
        (("export", "ctx_" + "0" * 16, "{out}"), "context has no selected snapshot"),
        (("verify-bundle", "{out}"), "MANIFEST.sha256"),
    ],
)
def test_export_refuses_what_it_cannot_vouch_for(
    tmp_path: Path, args: tuple[str, ...], expected: str
):
    """A malformed id, an unknown context and a directory that is not a bundle all fail loudly.

    The failure that matters here is the quiet one: exiting zero on an empty directory would
    make `verify-bundle` a rubber stamp.
    """
    env = daemon.daemon_env(tmp_path / "state")
    with daemon.running(env):
        pass  # Establish an empty target store before asking it about an absent context.
    out = tmp_path / "bundle"
    out.mkdir()
    result = cli(env, *(a.format(out=out) for a in args))
    assert result.returncode != 0, result.stderr
    assert expected in result.stderr, result.stderr

"""Immutable source acquisition through actual HTTP, safe extraction, worker and stdio MCP."""

import hashlib
import io
import json
import sys
import tarfile
from pathlib import Path

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary
COMMIT = "0123456789abcdef0123456789abcdef01234567"
TREE = "1234567890abcdef1234567890abcdef12345678"


def revision_upstream(root: Path, repository: str, *, wrong_sha=False, bad_pax=False):
    for kind in ("commits", "tarball"):
        (root / "repos" / repository / kind).mkdir(parents=True, exist_ok=True)
    (root / "repos" / repository / "commits" / COMMIT).write_text(
        json.dumps({"sha": TREE if wrong_sha else COMMIT, "commit": {"tree": {"sha": TREE}}})
    )
    path = root / "repos" / repository / "tarball" / COMMIT
    files = {
        "py/pyproject.toml": '[project]\nname="revision-demo"\nversion="1.2.0"\n',
        "py/src/revision_demo/__init__.py": 'raise RuntimeError("MUST NOT IMPORT")\n'
        'def capability(value: int) -> int:\n    """Revision capability."""\n    return value\n',
        "py/README.md": "# Revision docs\nPrecise immutable source support.\n",
        "rs/Cargo.toml": '[package]\nname="revision-demo"\nversion="1.2.0"\n[features]\nfast=[]\n',
        "rs/src/lib.rs": "pub fn capability() {}\n",
        "rs/README.md": "# Revision docs\nPrecise immutable Rust support.\n",
        "rs/CHANGELOG.md": "# Changes\nWhitespace is preserved.\n",
    }
    with tarfile.open(
        path,
        "w:gz",
        format=tarfile.PAX_FORMAT,
        pax_headers={"comment": TREE if bad_pax else COMMIT},
    ) as archive:
        for name, content in files.items():
            payload = content.encode()
            item = tarfile.TarInfo("wrapper-abbreviated/" + name)
            item.size = len(payload)
            archive.addfile(item, io.BytesIO(payload))
    return hashlib.sha256(path.read_bytes()).hexdigest()


def configuration(path, base):
    path.write_text(
        'config_version="1.0"\n[policy]\nenabled_profiles=["static"]\n'
        f'[producers]\ngithub_api_url="{base}"\n'
        f'[producers.python]\nworker_python="{sys.executable}"\n'
    )


async def call(client, tool, **arguments):
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, arguments, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def test_revision_fixture_exact_roots_static_api_and_registry_scoped_offline(tmp_path):
    root = tmp_path / "upstream"
    digest = revision_upstream(root, "org/one")
    revision_upstream(root, "org/two")
    config = tmp_path / "config.toml"
    canary = tmp_path / "studied-repository"
    canary.mkdir()
    (canary / "important.py").write_text("original\n")
    before = {p.relative_to(canary): p.read_bytes() for p in canary.rglob("*") if p.is_file()}
    answers = []
    with serve(root) as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env, cwd=canary)) as client:
                for repository in ("org/one", "org/two"):
                    answer = await call(
                        client,
                        "resolve_library",
                        ecosystem="python",
                        name="revision-demo",
                        repository=f"https://github.com/{repository}",
                        revision=COMMIT,
                        package_subdir="py",
                        mode="revision",
                        python_version="3.14",
                    )
                    assert answer["status"] == "partial", answer
                    release = answer["data"]["release"]
                    assert release["version"] == COMMIT
                    assert release["registry"] == f"git:https://github.com/{repository}#path=py"
                    assert release["artifact_digest"]
                    assert not answer["freshness"]["latest_verified"]
                    assert "distribution_source" in answer["coverage"]["missing"]
                    answers.append(answer)
                    symbol = await call(
                        client,
                        "inspect_symbol",
                        context_id=answer["context_id"],
                        symbol_path="revision_demo.capability",
                    )
                    assert symbol["status"] != "error", symbol
                    assert "capability" in json.dumps(symbol["data"])
                assert answers[0]["context_id"] != answers[1]["context_id"]
                assert answers[0]["data"]["release"]["artifact_digest"] == digest
                rust = await call(
                    client,
                    "resolve_library",
                    ecosystem="rust",
                    name="revision-demo",
                    repository="https://github.com/org/one",
                    revision=COMMIT,
                    package_subdir="rs",
                    mode="revision",
                )
                assert rust["status"] == "partial", rust
                assert "public_api" in rust["coverage"]["missing"]
                assert rust["data"]["snapshot"]["counts"]["symbols"] == 0
                docs = await call(
                    client, "search_evidence", context_id=rust["context_id"], query="Whitespace"
                )
                assert docs["data"]["hits"], docs
                overview = await call(client, "library_overview", context_id=rust["context_id"])
                assert "fast" in json.dumps(overview["data"])
                no_root = await call(
                    client,
                    "resolve_library",
                    ecosystem="rust",
                    name="revision-demo",
                    repository="https://github.com/org/one",
                    revision=COMMIT,
                    mode="revision",
                )
                assert no_root["status"] == "error"
                assert "package_subdir" in no_root["error"]["message"]
                branch = await call(
                    client,
                    "resolve_library",
                    ecosystem="rust",
                    name="revision-demo",
                    repository="https://github.com/org/one",
                    revision="main",
                    mode="revision",
                )
                assert branch["status"] == "error"
        request_count = len(upstream.request_log)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                for repository, previous in zip(("org/one", "org/two"), answers, strict=True):
                    replay = await call(
                        client,
                        "resolve_library",
                        ecosystem="python",
                        name="revision-demo",
                        repository=f"https://github.com/{repository}",
                        revision=COMMIT,
                        package_subdir="py",
                        mode="revision",
                        python_version="3.14",
                        freshness="offline",
                    )
                    assert replay["context_id"] == previous["context_id"], replay
                    assert replay["snapshot_id"] == previous["snapshot_id"]
                assert len(upstream.request_log) == request_count
    assert before == {
        p.relative_to(canary): p.read_bytes() for p in canary.rglob("*") if p.is_file()
    }
    assert not list((tmp_path / "state/cache/unpacked").glob("revision-*"))


@pytest.mark.parametrize("wrong_sha,bad_pax", [(True, False), (False, True)])
async def test_revision_fixture_mismatched_identity_never_publishes(tmp_path, wrong_sha, bad_pax):
    root = tmp_path / "upstream"
    revision_upstream(root, "org/one", wrong_sha=wrong_sha, bad_pax=bad_pax)
    config = tmp_path / "config.toml"
    with serve(root) as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                result = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="revision-demo",
                    repository="https://github.com/org/one",
                    revision=COMMIT,
                    package_subdir="py",
                    mode="revision",
                )
                assert result["status"] == "error", result
                expected = "Resolved commit SHA" if wrong_sha else "PAX"
                assert expected in result["error"]["message"], result
                assert not result.get("snapshot_id")
                assert not list((tmp_path / "state/data").rglob("manifest.json"))
    assert not list((tmp_path / "state/cache/unpacked").glob("revision-*"))


@pytest.mark.parametrize("ecosystem,subdir", [("rust", "rs"), ("python", "py")])
async def test_revision_revalidation_preserves_snapshot_and_retains_new_receipt(
    tmp_path, ecosystem, subdir
):
    root = tmp_path / "upstream"
    revision_upstream(root, "org/one")
    config = tmp_path / "config.toml"
    with serve(root) as upstream:
        configuration(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                arguments = dict(
                    ecosystem=ecosystem,
                    name="revision-demo",
                    repository="https://github.com/org/one",
                    revision=COMMIT,
                    package_subdir=subdir,
                    mode="revision",
                )
                first = await call(client, "resolve_library", **arguments)
                assert first["status"] in {"ok", "partial"}, first
                data = Path(env["LIBENR_DATA_HOME"])
                snapshot = data / "snapshots" / first["snapshot_id"]
                before = {p.name: p.read_bytes() for p in snapshot.iterdir() if p.is_file()}
                old_receipts = {
                    run["log"]
                    for run in first["data"]["producer_runs"]
                    if run["producer"] == "github-revision"
                }
                assert len(old_receipts) == 1 and None not in old_receipts
                # Conditional HTTP 304 changes the acquisition receipt even within the same
                # clock second. Its operational observation must not become a semantic input.
                second = await call(client, "resolve_library", freshness="revalidate", **arguments)
                assert second["status"] in {"ok", "partial"}, second
                assert second["context_id"] == first["context_id"]
                assert second["snapshot_id"] == first["snapshot_id"]
                assert before == {p.name: p.read_bytes() for p in snapshot.iterdir() if p.is_file()}
                new_receipts = {
                    run["log"]
                    for run in second["data"]["producer_runs"]
                    if run["producer"] == "github-revision"
                }
                assert len(new_receipts) == 1 and None not in new_receipts
                assert old_receipts.isdisjoint(new_receipts)
                for artifact_id in old_receipts | new_receipts:
                    receipt = await call(client, "read_artifact", artifact_id=artifact_id)
                    assert receipt["status"] in {"ok", "partial"}, receipt
                    assert json.loads(receipt["data"]["content"])["source_revision"] == COMMIT
                assert not (data / "acquisitions").exists()

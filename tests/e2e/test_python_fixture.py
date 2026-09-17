"""Static Python extraction through real stdio MCP and immutable Rust snapshots."""

import hashlib
import json
import sys
import zipfile
import zlib
from pathlib import Path

import pytest
from fastmcp import Client

from support import daemon
from support.fixture_upstream import serve

pytestmark = daemon.requires_daemon_binary


def build_upstream(root, canary):
    metadata_dir = root / "pypi/evidence-demo"
    metadata_dir.mkdir(parents=True)
    (root / "static").mkdir()
    (root / "docs").mkdir()
    header = (
        b"# Sphinx inventory version 2\n# Project: fixture\n# Version: 0.0.0\n"
        b"# The remainder is compressed using zlib.\n"
    )
    (root / "docs/objects.inv").write_bytes(
        header + zlib.compress(b"overview std:label 1 guide.html Overview\n")
    )
    (root / "docs/guide.html").write_text(
        "<h1>Overview</h1><p>Fixture deployment instructions.</p>"
    )
    for version in ("1.0", "2.0"):
        filename = f"evidence_demo-{version}-py3-none-any.whl"
        wheel = root / "static" / filename
        with zipfile.ZipFile(wheel, "w", compression=zipfile.ZIP_DEFLATED) as archive:
            archive.writestr(
                f"evidence_demo-{version}.dist-info/METADATA",
                f"Metadata-Version: 2.4\nName: evidence-demo\nVersion: {version}\n"
                "Requires-Python: >=3.10\nProvides-Extra: speed\n"
                'Requires-Dist: optional-dep; extra == "speed"\n\n',
            )
            archive.writestr(
                f"evidence_demo-{version}.dist-info/WHEEL",
                "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
            )
            archive.writestr(
                "different/__init__.py",
                f'from pathlib import Path\nPath({str(canary)!r}).write_text("IMPORTED")\n'
                "from .api import Thing as PublicThing, coerce\n"
                '__all__ = ["PublicThing", "coerce"]\n',
            )
            archive.writestr(
                "different/api.py",
                'class Thing:\n    """An API absent from the docs inventory."""\n'
                "    def convert(self, value: int) -> int: return value\n"
                "def coerce(value: int) -> int: return value\n",
            )
            archive.writestr(
                "different/api.pyi",
                "from typing import overload\nclass Thing:\n"
                "    @overload\n    def convert(self, value: str) -> str: ...\n"
                "    @overload\n    def convert(self, value: bytes) -> bytes: ...\n"
                "def coerce(value: str) -> str: ...\n",
            )
            if version == "2.0":
                archive.writestr(
                    "different/new_api.py",
                    "def batch(values: list[int]) -> list[int]: return values\n",
                )
            archive.writestr("different/py.typed", "")
            archive.writestr("shared/portion.py", "def available() -> bool: return True\n")
            archive.writestr("README.md", "# Evidence fixture\nA small static capability.\n")
            archive.writestr("CHANGELOG.md", "# Changes\nBehavior now preserves whitespace.\n")
        record = {
            "info": {
                "name": "evidence-demo",
                "version": version,
                "project_urls": {"Documentation": "{{BASE_URL}}/docs/"},
            },
            "urls": [
                {
                    "filename": filename,
                    "packagetype": "bdist_wheel",
                    "url": "{{BASE_URL}}/static/" + filename,
                    "digests": {"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
                    "requires_python": ">=3.10",
                    "yanked": False,
                }
            ],
        }
        (metadata_dir / f"{version}.json").write_text(json.dumps(record))
    (metadata_dir / "index.json").write_text(json.dumps({"versions": ["1.0", "2.0"]}))


def config_for(path, base):
    path.write_text(
        '[policy]\nenabled_profiles = ["static"]\n'
        f'[producers.python]\npypi_url = "{base}/pypi"\nsimple_url = "{base}/simple"\n'
        f'worker_python = "{sys.executable}"\n'
    )


async def call(client, tool, **arguments):
    result = await daemon.read_complete_answer(
        client, (await client.call_tool(tool, arguments, raise_on_error=False)).structured_content
    )
    return await daemon.wait_for_answer(client, result) if tool == "resolve_library" else result


async def test_same_path_definitions_can_be_selected_without_merging_kinds(tmp_path: Path):
    root = tmp_path / "upstream"
    canary = tmp_path / "must-not-import"
    build_upstream(root, canary)
    wheel = root / "static/evidence_demo-1.0-py3-none-any.whl"
    with zipfile.ZipFile(wheel) as archive:
        entries = {name: archive.read(name) for name in archive.namelist()}
    entries["different/api.pyi"] = b"def Thing() -> int: ...\n"
    with zipfile.ZipFile(wheel, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, content in entries.items():
            archive.writestr(name, content)
    metadata = root / "pypi/evidence-demo/1.0.json"
    record = json.loads(metadata.read_text())
    record["urls"][0]["digests"]["sha256"] = hashlib.sha256(wheel.read_bytes()).hexdigest()
    metadata.write_text(json.dumps(record))
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                )
                selected = {
                    "context_id": resolved["context_id"],
                    "snapshot_id": resolved["snapshot_id"],
                    "symbol_path": "different.api.Thing",
                }
                ambiguous = await call(client, "inspect_symbol", **selected)
                assert ambiguous["status"] == "partial", ambiguous
                assert ambiguous["data"]["symbol"] is None
                candidates = ambiguous["data"]["candidates"]
                assert {c["kind"] for c in candidates} == {"class", "function"}, candidates
                assert len(candidates) == len({c["definition_id"] for c in candidates}) == 2
                assert {c["path"] for c in candidates} == {"different.api.Thing"}
                for candidate in candidates:
                    answer = await call(
                        client,
                        "inspect_symbol",
                        **selected,
                        definition_id=candidate["definition_id"],
                        selection={
                            "mode": "explicit",
                            "aspects": [
                                {"aspect": name}
                                for name in ("signature", "availability", "documentation", "source")
                            ],
                        },
                    )
                    symbol = answer["data"]["symbol"]
                    assert symbol["definition_id"] == candidate["definition_id"]
                    assert symbol["kind"] == candidate["kind"]
                    expected_origin = "source" if candidate["kind"] == "class" else "stub"
                    assert {o["origin"] for o in answer["data"]["observations"]} == {
                        expected_origin
                    }
                    assert answer["data"]["source"] is not None
                    assert not answer["data"]["candidates"]
                wrong = await call(
                    client,
                    "inspect_symbol",
                    **(selected | {"symbol_path": "api.Thing"}),
                    definition_id=candidates[0]["definition_id"],
                )
                assert wrong["status"] == "error", wrong
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                for candidate in candidates:
                    answer = await call(
                        client,
                        "inspect_symbol",
                        **selected,
                        definition_id=candidate["definition_id"],
                    )
                    assert answer["data"]["symbol"]["definition_id"] == candidate["definition_id"]
    assert not canary.exists()


async def test_python_distribution_mapping_conflicts_namespaces_and_offline(tmp_path: Path):
    root = tmp_path / "upstream"
    canary = tmp_path / "IMPORT_SIDE_EFFECT"
    build_upstream(root, canary)
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="Evidence.Demo",
                    version="1.0",
                    python_version="3.14",
                )
                assert resolved["status"] != "error", resolved
                assert resolved["data"]["release"]["version"] == "1.0"
                distribution = resolved["data"]["python"]
                assert distribution["import_roots"] == ["different", "shared"]
                assert distribution["metadata"]["requires-dist"] == [
                    'optional-dep; extra == "speed"'
                ]
                assert len(distribution["inventory"]) == 1
                assert distribution["inventory"][0]["role"] == "std:label"
                assert not canary.exists()
                context, snapshot = resolved["context_id"], resolved["snapshot_id"]
                inspected = await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    snapshot_id=snapshot,
                    symbol_path="different.api.Thing.convert",
                    selection={
                        "mode": "explicit",
                        "aspects": [
                            {"aspect": name}
                            for name in ("signature", "availability", "documentation", "source")
                        ],
                    },
                )
                assert inspected["status"] != "error", inspected
                observations = inspected["data"]["observations"]
                assert len({o["payload"]["signature"] for o in observations}) > 1
                assert observations[0]["payload"]["python"]["publicness"]["exported"] is None
                assert not observations[0]["payload"]["python"]["publicness"]["underscore"]
                assert {o["origin"] for o in observations} == {"source", "stub"}
                assert (
                    len(
                        next(o for o in observations if o["origin"] == "stub")["payload"]["python"][
                            "overloads"
                        ]
                    )
                    == 2
                )
                assert {o["source"]["locator"]["file"] for o in observations} == {
                    "different/api.py",
                    "different/api.pyi",
                }
                assert inspected["data"]["source"] is None
                assert (
                    next(
                        a for a in inspected["data"]["aspect_outcomes"] if a["aspect"] == "source"
                    )["state"]
                    == "unavailable"
                )
                alias = await call(
                    client,
                    "inspect_symbol",
                    context_id=context,
                    symbol_path="different.PublicThing",
                )
                assert alias["data"]["symbol"]["definition_path"] == "different.api.Thing"
                assert any(
                    o["subject"].get("symbol_id") == alias["data"]["symbol"]["symbol_id"]
                    and o["payload"]["python"]["publicness"]["exported"] is True
                    for o in alias["data"]["observations"]
                )
                conflict_alias = await call(
                    client, "inspect_symbol", context_id=context, symbol_path="different.coerce"
                )
                alias_symbol = conflict_alias["data"]["symbol"]
                assert alias_symbol["signature"] is None
                assert (
                    len({o["payload"]["signature"] for o in conflict_alias["data"]["observations"]})
                    > 1
                )
                assert {o["origin"] for o in conflict_alias["data"]["observations"]} == {
                    "source",
                    "stub",
                }
                overview = await call(client, "library_overview", context_id=context)
                assert any(n["path"] == "shared" for n in overview["data"]["namespaces"])
                thing = next(
                    child
                    for namespace in overview["data"]["namespaces"]
                    for child in namespace["children"]
                    if child["path"] == "different.api.Thing"
                )
                assert thing["doc_summary"] == "An API absent from the docs inventory."
                status = await call(client, "service_status", component="griffe")
                assert status["data"]["producers"][0]["available"] is True
                assert status["data"]["producers"][0]["version"] == "2.3.0"
                search = await call(client, "search_evidence", context_id=context, query="convert")
                assert search["data"]["hits"]
                assert search["evidence"][0]["artifact_id"]
                assert search["evidence"][0]["producer"] == "python-static"
                docs = await call(
                    client,
                    "search_evidence",
                    context_id=context,
                    query="deployment",
                    kinds=["docs"],
                )
                assert docs["data"]["hits"]
                assert all(e["source_version_match"] == "unknown" for e in docs["evidence"])
                latest = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    python_version="3.14",
                )
                assert latest["data"]["release"]["version"] == "2.0"
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            replay = await call(
                client,
                "resolve_library",
                ecosystem="python",
                name="evidence-demo",
                version="1.0",
                python_version="3.14",
                freshness="offline",
            )
            assert replay["snapshot_id"] == snapshot
            assert replay["data"]["answered_from_cache"]
            again = await call(
                client,
                "inspect_symbol",
                context_id=context,
                snapshot_id=snapshot,
                symbol_path="different.api.Thing.convert",
                selection={
                    "mode": "explicit",
                    "aspects": [
                        {"aspect": name}
                        for name in ("signature", "availability", "documentation", "source")
                    ],
                },
            )
            assert again["data"] == inspected["data"]
    assert not canary.exists()


@pytest.mark.parametrize("with_stubs", [True, False], ids=["with-stubs", "without-stubs"])
async def test_extension_only_package_returns_stubs_and_explicit_runtime_gaps(tmp_path, with_stubs):
    import shutil
    import subprocess
    import sysconfig

    compiler = shutil.which("cc")
    include = Path(sysconfig.get_paths()["include"])
    if compiler is None or not (include / "Python.h").is_file():
        pytest.skip("Native fixture requires a C compiler and selected Python development headers")
    build = tmp_path / "native-build"
    build.mkdir()
    source = build / "native.c"
    source.write_text(
        "#include <Python.h>\n"
        "static PyObject *ping(PyObject *self, PyObject *args) { Py_RETURN_NONE; }\n"
        'static PyMethodDef methods[] = {{"ping", ping, METH_NOARGS, NULL}, '
        "{NULL, NULL, 0, NULL}};\n"
        "static struct PyModuleDef module = {PyModuleDef_HEAD_INIT, "
        '"native_only", NULL, -1, methods};\n'
        "PyMODINIT_FUNC PyInit_native_only(void) { return PyModule_Create(&module); }\n"
    )
    binary = build / "native_only.so"
    result = subprocess.run(
        [compiler, "-shared", "-fPIC", f"-I{include}", str(source), "-o", str(binary)],
        capture_output=True,
        text=True,
        cwd=build,
        timeout=30,
    )
    assert result.returncode == 0, result.stderr
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "never-imported")
    filename = "evidence_demo-1.0-cp314-cp314-linux_x86_64.whl"
    wheel = root / "static" / filename
    with zipfile.ZipFile(wheel, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        archive.write(binary, "native_only.so")
        archive.writestr(
            "evidence_demo-1.0.dist-info/WHEEL",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: false\nTag: cp314-cp314-linux_x86_64\n",
        )
        if with_stubs:
            archive.writestr(
                "native_only.pyi", 'def ping() -> None:\n    """Native ping declaration."""\n'
            )
        archive.writestr(
            "evidence_demo-1.0.dist-info/METADATA",
            "Metadata-Version: 2.4\nName: evidence-demo\nVersion: 1.0\n",
        )
        archive.writestr("README.md", "# Native fixture\nStatic stubs describe ping.\n")
    record = json.loads((root / "pypi/evidence-demo/1.0.json").read_text())
    record["urls"][0].update(
        filename=filename,
        url="{{BASE_URL}}/static/" + filename,
        digests={"sha256": hashlib.sha256(wheel.read_bytes()).hexdigest()},
    )
    (root / "pypi/evidence-demo/1.0.json").write_text(json.dumps(record))
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                    target="linux_x86_64",
                )
                assert resolved["status"] == "partial", resolved
                assert "runtime_api" in resolved["coverage"]["missing"]
                assert resolved["data"]["python"]["native_files"] == ["native_only.so"]
                assert all(f["origin"] == "stub" for f in resolved["data"]["python"]["files"])
                runs = resolved["data"]["producer_runs"]
                assert runs[-1]["outcome"] == "partial"
                assert runs[-1]["gaps"] == resolved["data"]["gaps"]
                contents = await client.read_resource(
                    f"library-evidence://snapshots/{resolved['snapshot_id']}/manifest"
                )
                manifest = json.loads(contents[0].text)
                assert manifest["data"]["producer_runs"] == runs
                assert manifest["data"]["manifest"]["snapshot_id"] == resolved["snapshot_id"]
                if not with_stubs:
                    assert "public_api" in resolved["coverage"]["missing"]
                    docs = await call(
                        client,
                        "search_evidence",
                        context_id=resolved["context_id"],
                        query="Native fixture",
                        kinds=["docs"],
                    )
                    assert docs["data"]["hits"]
                    return
                inspected = await call(
                    client,
                    "inspect_symbol",
                    context_id=resolved["context_id"],
                    symbol_path="native_only.ping",
                )
                assert inspected["status"] == "partial"
                runtime_scope = await call(
                    client,
                    "inspect_symbol",
                    context_id=resolved["context_id"],
                    symbol_path="native_only.ping",
                    selection={"mode": "explicit", "aspects": [{"aspect": "runtime"}]},
                )
                assert "runtime_api" in runtime_scope["coverage"]["missing"]
                assert not runtime_scope["data"]["execution_observations"]
                assert "ping" in inspected["data"]["symbol"]["signature"]
                assert any(
                    "Native ping declaration" in item["fragment"]["text"]
                    for item in inspected["data"]["fragments"]
                )
                assert inspected["data"]["observations"][0]["origin"] == "stub"
                assert not any(
                    e["evidence_class"] == "runtime_observed" for e in inspected["evidence"]
                )


async def test_added_python_function_is_discovered_independently_of_break_checks(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                sides = []
                for version in ["1.0", "2.0"]:
                    sides.append(
                        await call(
                            client,
                            "resolve_library",
                            ecosystem="python",
                            name="evidence-demo",
                            version=version,
                            python_version="3.14",
                        )
                    )
                result = await call(
                    client,
                    "compare_releases",
                    before_context_id=sides[0]["context_id"],
                    after_context_id=sides[1]["context_id"],
                    scopes=["api"],
                    max_items=1,
                )
                assert result["status"] == "partial", result
                assert not result["data"][
                    "api_complete"
                ]  # external alias targets remain unresolved
                assert not result["data"][
                    "comparable"
                ]  # unknown target/dependency graph remains unknown
                changes = result["data"]["changes"]
                while result["data"]["page"]["next_cursor"]:
                    result = await call(
                        client,
                        "compare_releases",
                        before_context_id=sides[0]["context_id"],
                        after_context_id=sides[1]["context_id"],
                        scopes=["api"],
                        max_items=1,
                        cursor=result["data"]["page"]["next_cursor"],
                    )
                    changes.extend(result["data"]["changes"])
                addition = next(c for c in changes if c["subject"] == "different.new_api.batch")
                assert addition["kind"] == "added"
                assert addition["before"] is None
                assert "observed only on the after side" in addition["interpretation"]
                assert "confirm coverage" in addition["interpretation"]
                assert addition["after"][0]["source"]["artifact_id"]
                assert all(c["kind"] == "added" for c in changes)
                invalid = await call(
                    client,
                    "compare_releases",
                    before_context_id=sides[0]["context_id"],
                    after_context_id=sides[1]["context_id"],
                    ecosystem="python",
                )
                assert invalid["status"] == "error"


async def test_mutable_docs_revalidate_without_changing_release_and_keep_old_snapshot(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                args = dict(
                    ecosystem="python", name="evidence-demo", version="1.0", python_version="3.14"
                )
                first = await call(client, "resolve_library", **args)
                same = await call(client, "resolve_library", **args, freshness="revalidate")
                assert first["snapshot_id"] == same["snapshot_id"]
                assert any(
                    status == 304 and validator for _, status, validator in upstream.response_log
                )
                path = root / "docs/guide.html"
                path.write_text(path.read_text().replace("deployment", "production"))
                changed = await call(client, "resolve_library", **args, freshness="revalidate")
                assert changed["context_id"] == first["context_id"]
                assert changed["snapshot_id"] != first["snapshot_id"]
                assert (
                    changed["data"]["release"]["release_id"]
                    == first["data"]["release"]["release_id"]
                )
                old = await call(
                    client,
                    "search_evidence",
                    context_id=first["context_id"],
                    snapshot_id=first["snapshot_id"],
                    query="deployment",
                    kinds=["docs"],
                )
                new = await call(
                    client,
                    "search_evidence",
                    context_id=first["context_id"],
                    query="production",
                    kinds=["docs"],
                )
                assert old["data"]["hits"] and new["data"]["hits"]
                assert old["evidence"][0]["artifact_id"] != new["evidence"][0]["artifact_id"]
                delta = await call(
                    client,
                    "compare_releases",
                    before_context_id=first["context_id"],
                    after_context_id=changed["context_id"],
                    before_snapshot_id=first["snapshot_id"],
                    after_snapshot_id=changed["snapshot_id"],
                    scopes=["docs"],
                )
                assert delta["data"]["same_release"]
                assert any("production" in str(c["after"]) for c in delta["data"]["changes"])
        # A new daemon process reuses verified cache bytes for HTTP 304 responses.
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                replay = await call(client, "resolve_library", **args, freshness="revalidate")
                assert replay["snapshot_id"] == changed["snapshot_id"]


async def test_negative_cache_is_durable_bounded_and_revalidation_bypasses_it(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        args = dict(ecosystem="python", name="evidence-demo", version="9.9", python_version="3.14")
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                first = await call(client, "resolve_library", **args)
                assert first["status"] == "error"
        initial = upstream.request_log.count("/pypi/evidence-demo/9.9/json")
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                second = await call(client, "resolve_library", **args)
                assert second["status"] == "error"
                assert upstream.request_log.count("/pypi/evidence-demo/9.9/json") == initial
                await call(client, "resolve_library", **args, freshness="revalidate")
                assert upstream.request_log.count("/pypi/evidence-demo/9.9/json") == initial + 1
        config.write_text(config.read_text() + "\n[freshness]\nnegative_cache_ttl_seconds = 0\n")
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                await call(client, "resolve_library", **args)
        assert upstream.request_log.count("/pypi/evidence-demo/9.9/json") == initial + 2


async def test_unavailable_latest_does_not_promote_cached_release_to_current_latest(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                first = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    python_version="3.14",
                )
                assert first["freshness"]["latest_verified"]
    with daemon.running(env):
        async with Client(daemon.transport(env)) as client:
            latest = await call(
                client,
                "resolve_library",
                ecosystem="python",
                name="evidence-demo",
                python_version="3.14",
            )
            assert latest["status"] == "error"
            assert not latest["freshness"]["latest_verified"]
            cached = await call(
                client,
                "resolve_library",
                ecosystem="python",
                name="evidence-demo",
                version="2.0",
                python_version="3.14",
                mode="upstream",
                freshness="offline",
            )
            assert cached["snapshot_id"] == first["snapshot_id"]
            assert not cached["freshness"]["latest_verified"]


async def test_namespace_search_and_cursor_scope_remain_deterministic(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                )
                args = dict(
                    context_id=resolved["context_id"],
                    query="Thing",
                    area="different",
                    kinds=["api"],
                    max_items=1,
                )
                first = await call(client, "search_evidence", **args)
                again = await call(client, "search_evidence", **args)
                assert first["data"] == again["data"]
                assert first["data"]["page"] == again["data"]["page"]
                assert first["data"]["area"] == "different"
                assert all(h["path"].startswith("different.") for h in first["data"]["hits"])
                cursor = first["data"]["page"]["next_cursor"]
                assert cursor
                bad = await call(
                    client, "search_evidence", **{**args, "area": "shared"}, cursor=cursor
                )
                assert bad["error"]["code"] == "INVALID_CURSOR"
                empty = await call(
                    client,
                    "search_evidence",
                    context_id=resolved["context_id"],
                    query="Thing",
                    area="shared",
                )
                assert empty["data"]["hits"] == []
                assert any(
                    "does not establish that a capability is absent" in note
                    for note in empty["coverage"]["limitations"]
                )
                qualified = await call(
                    client,
                    "search_evidence",
                    context_id=resolved["context_id"],
                    query="api.Thing",
                    kinds=["api"],
                )
                assert any(
                    f["name"] == "path_suffix" for f in qualified["data"]["hits"][0]["factors"]
                )


async def test_mcp_budget_overflow_artifact_recovers_exact_escaped_query(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                )
                query = 'missing_"\\漢字🙂' * 1000
                args = dict(context_id=resolved["context_id"], query=query, max_bytes=4096)
                bounded = (await client.call_tool("search_evidence", args)).structured_content
                assert bounded["status"] in {"ok", "partial"}, bounded
                assert bounded["delivery"]["mode"] == "artifact"
                assert (
                    len(json.dumps(bounded, ensure_ascii=False, separators=(",", ":")).encode())
                    <= 4096
                )
                again = (await client.call_tool("search_evidence", args)).structured_content
                assert again["delivery"]["artifact_id"] == bounded["delivery"]["artifact_id"]
                complete = await daemon.read_complete_answer(client, bounded)
                assert complete["data"]["query"] == query
                assert complete["data"]["hits"] == []


async def test_search_complexity_is_bounded_before_query_planning(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                result = (
                    await client.call_tool(
                        "search_evidence",
                        {
                            "context_id": "irrelevant",
                            "query": " ".join(f"term{i}" for i in range(1000)),
                        },
                        raise_on_error=False,
                    )
                ).structured_content
                assert result["error"]["code"] == "UNSUPPORTED_FORMAT"
                assert "64 terms" in result["error"]["message"]
                status = (await client.call_tool("service_status", {})).structured_content
                assert status["status"] != "error"


async def test_failed_worker_preserves_acquired_source_and_documentation(tmp_path):
    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        config.write_text(
            config.read_text().replace(str(sys.executable), str(tmp_path / "absent-worker"))
        )
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                )
                assert resolved["status"] == "partial", resolved
                assert "public_api" in resolved["coverage"]["missing"]
                assert "distribution_source" in resolved["coverage"]["indexed"]
                assert any(
                    "worker unavailable" in gap["detail"] for gap in resolved["data"]["gaps"]
                )
                assert resolved["data"]["producer_runs"][-1]["outcome"] == "partial"
                assert resolved["data"]["producer_runs"][-1]["finished_at"]
                docs = await call(
                    client,
                    "search_evidence",
                    context_id=resolved["context_id"],
                    query="capability",
                    kinds=["docs"],
                )
                assert docs["data"]["hits"], docs
                artifact = docs["evidence"][0]["artifact_id"]
                raw = await call(client, "read_artifact", artifact_id=artifact)
                assert "capability" in raw["data"]["content"]
                replay = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                    freshness="offline",
                )
                assert replay["status"] == "partial"
                assert replay["snapshot_id"] == resolved["snapshot_id"]


async def test_complete_stdio_frames_have_one_envelope_and_measured_allowance(tmp_path):
    import asyncio


    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "side-effect")
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                resolved = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                    python_version="3.14",
                )
            # Actual protocol bytes, including text, structured content, metadata and framing.
            process = await asyncio.create_subprocess_exec(
                sys.executable,
                "-m",
                "enrichment_mcp",
                env=env,
                stdin=asyncio.subprocess.PIPE,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.DEVNULL,
            )
            stdin, stdout = process.stdin, process.stdout
            assert stdin is not None and stdout is not None

            async def exchange(request):
                stdin.write(json.dumps(request).encode() + b"\n")
                await stdin.drain()
                while True:
                    frame = await asyncio.wait_for(stdout.readline(), timeout=15)
                    assert frame, "adapter exited before answering"
                    response = json.loads(frame)
                    if response.get("id") == request["id"]:
                        return frame, response

            try:
                _, initialized = await exchange(
                    {
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "initialize",
                        "params": {
                            "protocolVersion": "2025-11-25",
                            "capabilities": {},
                            "clientInfo": {"name": "raw-budget-test", "version": "1"},
                        },
                    }
                )
                assert "result" in initialized, initialized
                stdin.write(b'{"jsonrpc":"2.0","method":"notifications/initialized"}\n')
                await stdin.drain()
                for index, budget in enumerate((1024, 2048, 4096), start=2):
                    frame, response = await exchange(
                        {
                            "jsonrpc": "2.0",
                            "id": index,
                            "method": "tools/call",
                            "params": {
                                "name": "search_evidence",
                                "arguments": {
                                    "context_id": resolved["context_id"],
                                    "query": 'missing_"\\漢字🙂' * 1000,
                                    "max_bytes": budget,
                                },
                            },
                        }
                    )
                    result = response["result"]
                    structured = result["structuredContent"]
                    size = len(
                        json.dumps(structured, ensure_ascii=False, separators=(",", ":")).encode()
                    )
                    assert size <= budget, (budget, size)
                    assert len(frame) <= budget, (budget, len(frame))
                    assert 1 <= len(result["content"]) <= 2
                    assert all(item["type"] == "resource_link" for item in result["content"][1:])
                    assert result["content"][0]["type"] == "text"
                    if structured["status"] == "error":
                        preview = json.loads(result["content"][0]["text"])
                        assert preview["format"] == "research-error-preview/1"
                        assert preview["code"] == structured["error"]["code"]
                        assert preview["next_action"] == structured["error"]["next_action"]
                        assert budget < 4096
                        assert structured["error"]["code"] == "BUDGET_EXCEEDED"
                        diagnostic = structured["error"]["diagnostic"]
                        assert diagnostic["allowed"] == budget
                        assert diagnostic["observed"] > budget
                        assert not structured["error"]["retryable"]
                        assert result["isError"]
                    else:
                        assert (
                            len(
                                json.dumps(
                                    result["content"][0]["text"], ensure_ascii=False
                                ).encode()
                            )
                            <= 160
                        )
                        assert structured["status"] in {"ok", "partial"}
                        assert structured["delivery"]["artifact_id"]
                        assert not result.get("isError", False)
                    assert "schema_version" not in result["content"][0]["text"]
            finally:
                stdin.close()
                try:
                    await asyncio.wait_for(process.wait(), timeout=10)
                except TimeoutError:
                    process.kill()
                    await process.wait()


async def test_binary_artifact_pages_reconstruct_exact_bytes_under_encoded_cap(tmp_path):
    import base64

    root = tmp_path / "upstream"
    build_upstream(root, tmp_path / "must-not-import")
    wheel = root / "static/evidence_demo-1.0-py3-none-any.whl"
    with zipfile.ZipFile(wheel, "a") as archive:
        archive.writestr(
            "evidence_demo-1.0.dist-info/binary-fixture.bin",
            bytes(range(256)) * 128,
            compress_type=zipfile.ZIP_STORED,
        )
    expected = wheel.read_bytes()
    digest = hashlib.sha256(expected).hexdigest()
    metadata_path = root / "pypi/evidence-demo/1.0.json"
    metadata = json.loads(metadata_path.read_text())
    metadata["urls"][0]["digests"]["sha256"] = digest
    metadata_path.write_text(json.dumps(metadata))
    config = tmp_path / "service.toml"
    with serve(root) as upstream:
        config_for(config, upstream.base_url)
        env = daemon.daemon_env(tmp_path / "state", config)
        with daemon.running(env):
            async with Client(daemon.transport(env)) as client:
                result = await call(
                    client,
                    "resolve_library",
                    ecosystem="python",
                    name="evidence-demo",
                    version="1.0",
                )
                assert result["status"] in {"ok", "partial"}, result
                artifact = next(a for a in result["data"]["artifacts"] if a["sha256"] == digest)
                cursor = None
                chunks = []
                for _ in range(100):
                    page = await call(
                        client,
                        "read_artifact",
                        artifact_id=artifact["artifact_id"],
                        cursor=cursor,
                        max_bytes=4096,
                    )
                    assert page["status"] == "ok", page
                    assert page["data"]["encoding"] == "base64"
                    size = len(json.dumps(page, ensure_ascii=False, separators=(",", ":")).encode())
                    assert size <= 4096
                    chunk = base64.b64decode(page["data"]["content"], validate=True)
                    assert chunk
                    assert hashlib.sha256(chunk).hexdigest() == page["data"]["content_digest"]
                    chunks.append(chunk)
                    following = page["data"]["page"]["next_cursor"]
                    if following is None:
                        break
                    assert following != cursor
                    assert size >= 4080
                    cursor = following
                else:
                    raise AssertionError("binary traversal did not finish")
                assert len(chunks) > 2
                assert b"".join(chunks) == expected

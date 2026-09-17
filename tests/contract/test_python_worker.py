"""The actual static worker preserves evidence without executing import side effects."""

import io
import os
import subprocess
import sys

import pyarrow as pa

from enrichment_mcp._generated.worker_schema import Observation, WorkerRequest
from enrichment_worker.__main__ import CONTRACT, SCHEMA, extract


def facts(job):
    output = io.BytesIO()
    extract(job, output)
    table = pa.ipc.open_stream(output.getvalue()).read_all()
    assert table.schema.equals(SCHEMA, check_metadata=True)
    rows = table.to_pylist()
    complete = {r["file"] for r in rows if r["fact"] == "file_complete"}
    observations = [
        Observation.model_validate(r["observation"])
        for r in rows
        if r["fact"] == "observation" and r["file"] in complete
    ]
    gaps = [r for r in rows if r["fact"] == "file_gap"]
    return observations, gaps, complete


def request(root, files):
    return WorkerRequest.model_validate(
        {
            "schema_version": CONTRACT["protocol"],
            "root": str(root),
            "max_observations": 10000,
            "max_memory_bytes": 1024 * 1024 * 1024,
            "max_cpu_seconds": 30,
            "files": [
                {"file": file, "module": module, "origin": origin} for file, module, origin in files
            ],
        }
    )


def test_source_stub_overloads_exports_and_aliases_remain_separate(tmp_path):
    (tmp_path / "different").mkdir()
    (tmp_path / "different/__init__.py").write_text(
        'from .api import Thing as PublicThing\n__all__ = ["PublicThing"]\n'
    )
    (tmp_path / "different/api.py").write_text(
        '"""Runtime source declaration."""\nclass Thing:\n'
        "    def convert(self, value: int, /, *, strict: bool = True) -> int: ...\n"
    )
    (tmp_path / "different/api.pyi").write_text(
        "from typing import overload\nclass Thing:\n"
        "    @overload\n    def convert(self, value: str, /) -> str: ...\n"
        "    @overload\n    def convert(self, value: bytes, /) -> bytes: ...\n"
    )
    observations, gaps, _processed = facts(
        request(
            tmp_path,
            [
                ("different/__init__.py", "different", "source"),
                ("different/api.py", "different.api", "source"),
                ("different/api.pyi", "different.api", "stub"),
            ],
        )
    )
    assert not gaps
    alias = next(o for o in observations if o.path == "different.PublicThing")
    assert alias.alias_target == "different.api.Thing"
    assert alias.publicness.exported is True
    methods = [o for o in observations if o.path == "different.api.Thing.convert"]
    assert len(methods) == 3
    assert {o.origin.value for o in methods} == {"source", "stub"}
    stubs = sorted(
        (o for o in methods if o.origin.value == "stub"), key=lambda o: o.overload_ordinal
    )
    assert [o.overload_ordinal for o in stubs] == [0, 1]
    assert all(not o.overloads for o in stubs)
    assert "-> str" in stubs[0].signature
    assert "-> bytes" in stubs[1].signature
    assert stubs[0].callable.returns == "str"
    assert stubs[1].callable.returns == "bytes"


def test_separate_worker_does_not_execute_target_or_use_its_cwd(tmp_path):
    root = tmp_path / "archive"
    root.mkdir()
    canary = tmp_path / "imported"
    (root / "fixture.py").write_text(
        f'from pathlib import Path\nPath({str(canary)!r}).write_text("executed")\n'
        "def capability(value: int) -> int:\n    return value + 1\n"
    )
    cwd = tmp_path / "worker-cwd"
    cwd.mkdir()
    job = request(root, [("fixture.py", "fixture", "source")])
    env = {"PATH": os.environ["PATH"], "HOME": str(cwd), "TMPDIR": str(cwd)}
    result = subprocess.run(
        [sys.executable, "-I", "-B", "-m", "enrichment_worker"],
        input=job.model_dump_json().encode(),
        capture_output=True,
        cwd=cwd,
        env=env,
        timeout=20,
    )
    assert result.returncode == 0, result.stderr
    rows = pa.ipc.open_stream(result.stdout).read_all().to_pylist()
    response = next(r for r in rows if r["fact"] == "producer")
    observations = [
        Observation.model_validate(r["observation"]) for r in rows if r["fact"] == "observation"
    ]
    assert not canary.exists()
    assert not list(cwd.iterdir())
    assert response["griffe_version"] == "2.3.0"
    assert any(o.path == "fixture.capability" for o in observations)
    assert response["worker_python"]


def test_worker_rejects_path_escape_and_names_parse_failure(tmp_path):
    (tmp_path / "broken.py").write_text("def incomplete(:\n")
    observations, gaps, processed = facts(
        request(
            tmp_path,
            [
                ("../outside.py", "outside", "source"),
                ("broken.py", "broken", "source"),
            ],
        )
    )
    assert len(gaps) == 2
    assert not observations
    assert not processed
    assert "relative" in gaps[0]["detail"]
    assert "SyntaxError" in gaps[1]["detail"]


def test_long_documentation_is_retained_and_oversized_declarations_are_explicit(tmp_path):
    docs = ("λ documentation beyond the old truncation limit. " * 1000).strip()
    (tmp_path / "long.py").write_text(f'def capability():\n    """{docs}"""\n')
    job = request(tmp_path, [("long.py", "long", "source")])
    observations, gaps, processed = facts(job)
    assert not gaps
    assert next(o.docs for o in observations if o.path == "long.capability") == docs
    (tmp_path / "long.py").write_text(
        'def capability():\n    """' + "x" * (CONTRACT["batch_bytes"] + 1) + '"""\n'
    )
    observations, gaps, processed = facts(job)
    assert not processed
    assert not observations
    assert "declaration exceeds Arrow batch byte budget" in gaps[0]["detail"]


def test_callable_fields_preserve_parameter_kinds_and_explicit_none_default(tmp_path):
    root = tmp_path / "archive"
    root.mkdir()
    (root / "shapes.py").write_text(
        "async def shaped(a: int, /, b=None, *args: str, flag: bool=True, **kwargs: bytes) -> str:\n"
        "    return str(a)\n"
    )
    observations, gaps, _ = facts(request(root, [("shapes.py", "shapes", "source")]))
    assert not gaps
    function = next(o for o in observations if o.path == "shapes.shaped")
    callable_value = function.callable
    assert callable_value is not None
    assert [p.ordinal for p in callable_value.parameters] == [0, 1, 2, 3, 4]
    assert [p.kind.value for p in callable_value.parameters] == [
        "positional-only",
        "positional or keyword",
        "variadic positional",
        "keyword-only",
        "variadic keyword",
    ]
    assert [p.reported_default for p in callable_value.parameters] == [
        None,
        "None",
        "()",
        "True",
        "{}",
    ]
    # The worker preserves Griffe's implicit variadic values; native normalization derives origin.
    assert all(p.default_origin is None for p in callable_value.parameters)
    assert [p.annotation for p in callable_value.parameters] == [
        "int",
        None,
        "str",
        "bool",
        "bytes",
    ]
    assert callable_value.returns == "str"
    assert "async" in callable_value.labels

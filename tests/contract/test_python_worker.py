"""The actual static worker preserves evidence without executing import side effects."""

import os
import subprocess
import sys

from enrichment_mcp._generated.worker_schema import WorkerRequest, WorkerResponse
from enrichment_worker.__main__ import extract


def request(root, files):
    return WorkerRequest.model_validate(
        {
            "schema_version": "1.0",
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
    result = extract(
        request(
            tmp_path,
            [
                ("different/__init__.py", "different", "source"),
                ("different/api.py", "different.api", "source"),
                ("different/api.pyi", "different.api", "stub"),
            ],
        )
    )
    assert not result.gaps
    alias = next(o for o in result.observations if o.path == "different.PublicThing")
    assert alias.alias_target == "different.api.Thing"
    assert alias.publicness.exported is True
    methods = [o for o in result.observations if o.path == "different.api.Thing.convert"]
    assert len(methods) == 2
    assert {o.origin.value for o in methods} == {"source", "stub"}
    stub = next(o for o in methods if o.origin.value == "stub")
    assert len(stub.overloads) == 2
    assert "-> str" in stub.overloads[0]
    assert "-> bytes" in stub.overloads[1]


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
        input=job.model_dump_json(),
        text=True,
        capture_output=True,
        cwd=cwd,
        env=env,
        timeout=20,
    )
    assert result.returncode == 0, result.stderr
    response = WorkerResponse.model_validate_json(result.stdout)
    assert not canary.exists()
    assert not list(cwd.iterdir())
    assert response.griffe_version == "2.3.0"
    assert any(o.path == "fixture.capability" for o in response.observations)
    assert response.worker_python


def test_worker_rejects_path_escape_and_names_parse_failure(tmp_path):
    (tmp_path / "broken.py").write_text("def incomplete(:\n")
    result = extract(
        request(
            tmp_path,
            [
                ("../outside.py", "outside", "source"),
                ("broken.py", "broken", "source"),
            ],
        )
    )
    assert len(result.gaps) == 2
    assert not result.observations
    assert not result.processed_files
    assert "relative" in result.gaps[0].reason
    assert "SyntaxError" in result.gaps[1].reason


def test_long_documentation_is_retained_and_oversized_declarations_are_explicit(tmp_path):
    docs = ("λ documentation beyond the old truncation limit. " * 1000).strip()
    (tmp_path / "long.py").write_text(f'def capability():\n    """{docs}"""\n')
    job = request(tmp_path, [("long.py", "long", "source")])
    result = extract(job)
    assert not result.gaps
    assert next(o.docs for o in result.observations if o.path == "long.capability") == docs
    (tmp_path / "long.py").write_text('def capability():\n    """' + "x" * 1_100_000 + '"""\n')
    result = extract(job)
    assert not result.processed_files
    assert not result.observations
    assert "declaration exceeds 1 MiB" in result.gaps[0].reason

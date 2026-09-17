"""One bounded static Griffe job; target code is never imported or executed."""

from __future__ import annotations

import importlib.metadata
import io
import json
import platform
import resource
import sys
import tokenize
from collections.abc import Buffer, Iterator
from importlib.resources import files
from pathlib import Path
from typing import BinaryIO

import griffe
import pyarrow as pa

from enrichment_mcp._generated.worker_schema import (
    Observation,
    Publicness,
    PythonBase,
    PythonCallable,
    PythonOverload,
    PythonParameter,
    PythonParameterKind,
    WorkerFile,
    WorkerRequest,
)

MAX_INPUT = 4 * 1024 * 1024
with files("enrichment_worker").joinpath("_schemas/worker.arrow").open("rb") as _schema_file:
    SCHEMA = pa.ipc.open_stream(_schema_file).schema
CONTRACT = json.loads(SCHEMA.metadata[b"enrichment.worker"])


def _publicness(obj: griffe.Object | griffe.Alias, parent: griffe.Object | None) -> Publicness:
    exports = getattr(parent, "exports", None)
    literals = sorted(e for e in (exports or []) if isinstance(e, str))
    unresolved = sorted(str(e) for e in (exports or []) if not isinstance(e, str))
    alias = isinstance(obj, griffe.Alias)
    return Publicness(
        exported=(obj.name in literals) if exports is not None and not unresolved else None,
        underscore=obj.name.startswith("_"),
        reexport=alias,
        docstring=False if alias else obj.docstring is not None,
        declared_exports=literals,
        unresolved_exports=unresolved,
    )


def _callable(function: griffe.Function) -> PythonCallable:
    return PythonCallable(
        parameters=[
            PythonParameter(
                ordinal=ordinal,
                name=parameter.name,
                kind=PythonParameterKind(parameter.kind.value)
                if parameter.kind is not None
                else None,
                annotation=str(parameter.annotation) if parameter.annotation is not None else None,
                reported_default=str(parameter.default) if parameter.default is not None else None,
                default_origin=None,
            )
            for ordinal, parameter in enumerate(function.parameters)
        ],
        returns=str(function.returns) if function.returns is not None else None,
        labels=sorted(function.labels),
    )


def _observation(
    obj: griffe.Object | griffe.Alias,
    file: WorkerFile,
    parent: griffe.Object | None,
    overload_ordinal: int | None = None,
) -> Observation:
    alias = isinstance(obj, griffe.Alias)
    signature = None
    bases: list[PythonBase] = []
    overloads: list[PythonOverload] = []
    callable_value = None
    if isinstance(obj, griffe.Function):
        signature = obj.signature()
        callable_value = _callable(obj)
        if overload_ordinal is None:
            overloads = [
                PythonOverload(
                    ordinal=ordinal, signature=function.signature(), callable=_callable(function)
                )
                for ordinal, function in enumerate(obj.overloads or [])
            ]
    elif isinstance(obj, griffe.Class):
        bases = [
            PythonBase(ordinal=ordinal, rendering=str(base))
            for ordinal, base in enumerate(obj.bases)
        ]
        signature = f"class {obj.name}({', '.join(base.rendering for base in bases)})"
    elif isinstance(obj, griffe.Attribute):
        signature = f"{obj.name}: {obj.annotation}" if obj.annotation is not None else obj.name
    return Observation(
        path=obj.path,
        kind="alias" if alias else obj.kind.value,
        origin=file.origin,
        file=file.file,
        line=obj.alias_lineno if alias else obj.lineno,
        signature=signature,
        callable=callable_value,
        overload_ordinal=overload_ordinal,
        overloads=overloads,
        docs=None if alias or obj.docstring is None else obj.docstring.value,
        alias_target=obj.target_path if alias else None,
        bases=bases,
        publicness=_publicness(obj, parent),
    )


def _walk(
    obj: griffe.Object | griffe.Alias, file: WorkerFile, parent: griffe.Object | None = None
) -> Iterator[Observation]:
    yield _observation(obj, file, parent)
    if isinstance(obj, griffe.Alias):
        return
    for name in sorted(obj.members):
        yield from _walk(obj.members[name], file, obj)
    if isinstance(obj, griffe.Module | griffe.Class):
        for name, overloads in sorted(obj.overloads.items()):
            if name not in obj.members:
                for ordinal, function in enumerate(overloads):
                    yield _observation(function, file, obj, ordinal)


class BoundedSink(io.RawIOBase):
    """Transport byte ceiling, applied before every native IPC write."""

    def __init__(self, output: BinaryIO) -> None:
        self.output = output
        self.position = 0

    def writable(self) -> bool:
        return True

    def tell(self) -> int:
        return self.position

    def write(self, data: Buffer, /) -> int:
        if self.position + memoryview(data).nbytes > CONTRACT["max_bytes"]:
            raise ValueError("worker Arrow output exceeds byte budget")
        written = self.output.write(data)
        self.position += written
        return written


def extract(request: WorkerRequest, output: BinaryIO) -> None:
    """Stream mechanical facts and per-file receipts under the Rust-owned schema."""
    if request.schema_version != CONTRACT["protocol"]:
        raise ValueError("unsupported worker protocol")
    max_observations = int(request.max_observations)
    if not 1 <= max_observations <= CONTRACT["max_observations"]:
        raise ValueError("unsupported worker observation bound")
    if len(request.files) > CONTRACT["max_files"]:
        raise ValueError("worker file inventory exceeds bound")
    if importlib.metadata.version("pyarrow") != CONTRACT["encoder"]:
        raise ValueError("worker Arrow encoder pin mismatch")
    if importlib.metadata.version("griffe") != CONTRACT["griffe"]:
        raise ValueError("worker Griffe pin mismatch")
    root = Path(request.root)
    if not root.is_absolute() or not root.is_dir():
        raise ValueError("worker needs an explicit absolute input root")
    root = root.resolve(strict=True)
    loader = griffe.GriffeLoader(
        search_paths=[root], allow_inspection=False, force_inspection=False
    )
    options = pa.ipc.IpcWriteOptions(
        metadata_version=pa.ipc.MetadataVersion.V5, use_legacy_format=False, compression=None
    )
    with pa.ipc.new_stream(BoundedSink(output), SCHEMA, options=options) as writer:
        pending: list[pa.RecordBatch] = []
        pending_bytes = 0

        def flush() -> None:
            nonlocal pending_bytes
            if pending:
                writer.write_table(pa.Table.from_batches(pending).combine_chunks())
                pending.clear()
                pending_bytes = 0

        def emit(**values: object) -> None:
            nonlocal pending_bytes
            batch = pa.RecordBatch.from_pylist([values], schema=SCHEMA)
            batch.validate(full=True)
            if batch.nbytes > CONTRACT["batch_bytes"]:
                raise ValueError("declaration exceeds Arrow batch byte budget")
            if (
                len(pending) >= CONTRACT["batch_rows"]
                or pending_bytes + batch.nbytes > CONTRACT["batch_bytes"]
            ):
                flush()
            pending.append(batch)
            pending_bytes += batch.nbytes

        emit(
            fact="producer",
            griffe_version=importlib.metadata.version("griffe"),
            worker_python=platform.python_version(),
            encoder_version=pa.__version__,
        )
        total = 0
        for file in request.files:
            count = 0
            try:
                relative = Path(file.file)
                if relative.is_absolute() or ".." in relative.parts:
                    raise ValueError("file is not an archive-relative path")
                path = root / relative
                if path.is_symlink() or not path.resolve(strict=True).is_relative_to(root):
                    raise ValueError("file escaped the sanitized input root")
                if path.stat().st_size > 8 * 1024 * 1024:
                    raise ValueError("source file exceeds static worker byte budget")
                with tokenize.open(path) as stream:
                    code = stream.read()
                module = griffe.visit(
                    file.module,
                    path,
                    code,
                    extensions=loader.extensions,
                    lines_collection=loader.lines_collection,
                    modules_collection=loader.modules_collection,
                )
                for fact in _walk(module, file):
                    if total >= max_observations:
                        raise ValueError("observation budget exhausted")
                    emit(
                        fact="observation",
                        file=file.file,
                        ordinal=count,
                        observation=fact.model_dump(mode="json"),
                    )
                    total += 1
                    count += 1
                emit(fact="file_complete", file=file.file, count=count)
            except (OSError, ValueError, SyntaxError, UnicodeError, RecursionError) as error:
                # Already-streamed declarations remain raw facts. Native admission joins
                # only completed file receipts, so partial files never publish declarations.
                detail = f"{type(error).__name__}: {error}".replace(str(root), "<input>")[:4096]
                emit(fact="file_gap", file=file.file, count=count, detail=detail)
        emit(fact="complete", count=total)
        flush()


def main() -> int:
    """Read one request and emit one bounded Arrow IPC V5 fact stream."""
    try:
        raw = sys.stdin.buffer.read(MAX_INPUT + 1)
        if len(raw) > MAX_INPUT:
            raise ValueError("worker input exceeds byte budget")
        request = WorkerRequest.model_validate_json(raw)
        max_memory_bytes = int(request.max_memory_bytes)
        max_cpu_seconds = int(request.max_cpu_seconds)
        if max_memory_bytes <= 0 or max_cpu_seconds <= 0:
            raise ValueError("Rust worker allocation/deadline bounds are required")
        for kind, cap in (
            (resource.RLIMIT_AS, max_memory_bytes),
            (resource.RLIMIT_CPU, max_cpu_seconds),
            (resource.RLIMIT_CORE, 0),
        ):
            soft, hard = resource.getrlimit(kind)
            selected = (
                cap if soft == resource.RLIM_INFINITY else min(soft, cap),
                cap if hard == resource.RLIM_INFINITY else min(hard, cap),
            )
            resource.setrlimit(kind, selected)
            if resource.getrlimit(kind) != selected:
                raise ValueError("worker process limit installation failed")
        extract(request, sys.stdout.buffer)
        sys.stdout.buffer.flush()
        return 0
    except (OSError, ValueError, MemoryError) as error:
        sys.stderr.write(f"static worker failed: {error}\n")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())

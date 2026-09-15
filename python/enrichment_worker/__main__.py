"""One bounded static Griffe job; target code is never imported or executed."""

from __future__ import annotations

import importlib.metadata
import platform
import resource
import sys
import tokenize
from collections.abc import Iterator
from pathlib import Path

import griffe

from enrichment_mcp._generated.worker_schema import (
    Observation,
    Publicness,
    WorkerFile,
    WorkerGap,
    WorkerRequest,
    WorkerResponse,
)

MAX_INPUT = 4 * 1024 * 1024
MAX_OUTPUT = 16 * 1024 * 1024


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


def _observation(
    obj: griffe.Object | griffe.Alias,
    file: WorkerFile,
    parent: griffe.Object | None,
    overloads: list[griffe.Function] | None = None,
) -> Observation:
    alias = isinstance(obj, griffe.Alias)
    signature = None
    bases: list[str] = []
    if isinstance(obj, griffe.Function):
        signature = obj.signature()
        overloads = overloads or obj.overloads
    elif isinstance(obj, griffe.Class):
        bases = [str(base) for base in obj.bases]
        signature = f"class {obj.name}({', '.join(bases)})"
    elif isinstance(obj, griffe.Attribute):
        signature = f"{obj.name}: {obj.annotation}" if obj.annotation is not None else obj.name
    return Observation(
        path=obj.path,
        kind="alias" if alias else obj.kind.value,
        origin=file.origin,
        file=file.file,
        line=obj.alias_lineno if alias else obj.lineno,
        signature=signature,
        overloads=[function.signature() for function in (overloads or [])],
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
            if name not in obj.members and overloads:
                yield _observation(overloads[0], file, obj, overloads)


def extract(request: WorkerRequest) -> WorkerResponse:
    """Return independent per-file observations under the Rust-provided inventory."""
    if request.schema_version != "1.0" or not 1 <= request.max_observations <= 100000:
        raise ValueError("unsupported worker protocol or observation bound")
    root = Path(request.root)
    if not root.is_absolute() or not root.is_dir():
        raise ValueError("worker needs an explicit absolute input root")
    root = root.resolve(strict=True)
    # The loader provides reviewed extensions and shared collections to the AST visitor.
    # Never call load(): independent visits preserve source/stub disagreements and avoid
    # sys.path discovery, .pth redirects, package imports and implicit source/stub merging.
    loader = griffe.GriffeLoader(
        search_paths=[root], allow_inspection=False, force_inspection=False
    )
    observations: list[Observation] = []
    processed: list[str] = []
    gaps: list[WorkerGap] = []
    retained_bytes = 0
    for file in request.files:
        start = len(observations)
        previous_bytes = retained_bytes
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
                size = len(fact.model_dump_json().encode())
                if size > 1024 * 1024:
                    raise ValueError("declaration exceeds 1 MiB; source artifact remains available")
                if len(observations) >= request.max_observations:
                    raise ValueError("observation budget exhausted")
                if retained_bytes + size > MAX_OUTPUT - 65536:
                    raise ValueError("retained observation byte budget exhausted")
                observations.append(fact)
                retained_bytes += size
            processed.append(file.file)
        except (OSError, ValueError, SyntaxError, UnicodeError, RecursionError) as error:
            del observations[start:]
            retained_bytes = previous_bytes
            gaps.append(
                WorkerGap(
                    file=file.file,
                    reason=f"{type(error).__name__}: {error}".replace(str(root), "<input>"),
                )
            )
    return WorkerResponse(
        schema_version="1.0",
        griffe_version=importlib.metadata.version("griffe"),
        worker_python=platform.python_version(),
        observations=observations,
        processed_files=processed,
        gaps=gaps,
    )


def main() -> int:
    """Read one schema-validated job and write exactly one JSON response."""
    try:
        raw = sys.stdin.buffer.read(MAX_INPUT + 1)
        if len(raw) > MAX_INPUT:
            raise ValueError("worker input exceeds byte budget")
        request = WorkerRequest.model_validate_json(raw)
        if request.max_memory_bytes <= 0 or request.max_cpu_seconds <= 0:
            raise ValueError("Rust worker allocation/deadline bounds are required")
        for kind, cap in (
            (resource.RLIMIT_AS, request.max_memory_bytes),
            (resource.RLIMIT_CPU, request.max_cpu_seconds),
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
        result = extract(request)
        output = result.model_dump_json().encode()
        if len(output) > MAX_OUTPUT:
            raise ValueError("worker output exceeds byte budget")
        sys.stdout.buffer.write(output + b"\n")
        sys.stdout.buffer.flush()
        return 0
    except (OSError, ValueError, MemoryError) as error:
        sys.stderr.write(f"static worker failed: {error}\n")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())

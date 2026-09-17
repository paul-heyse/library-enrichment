"""Index the library crates behind both tools, plus the JS and Python bindings.

The CLI is one surface; the crates underneath are another, and they are the reusable half. The
file-set machinery an agent most often wants to borrow -- gitignore semantics, glob matching --
is `ignore` and `globset`, and neither has anything to do with searching.

Versions come from ripgrep's own `Cargo.lock` at the pinned tag, not from crates.io. ripgrep
15.2.0 links `ignore` 0.4.29 while crates.io offers 0.4.33; indexing the latter would describe
code nobody here is running, and would quietly contradict every probe in this repository.

The bindings are indexed from their shipped type declarations rather than re-derived, and they
are parsed with ast-grep itself. Using the subject to index the subject keeps this build to the
standard library plus the two pinned binaries, with no extraction toolchain to pin separately.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

import acquire
import emit
import model

# Format versions observed across these crates on docs.rs. `probe_format` checks before parsing,
# because an unsupported rustdoc document parses far enough to produce confident nonsense.
SUPPORTED_FORMATS = [57, 59, 60, 61]

# Which family each crate belongs to, so a reader can tell at a glance whether a symbol comes
# from the tool, from its engine, or from the file-walking layer.
FAMILY = {
    "grep": "ripgrep",
    "grep-regex": "ripgrep",
    "grep-searcher": "ripgrep",
    "grep-printer": "ripgrep",
    "grep-matcher": "ripgrep",
    "grep-pcre2": "ripgrep",
    "grep-cli": "ripgrep",
    "ignore": "ripgrep",
    "globset": "ripgrep",
    "ast-grep-core": "ast-grep",
    "ast-grep-config": "ast-grep",
    "ast-grep-language": "ast-grep",
    "ast-grep-dynamic": "ast-grep",
    "ast-grep-lsp": "ast-grep",
    "ast-grep-outline": "ast-grep",
}


def build_models(cache: Path, versions: dict[str, str]) -> dict[str, model.CrateModel]:
    """Parse every acquired rustdoc document into a crate model."""
    models: dict[str, model.CrateModel] = {}
    for name, version in sorted(versions.items()):
        payload = acquire.read_rustdoc(cache, name, version)
        models[name] = model.build_crate(name, version, payload, SUPPORTED_FORMATS)
    return models


def emit_api(models: dict[str, model.CrateModel], content: Path) -> dict[str, int]:
    """Write `api/`, `model/`, and the symbol-level indexes."""
    items, _alias_index, unresolved = model.stitch(models)
    grouped = emit.group_by_module(items)

    modules = emit.write_model(grouped, content)
    emit.write_api(grouped, content)

    symbols, methods, alias_rows, impls = [], [], [], []
    for path, item in sorted(items.items()):
        family = FAMILY.get(item.crate, "engine")
        symbols.append(
            (
                path,
                item.kind,
                item.crate,
                family,
                emit.api_file(item.module),
                str(len(item.aliases)),
                str(len(item.methods)),
                item.summary,
            )
        )
        for method in item.methods:
            methods.append(
                (path, method.name, method.via_trait or "-", method.signature, method.summary)
            )
        for access in item.aliases:
            alias_rows.append((access, path, item.kind))
        for implemented in item.implements:
            impls.append((implemented, path, item.crate))

    return {
        "modules": modules,
        "symbols": symbols,
        "methods": methods,
        "aliases": alias_rows,
        "impls": impls,
        "unresolved": [(path,) for path in sorted(unresolved)],
    }


# --------------------------------------------------------------------------- bindings


def binding_rows(acquired: Path) -> list[tuple]:
    """Index the JS and Python binding surfaces from their shipped type declarations.

    Extraction is `ast-grep outline`, not a hand-written pattern. Patterns would have to
    enumerate every declaration form the file happens to use -- `export interface`,
    `export declare class`, `export declare function` -- and would silently miss whichever form
    was forgotten, producing an index that looks complete and is not. `outline` already knows
    the grammar's declaration shapes, and it reports members as well as top-level items.

    A declaration file establishes what the binding *declares*, which is stronger than prose and
    weaker than a type checker: it does not establish that a name resolves at runtime.
    """
    rows: list[tuple] = []
    for binding, language in (("napi", "ts"), ("pyo3", "python")):
        root = acquired / "bindings" / binding
        if not root.exists():
            continue
        rows.extend(_outline_rows(root, language, binding))
    return sorted(set(rows))


def _outline_rows(root: Path, language: str, binding: str) -> list[tuple]:
    done = subprocess.run(
        ["ast-grep", "outline", str(root), "--items", "all", "--json=stream"],
        capture_output=True,
        text=True,
        check=False,
    )
    rows: list[tuple] = []
    for line in done.stdout.splitlines():
        if not line.strip():
            continue
        document = json.loads(line)
        source = Path(document["path"]).name
        for item in document.get("items", []):
            if item.get("isImport"):
                continue
            rows.append(
                (
                    binding,
                    language,
                    item.get("symbolType", ""),
                    item.get("name", ""),
                    "item",
                    source,
                )
            )
            for member in item.get("members", []):
                rows.append(
                    (
                        binding,
                        language,
                        member.get("symbolType", ""),
                        f"{item.get('name', '')}.{member.get('name', '')}",
                        "member",
                        source,
                    )
                )
    return rows

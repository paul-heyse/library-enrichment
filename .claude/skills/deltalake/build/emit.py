"""Write the content repository: structured model, prose pages, and provenance.

Each fact is stored once, in the format matched to how it is read.

    model/<module>.json   structure, signatures, aliases, edges -- queried with ast-grep
    api/<module>.md       the full doc prose -- read directly
    index/*.tsv           a regenerated projection -- grepped

A model record points at its prose (`"doc": "api/<module>.md#Name"`) instead of copying it, so
enlarging a doc comment never desynchronizes two files.

File naming is a rule, not a lookup: a canonical path's module part with `::` replaced by `.`.
`deltalake_core::table::DeltaTable` lives in `model/deltalake_core.table.json` and
`api/deltalake_core.table.md`, and an agent can construct either without consulting an index.
"""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from dataclasses import asdict
from pathlib import Path

from model import Item

MODEL_DIR = "model"
API_DIR = "api"

# Traits that every type implements and no reader is looking for. Listing them costs a line per
# type across 1,300 pages, and their methods (`clone`, `fmt`, `eq`) bury the real surface. They
# are summarized on a single "Derives" line instead of being dropped, because whether a type is
# `Clone` does occasionally matter.
UBIQUITOUS_TRAITS = frozenset(
    {
        "core::clone::Clone",
        "core::cmp::Eq",
        "core::cmp::Ord",
        "core::cmp::PartialEq",
        "core::cmp::PartialOrd",
        "core::default::Default",
        "core::fmt::Debug",
        "core::hash::Hash",
        "core::marker::Copy",
        "core::marker::Freeze",
        "core::marker::Send",
        "core::marker::StructuralPartialEq",
        "core::marker::Sync",
        "core::marker::Unpin",
    }
)


def is_ubiquitous(trait_path: str) -> bool:
    return trait_path in UBIQUITOUS_TRAITS


def module_slug(module_path: str) -> str:
    return module_path.replace("::", ".")


def model_file(module_path: str) -> str:
    return f"{MODEL_DIR}/{module_slug(module_path)}.json"


def api_file(module_path: str) -> str:
    return f"{API_DIR}/{module_slug(module_path)}.md"


def doc_anchor(item: Item) -> str:
    return f"{api_file(item.module)}#{item.name.lower()}"


def group_by_module(items: dict[str, Item]) -> dict[str, list[Item]]:
    grouped: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        grouped[item.module].append(item)
    for bucket in grouped.values():
        bucket.sort(key=lambda entry: (entry.kind, entry.name))
    return grouped


def _model_record(item: Item) -> dict:
    record = {
        "path": item.path,
        "name": item.name,
        "kind": item.kind,
        "crate": item.crate,
        "signature": item.signature,
        "summary": item.summary,
        "doc": doc_anchor(item) if item.docs else None,
    }
    if item.deprecated:
        record["deprecated"] = item.deprecated
    if item.aliases:
        record["aliases"] = item.aliases
    if item.variants:
        record["variants"] = item.variants
    if item.fields:
        record["fields"] = item.fields
    notable = [t for t in item.implements if not is_ubiquitous(t)]
    if notable:
        record["implements"] = notable
    if item.implementors:
        record["implementors"] = item.implementors
    if item.kind == "trait":
        record["required_methods"] = sorted(set(item.required_methods))
        record["provided_methods"] = sorted(set(item.provided_methods))
    if item.methods:
        seen: set[tuple[str, str | None]] = set()
        methods = []
        for method in sorted(item.methods, key=lambda m: (m.name, m.via_trait or "")):
            if method.via_trait and is_ubiquitous(method.via_trait):
                continue
            key = (method.name, method.via_trait)
            if key in seen:
                continue
            seen.add(key)
            entry = {"name": method.name, "signature": method.signature}
            if method.via_trait:
                entry["via_trait"] = method.via_trait
            if method.summary:
                entry["summary"] = method.summary
            methods.append(entry)
        record["methods"] = methods
    return record


def write_model(grouped: dict[str, list[Item]], root: Path) -> int:
    (root / MODEL_DIR).mkdir(parents=True, exist_ok=True)
    for module_path, members in grouped.items():
        document = {
            "module": module_path,
            "crate": members[0].crate,
            "api": api_file(module_path),
            "items": [_model_record(item) for item in members],
        }
        target = root / model_file(module_path)
        target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    return len(grouped)


def _method_block(item: Item) -> list[str]:
    if not item.methods:
        return []
    inherent = [m for m in item.methods if not m.via_trait]
    by_trait: dict[str, list] = defaultdict(list)
    for method in item.methods:
        if method.via_trait and not is_ubiquitous(method.via_trait):
            by_trait[method.via_trait].append(method)

    lines: list[str] = []
    if inherent:
        lines.append("")
        lines.append(f"**Methods** ({len(inherent)})")
        lines.append("")
        lines.append("```rust")
        for method in sorted(inherent, key=lambda m: m.name):
            lines.append(method.signature)
        lines.append("```")
    for trait_path in sorted(by_trait):
        methods = by_trait[trait_path]
        lines.append("")
        lines.append(f"**via `{trait_path}`**")
        lines.append("")
        lines.append("```rust")
        for method in sorted(methods, key=lambda m: m.name):
            lines.append(method.signature)
        lines.append("```")
    return lines


def write_api(grouped: dict[str, list[Item]], root: Path) -> int:
    (root / API_DIR).mkdir(parents=True, exist_ok=True)
    for module_path, members in grouped.items():
        lines = [
            f"# `{module_path}`",
            "",
            f"Crate `{members[0].crate}` · {len(members)} public items · "
            f"structured records in [`{model_file(module_path)}`]"
            f"(../{model_file(module_path)})",
            "",
        ]
        for item in members:
            lines.append(f"## {item.name}")
            lines.append("")
            lines.append(f"`{item.kind}` · `{item.path}`")
            full_page = item.path.replace("::", ".") + ".md"
            lines.append(
                "[Full member contracts, output types and access classification]"
                f"(../operations/{full_page})"
            )
            if item.deprecated:
                lines.append("")
                lines.append(f"> **Deprecated** — {item.deprecated}")
            if item.aliases:
                lines.append("")
                shown = ", ".join(f"`{alias}`" for alias in item.aliases[:8])
                more = f" (+{len(item.aliases) - 8} more)" if len(item.aliases) > 8 else ""
                lines.append(f"Also reachable as {shown}{more}")
            lines.append("")
            lines.append("```rust")
            lines.append(item.signature)
            lines.append("```")
            if item.variants:
                lines.append("")
                lines.append("**Variants**: " + ", ".join(f"`{v}`" for v in item.variants))
            if item.fields:
                lines.append("")
                lines.append("**Fields**: " + ", ".join(f"`{f}`" for f in item.fields))
            if item.implementors:
                lines.append("")
                lines.append(f"**Implementors** ({len(item.implementors)})")
                lines.append("")
                for implementor in item.implementors:
                    lines.append(f"- `{implementor}`")
            notable = [t for t in item.implements if not is_ubiquitous(t)]
            derived = [t.rsplit("::", 1)[-1] for t in item.implements if is_ubiquitous(t)]
            if notable:
                lines.append("")
                lines.append("**Implements**: " + ", ".join(f"`{t}`" for t in notable))
            if derived:
                lines.append("")
                lines.append("**Derives**: " + ", ".join(sorted(derived)))
            lines.extend(_method_block(item))
            if item.docs:
                lines.append("")
                lines.append(item.docs.rstrip())
            lines.append("")
            lines.append("---")
            lines.append("")
        (root / api_file(module_path)).write_text("\n".join(lines))
    return len(grouped)


def digest_tree(root: Path) -> dict[str, str]:
    """Hash every generated file, so a rebuild can be proven byte-identical."""
    digests: dict[str, str] = {}
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.name != "PROVENANCE.json":
            relative = path.relative_to(root).as_posix()
            digests[relative] = hashlib.sha256(path.read_bytes()).hexdigest()
    return digests


def write_provenance(
    root: Path,
    manifest: dict,
    crate_facts: dict[str, dict],
    counts: dict[str, int],
    tool_versions: dict[str, str],
) -> None:
    provenance = {
        "repository": manifest["repository"]["name"],
        "generated_at": manifest["repository"]["reference_date"],
        "tools": tool_versions,
        "counts": counts,
        # The cache-invalidation key: anything here changing invalidates the whole repository.
        "crates": crate_facts,
        "files": digest_tree(root),
    }
    (root / "PROVENANCE.json").write_text(
        json.dumps(provenance, indent=2, ensure_ascii=False) + "\n"
    )


def item_as_dict(item: Item) -> dict:
    return asdict(item)

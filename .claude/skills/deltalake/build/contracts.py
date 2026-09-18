"""Preserve rustdoc contracts and render addressable member/module documentation.

Raw type trees remain artifact-scoped. Display signatures are a separate projection;
in particular async-trait's expanded Future type must not be mistaken for source syntax.
"""

from __future__ import annotations

import hashlib
import json
import re
from collections import defaultdict
from collections.abc import Callable
from pathlib import Path
from types import SimpleNamespace

from render import render_function, render_item_header, render_type


def identifier(value: object) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()[:24]


def logical_span(span: dict | None) -> dict | None:
    if not span:
        return span
    filename = re.sub(r"^.*?/git/checkouts/[^/]+/[0-9a-f]{7,40}/", "", span["filename"])
    return {**span, "filename": filename}


def slug(path: str) -> str:
    return path.replace("::", ".")


def collect(name: str, version: str, document: dict, public_paths: set[str]) -> list[dict]:
    index = document["index"]
    paths = document["paths"]
    source_digest = hashlib.sha256(json.dumps(document, sort_keys=True).encode()).hexdigest()
    records: list[dict] = []
    visited: set[tuple[str, str, str]] = set()

    def canonical(item_id: object) -> str | None:
        path = paths.get(str(item_id))
        return "::".join(path["path"]) if path else None

    def references(node: object) -> dict:
        found = {}
        if isinstance(node, dict):
            if "id" in node:
                key = str(node["id"])
                found[key] = canonical(key)
            for value in node.values():
                found.update(references(value))
        elif isinstance(node, list):
            for value in node:
                found.update(references(value))
        return found

    def add(item_id: object, path: str, owner: str | None, context: dict | None = None) -> None:
        entry = index.get(str(item_id))
        if not entry:
            return
        key = (str(item_id), path, identifier(context))
        if key in visited:
            return
        visited.add(key)
        inner = entry.get("inner") or {}
        kind = next(iter(inner), "unknown")
        body = inner.get(kind)
        item_name = entry.get("name") or path.rsplit("::", 1)[-1]
        if kind == "function":
            if not isinstance(body, dict):
                raise ValueError(f"Invalid function payload: {path}")
            signature = render_function(item_name, body)
        elif kind == "struct_field":
            signature = f"{item_name}: {render_type(body)}"
        elif kind == "module":
            signature = f"mod {item_name}"
        elif kind == "assoc_type":
            value = body.get("type") if isinstance(body, dict) else None
            signature = f"type {item_name}" + (f" = {render_type(value)}" if value else "")
        else:
            signature = render_item_header(item_name, kind, body if isinstance(body, dict) else {})
        stable_context = (
            {
                "impl": {**context, "span": logical_span(context.get("span"))},
                "span": logical_span(entry.get("span")),
            }
            if context
            else None
        )
        op_id = identifier([name, version, path, kind, stable_context])
        record = {
            "id": op_id,
            "path": path,
            "name": item_name,
            "owner": owner,
            "crate": name,
            "version": version,
            "kind": kind,
            "signature": signature,
            "docs": entry.get("docs") or "",
            "span": entry.get("span"),
            "visibility": entry.get("visibility"),
            "attributes": entry.get("attrs", []),
            "deprecation": entry.get("deprecation"),
            "type_tree": inner,
            "type_references": references(inner),
            "impl_context": context,
            "links": entry.get("links", {}),
            "link_paths": {
                text: canonical(target) for text, target in entry.get("links", {}).items()
            },
            "artifact": {
                "id": source_digest,
                "format_version": document["format_version"],
                "item_id": str(item_id),
                "origin": f"https://docs.rs/crate/{name}/{version}/json",
            },
            "availability": (
                "Observed in the hosted documentation configuration; consumer features unverified."
            ),
        }
        records.append(record)
        if not isinstance(body, dict):
            return
        children = list(body.get("variants") or [])
        if kind == "trait":
            children += list(body.get("items") or [])
        layout = body.get("kind")
        if isinstance(layout, dict):
            plain = layout.get("plain") or layout.get("struct") or {}
            children += list(plain.get("fields") or [])
            children += [i for i in layout.get("tuple", []) if i is not None]
        if kind == "union":
            children += list(body.get("fields") or [])
        for child_id in children:
            child = index.get(str(child_id))
            if child and child.get("name") is not None:
                add(child_id, f"{path}::{child['name']}", path)

    for item_id, entry in index.items():
        path = canonical(item_id)
        if path in public_paths:
            add(item_id, path, None)
        if "module" in entry.get("inner", {}) and entry.get("crate_id") == 0 and path:
            add(item_id, path, None)
    for entry in index.values():
        impl = entry.get("inner", {}).get("impl")
        if not impl or impl.get("is_synthetic") or impl.get("blanket_impl"):
            continue
        target = (impl.get("for") or {}).get("resolved_path")
        owner = canonical(target["id"]) if target else None
        if owner not in public_paths:
            continue
        trait = impl.get("trait")
        context = {
            "for": impl.get("for"),
            "trait": impl.get("trait"),
            "trait_path": canonical(trait["id"]) if trait else None,
            "generics": impl.get("generics"),
            "is_negative": impl.get("is_negative", False),
            "span": entry.get("span"),
        }

        # Replace document-local ids in the identity-bearing context, but retain the raw
        # function type tree and its reference table separately.
        def resolved(node: object) -> object:
            if isinstance(node, dict):
                return {
                    k: (canonical(v) or "unresolved") if k == "id" else resolved(v)
                    for k, v in node.items()
                }
            if isinstance(node, list):
                return [resolved(v) for v in node]
            return node

        context = resolved(context)
        if not isinstance(context, dict):
            raise ValueError(f"Invalid impl context: {owner}")
        for member_id in impl.get("items") or []:
            member = index.get(str(member_id))
            if member and member.get("name"):
                # Private inherent helpers are not public API. Trait members use default visibility.
                if not trait and member.get("visibility") != "public":
                    continue
                add(member_id, f"{owner}::{member['name']}", owner, context)
    unique = {}
    for record in records:
        if record["id"] in unique and record != unique[record["id"]]:
            raise ValueError(f"Conflicting contract identity: {record['path']}")
        unique[record["id"]] = record
    return sorted(unique.values(), key=lambda r: (r["path"], r["id"]))


def acquire_contracts(
    models: dict, items: dict, specs: list[dict], acquired_root: Callable[[dict], Path]
) -> dict:
    """Merge public/private records by semantic identity, retaining each raw source identity."""
    import fetch

    result = {}
    for spec in specs:
        crate = spec["package"]
        directory = acquired_root(spec["crate_set"])
        receipt = fetch.acquisition(directory)
        documents = [
            ("public", json.loads(fetch.rustdoc_json_local(directory, receipt, spec["lib"]))),
            (
                "private",
                json.loads(fetch.rustdoc_supplement_local(directory, receipt, spec["lib"])),
            ),
        ]
        members = {p for p, item in items.items() if item.crate == crate}
        public_modules = models[crate].public_modules
        git = spec["crate_set"]["git"]
        source_repo, source_rev = git["url"], git["rev"]
        if crate.startswith("buoyant_"):
            source_repo = "https://github.com/buoyant-data/delta-kernel-rs"
            source_rev = git["lock_deps"][crate]
        merged = {}
        for role, document in documents:
            filename = spec["lib"] + (".private" if role == "private" else "") + ".json.zst"
            for record in collect(crate, spec["version"], document, members):
                record["artifact"].update(
                    {
                        "role": role,
                        "file": f"build/acquired/{directory.name}/{filename}",
                        "compressed_sha256": receipt["files"][filename]["sha256"],
                        "origin": f"{source_repo}/tree/{source_rev}",
                    }
                )
                record["availability"] = {
                    "profile": "content/profile.json",
                    "features": receipt["resolved_features"].get(
                        crate, spec.get("features_on", [])
                    ),
                    "note": "Observed in this git capture; consumer feature graph must be matched.",
                }
                owner = record["owner"] or record["path"]
                item = items.get(owner)
                if record["kind"] == "module":
                    access = "public" if owner in public_modules else "internal_module"
                elif item and (item.aliases or item.module in public_modules):
                    access = "public"
                elif item:
                    access = "returned_inferred"
                else:
                    # Fields/variants inherit the admitted containing type's access classification.
                    parent = next(
                        (
                            items[p]
                            for p in sorted(items, key=len, reverse=True)
                            if owner.startswith(p + "::")
                        ),
                        None,
                    )
                    access = (
                        "public"
                        if parent and (parent.aliases or parent.module in public_modules)
                        else "internal"
                    )
                context = record.get("impl_context") or {}
                trait_path = context.get("trait_path")
                if trait_path and trait_path.startswith(spec["lib"] + "::"):
                    trait_item = items.get(trait_path)
                    if not trait_item or not (
                        trait_item.aliases or trait_item.module in public_modules
                    ):
                        access = "internal_trait"
                if record["kind"] == "struct_field" and record["visibility"] != "public":
                    access = "internal_field"
                record["access"] = access
                record["source_url"] = None
                if record["span"]:
                    record["source_url"] = (
                        f"{source_repo}/blob/{source_rev}/{logical_span(record['span'])['filename']}"
                        f"#L{record['span']['begin'][0]}"
                    )
                # The public record wins duplicates. Private supplements add missing members only.
                existing = merged.get(record["id"])
                if existing:
                    existing.setdefault("supplemental_artifacts", []).append(record["artifact"])
                else:
                    merged[record["id"]] = record
        result[crate] = SimpleNamespace(contracts=list(merged.values()))
    return result


def write(models: dict, items: dict, root: Path, notes_path: Path | None = None) -> dict[str, int]:
    records = [r for model in models.values() for r in model.contracts]
    notes = json.loads(notes_path.read_text()) if notes_path else []
    ids = {(r["artifact"]["id"], r["artifact"]["item_id"]): r for r in records}
    targets = {r["path"]: r for r in records if not r["impl_context"]}
    groups = defaultdict(list)
    diagnostics = []
    rows = []
    for record in records:
        record["reader_notes"] = [
            n for n in notes if n["path"] == record["path"] and n["version"] == record["version"]
        ]
        owner = record["owner"] or record["path"]
        family = "modules" if record["kind"] == "module" else "operations"
        record["page"] = f"{family}/{slug(owner)}.md"
        record["record_file"] = f"{family}/{slug(owner)}.json"
        record["anchor"] = f"op-{record['id']}"
        record["aliases"] = []
        if record["path"] in items:
            record["aliases"] = items[record["path"]].aliases
        elif record["owner"] in items:
            record["aliases"] = [f"{a}::{record['name']}" for a in items[record["owner"]].aliases]
        groups[(family, owner)].append(record)
    for record in records:
        mapped = {}
        for text, target_id in record["links"].items():
            target = ids.get((record["artifact"]["id"], str(target_id)))
            if target is None:
                target_path = record["link_paths"].get(text)
                target = targets.get(target_path)
            if target:
                mapped[text] = f"../{target['page']}#{target['anchor']}"
            else:
                diagnostics.append(
                    {
                        "operation": record["id"],
                        "reference": text,
                        "artifact_local_target": str(target_id),
                        "state": "unresolved",
                    }
                )
        record["resolved_doc_links"] = mapped
        if re.search(r"(?:^|[ <,:])_(?:$|[ >,)])", record["signature"]):
            diagnostics.append(
                {
                    "operation": record["id"],
                    "state": "display_placeholder",
                    "note": "Inspect preserved type_tree; display is not type authority.",
                }
            )
        rows.append(
            "\t".join(
                [
                    record["path"],
                    record["id"],
                    record["kind"],
                    record["crate"],
                    record["record_file"],
                    f"{record['page']}#{record['anchor']}",
                    record["signature"].replace("\n", " ").replace("\t", " "),
                ]
            )
        )
    unresolved_by_operation = defaultdict(list)
    for diagnostic in diagnostics:
        if "reference" in diagnostic:
            unresolved_by_operation[diagnostic["operation"]].append(diagnostic["reference"])
    for (family, owner), members in sorted(groups.items()):
        out = root / family
        out.mkdir(exist_ok=True)
        (out / f"{slug(owner)}.json").write_text(
            json.dumps(members, ensure_ascii=False, indent=2) + "\n"
        )
        lines = [
            f"# `{owner}`",
            "",
            "Full upstream contracts; raw type trees and source locators in "
            f"[structured records]({slug(owner)}.json).",
            "",
        ]
        for record in members:
            lines += [
                f'<a id="{record["anchor"]}"></a>',
                f"## {record['name']}",
                "",
                f"`{record['kind']}` · `{record['path']}` · {record['crate']} {record['version']}",
                "",
                f"Access: **{record.get('access', 'unclassified')}**. "
                "Canonical source location is not automatically a valid import path.",
                "",
                "```rust",
                record["signature"],
                "```",
                "",
            ]
            if record.get("source_url"):
                lines += [f"[Exact source]({record['source_url']}).", ""]
            if record["impl_context"]:
                lines += [
                    "Implementation context: "
                    f"`{json.dumps(record['impl_context'], sort_keys=True)}`",
                    "",
                ]
            if record["span"]:
                span = record["span"]
                lines += [
                    f"Source: `{span['filename']}:{span['begin'][0]}`. "
                    f"[Exact documentation build]({record['artifact']['origin']}).",
                    "",
                ]
            docs = record["docs"]
            for note in record["reader_notes"]:
                lines += [
                    f"**Reference annotation ({note['kind']}, separate from upstream):** "
                    f"{note['text']} [Evidence]({note['evidence']}).",
                    "",
                ]
            for label, target in sorted(
                record["resolved_doc_links"].items(), key=lambda x: -len(x[0])
            ):
                # Do not rewrite fenced examples or existing Markdown inline links.
                pieces = re.split(r"(```.*?```)", docs, flags=re.DOTALL)
                docs = "".join(
                    p
                    if p.startswith("```")
                    else re.sub(
                        re.escape(f"[{label}]") + r"(?![\[(])",
                        lambda _, label=label, target=target: f"[{label}]({target})",
                        p,
                    )
                    for p in pieces
                )
            lines += [
                docs or "No upstream documentation on this item; consult its owner/trait contract.",
                "",
            ]
            unresolved = unresolved_by_operation[record["id"]]
            if unresolved:
                lines += [
                    "Unresolved upstream links (retained, not inferred): "
                    + ", ".join(f"`{x}`" for x in unresolved)
                    + ".",
                    "",
                ]
        (out / f"{slug(owner)}.md").write_text("\n".join(lines))
    (root / "index/operations.tsv").write_text("\n".join(sorted(rows)) + "\n")
    (root / "contract-diagnostics.json").write_text(json.dumps(diagnostics, indent=2) + "\n")
    return {
        "operation_contracts": len(records),
        "contract_pages": len(groups),
        "documented_contracts": sum(bool(r["docs"]) for r in records),
        "module_contracts": sum(r["kind"] == "module" for r in records),
        "contract_diagnostics": len(diagnostics),
    }

"""Preserve rustdoc contracts and render addressable member/module documentation.

Raw type trees remain artifact-scoped. Display signatures are a separate projection;
in particular async-trait's expanded Future type must not be mistaken for source syntax.
"""

from __future__ import annotations

import hashlib
import json
import re
from collections import defaultdict
from pathlib import Path

from render import render_function, render_item_header, render_type


def identifier(value: object) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()[:24]


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
        else:
            signature = render_item_header(item_name, kind, body if isinstance(body, dict) else {})
        stable_context = {"impl": context, "span": entry.get("span")} if context else None
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


def write(models: dict, items: dict, root: Path, notes_path: Path | None = None) -> dict[str, int]:
    records = [r for model in models.values() for r in model.contracts]
    notes = json.loads(notes_path.read_text()) if notes_path else []
    ids = {(r["crate"], r["artifact"]["item_id"]): r for r in records}
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
            target = ids.get((record["crate"], str(target_id)))
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
                "```rust",
                record["signature"],
                "```",
                "",
            ]
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

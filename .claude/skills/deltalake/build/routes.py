"""Derive Delta operation shapes, protocol vocabulary and integration entry routes."""

from __future__ import annotations

import json
import re
from collections import defaultdict
from pathlib import Path

from render import render_type


def write(root: Path, content: Path) -> dict[str, int]:
    records = [
        r
        for folder in ["operations", "modules"]
        for p in (content / folder).glob("*.json")
        for r in json.loads(p.read_text())
    ]
    by_path = defaultdict(list)
    for record in records:
        by_path[record["path"]].append(record)
    outputs = {}
    for record in records:
        trait = (record["impl_context"] or {}).get("trait_path") or ""
        if record["name"] == "Output" and trait.endswith("::IntoFuture"):
            outputs[record["owner"]] = {
                "display": record["signature"],
                "contract": record["page"] + "#" + record["anchor"],
                "type_tree": record["type_tree"],
                "artifact": record["artifact"],
            }
    constructors = defaultdict(set)

    def referenced_types(node: object, refs: dict) -> set[str]:
        found = set()
        if isinstance(node, dict):
            if "resolved_path" in node:
                path = refs.get(str(node["resolved_path"]["id"]))
                if path:
                    found.add(path)
            for value in node.values():
                found.update(referenced_types(value, refs))
        elif isinstance(node, list):
            for value in node:
                found.update(referenced_types(value, refs))
        return found

    callable_records = [
        r
        for r in records
        if r["kind"] == "function" and r["access"] in {"public", "returned_inferred"}
    ]
    for record in callable_records:
        sig = record["type_tree"]["function"].get("sig", {})
        for target in referenced_types(sig.get("output"), record["type_references"]):
            if target != record["owner"] and target in by_path:
                constructors[target].add(record["path"])
    rows = []
    # Include awaitable builders, explicit build/execute, staging writers and clause helpers.
    owners = set(outputs)
    owners.update(
        r["owner"]
        for r in callable_records
        if r["name"] in {"build", "build_with_metrics", "execute", "flush", "flush_and_commit"}
        or r["name"].startswith("when_")
    )
    owners.discard(None)
    for owner in sorted(owners):
        own = [r for r in callable_records if r["owner"] == owner]
        if not own:
            continue
        methods = [
            {
                "path": r["path"],
                "signature": r["signature"],
                "output_display": render_type(
                    r["type_tree"]["function"].get("sig", {}).get("output")
                ),
                "contract": r["page"] + "#" + r["anchor"],
            }
            for r in own
            if not r["impl_context"]
            or not r["impl_context"].get("trait_path")
            or r["name"] in {"flush", "flush_and_commit", "write"}
        ]
        rows.append(
            {
                "owner": owner,
                "access": own[0]["access"],
                "constructed_by": sorted(constructors[owner]),
                "await_output": outputs.get(owner),
                "methods": methods,
                "evidence": (
                    "Structured signature outputs, not closure bounds or leaf-name matching"
                ),
            }
        )
    (content / "catalogs/operation-map.json").write_text(json.dumps(rows, indent=2) + "\n")
    lines = [
        "# Operation construction and results",
        "",
        "Awaitable builders are one subset. Explicit build/execute and writer flush/commit paths",
        "are included below. Constructor edges use nominal structured return types; clause",
        "closures do not construct standalone delete/update operations. Defaults and effects",
        "are characterized in the [task routes](../routes/tasks.md).",
        "",
        "| Owner | Access | Constructed by | Awaited result / explicit methods |",
        "|---|---|---|---|",
    ]
    for row in rows:
        output = row["await_output"]
        result = (
            f"[`{output['display']}`](../{output['contract']})"
            if output
            else "Explicit build/execute/flush; see full contracts"
        )
        lines.append(
            f"| `{row['owner']}` | {row['access']} | "
            f"{'; '.join(row['constructed_by']) or 'Inspect full contract'} | {result} |"
        )
    lines += [
        "",
        "[Structured map with configuration and method contracts](operation-map.json).",
        "Returned-inferred types are callable through public return values; their private module",
        (
            "canonical paths are not import paths. Internal trait methods"
            " are excluded from caller routes."
        ),
        "",
    ]
    (content / "catalogs/operations.md").write_text("\n".join(lines))

    source = root / (
        "skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/transaction/protocol.rs"
    )
    text = source.read_text().split("pub static INSTANCE:", 1)[1].split("#[cfg(test)]", 1)[0]
    text = re.sub(r"//[^\n]*", "", text)
    admission = {
        side: set(re.findall(side + r"_features\.insert\(TableFeature::(\w+)\)", text))
        for side in ["reader", "writer"]
    }
    names = sorted(
        {
            r["name"]
            for r in records
            if r["kind"] == "variant"
            and (r["owner"] or "").endswith(("::TableFeatures", "::TableFeature"))
        }
    )
    feature_rows = []
    for name in names:
        gates = {"reader": None, "writer": None}
        if name == "TimestampNanos":
            gates = {side: "nanosecond-timestamps" for side in gates}
        if name == "ColumnMapping":
            gates = {side: "datafusion" for side in gates}
        if name in {"ChangeDataFeed", "Invariants", "CheckConstraints", "GeneratedColumns"}:
            gates["writer"] = "datafusion"
        feature_rows.append(
            {
                "feature": name,
                "recognized": True,
                "explicit_admission": {
                    side: {
                        "present_in_source_set": name in admission[side],
                        "requires_feature": gates[side],
                    }
                    for side in gates
                },
                "operation_limits": ["CDF build rejects non-None column mapping"]
                if name == "ColumnMapping"
                else [],
                "runtime_evidence": (
                    "See protocol and fixture assertions in probe-results.json; e"
                    "numeration is not a runtime result"
                ),
                "other_operation_support": (
                    "unknown until an operation-specific contract or fixture establishes it"
                ),
            }
        )
    (content / "catalogs/protocol-matrix.json").write_text(
        json.dumps(feature_rows, indent=2) + "\n"
    )
    (content / "catalogs/table-features.md").write_text(
        "# Protocol recognition and admission\n\n"
        + (
            "The enum vocabulary below is not a list of supported operati"
            "ons. These source-derived sets\n"
        )
        + (
            "are the default checker's explicit feature admission; legacy"
            " protocol-version handling and\n"
        )
        + (
            "operation-specific restrictions still apply. Feature gates d"
            "epend on the consumer profile.\n\n"
        )
        + (
            "| Feature | Reader insertion | Writer insertion | Operation "
            "restriction |\n|---|---|---|---|\n"
        )
        + "\n".join(
            f"| {r['feature']} | {r['explicit_admission']['reader']} | {
                r['explicit_admission']['writer']
            } | {
                '; '.join(r['operation_limits'])
                or ('Consult operation contract; uncharacterized cells remain unknown')
            } |"
            for r in feature_rows
        )
        + (
            "\n\n[Structured matrix](protocol-matrix.json). [Reviewed featu"
            "re decisions](../capabilities/delta.features.md).\n"
        )
    )
    impl_rows = []
    for line in (content / "index/foreign-impls.tsv").read_text().splitlines():
        trait, implementor, nameable, crate = line.split("\t")
        if trait.split("::")[0] in {"std", "core", "alloc", "serde_core"}:
            continue
        family = "delta.kernel" if trait.startswith("buoyant_") else "delta.storage"
        if (
            not trait.startswith(("buoyant_", "datafusion_", "object_store"))
            and "RecordBatchStream" not in trait
        ):
            continue
        entry = (
            "logstore/object-store factories"
            if family == "delta.storage"
            else "kernel snapshot/engine APIs"
        )
        if trait.startswith("datafusion_") or "RecordBatchStream" in trait:
            family, entry = "delta.read", "DeltaTable::table_provider / scan_table"
            if any(t in implementor for t in ["merge::", "Merge"]):
                family, entry = "delta.merge", "DeltaTable::merge"
            elif any(t in implementor for t in ["cdf::", "Cdf"]):
                family, entry = "delta.cdf", "DeltaTable::scan_cdf → DeltaCdfTableProvider"
            elif "ZOrder" in implementor:
                family, entry = "delta.optimize", "DeltaTable::optimize with ZOrder"
            elif any(t in trait for t in ["CatalogProvider", "SchemaProvider"]):
                family, entry = "delta.catalog", "catalog/provider registration"
            elif any(t in implementor for t in ["write::", "Write", "DataSink", "DataValidation"]):
                family, entry = "delta.write", "DeltaTable::write and validation/planning machinery"
        impl_rows.append(
            {
                "trait": trait,
                "implementor": implementor,
                "nameable": nameable == "yes",
                "crate": crate,
                "entry_route": entry,
                "capability": family,
                "route_evidence": (
                    "Authored domain route; implementation identity from rustdoc,"
                    " not a verified call graph"
                ),
                "usage": (
                    "Read trait contract and public entry route; internal impleme"
                    "ntors are not caller extension points"
                ),
            }
        )
    (content / "catalogs/integrations.json").write_text(json.dumps(impl_rows, indent=2) + "\n")
    (content / "catalogs/integrations.md").write_text(
        (
            "# Integration routes\n\nFull impl identity remains in foreign-"
            "impls.tsv. Routes below are authored entry\n"
        )
        + (
            "leads, not an inferred call graph. Nameable does not by itse"
            "lf establish constructibility.\n\n"
        )
        + (
            "| Trait | Implementor | Nameable | Public entry route | Deci"
            "sion |\n|---|---|---|---|---|\n"
        )
        + "\n".join(
            f"| `{r['trait']}` | `{r['implementor']}` | {r['nameable']} | "
            f"{r['entry_route']} | [{r['capability']}](../capabilities/{r['capability']}.md) |"
            for r in impl_rows
        )
        + "\n"
    )
    return {
        "operation_families": len(rows),
        "protocol_vocabulary": len(feature_rows),
        "integration_routes": len(impl_rows),
    }

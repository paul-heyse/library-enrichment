"""Validate reviewed decision records and generate small task/crate/representation routes."""

from __future__ import annotations

import csv
import hashlib
import json
from collections import defaultdict
from pathlib import Path


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(root: Path, content: Path) -> dict[str, int]:
    operations = defaultdict(list)
    for line in (content / "index/operations.tsv").read_text().splitlines():
        path, op_id, kind, crate, record, page, signature = line.split("\t")
        operations[path].append(
            dict(id=op_id, kind=kind, crate=crate, record=record, page=page, signature=signature)
        )
    roles_path = root / "authoring/crate-roles.tsv"
    roles = list(csv.DictReader(roles_path.read_text().splitlines(), delimiter="\t"))
    packages = {
        line.split("\t")[2] for line in (content / "index/symbols.tsv").read_text().splitlines()
    }
    if {r["crate"] for r in roles} != packages or len(roles) != len(packages):
        raise ValueError("Crate routing must cover every pinned package exactly once")
    records = []
    dependencies = {}
    receipt_path = root / "skill_improvement/evidence/implementation/probe-results.json"
    receipt = json.loads(receipt_path.read_text()) if receipt_path.exists() else {}
    passed = set(receipt.get("passed_tests", [])) if receipt.get("state") == "passed" else set()
    for name, expected in receipt.get("sources", {}).items():
        if not (root / name).is_file() or digest(root / name) != expected:
            raise ValueError(f"Probe source changed since execution; rerun probes: {name}")
    for path in sorted((root / "authoring/capabilities").glob("*.json")):
        record = json.loads(path.read_text())
        required = {
            "id",
            "title",
            "task_aliases",
            "crates",
            "input",
            "output",
            "properties",
            "operations",
            "brief",
            "choices",
            "claims",
            "implementation",
            "unknowns",
            "evidence",
        }
        if not required <= record.keys() or not all(record[k] for k in required):
            raise ValueError(f"Incomplete capability: {path}")
        if record["id"] != path.stem or set(record["crates"]) - packages:
            raise ValueError(f"Invalid identity/crates: {path}")
        resolved = []
        dependency_files = {str(path.relative_to(root)): digest(path)}
        for common in ["build/manifests/datafusion.json", "content/index/features.tsv"]:
            dependency_files[common] = digest(root / common)
        for operation in record["operations"]:
            if operation not in operations:
                raise ValueError(f"Missing operation {operation} in {path}")
            for op in operations[operation]:
                resolved.append({"path": operation, **op})
                relative = f"content/{op['record']}"
                dependency_files[relative] = digest(content / op["record"])
        evidence_ids = {e["id"] for e in record["evidence"]}
        claim_ids = [c["id"] for c in record["claims"]]
        if len(set(claim_ids)) != len(claim_ids):
            raise ValueError(f"Duplicate claim id: {path}")
        for claim in record["claims"]:
            if not claim["evidence"] or set(claim["evidence"]) - evidence_ids:
                raise ValueError(f"Unbound claim: {claim}")
        for evidence in record["evidence"]:
            if evidence["kind"] == "runtime_observation":
                if set(evidence["tests"]) - passed:
                    raise ValueError(
                        f"Unexecuted runtime assertion: {record['id']} {evidence['tests']}"
                    )
                dependency_files[evidence["path"]] = digest(root / evidence["path"])
                dependency_files.update(receipt.get("sources", {}))
            if "path" in evidence and not (root / evidence["path"]).is_file():
                raise ValueError(f"Missing evidence path: {evidence}")
        record["resolved_operations"] = resolved
        record["coverage"] = "reviewed decision brief; runtime scope is limited to named tests"
        record["source"] = str(path.relative_to(root))
        records.append(record)
        dependencies[record["id"]] = dependency_files

    out = content / "capabilities"
    out.mkdir(exist_ok=True)
    for record in records:
        (out / f"{record['id']}.json").write_text(json.dumps(record, indent=2) + "\n")
        lines = [
            f"# {record['title']}",
            "",
            record["brief"],
            "",
            f"Reviewed {record['reviewed']}. {record['coverage']}.",
            "",
            "## Choice",
            "",
            "| Candidate | Choose when | Consideration |",
            "|---|---|---|",
        ]
        lines += [
            f"| {c['candidate']} | {c['choose_when']} | {c['tradeoff']} |"
            for c in record["choices"]
        ]
        lines += ["", "## Contract", ""]
        for claim in record["claims"]:
            lines += [
                f"**{claim['aspect']}.** {claim['statement']}",
                f"Claim `{claim['id']}`; {claim['kind']}; "
                f"evidence: {', '.join(claim['evidence'])}.",
                "",
            ]
        lines += ["## Implementation", ""] + [f"- {s}" for s in record["implementation"]]
        lines += ["", "## Limits and unknowns", ""] + [f"- {s}" for s in record["unknowns"]]
        lines += ["", "## Exact contracts", ""]
        lines += [
            f"- [`{op['path']}`](../{op['page']}) — `{op['signature']}`"
            for op in record["resolved_operations"]
        ]
        lines += [
            "",
            "## Evidence",
            "",
            "`upstream` refers to the exact contracts above, "
            "preserving source spans and raw types.",
        ]
        for evidence in record["evidence"]:
            if "path" in evidence:
                lines += [
                    f"- [{evidence['id']}](../../{evidence['path']}): {evidence['scope']}",
                    "  Tests: " + ", ".join(evidence.get("tests", [])),
                ]
        (out / f"{record['id']}.md").write_text("\n".join(lines) + "\n")
    (out / "catalog.json").write_text(json.dumps(records, indent=2) + "\n")
    routes = content / "routes"
    routes.mkdir(exist_ok=True)
    task_lines = [
        "# Task routes",
        "",
        "Reviewed depth is selective. Search the full operation index for unreviewed tasks.",
        "",
        "| Task vocabulary | Decision brief | Input → output |",
        "|---|---|---|",
    ]
    for r in records:
        task_lines.append(
            f"| {'; '.join(r['task_aliases'])} | [{r['title']}](../capabilities/{r['id']}.md) | "
            f"{', '.join(r['input'])} → {', '.join(r['output'])} |"
        )
    (routes / "tasks.md").write_text("\n".join(task_lines) + "\n")
    crate_lines = [
        "# Crate roles",
        "",
        "All pinned packages are routed; a role is not exhaustive semantic characterization.",
        "",
        "| Crate | Role | Representation | Task vocabulary | Reviewed briefs |",
        "|---|---|---|---|---|",
    ]
    for role in roles:
        cards = (
            ", ".join(
                f"[{r['id']}](../capabilities/{r['id']}.md)"
                for r in records
                if role["crate"] in r["crates"]
            )
            or "Full API only"
        )
        crate_lines.append(
            "| "
            + " | ".join(
                [role["crate"], role["role"], role["surface"], role["task_keywords"], cards]
            )
            + " |"
        )
    (routes / "crates.md").write_text("\n".join(crate_lines) + "\n")
    representation = defaultdict(list)
    for r in records:
        for value in set(r["input"] + r["output"]):
            representation[value].append(r)
    lines = ["# Representation routes", "", "| Representation | Reviewed decisions |", "|---|---|"]
    for value, rs in sorted(representation.items()):
        lines.append(
            f"| {value} | "
            + ", ".join(f"[{r['title']}](../capabilities/{r['id']}.md)" for r in rs)
            + " |"
        )
    (routes / "representations.md").write_text("\n".join(lines) + "\n")
    coverage = {
        "packages_routed": len(roles),
        "reviewed_capabilities": len(records),
        "referenced_operations": len({op for r in records for op in r["operations"]}),
        "scope": "Full pinned discovery is distinct from selective reviewed semantic depth.",
        "unreviewed_packages": sorted(packages - {c for r in records for c in r["crates"]}),
    }
    (out / "coverage.json").write_text(json.dumps(coverage, indent=2) + "\n")
    surfaces = defaultdict(list)
    for path, rows in operations.items():
        for op in rows:
            surfaces[op["crate"]].append([path, op["kind"], op["signature"]])
    dependency_state = {
        "records": dependencies,
        "crate_surfaces": {
            crate: hashlib.sha256(json.dumps(sorted(rows)).encode()).hexdigest()
            for crate, rows in surfaces.items()
        },
        "candidate_crates": {r["id"]: r["crates"] for r in records},
    }
    (out / "dependencies.json").write_text(
        json.dumps(dependency_state, indent=2, sort_keys=True) + "\n"
    )
    return {"reviewed_capabilities": len(records), "crate_routes": len(roles)}

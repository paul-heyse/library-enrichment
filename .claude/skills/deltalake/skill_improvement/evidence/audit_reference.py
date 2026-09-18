"""Retain exact-source evidence and measure information lost by the current Delta reader."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import tomllib
from collections import Counter
from compression import zstd
from datetime import UTC, datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL = HERE.parent.parent


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def write(name: str, value: object) -> None:
    (HERE / name).write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    args = parser.parse_args()
    manifest = json.loads((SKILL / "build/manifests/deltalake.json").read_text())
    pin = manifest["crate_sets"][0]["git"]
    head = subprocess.check_output(
        ["git", "-C", str(args.source), "rev-parse", "HEAD"], text=True
    ).strip()
    if head != pin["rev"]:
        raise RuntimeError(f"Expected {pin['rev']}, found {head}")
    acquired = next((SKILL / "build/acquired").glob("delta-rs@*/ACQUISITION.json"))
    acquisition = json.loads(acquired.read_text())
    bad = []
    for name, expected in acquisition["files"].items():
        if digest((acquired.parent / name).read_bytes()) != expected["sha256"]:
            bad.append(name)
    if bad:
        raise RuntimeError(f"Acquisition digest mismatch: {bad}")
    source_records = []
    # Narrow permanent source corpus: source authority for each reviewed planning example.
    paths = ["Cargo.toml", "LICENSE.txt"]
    for pattern in [
        "crates/core/src/operations/**/*.rs",
        "crates/core/src/kernel/transaction/**/*.rs",
        "crates/core/src/delta_datafusion/**/*.rs",
        "crates/core/src/logstore/**/*.rs",
        "crates/core/src/table/*.rs",
        "crates/core/src/kernel/models/actions.rs",
        "crates/core/src/protocol/*.rs",
        "crates/core/src/writer/*.rs",
    ]:
        paths.extend(str(p.relative_to(args.source)) for p in args.source.glob(pattern))
    for relative in sorted(set(paths)):
        data = subprocess.check_output(
            ["git", "-C", str(args.source), "show", f"{head}:{relative}"]
        )
        if data != (args.source / relative).read_bytes():
            raise RuntimeError(f"Working source differs from commit: {relative}")
        destination = HERE / "sources/delta-rs" / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(data)
        source_records.append(
            {
                "path": str(destination.relative_to(HERE)),
                "sha256": digest(data),
                "url": f"{pin['url']}/blob/{head}/{relative}",
            }
        )
    lock_bytes = (args.source / "Cargo.lock").read_bytes()
    if digest(lock_bytes) != acquisition["lock_sha256"]:
        raise RuntimeError("Local source lock does not match the original acquisition")
    (HERE / "acquisition.Cargo.lock").write_bytes(lock_bytes)
    lock = tomllib.loads(lock_bytes.decode())
    selected = [
        p
        for p in lock["package"]
        if p["name"]
        in {
            "datafusion",
            "arrow",
            "arrow-array",
            "parquet",
            "object_store",
            "buoyant_kernel",
            "buoyant_kernel_engine",
            "buoyant_kernel_derive",
        }
    ]
    write(
        "source-manifest.json",
        {
            "verified_at": datetime.now(UTC).isoformat(),
            "pin": pin,
            "source_files": source_records,
            "acquisition_lock_sha256": digest(lock_bytes),
            "selected_locked_packages": selected,
            "acquisition": acquisition,
        },
    )
    items = []
    for p in (SKILL / "content/model").glob("*.json"):
        items.extend(json.loads(p.read_text()).get("items", []))
    raw = json.loads(zstd.decompress((acquired.parent / "deltalake_core.json.zst").read_bytes()))
    private = json.loads(
        zstd.decompress((acquired.parent / "deltalake_core.private.json.zst").read_bytes())
    )
    fragments = []
    targets = {
        "with_predicate",
        "with_schema_mode",
        "with_safe_cast",
        "with_application_transaction",
        "with_dry_run",
        "with_enforce_retention_duration",
        "with_session_state",
        "table_provider",
        "scan_table",
        "with_input_execution_plan",
        "with_streaming",
    }
    for identifier, item in raw["index"].items():
        if item["crate_id"] == 0 and item.get("name") in targets:
            fragments.append(
                {"artifact": "deltalake_core.json.zst", "id": identifier, "item": item}
            )
    for identifier, item in private["index"].items():
        block = item.get("inner", {}).get("impl")
        if not block or not (block.get("trait") or {}).get("path", "").endswith("IntoFuture"):
            continue
        if not any(
            name in str(block.get("for"))
            for name in ["WriteBuilder", "LoadBuilder", "VacuumBuilder", "MergeBuilder"]
        ):
            continue
        fragments.append(
            {
                "artifact": "deltalake_core.private.json.zst",
                "id": identifier,
                "item": item,
                "members": [private["index"][str(i)] for i in block["items"]],
            }
        )
    write("rustdoc-fragments.json", fragments)
    methods = [m for item in items for m in item.get("methods", [])]
    foreign = [
        line.split("\t")
        for line in (SKILL / "content/index/foreign-impls.tsv").read_text().splitlines()
    ]
    seam = {}
    for trait in [
        "ExecutionPlan",
        "DisplayAs",
        "ExtensionPlanner",
        "PruningStatistics",
        "UserDefinedLogicalNodeCore",
        "ScalarUDFImpl",
        "TableProvider",
    ]:
        rows = [r for r in foreign if r[0].split("::")[-1] == trait]
        seam[trait] = rows
    write(
        "inventory.json",
        {
            "recorded_provenance_counts": json.loads(
                (SKILL / "content/PROVENANCE.json").read_text()
            )["counts"],
            "actual_items": len(items),
            "actual_methods": len(methods),
            "method_fields": sorted(set().union(*(set(m) for m in methods))),
            "method_summary_characters": dict(Counter(len(m.get("summary", "")) for m in methods)),
            "raw_core_kinds": dict(
                Counter(next(iter(x["inner"])) for x in raw["index"].values() if x["crate_id"] == 0)
            ),
            "foreign_integrations": seam,
            "interpretation": (
                "Counts establish inventory only; runtime suitability "
                "and cloud support are not inferred."
            ),
        },
    )
    write(
        "baseline-files.json",
        {
            str(p.relative_to(SKILL)): digest(p.read_bytes())
            for p in SKILL.rglob("*")
            if p.is_file()
            and p.relative_to(SKILL).parts[0] != "skill_improvement"
            and not any(part in {".cache", "__pycache__", ".git"} for part in p.parts)
        },
    )
    sys.stdout.write(
        f"Retained {len(source_records)} exact source files; "
        f"audited {len(items)} items and {len(methods)} methods\n"
    )


if __name__ == "__main__":
    main()

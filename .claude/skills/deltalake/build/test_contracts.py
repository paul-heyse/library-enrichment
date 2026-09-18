"""Exact-source preservation and reader behavior checks; run from any working directory."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

import fetch

ROOT = Path(__file__).resolve().parents[1]


def check_preservation(root: Path) -> dict:
    grouped = defaultdict(list)
    ids = set()
    for family in ["operations", "modules"]:
        for file in sorted((root / "content" / family).glob("*.json")):
            for record in json.loads(file.read_text()):
                assert record["id"] not in ids, record["id"]
                ids.add(record["id"])
                grouped[(record["crate"], record["version"])].append(record)
                assert (root / "content" / record["page"]).is_file()
    count = 0
    artifacts = {}
    for records in grouped.values():
        for record in records:
            filename = record["artifact"]["file"]
            if filename not in artifacts:
                matches = [root / filename]
                assert len(matches) == 1, filename
                blob = matches[0].read_bytes()
                import hashlib

                assert hashlib.sha256(blob).hexdigest() == record["artifact"]["compressed_sha256"]
                artifacts[filename] = json.loads(fetch._decompress_zstd(blob))
            raw = artifacts[filename]["index"][record["artifact"]["item_id"]]
            assert record["docs"] == (raw.get("docs") or ""), record["path"]
            assert record["type_tree"] == raw["inner"], record["path"]
            assert record["span"] == raw.get("span"), record["path"]
            assert record["links"] == raw.get("links", {}), record["path"]
            count += 1
    required = {
        "deltalake_core::operations::load::LoadBuilder::Output",
        "deltalake_core::operations::load_cdf::CdfLoadBuilder::build",
        "deltalake_core::operations::merge::MergeBuilder::Output",
        "buoyant_kernel::Engine",
    }
    assert required <= {r["path"] for rs in grouped.values() for r in rs}
    return {"state": "passed", "raw_records_compared": count, "crates": len(grouped)}


def check_reader(root: Path) -> dict:
    def run(*args: str) -> tuple[dict, str]:
        done = subprocess.run(
            [sys.executable, str(root / "scripts/reference.py"), *args],
            cwd="/",
            capture_output=True,
            text=True,
            check=True,
        )
        return json.loads(done.stdout), done.stdout

    a, _ = run(
        "show",
        "deltalake::DeltaTable",
        "--member",
        "load_version",
        "--view",
        "contract",
        "--max-bytes",
        "60000",
    )
    b, _ = run(
        "show",
        "deltalake_core::table::DeltaTable::load_version",
        "--view",
        "contract",
        "--max-bytes",
        "60000",
    )
    assert a["matches"] == b["matches"]
    full, _ = run(
        "show",
        "deltalake_core::operations::write::WriteBuilder::with_input_plan",
        "--view",
        "contract",
        "--max-bytes",
        "60000",
    )
    fragments = []
    offset = 0
    while True:
        part, text = run(
            "show",
            "deltalake_core::operations::write::WriteBuilder::with_input_plan",
            "--view",
            "contract",
            "--max-bytes",
            "1600",
            "--offset",
            str(offset),
        )
        assert len(text.encode()) <= 1600
        fragments.append(part["text_fragment"])
        offset = part["next_offset"]
        if offset is None:
            break
    assert json.loads("".join(fragments)) == full
    cases = [
        ("same application transaction marker replay", "delta.replay"),
        ("post commit error visible version", "delta.commit"),
        ("custom session wrapper UDF fallback", "delta.session"),
        ("CDF inclusive ending version clamp", "delta.cdf"),
        ("vacuum preview historical files", "delta.retention"),
        ("merge null duplicate source keys", "delta.merge"),
        ("partition projection wrong column", "delta.read"),
    ]
    for task, expected in cases:
        found, _ = run("find", "--task", task, "--limit", "3", "--max-bytes", "60000")
        assert expected in {r["id"] for r in found["capabilities"]}, task
    facet, _ = run("find", "--crate", "deltalake-aws", "--max-bytes", "60000")
    assert facet["capabilities"] and all(r["id"] == "delta.storage" for r in facet["capabilities"])
    compared, _ = run("compare", "delta.write", "delta.merge", "--max-bytes", "60000")
    assert len(compared["comparison"]) == 2
    # Every generated local contract link resolves, including its explicit anchor.
    anchors = {}
    linked = 0
    for folder in ["operations", "modules"]:
        for file in (root / "content" / folder).glob("*.json"):
            for record in json.loads(file.read_text()):
                for target in record["resolved_doc_links"].values():
                    relative, anchor = target.split("#", 1)
                    path = (file.parent / relative).resolve()
                    if path not in anchors:
                        anchors[path] = path.read_text()
                    assert f'id="{anchor}"' in anchors[path], target
                    linked += 1
    return {
        "state": "passed",
        "task_routes": len(cases),
        "contract_links": linked,
        "alias_equivalence": True,
        "bounded_lossless_continuation": True,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reader-only", action="store_true")
    args = parser.parse_args()
    results = {"reader": check_reader(ROOT)}
    if not args.reader_only:
        results["preservation"] = check_preservation(ROOT)
    sys.stdout.write(json.dumps(results, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

"""Retain a reproducible inventory and selected exact-release source evidence.

Run with uv run --no-project python evidence/collect_evidence.py from skill_improvement.
Only writes below this evidence directory. No service or host-project imports.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import tarfile
from collections import Counter
from datetime import UTC, datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL = HERE.parent.parent
SOURCES = {
    "arrow-row-59.3.0": ["src/lib.rs"],
    "arrow-select-59.3.0": ["src/filter.rs", "src/take.rs"],
    "arrow-cast-59.3.0": ["src/cast/mod.rs"],
    "arrow-array-59.3.0": ["src/record_batch.rs", "src/array/mod.rs"],
    "arrow-data-59.3.0": ["src/data.rs", "src/transform/mod.rs"],
    "arrow-schema-59.3.0": ["src/field.rs", "src/schema.rs"],
    "datafusion-execution-55.1.0": [
        "src/runtime_env.rs",
        "src/disk_manager.rs",
        "src/memory_pool/mod.rs",
    ],
    "datafusion-session-55.1.0": ["src/table.rs"],
    "datafusion-55.1.0": ["src/dataframe/mod.rs"],
    "datafusion-expr-55.1.0": ["src/udf.rs", "src/udaf.rs"],
    "datafusion-optimizer-55.1.0": ["src/simplify_expressions/expr_simplifier.rs"],
    "parquet-59.3.0": [
        "src/arrow/arrow_reader/mod.rs",
        "src/arrow/arrow_reader/selection/mod.rs",
        "src/arrow/arrow_reader/filter.rs",
    ],
}


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def command(name: str, args: list[str], cwd: Path) -> dict:
    done = subprocess.run(args, cwd=cwd, text=True, capture_output=True, check=False)
    log = HERE / "logs" / f"{name}.log"
    log.parent.mkdir(exist_ok=True)
    log.write_text(done.stdout + done.stderr)
    return {
        "name": name,
        "argv": args,
        "cwd_relative_to_skill": str(cwd.relative_to(SKILL)),
        "exit_code": done.returncode,
        "log": str(log.relative_to(HERE)),
        "log_sha256": digest(log.read_bytes()),
        "note": "rg exit 1 means no match; this is not capability absence",
    }


def main() -> None:
    provenance = json.loads((SKILL / "content/PROVENANCE.json").read_text())
    symbols = [
        line.split("\t") for line in (SKILL / "content/index/symbols.tsv").read_text().splitlines()
    ]
    inventory = {
        "observed_at": datetime.now(UTC).isoformat(),
        "provenance_sha256": digest((SKILL / "content/PROVENANCE.json").read_bytes()),
        "recorded_counts": provenance["counts"],
        "recorded_tools": provenance["tools"],
        "actual_index_rows": {
            path.name: len(path.read_text().splitlines())
            for path in sorted((SKILL / "content/index").glob("*.tsv"))
        },
        "symbols_by_crate": dict(sorted(Counter(row[2] for row in symbols).items())),
        "topic_word_counts": {
            path.name: len(path.read_text().split())
            for path in sorted((SKILL / "content/topics").glob("*.md"))
        },
        "topic_entrypoint_rows": {
            path.name: [
                line
                for line in path.read_text().splitlines()
                if line.startswith("| `") and "[prose]" in line
            ]
            for path in sorted((SKILL / "content/topics").glob("*.md"))
        },
        "navigation_probes": len(json.loads((SKILL / "build/probes.json").read_text())["probes"]),
        "input_hashes": {
            str(path.relative_to(SKILL)): digest(path.read_bytes())
            for path in sorted(SKILL.rglob("*"))
            if path.is_file()
            and "skill_improvement" not in path.parts
            and ".cache" not in path.parts
            and "__pycache__" not in path.parts
        },
    }
    write_json(HERE / "inventory.json", inventory)
    artifacts = []
    for release, selected in SOURCES.items():
        archive = SKILL / "build/.cache/crates" / f"{release}.crate"
        package, version = release.rsplit("-", 1)
        if not archive.exists():
            raise FileNotFoundError(f"Populate skill build cache first: {archive}")
        entry = {
            "package": package,
            "version": version,
            "origin": f"https://static.crates.io/crates/{package}/{release}.crate",
            "acquisition": "existing skill build cache; not a new network retrieval",
            "archive_sha256": digest(archive.read_bytes()),
            "files": [],
        }
        with tarfile.open(archive, "r:gz") as tar:
            available = set(tar.getnames())
            optional = [
                p
                for p in (
                    "Cargo.toml",
                    "Cargo.toml.orig",
                    ".cargo_vcs_info.json",
                    "LICENSE",
                    "LICENSE.txt",
                    "NOTICE",
                    "NOTICE.txt",
                )
                if f"{release}/{p}" in available
            ]
            for relative in selected + optional:
                member = tar.extractfile(f"{release}/{relative}")
                if member is None:
                    raise ValueError(f"Not a file: {release}/{relative}")
                data = member.read()
                target = HERE / "sources" / release / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
                entry["files"].append(
                    {
                        "path": str(target.relative_to(HERE)),
                        "sha256": digest(data),
                        "bytes": len(data),
                    }
                )
        artifacts.append(entry)
    write_json(HERE / "source-manifest.json", artifacts)
    queries = [
        (
            "arrow-discovery",
            [
                "rg",
                "-n",
                "RowConverter|FilterBuilder|filter_record_batch|"
                "take_record_batch|lexsort_to_indices|cast_with_options|MutableArrayData",
                "content/index/symbols.tsv",
            ],
            SKILL,
        ),
        (
            "arrow-topic-discovery",
            [
                "rg",
                "-n",
                "RowConverter|FilterBuilder|filter_record_batch|"
                "take_record_batch|lexsort_to_indices|cast_with_options|"
                "MutableArrayData",
                "content/topics",
            ],
            SKILL,
        ),
        (
            "source-contract-candidates",
            [
                "rg",
                "-n",
                "panics|# Errors|# Panics|not.*stable|"
                "same.*RowConverter|safe:|Inexact|limit.*push|"
                "memory.*limit|not.*enforce",
                "sources",
            ],
            HERE,
        ),
        (
            "runtime-defaults",
            [
                "rg",
                "-n",
                "impl Default|GreedyMemoryPool|UnboundedMemoryPool|"
                "DiskManagerConfig|Disabled|OsTmpDirectory|NewOs|new_specified",
                "sources/datafusion-execution-55.1.0/src",
            ],
            HERE,
        ),
        ("tools", ["ast-grep", "--version"], SKILL),
        ("rg-version", ["rg", "--version"], SKILL),
        ("toolchain", ["rustc", "-Vv"], SKILL),
        (
            "existing-skill-check",
            ["uv", "run", "--no-project", "python", "build/verify.py", "--skip-rebuild"],
            SKILL,
        ),
    ]
    records = [command(name, argv, cwd) for name, argv, cwd in queries]
    write_json(HERE / "commands.json", records)
    print(
        json.dumps(
            {
                "indexes": inventory["actual_index_rows"],
                "source_packages": len(artifacts),
                "commands": [{"name": r["name"], "exit_code": r["exit_code"]} for r in records],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()

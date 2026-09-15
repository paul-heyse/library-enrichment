"""Check retired authorities and the supported native build/deployment graph (Plan 12 DC1).

This is a small removal gate, not a Rust parser or a ban on ordinary domain collections.
Scoped ast-grep rules enforce forbidden code shapes; this checks filesystem/build consumers.
"""

import ast
import json
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    errors = []
    retired = {
        "crates/enrichment-store/src": ("catalog.rs", "snapshot.rs", "tables.rs", "parquet_io.rs"),
        "crates/enrichment-daemon/src/ops": ("source_cache.rs",),
    }
    for directory, names in retired.items():
        for name in names:
            path = ROOT / directory / name
            if path.exists():
                errors.append(f"retired authority exists: {path.relative_to(ROOT)}")
    for crate in ("enrichment-core", "enrichment-daemon"):
        manifest = tomllib.loads((ROOT / "crates" / crate / "Cargo.toml").read_text())
        for dependency in manifest.get("dependencies", {}):
            if dependency in {"datafusion", "parquet", "arrow-json", "duckdb", "rusqlite"}:
                errors.append(f"{crate}: store-owned dependency outside the store: {dependency}")
    for source in (ROOT / "python").rglob("*.py"):
        # Python modules are the adapter/worker, never an embedded evidence engine.
        for node in ast.walk(ast.parse(source.read_text())):
            if not isinstance(node, (ast.Import, ast.ImportFrom)):
                continue
            modules = []
            if isinstance(node, ast.Import):
                modules = [alias.name for alias in node.names]
            elif isinstance(node, ast.ImportFrom) and node.module:
                modules = [node.module]
            if any(
                m.split(".")[0] in {"pyarrow", "datafusion", "duckdb", "sqlite3"} for m in modules
            ):
                errors.append(f"{source.relative_to(ROOT)}:{node.lineno}: Python evidence engine")
    binary = "library-enrichment-native-worker"
    for name in ("justfile", "scripts/launch_configuration.py", "scripts/measure_arrow_fixture.py"):
        text = (ROOT / name).read_text()
        if binary not in text or "library-enrichment-parquet-admission" in text:
            errors.append(f"{name}: deployment must require only the current native worker")
    schemas = ROOT / "schemas/generated"
    envelope = json.loads((schemas / "research-envelope.schema.json").read_text())
    if "pagination" in envelope["properties"] or "delivery" not in envelope["required"]:
        errors.append("active research envelope exposes retired pagination or overflow")
    if envelope["properties"]["schema_version"].get("enum") != ["2.0"]:
        errors.append("active research envelope must emit only 2.0")
    request = json.loads((schemas / "request.schema.json").read_text())
    inspect = request["$defs"]["InspectRequest"]["properties"]
    if "depth" in inspect or "aspects" in inspect or "selection" not in inspect:
        errors.append("active inspection request must expose only typed selection")
    candidate = ROOT / "contracts/research-v2/research-envelope.schema.json"
    if json.loads(candidate.read_text()) != envelope:
        errors.append("candidate research contract differs from its native generated authority")
    for error in errors:
        sys.stderr.write(error + "\n")
    if errors:
        return 1
    result = subprocess.run(["ast-grep", "scan", "crates"], cwd=ROOT, check=False)
    if result.returncode:
        return result.returncode
    sys.stdout.write("architecture-check: native authority and removal boundaries passed\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

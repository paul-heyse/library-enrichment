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
            if dependency in {"duckdb", "rusqlite"}:
                errors.append(f"{crate}: competing execution engine: {dependency}")
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
                modules.extend(f"{node.module}.{alias.name}" for alias in node.names)
            if any(m.split(".")[0] in {"datafusion", "duckdb", "sqlite3"} for m in modules):
                errors.append(f"{source.relative_to(ROOT)}:{node.lineno}: Python evidence engine")
            if any(m.split(".")[0] == "pyarrow" for m in modules):
                if source.relative_to(ROOT).parts[:2] != ("python", "enrichment_worker"):
                    errors.append(f"{source.relative_to(ROOT)}:{node.lineno}: Arrow outside worker")
                if any(
                    m in {"pyarrow.compute", "pyarrow.dataset", "pyarrow.parquet"} for m in modules
                ):
                    errors.append(
                        f"{source.relative_to(ROOT)}:{node.lineno}: worker semantic engine"
                    )
    lock = tomllib.loads((ROOT / "Cargo.lock").read_text())
    pins = {
        "deltalake": "58f07cd62bfbce3649a7e1c87c696288068ae184",
        "buoyant_kernel": "8ba063f8f84fec222000f66d40d70911d7c79675",
        "buoyant_kernel_engine": "8ba063f8f84fec222000f66d40d70911d7c79675",
    }
    packages = {package["name"]: package for package in lock["package"]}
    for name, revision in pins.items():
        if not packages.get(name, {}).get("source", "").endswith("#" + revision):
            errors.append(f"{name}: native execution revision does not match ADR-0042")
    native_versions: dict[str, set[str]] = {}
    for package in lock["package"]:
        name = package["name"]
        if name.startswith(("arrow", "datafusion", "deltalake", "buoyant_kernel")) or name in {
            "parquet",
            "object_store",
        }:
            native_versions.setdefault(name, set()).add(package["version"])
    for name, versions in native_versions.items():
        if len(versions) != 1:
            errors.append(f"{name}: competing native interface versions: {sorted(versions)}")
    for name, version in {
        "datafusion": "55.1.0",
        "arrow": "59.3.0",
        "parquet": "59.3.0",
        "object_store": "0.13.2",
    }.items():
        if native_versions.get(name) != {version}:
            errors.append(f"{name}: native target requires {version}")
    binary = "library-enrichment-native-worker"
    for name in ("justfile", "scripts/launch_configuration.py", "scripts/measure_arrow_fixture.py"):
        text = (ROOT / name).read_text()
        if binary not in text or "library-enrichment-parquet-admission" in text:
            errors.append(f"{name}: deployment must require only the current native worker")
    schemas = ROOT / "schemas/generated"
    envelope = json.loads((schemas / "research-envelope.schema.json").read_text())
    if "pagination" in envelope["properties"] or "delivery" not in envelope["required"]:
        errors.append("active research envelope exposes retired pagination or overflow")
    # The Rust declaration owns the epoch. This gate must not become a second version registry.
    declaration = subprocess.run(
        [
            "ast-grep", "run", "--lang", "rust", "--pattern",
            "pub const SCHEMA_VERSION: &str = wire::SchemaVersion::Current.as_str();",
            "--json=compact",
            "crates/enrichment-core/src/lib.rs",
        ], cwd=ROOT, check=True, text=True, capture_output=True,
    )
    versions = json.loads(declaration.stdout)
    if len(versions) != 1:
        errors.append("core requires one native wire epoch declaration")
    # Reproducible schema generation checks the vocabulary-derived value. This static
    # gate checks the ownership edge, never a second copy of the epoch literal.
    epochs = envelope["properties"]["schema_version"].get("enum")
    if not isinstance(epochs, list) or len(epochs) != 1:
        errors.append("generated research envelope requires one current wire epoch")
    request = json.loads((schemas / "request.schema.json").read_text())
    inspect = request["$defs"]["InspectRequest"]["properties"]
    if "depth" in inspect or "aspects" in inspect or "selection" not in inspect:
        errors.append("active inspection request must expose only typed selection")
    artifact = envelope["$defs"]["ArtifactHandle"]
    if set(artifact["properties"]) != {"receipt", "uri", "description"}:
        errors.append("artifact handles require exact native acquisition receipts")
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

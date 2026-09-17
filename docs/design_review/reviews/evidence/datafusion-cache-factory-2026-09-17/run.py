"""Compile only the probe, selecting matching cached dependencies by Cargo fingerprints."""

import hashlib
import json
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]
HERE = Path(__file__).resolve().parent
DEPS = ROOT / "target/debug/deps"
FINGERPRINTS = ROOT / "target/debug/.fingerprint"
BUILD = ROOT / ".dev-state/plan17/cache-factory-probe"


def sha(path: Path) -> str:
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def artifact(name: str, fingerprint: int) -> Path:
    for marker in FINGERPRINTS.glob(f"{name.replace('_', '-')}-*/lib-{name}"):
        if int.from_bytes(bytes.fromhex(marker.read_text().strip()), "little") == fingerprint:
            suffix = marker.parent.name.rsplit("-", 1)[1]
            extension = "so" if name == "async_trait" else "rlib"
            path = DEPS / f"lib{name}-{suffix}.{extension}"
            if path.is_file():
                return path
    raise RuntimeError(f"No compiled {name} matches fingerprint {fingerprint}")


BUILD.mkdir(parents=True, exist_ok=True)
df = max(DEPS.glob("libdatafusion-*.rlib"), key=lambda p: p.stat().st_mtime)
suffix = df.stem.rsplit("-", 1)[1]
fingerprint = FINGERPRINTS / f"datafusion-{suffix}/lib-datafusion.json"
model = json.loads(fingerprint.read_text())
selected = {"datafusion": df}
for _, name, _, witness in model["deps"]:
    if name in {"tokio", "async_trait", "futures"}:
        selected[name] = artifact(name, witness)
assert selected.keys() == {"datafusion", "tokio", "async_trait", "futures"}
argv = [
    "rustc",
    "--edition=2024",
    str(HERE / "probe.rs"),
    "-L",
    f"dependency={DEPS}",
    "-C",
    "debuginfo=0",
    "-o",
    str(BUILD / "probe"),
]
for name, path in selected.items():
    argv += ["--extern", f"{name}={path}"]
receipt = {
    "started_unix": time.time(),
    "command": argv,
    "toolchain": subprocess.check_output(["rustc", "-Vv"], text=True),
    "source": {
        p.name: sha(p) for p in (HERE / "probe.rs", HERE / "upstream_example.rs", HERE / "run.py")
    },
    "libraries": {name: {"path": str(p), "sha256": sha(p)} for name, p in selected.items()},
    "fingerprint": json.loads(fingerprint.read_text()),
}
with (HERE / "compile.log").open("w") as log:
    compiled = subprocess.run(argv, stdout=log, stderr=subprocess.STDOUT, cwd=ROOT)
receipt["compile_exit"] = compiled.returncode
if compiled.returncode == 0:
    with (HERE / "probe-run.log").open("w") as log:
        executed = subprocess.run(
            [str(BUILD / "probe")], stdout=log, stderr=subprocess.STDOUT, cwd=BUILD, timeout=60
        )
    receipt["run_exit"] = executed.returncode
receipt["finished_unix"] = time.time()
receipt["logs"] = {
    p.name: sha(p) for p in (HERE / "compile.log", HERE / "probe-run.log") if p.exists()
}
(HERE / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
sys.stdout.write(
    json.dumps({k: receipt[k] for k in ("compile_exit", "run_exit") if k in receipt}) + "\n"
)
sys.exit(receipt.get("run_exit", receipt["compile_exit"]))

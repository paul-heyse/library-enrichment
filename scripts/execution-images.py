#!/usr/bin/env python3
"""Build the two producer images an execution profile runs in.

Operator setup, not a gate. It reaches the network, writes into the service-owned execution
root, and builds container images -- so it previews by default and does nothing until `--apply`.

Every input is pinned by digest (`execution-images/inputs.toml`): both bases by manifest digest,
both wheels by sha256, both rustup channel manifests by sha256. A tag would be a mutable
pointer, and an execution image whose contents can change is not a recorded producer identity
(blueprint §6.1, §10).

Building an image does not make a profile operational. `scripts/execution-qualify.py` is what
records an admission receipt, and only an actual containment run writes one.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
import tomllib
import urllib.request
from datetime import UTC, datetime
from pathlib import Path

from cli import say, warn
from execution_broker import broker, broker_env, execution_root, subdirectories

ROOT = Path(__file__).resolve().parent.parent
INPUTS = ROOT / "execution-images" / "inputs.toml"
#: Bounded so a redirected or replaced artifact cannot exhaust memory before its hash is checked.
MAX_DOWNLOAD_BYTES = 128 * 1024 * 1024


def fetch(url: str, expected: str) -> bytes:
    """Download and verify, or fail. The hash is the pin; the URL is only how we got there."""
    request = urllib.request.Request(url, headers={"User-Agent": "library-enrichment-setup"})
    with urllib.request.urlopen(request, timeout=120) as response:
        payload = response.read(MAX_DOWNLOAD_BYTES + 1)
    if len(payload) > MAX_DOWNLOAD_BYTES:
        sys.exit(f"{url} exceeds the {MAX_DOWNLOAD_BYTES}-byte download bound")
    actual = hashlib.sha256(payload).hexdigest()
    if actual != expected:
        sys.exit(f"{url} hashed {actual}, expected {expected}")
    return payload


def wheel_url(name: str, version: str, filename: str) -> str:
    """Resolve a wheel's current download URL from the PyPI JSON API by exact filename."""
    api = f"https://pypi.org/pypi/{name}/{version}/json"
    request = urllib.request.Request(api, headers={"User-Agent": "library-enrichment-setup"})
    with urllib.request.urlopen(request, timeout=60) as response:
        record = json.loads(response.read(8 * 1024 * 1024))
    for entry in record.get("urls", []):
        if entry.get("filename") == filename:
            return str(entry["url"])
    sys.exit(f"PyPI has no file named {filename} for {name} {version}")


def run(argv: list[str], env: dict[str, str], cwd: Path) -> subprocess.CompletedProcess[str]:
    say(f"    $ {' '.join(argv)}")
    return subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, check=False)


def build(spec: dict, name: str, root: Path, staging: Path, epoch: int, date: str) -> dict:
    """Fetch this image's pinned inputs, build it, and report what was actually produced."""
    context = staging / name
    context.mkdir(parents=True, exist_ok=True)
    source = ROOT / "execution-images" / name
    digests: dict[str, str] = {}

    for item in sorted(p.name for p in source.iterdir() if p.is_file()):
        shutil.copy2(source / item, context / item)
        digests[item] = hashlib.sha256((context / item).read_bytes()).hexdigest()

    for wheel in spec.get("wheels", []):
        wheels = context / "wheels"
        wheels.mkdir(exist_ok=True)
        url = wheel_url(wheel["name"], wheel["version"], wheel["file"])
        say(f"    fetching {wheel['file']}")
        (wheels / wheel["file"]).write_bytes(fetch(url, wheel["sha256"]))
        digests[f"wheels/{wheel['file']}"] = wheel["sha256"]
    if (context / "requirements.txt").exists():
        shutil.move(str(context / "requirements.txt"), str(context / "wheels/requirements.txt"))
        digests["wheels/requirements.txt"] = digests.pop("requirements.txt")

    for manifest in spec.get("manifests", []):
        provenance = context / "provenance"
        provenance.mkdir(exist_ok=True)
        say(f"    fetching {manifest['file']}")
        (provenance / manifest["file"]).write_bytes(fetch(manifest["url"], manifest["sha256"]))
        digests[f"provenance/{manifest['file']}"] = manifest["sha256"]

    iidfile = context / "image.id"
    result = run(
        [
            *broker(root),
            "build",
            f"--timestamp={epoch}",
            "--file",
            str(context / "Containerfile"),
            f"--iidfile={iidfile}",
            "--tag",
            f"localhost/libenr-{name}:{date}",
            str(context),
        ],
        broker_env(root),
        context,
    )
    if result.returncode != 0:
        warn(result.stdout[-4000:])
        warn(result.stderr[-4000:])
        sys.exit(f"building the {name} producer image failed")
    image = iidfile.read_text().strip()
    if not image.startswith("sha256:"):
        image = f"sha256:{image}"
    expected = spec.get("recorded_image_id")
    if expected and image != expected:
        # Not an error: upstream base layers are not bit-reproducible, so a rebuild may
        # legitimately differ. It is a fact worth printing rather than hiding.
        say(f"    note: built {image}, differs from the recorded {expected}")
    say(f"    built {name}: {image}")
    return {"image_id": image, "base": spec["base"], "input_digests": digests}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="actually fetch and build")
    parser.add_argument("--root", help="service-owned Podman root (default: <cache>/podman)")
    parser.add_argument("--only", choices=["python", "rust"], help="build one image")
    args = parser.parse_args()

    inputs = tomllib.loads(INPUTS.read_text())
    root = execution_root(args.root)
    date = datetime.now(UTC).strftime("%Y%m%d")
    names = [args.only] if args.only else ["python", "rust"]

    say(f"execution root   {root}")
    say(f"broker           {broker(root)[0]}")
    say(f"source date      {inputs['source_date_epoch']}")
    for name in names:
        spec = inputs[name]
        say(f"\n{name} producer image")
        say(f"  base           {spec['base']}")
        say(f"  recorded id    {spec.get('recorded_image_id', '(none)')}")
        for wheel in spec.get("wheels", []):
            say(f"  wheel          {wheel['file']}  sha256:{wheel['sha256'][:16]}...")
        for manifest in spec.get("manifests", []):
            say(f"  manifest       {manifest['file']}  sha256:{manifest['sha256'][:16]}...")
        say(f"  tag            localhost/libenr-{name}:{date}")

    if not args.apply:
        say(
            "\nPreview only; nothing was fetched, written or built."
            "\nRe-run with --apply to build, then `just execution-qualify` to admit the result."
            "\nBuilding an image does not make an execution profile operational."
        )
        return 0

    for suffix in subdirectories(root):
        (root / suffix).mkdir(parents=True, exist_ok=True)
    staging = root / "build" / date
    staging.mkdir(parents=True, exist_ok=True)

    target = root / "built-images.json"
    # `--only python` must not erase what `--only rust` recorded. The record describes the
    # execution root, not this invocation, and a half-record would make the qualification step
    # silently skip an image rather than fail.
    built = json.loads(target.read_text())["images"] if target.exists() else {}
    for name in names:
        say(f"\nbuilding {name}")
        built[name] = build(inputs[name], name, root, staging, inputs["source_date_epoch"], date)

    record = {
        "built_at": datetime.now(UTC).isoformat(timespec="seconds"),
        "execution_root": str(root),
        "source_date_epoch": inputs["source_date_epoch"],
        "images": built,
        "note": (
            "Built images only. Readiness is recorded by scripts/execution-qualify.py after an "
            "actual containment run; configuration alone never establishes an operational profile."
        ),
    }
    target.write_text(json.dumps(record, indent=1, sort_keys=True) + "\n")
    say(f"\nrecorded {target}")
    say("Next: just execution-qualify --apply")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

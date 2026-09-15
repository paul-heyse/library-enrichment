#!/usr/bin/env python3
"""Qualify built producer images and record an admission receipt.

Configuring an image never makes an execution profile operational. This is the step that does,
and it earns the claim the only way it can be earned: by running the real containment tests
against the real images and recording what actually happened.

Previews by default. `--apply` probes each image's tool identities inside the same containment
the service uses, runs the ignored real-container test tier, and -- only if every step
succeeded -- writes `admitted-images.json` beside the images in the execution root. A failure
writes nothing, so a stale receipt can never outlive the images it describes.

The receipt is written beside the images it qualifies. `service.status` reads it and reports
`usage-verification` and `ty` as available only when it covers the configured image IDs.
"""

from __future__ import annotations

import argparse
import fcntl
import json
import os
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path

from cli import say, warn
from execution_broker import description, execution_root, probe

ROOT = Path(__file__).resolve().parent.parent

#: The real containment tier. Ignored by default precisely because it needs these images.
CONTAINMENT = [
    "cargo",
    "test",
    "--locked",
    "-p",
    "enrichment-daemon",
    "--test",
    "execution_boundary",
    "--test",
    "execution_cleanup",
    "--",
    "--ignored",
    "--test-threads=1",
]


def configured_images(root: Path, args: argparse.Namespace) -> dict[str, str]:
    """The images to qualify: explicit flags, else whatever `just execution-images` built."""
    images = {}
    if args.python_image:
        images["python"] = args.python_image
    if args.rust_image:
        images["rust"] = args.rust_image
    if images:
        return images
    built = root / "built-images.json"
    if not built.exists():
        sys.exit(
            f"no built images at {built}; run `just execution-images --apply` first, "
            "or pass --python-image/--rust-image"
        )
    record = json.loads(built.read_text())
    return {name: spec["image_id"] for name, spec in record["images"].items()}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="actually probe, test and record")
    parser.add_argument("--root", help="service-owned Podman root (default: <cache>/podman)")
    parser.add_argument("--python-image", help="qualify this Python image instead of the built one")
    parser.add_argument("--rust-image", help="qualify this Rust image instead of the built one")
    parser.add_argument("--profile", choices=("debug", "release"), default="debug")
    args = parser.parse_args()

    root = execution_root(args.root)
    # Beside the images it qualifies, not under a data root: qualification is a property of an
    # image in an execution root. Every daemon configured for this root finds the same receipt;
    # one pointed elsewhere inherits nothing, which is correct.
    receipt = root / "admitted-images.json"

    if args.apply:
        root.mkdir(parents=True, exist_ok=True)
        lock_fd = os.open(
            root / ".qualification.lock", os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600
        )
        with os.fdopen(lock_fd, "r+b") as lock:
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            receipt.unlink(missing_ok=True)
            root_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            try:
                os.fsync(root_fd)
            finally:
                os.close(root_fd)
            return apply_qualification(root, configured_images(root, args), receipt, args.profile)

    images = configured_images(root, args)
    say(f"execution root   {root}")
    say(f"receipt          {receipt}")
    for name, image in sorted(images.items()):
        say(f"{name:<16} {image}")
        for item in description(root, profile=args.profile)["probes"][name]:
            say(f"  probe {item['tool']:<12} expecting {item['expected']!r}")
    say(f"containment      {' '.join(containment_command(args.profile))}")

    say(
        "\nPreview only; nothing was probed, run or recorded."
        "\nRe-run with --apply to qualify. Until a receipt exists, service.status reports"
        "\nexecution as configured-but-not-qualified, and that is the truthful answer."
    )
    return 0


def containment_command(profile: str) -> list[str]:
    return CONTAINMENT[:2] + (["--release"] if profile == "release" else []) + CONTAINMENT[2:]


def apply_qualification(
    root: Path, images: dict[str, str], receipt: Path, profile: str = "debug"
) -> int:
    # Invalidate readiness before any attempted build/probe. A failed requalification
    # cannot leave an older receipt claiming that this attempt succeeded.
    built = subprocess.run(
        [
            "cargo",
            "build",
            "--locked",
            "-p",
            "enrichment-daemon",
            "-p",
            "enrichment-core",
            "--bins",
        ]
        + (["--release"] if profile == "release" else []),
        cwd=ROOT,
        check=False,
    )
    if built.returncode:
        sys.exit("the daemon/helper did not build; no qualification receipt remains")
    contract = description(root, profile=profile)
    if not contract["containment_identity"]:
        sys.exit(contract["qualification_unavailable"])

    observed = {}
    for name, image in sorted(images.items()):
        say(f"\nprobing {name} {image}")
        observed[name] = probe(root, name, image, profile=profile)

    command = containment_command(profile)
    say(f"\nrunning the real containment tier\n    $ {' '.join(command)}")
    env = dict(os.environ)
    env["LIBENR_EXECUTION_TEST_ROOT"] = str(root)
    if "python" in images:
        env["LIBENR_EXECUTION_TEST_PYTHON"] = images["python"]
    if "rust" in images:
        env["LIBENR_EXECUTION_TEST_RUST"] = images["rust"]
    result = subprocess.run(command, env=env, cwd=ROOT, check=False)
    if result.returncode != 0:
        warn("\nthe containment tier failed; no admission receipt was written")
        warn("an image that cannot be contained is not qualified, however well it was built")
        return 1

    if (
        description(root, profile=profile)["containment_identity"]
        != contract["containment_identity"]
    ):
        sys.exit("execution contract changed during qualification; no receipt written")
    receipt.parent.mkdir(parents=True, exist_ok=True)
    import tempfile

    with tempfile.NamedTemporaryFile(mode="w", dir=root, delete=False) as temporary:
        temporary.write(
            json.dumps(
                {
                    "qualified_at": datetime.now(UTC).isoformat(timespec="seconds"),
                    "execution_root": str(root),
                    "images": images,
                    "tools": {
                        name: {
                            tool: result["stdout"].strip()
                            for tool, result in probes["tools"].items()
                        }
                        for name, probes in observed.items()
                    },
                    "resources": {name: probes["resources"] for name, probes in observed.items()},
                    "observations": observed,
                    "containment_identity": contract["containment_identity"],
                    "containment_command": command,
                    "build_profile": profile,
                    "broker": contract["broker"]["program"],
                    "configuration": contract["configuration"],
                    "note": (
                        "Written only after the real containment tier passed against these exact "
                        "image IDs. It qualifies these images in this execution root; it makes no "
                        "claim about any other image, root or platform."
                    ),
                },
                indent=1,
                sort_keys=True,
            )
            + "\n"
        )
        temporary.flush()
        os.fsync(temporary.fileno())
    os.replace(temporary.name, receipt)
    root_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(root_fd)
    finally:
        os.close(root_fd)
    say(f"\nrecorded {receipt}")
    say("service.status will now report execution as qualified for these image IDs.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

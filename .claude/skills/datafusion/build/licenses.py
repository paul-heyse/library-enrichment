"""Retain license and notice files from every exact published crate archive."""

from __future__ import annotations

import hashlib
import json
import tarfile
import tomllib
from pathlib import Path, PurePosixPath


def write(pairs: list[tuple[str, str]], cache: Path, content: Path) -> None:
    out = content / "licenses"
    out.mkdir(exist_ok=True)
    records = []
    for name, version in pairs:
        stem = f"{name}-{version}"
        archive = cache / "crates" / f"{stem}.crate"
        files = []
        with tarfile.open(archive) as bundle:
            cargo = bundle.extractfile(f"{stem}/Cargo.toml")
            if cargo is None:
                raise ValueError(f"Missing manifest in {stem}")
            declared = tomllib.loads(cargo.read().decode())["package"].get("license")
            for member in bundle:
                leaf = PurePosixPath(member.name).name
                if member.isfile() and leaf.startswith(("LICENSE", "NOTICE", "COPYING")):
                    source = bundle.extractfile(member)
                    if source is None:
                        raise ValueError(member.name)
                    data = source.read()
                    relative = PurePosixPath(member.name)
                    if relative.is_absolute() or ".." in relative.parts:
                        raise ValueError(member.name)
                    dest = out / str(relative)
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    dest.write_bytes(data)
                    files.append(
                        {"file": str(relative), "sha256": hashlib.sha256(data).hexdigest()}
                    )
        records.append(
            {
                "package": name,
                "version": version,
                "origin": f"https://crates.io/api/v1/crates/{name}/{version}/download",
                "archive_sha256": hashlib.sha256(archive.read_bytes()).hexdigest(),
                "files": files,
                "declared_license": declared,
                "state": "retained" if files else "not_in_archive",
            }
        )
    (out / "manifest.json").write_text(json.dumps(records, indent=2) + "\n")

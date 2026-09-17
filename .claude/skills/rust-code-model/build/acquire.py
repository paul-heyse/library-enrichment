"""Acquire every pinned input. The only stage that touches the network.

Two destinations, on one criterion -- whether anyone else could re-serve the bytes.

    build/.cache/     raw downloads, gitignored. Re-fetchable from the pin at any time.
    build/acquired/   the extracted inputs build.py actually reads, committed. This is what
                      makes `build.py` offline and the determinism check meaningful: a rebuild
                      reads bytes that are in the tree, not bytes a server chose to send today.

`build.py` never imports this module. If acquisition could run inside the build, a rebuild
could pick up different bytes from the network and still call itself reproducible.

Standard library only.
"""

from __future__ import annotations

import json
import shutil
import sys
from pathlib import Path

import fetch

HERE = Path(__file__).resolve().parent
MANIFEST = HERE / "manifests" / "rust-code-model.json"
CACHE = HERE / ".cache"
ACQUIRED = HERE / "acquired"


class AcquisitionError(RuntimeError):
    """A pinned input could not be acquired, or arrived describing something else."""


def _log(message: str) -> None:
    sys.stdout.write(f"{message}\n")
    sys.stdout.flush()


def load_manifest() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


# --------------------------------------------------------------------------- crates


def crate_specs(manifest: dict) -> list[dict]:
    """Flatten every crate set into one record per crate, carrying its set's pin."""
    specs: list[dict] = []
    for crate_set in manifest["crate_sets"]:
        for entry in crate_set["crates"]:
            spec = {"package": entry} if isinstance(entry, str) else dict(entry)
            spec["version"] = crate_set["version"]
            spec["crate_set"] = crate_set["name"]
            spec["lib"] = spec.get("lib") or spec["package"].replace("-", "_")
            specs.append(spec)
    return specs


def acquire_crates(manifest: dict, cache: fetch.Cache) -> dict[str, dict]:
    """Fetch hosted rustdoc JSON and the published Cargo.toml for every pinned crate.

    The compressed payload is stored verbatim rather than re-compressed. Re-compressing would
    make the committed bytes depend on the local zstd build, which is exactly the kind of
    environmental coupling the determinism check exists to catch.
    """
    records: dict[str, dict] = {}
    rustdoc_dir = ACQUIRED / "rustdoc"
    manifest_dir = ACQUIRED / "manifests"
    rustdoc_dir.mkdir(parents=True, exist_ok=True)
    manifest_dir.mkdir(parents=True, exist_ok=True)

    for spec in crate_specs(manifest):
        package, version = spec["package"], spec["version"]
        compressed = cache.read_or_fetch(
            ("rustdoc", f"{package}-{version}.json.zst"),
            fetch.DOCS_RS.format(name=package, version=version),
        )
        payload = fetch.decompress_zstd(compressed)
        document = json.loads(payload)

        served = document.get("crate_version")
        if served != version:
            raise AcquisitionError(
                f"{package}: asked docs.rs for {version} and it served {served!r}. The pin and "
                f"the payload disagree, so nothing downstream would describe the pinned release."
            )

        (rustdoc_dir / f"{package}@{version}.json.zst").write_bytes(compressed)
        (manifest_dir / f"{package}.Cargo.toml").write_text(
            fetch.crate_manifest(cache, package, version), encoding="utf-8"
        )

        records[package] = {
            "package": package,
            "lib": spec["lib"],
            "version": version,
            "crate_set": spec["crate_set"],
            "layer": spec.get("layer", "unassigned"),
            "role": spec.get("role", "primary"),
            # Recorded per crate, never assumed. docs.rs builds on its own schedule and its
            # fleet is not uniform: the crate that DEFINES format 61 is itself served at 60.
            "format_version": document.get("format_version"),
            "index_items": len(document.get("index", {})),
            "sha256": fetch.digest(payload),
            "compressed_sha256": fetch.digest(compressed),
            "compressed_bytes": len(compressed),
        }
        _log(
            f"  {package} {version}: format {records[package]['format_version']}, "
            f"{records[package]['index_items']} index items"
        )
    return records


def assert_family_uniform(records: dict[str, dict], manifest: dict) -> None:
    """Every crate in one set must carry that set's single version.

    A mixed-version ra_ap set would describe a library that never existed: the crates are
    released in lockstep and freely make breaking changes to each other between releases.
    """
    for crate_set in manifest["crate_sets"]:
        versions = {
            records[c["package"] if isinstance(c, dict) else c]["version"]
            for c in crate_set["crates"]
        }
        if len(versions) != 1:
            raise AcquisitionError(
                f"crate set {crate_set['name']} resolved to {sorted(versions)}; it must be one "
                f"version. These crates make breaking changes to each other between releases."
            )


# --------------------------------------------------------------------------- pinned source


def acquire_sources(manifest: dict, cache: fetch.Cache) -> dict[str, dict]:
    """Fetch individual files from each pinned repository ref."""
    records: dict[str, dict] = {}
    for source in manifest["sources"]:
        repo, ref, dest = source["repo"], source["ref"], source["dest"]
        target = ACQUIRED / "source" / dest
        target.mkdir(parents=True, exist_ok=True)
        files: dict[str, str] = {}
        for local_name, path in source["files"].items():
            payload = fetch.raw_file(cache, repo, ref, path)
            (target / local_name).write_bytes(payload)
            files[local_name] = fetch.digest(payload)
        records[source["name"]] = {
            "repo": repo,
            "ref": ref,
            "ref_kind": source["ref_kind"],
            "files": files,
        }
        _log(f"  {repo}@{ref}: {len(files)} file(s)")
    return records


# --------------------------------------------------------------------------- entry point


def main() -> int:
    manifest = load_manifest()
    cache = fetch.Cache(CACHE)

    if ACQUIRED.exists():
        shutil.rmtree(ACQUIRED)
    ACQUIRED.mkdir(parents=True)

    _log("rustdoc JSON and crate manifests:")
    crates = acquire_crates(manifest, cache)
    assert_family_uniform(crates, manifest)

    _log("pinned source:")
    sources = acquire_sources(manifest, cache)

    formats = sorted({r["format_version"] for r in crates.values()})
    supported = manifest["tools"]["rustdoc_format_supported"]
    unsupported = [f for f in formats if f not in supported]
    if unsupported:
        raise AcquisitionError(
            f"docs.rs served rustdoc format version(s) {unsupported}, which model.py does not "
            f"parse. Supported: {supported}. An unsupported version parses far enough to "
            f"produce confident nonsense, so this is a stop rather than a warning."
        )

    (ACQUIRED / "ACQUISITION.json").write_text(
        json.dumps(
            {
                "repository": manifest["repository"],
                "tools": manifest["tools"],
                "crates": crates,
                "sources": sources,
                "format_versions_served": formats,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    _log(f"acquired {len(crates)} crate(s), {len(sources)} pinned source ref(s)")
    _log(f"rustdoc format versions served by docs.rs: {formats}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AcquisitionError, fetch.FetchError) as error:
        sys.stderr.write(f"acquisition failed: {error}\n")
        raise SystemExit(1) from error

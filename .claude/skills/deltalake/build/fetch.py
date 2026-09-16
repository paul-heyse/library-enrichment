"""Acquire pinned upstream inputs: rustdoc JSON, crate manifests, and repository corpora.

Two source kinds, one seam. A `docs_rs` crate set downloads hosted rustdoc JSON keyed by a
published version. A `git` crate set reads rustdoc JSON that `acquire.py` produced locally from
a pinned commit, because an unpublished commit has no hosted documentation. Both return raw
bytes to the same `model.build_crate` call, which asks nothing about provenance.

Standard library only. Zstd comes from `compression.zstd` on Python 3.14+, and falls back to
the `zstd` command-line tool elsewhere. Everything is cached under `.cache/` keyed by the exact
pinned version, so a rebuild after the first run performs no network I/O.
"""

from __future__ import annotations

import gzip
import hashlib
import io
import shutil
import subprocess
import tarfile
import urllib.error
import urllib.request
from pathlib import Path

USER_AGENT = "rust-capability-skill-builder (+https://github.com/delta-io/delta-rs)"
DOCS_RS = "https://docs.rs/crate/{name}/{version}/json"
CRATES_IO_DOWNLOAD = "https://crates.io/api/v1/crates/{name}/{version}/download"
# A tag and a commit take different archive paths, and a commit-pinned corpus is the normal
# case once a crate set is sourced from git rather than from a release.
GITHUB_TARBALL = "https://github.com/{repo}/archive/refs/tags/{ref}.tar.gz"
GITHUB_TARBALL_REF = "https://github.com/{repo}/archive/{ref}.tar.gz"

MAX_DOWNLOAD_BYTES = 512 * 1024 * 1024


class FetchError(RuntimeError):
    """A pinned input could not be acquired."""


def _decompress_zstd(payload: bytes) -> bytes:
    try:
        from compression import zstd  # Python 3.14+
    except ImportError:
        pass
    else:
        return zstd.decompress(payload)

    if shutil.which("zstd") is None:
        raise FetchError(
            "zstd decompression needs Python 3.14+ (compression.zstd) or the zstd CLI on PATH"
        )
    done = subprocess.run(["zstd", "-dc"], input=payload, capture_output=True, check=False)
    if done.returncode != 0:
        raise FetchError(f"zstd failed: {done.stderr.decode('utf-8', 'replace')[:200]}")
    return done.stdout


def _get(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            payload = response.read(MAX_DOWNLOAD_BYTES + 1)
    except urllib.error.HTTPError as error:
        raise FetchError(f"{url} -> HTTP {error.code}") from error
    except urllib.error.URLError as error:
        raise FetchError(f"{url} -> {error.reason}") from error
    if len(payload) > MAX_DOWNLOAD_BYTES:
        raise FetchError(f"{url} exceeded {MAX_DOWNLOAD_BYTES} bytes")
    return payload


class Cache:
    """Content cache for pinned inputs. A cached entry is never revalidated: the key is a pin."""

    def __init__(self, root: Path) -> None:
        self.root = root
        self.root.mkdir(parents=True, exist_ok=True)

    def path(self, *parts: str) -> Path:
        target = self.root.joinpath(*parts)
        target.parent.mkdir(parents=True, exist_ok=True)
        return target

    def read_or_fetch(self, key: tuple[str, ...], url: str) -> bytes:
        target = self.path(*key)
        if target.exists():
            return target.read_bytes()
        payload = _get(url)
        target.write_bytes(payload)
        return payload


def rustdoc_json(cache: Cache, name: str, version: str) -> bytes:
    """Return the decompressed rustdoc JSON document for one pinned crate release."""
    compressed = cache.read_or_fetch(
        ("rustdoc", f"{name}-{version}.json.zst"),
        DOCS_RS.format(name=name, version=version),
    )
    return _decompress_zstd(compressed)


def crate_manifest(cache: Cache, name: str, version: str) -> str:
    """Return the Cargo.toml text from the published .crate tarball."""
    archive = cache.read_or_fetch(
        ("crates", f"{name}-{version}.crate"),
        CRATES_IO_DOWNLOAD.format(name=name, version=version),
    )
    wanted = f"{name}-{version}/Cargo.toml"
    with tarfile.open(fileobj=io.BytesIO(gzip.decompress(archive))) as bundle:
        member = bundle.extractfile(wanted)
        if member is None:
            raise FetchError(f"{wanted} absent from the published tarball")
        return member.read().decode("utf-8", "replace")


def repo_files(
    cache: Cache,
    repo: str,
    ref: str,
    prefix: str,
    suffixes: tuple[str, ...],
    ref_kind: str = "tag",
) -> dict[str, bytes]:
    """Return a mapping of path-relative-to-prefix to bytes for one directory of a repo."""
    slug = repo.replace("/", "_")
    template = GITHUB_TARBALL if ref_kind == "tag" else GITHUB_TARBALL_REF
    archive = cache.read_or_fetch(
        ("repos", f"{slug}-{ref}.tar.gz"),
        template.format(repo=repo, ref=ref),
    )
    found: dict[str, bytes] = {}
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:gz") as bundle:
        for member in bundle:
            if not member.isfile():
                continue
            # Strip the single "<repo>-<ref>" root directory the archive wraps everything in.
            _, _, relative = member.name.partition("/")
            if not relative.startswith(prefix):
                continue
            if suffixes and not relative.endswith(suffixes):
                continue
            handle = bundle.extractfile(member)
            if handle is not None:
                found[relative[len(prefix) :]] = handle.read()
    return found


def digest(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


class AcquisitionMissing(FetchError):
    """A git-sourced crate set has not been acquired, or was acquired under a different pin."""


def acquisition(directory: Path) -> dict:
    """Read the record `acquire.py` wrote, which is also the integrity manifest."""
    record = directory / "ACQUISITION.json"
    if not record.exists():
        raise AcquisitionMissing(
            f"{directory.name} has not been acquired. Run:\n"
            f"    python3 build/acquire.py --manifest <manifest>\n"
            f"which needs a network, git, cargo and the pinned nightly. "
            f"Every later stage runs offline."
        )
    import json

    return json.loads(record.read_text())


def rustdoc_json_local(directory: Path, record: dict, lib: str) -> bytes:
    """Return rustdoc JSON that `acquire.py` produced, after checking it is the same bytes.

    The docs.rs path treats a cache hit as authoritative because the key is a published pin that
    cannot change. Locally produced bytes have no such guarantee -- nothing stops an edit or a
    truncated write -- so they are verified against the digest recorded at acquisition time.
    """
    name = f"{lib}.json.zst"
    entry = (record.get("files") or {}).get(name)
    if entry is None:
        raise AcquisitionMissing(f"{name} is not in {directory.name}/ACQUISITION.json")
    blob = (directory / name).read_bytes()
    actual = hashlib.sha256(blob).hexdigest()
    if actual != entry["sha256"]:
        raise FetchError(
            f"{name} does not match its recorded digest "
            f"({actual[:12]} vs {entry['sha256'][:12]}); re-run acquire.py"
        )
    return _decompress_zstd(blob)


def rustdoc_supplement_local(directory: Path, record: dict, lib: str) -> bytes | None:
    """The optional `--document-private-items` capture, if acquisition produced one."""
    name = f"{lib}.private.json.zst"
    if name not in (record.get("files") or {}):
        return None
    return rustdoc_json_local(directory, record, f"{lib}.private")


def crate_manifest_local(directory: Path, package: str) -> str:
    """Return the Cargo.toml `acquire.py` copied out of the pinned checkout.

    A git-pinned crate has no published tarball to read the feature table from, and feature
    gating is the one fact rustdoc JSON never carries.
    """
    path = directory / "manifests" / f"{package}.Cargo.toml"
    if not path.exists():
        raise AcquisitionMissing(f"{package}.Cargo.toml absent from {directory.name}/manifests")
    return path.read_text()


def parse_manifest_facts(text: str, workspace: str | None = None) -> dict:
    """Extract the feature table and the docs.rs build configuration from a Cargo.toml.

    Two facts matter downstream and neither can be derived from rustdoc JSON. Feature gating is
    absent from the item walk entirely -- formats 57 to 61 record only an opaque marker, never
    `cfg(feature = ...)` -- so `[features]` is the only source. And whether the hosted
    documentation covers the full surface depends on `all-features`, which is a per-crate choice
    rather than a docs.rs default.
    """
    import tomllib

    try:
        document = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        return {"error": f"unparsable manifest: {error}"}

    package = document.get("package") or {}
    docs_rs = ((package.get("metadata") or {}).get("docs") or {}).get("rs") or {}
    features = document.get("features") or {}

    # A workspace member writes `rust-version.workspace = true`, which parses to a dict. Left
    # alone it would land in the provenance record as `{"workspace": true}` -- a value that
    # looks like data but says nothing. Resolve it against the workspace root when we have one.
    rust_version = package.get("rust-version")
    if isinstance(rust_version, dict):
        rust_version = None
        if workspace:
            try:
                root = tomllib.loads(workspace)
            except tomllib.TOMLDecodeError:
                root = {}
            rust_version = ((root.get("workspace") or {}).get("package") or {}).get("rust-version")

    optional_dependencies = sorted(
        name
        for name, spec in (document.get("dependencies") or {}).items()
        if isinstance(spec, dict) and spec.get("optional")
    )

    return {
        "features": {name: sorted(enables) for name, enables in sorted(features.items())},
        "default_features": sorted(features.get("default", [])),
        "optional_dependencies": optional_dependencies,
        "docs_rs": {
            "declared": bool(docs_rs),
            "all_features": bool(docs_rs.get("all-features")),
            "no_default_features": bool(docs_rs.get("no-default-features")),
            "features": sorted(docs_rs.get("features", [])),
            "default_target": docs_rs.get("default-target"),
        },
        "rust_version": rust_version,
    }

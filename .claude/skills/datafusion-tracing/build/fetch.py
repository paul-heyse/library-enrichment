"""Transport for pinned upstream inputs: rustdoc JSON, crate manifests, and named source files.

Every entry point takes a pin and returns bytes. Nothing here decides what to fetch; the
manifest does that, and `acquire.py` drives it.

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

USER_AGENT = (
    "datafusion-tracing-skill-builder "
    "(+https://github.com/datafusion-contrib/datafusion-tracing)"
)
DOCS_RS = "https://docs.rs/crate/{name}/{version}/json"
CRATES_IO_DOWNLOAD = "https://crates.io/api/v1/crates/{name}/{version}/download"
# Single files at a pinned ref. Fetching an archive is right for
# a whole corpus and impossible for rust-lang/rust (several hundred MB) when the build wants
# seven source files out of it. The ref must be a commit or tag, never a branch: a branch
# name would silently re-point and the digest recorded in PROVENANCE would stop meaning anything.
GITHUB_RAW = "https://raw.githubusercontent.com/{repo}/{ref}/{path}"

MAX_DOWNLOAD_BYTES = 512 * 1024 * 1024


class FetchError(RuntimeError):
    """A pinned input could not be acquired."""


def decompress_zstd(payload: bytes) -> bytes:
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


def compress_zstd(payload: bytes, level: int = 19) -> bytes:
    """Compress a locally produced document for committing.

    Only the private capture needs this. The docs.rs payload is stored exactly as served, so
    that half of the acquisition never depends on which zstd this machine has. This half has no
    upstream form to preserve, so the level is fixed rather than left to a default that could
    differ between the library and the CLI fallback.
    """
    try:
        from compression import zstd  # Python 3.14+
    except ImportError:
        pass
    else:
        return zstd.compress(payload, level=level)

    if shutil.which("zstd") is None:
        raise FetchError(
            "zstd compression needs Python 3.14+ (compression.zstd) or the zstd CLI on PATH"
        )
    done = subprocess.run(
        ["zstd", f"-{level}", "-c"], input=payload, capture_output=True, check=False
    )
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
    return decompress_zstd(compressed)


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


def raw_file(cache: Cache, repo: str, ref: str, path: str) -> bytes:
    """Return one file from a repository at a pinned commit, without downloading the archive.

    The cache key carries the ref, so two builds pinned to different commits never collide and
    a cached file is never revalidated -- the commit *is* the validation.
    """
    slug = repo.replace("/", "_")
    return cache.read_or_fetch(
        ("raw", f"{slug}@{ref}", path.replace("/", "__")),
        GITHUB_RAW.format(repo=repo, ref=ref, path=path),
    )


def digest(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


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


# --------------------------------------------------------------------------- this subject

CRATES_IO_VERSIONS = "https://crates.io/api/v1/crates/{name}/versions"
GITHUB_TARBALL_TAG = "https://github.com/{repo}/archive/refs/tags/{ref}.tar.gz"
GITHUB_COMMIT = "https://api.github.com/repos/{repo}/commits/{ref}"


def crates_io_versions(cache: Cache, name: str) -> list[dict]:
    """Every published release of one crate, newest first.

    The compatibility matrix is a claim about release history, so it is read from the registry
    rather than recalled. Cached like everything else -- but note the key is the crate name, not
    a pin, so this is the one entry here that can go stale. `acquire.py` is the only caller and
    records the retrieval date beside the result.
    """
    import json

    payload = cache.read_or_fetch(("registry", f"{name}.versions.json"),
                                  CRATES_IO_VERSIONS.format(name=name))
    return json.loads(payload)["versions"]


def resolve_commit(cache: Cache, repo: str, ref: str) -> str:
    """Resolve a tag to the commit it pointed at, so the corpus pin is recorded as a SHA.

    A tag is a pin only as long as nobody moves it. Recording the resolved commit turns the
    corpus into something a later reader can check rather than trust.
    """
    import json

    payload = cache.read_or_fetch(("registry", f"{repo.replace('/', '_')}-{ref}.commit.json"),
                                  GITHUB_COMMIT.format(repo=repo, ref=ref))
    return json.loads(payload)["sha"]


def repo_files(
    cache: Cache,
    repo: str,
    ref: str,
    prefix: str,
    suffixes: tuple[str, ...] = (),
) -> dict[str, bytes]:
    """Return {path-relative-to-prefix: bytes} for one directory of a tagged repository."""
    slug = repo.replace("/", "_")
    archive = cache.read_or_fetch(
        ("repos", f"{slug}-{ref}.tar.gz"),
        GITHUB_TARBALL_TAG.format(repo=repo, ref=ref),
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
                found[relative[len(prefix):]] = handle.read()
    return found


def extract_crate(cache: Cache, name: str, version: str, destination: Path) -> Path:
    """Unpack a published .crate into `destination/<name>-<version>/` and return that path.

    The private-items capture documents the published source as a standalone package rather
    than as a dependency of something else. Documenting it as a dependency would leave
    `--document-private-items` applying to the wrong crate, which fails silently: the document
    is produced, parses, and simply lacks the items the whole capture exists to recover.
    """
    archive = cache.read_or_fetch(
        ("crates", f"{name}-{version}.crate"),
        CRATES_IO_DOWNLOAD.format(name=name, version=version),
    )
    root = destination / f"{name}-{version}"
    if root.exists():
        shutil.rmtree(root)
    destination.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(gzip.decompress(archive)), mode="r:") as bundle:
        bundle.extractall(destination, filter="data")
    if not root.is_dir():
        raise FetchError(f"{name}-{version}.crate did not unpack to {root.name}/")
    return root

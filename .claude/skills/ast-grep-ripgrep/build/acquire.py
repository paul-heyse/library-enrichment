"""Fetch every pinned upstream input. The only stage that touches the network.

`build.py` never imports this module. That separation is what lets `verify.py` rebuild the whole
repository byte-for-byte and compare: if acquisition could run inside the build, a rebuild could
silently pick up different bytes and still look deterministic.

Two destinations, on one criterion -- whether anyone else could re-serve the bytes.

* `.cache/` is gitignored. Raw tarballs and rustdoc archives live here, keyed by exact pin and
  never revalidated, because the key *is* the pin.
* `acquired/` is committed. It holds the extracted inputs the offline build actually reads, so a
  clone can rebuild without reaching docs.rs, GitHub or a registry at all.

One pin deserves explanation. ripgrep's library crates are resolved from `Cargo.lock` at the
pinned tag rather than from crates.io's latest. The version of `ignore` that ripgrep 15.2.0
actually links is the one whose behaviour the probes observe; a newer `ignore` on crates.io would
describe a tool nobody here is running.
"""

from __future__ import annotations

import io
import json
import re
import shutil
import subprocess
import sys
import tarfile
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

DOCS_RS_JSON = "https://docs.rs/crate/{name}/{version}/json"
GITHUB_TARBALL = "https://github.com/{repo}/archive/refs/tags/{ref}.tar.gz"
GITHUB_BRANCH_TARBALL = "https://github.com/{repo}/archive/refs/heads/{ref}.tar.gz"
NPM_TARBALL = "https://registry.npmjs.org/{name}/-/{basename}-{version}.tgz"
PYPI_JSON = "https://pypi.org/pypi/{name}/{version}/json"

MAX_DOWNLOAD_BYTES = 512 * 1024 * 1024
USER_AGENT = "ast-grep-ripgrep-capability-repository/1.0"


class AcquisitionError(RuntimeError):
    """A pinned input could not be retrieved."""


def _get(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            payload = response.read(MAX_DOWNLOAD_BYTES + 1)
    except urllib.error.HTTPError as exc:
        raise AcquisitionError(f"{url} returned HTTP {exc.code}") from exc
    except urllib.error.URLError as exc:
        raise AcquisitionError(f"{url} could not be reached: {exc.reason}") from exc
    if len(payload) > MAX_DOWNLOAD_BYTES:
        raise AcquisitionError(f"{url} exceeded {MAX_DOWNLOAD_BYTES} bytes")
    return payload


def _cached(cache: Path, key: str, url: str) -> bytes:
    """Fetch `url` once. A cached entry is never revalidated: the key is a pin."""
    target = cache / key
    if target.exists():
        return target.read_bytes()
    payload = _get(url)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(payload)
    return payload


# --------------------------------------------------------------------------- repositories


def _tarball(cache: Path, repo: str, ref: str) -> tarfile.TarFile:
    key = f"repos/{repo.replace('/', '_')}@{ref}.tar.gz"
    for template in (GITHUB_TARBALL, GITHUB_BRANCH_TARBALL):
        try:
            payload = _cached(cache, key, template.format(repo=repo, ref=ref))
        except AcquisitionError:
            continue
        return tarfile.open(fileobj=io.BytesIO(payload), mode="r:gz")
    raise AcquisitionError(f"neither a tag nor a branch named {ref!r} exists in {repo}")


def acquire_repo(spec: dict, cache: Path, out: Path) -> dict:
    """Extract the named files and trees of one pinned repository."""
    archive = _tarball(cache, spec["repo"], spec["ref"])
    members = archive.getmembers()
    if not members:
        raise AcquisitionError(f"{spec['repo']}@{spec['ref']} produced an empty archive")
    root = members[0].name.split("/")[0] + "/"
    dest_prefix = spec.get("dest_prefix", spec["name"])
    written: list[str] = []

    wanted = {root + name: name for name in spec.get("files", [])}
    for member in members:
        if member.name in wanted:
            target = out / dest_prefix / wanted[member.name]
            _write_member(archive, member, target)
            written.append(str(target.relative_to(out)))

    for tree in spec.get("trees", []):
        prefix = root + tree["source_prefix"]
        suffixes = tuple(tree.get("suffixes", []))
        for member in members:
            if not member.isfile() or not member.name.startswith(prefix):
                continue
            if suffixes and not member.name.endswith(suffixes):
                continue
            relative = member.name[len(prefix) :]
            target = out / tree["dest"] / relative
            _write_member(archive, member, target)
            written.append(str(target.relative_to(out)))

    archive.close()
    return {"repo": spec["repo"], "ref": spec["ref"], "files": sorted(written)}


def _write_member(archive: tarfile.TarFile, member: tarfile.TarInfo, target: Path) -> None:
    handle = archive.extractfile(member)
    if handle is None:
        return
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(handle.read())


# --------------------------------------------------------------------------- crates


LOCK_ENTRY_RE = re.compile(
    r'\[\[package\]\]\s*\nname = "([^"]+)"\s*\nversion = "([^"]+)"', re.MULTILINE
)


def resolve_lock_versions(lock_text: str) -> dict[str, str]:
    """Map crate name to the version a `Cargo.lock` pins."""
    return dict(LOCK_ENTRY_RE.findall(lock_text))


def acquire_crates(manifest: dict, cache: Path, out: Path) -> dict:
    """Download rustdoc JSON for every crate in every crate set.

    docs.rs serves rustdoc JSON zstd-compressed inside a zip. It is stored in the cache exactly
    as served and expanded during the build, so the acquisition step stays a pure download.
    """
    lock_path = out / "ripgrep" / "Cargo.lock"
    lock_versions: dict[str, str] = {}
    if lock_path.exists():
        lock_versions = resolve_lock_versions(lock_path.read_text(encoding="utf-8"))

    resolved: dict[str, str] = {}
    for crate_set in manifest["crate_sets"]:
        for name in crate_set["crates"]:
            if crate_set.get("resolve_from_lock"):
                version = lock_versions.get(name)
                if not version:
                    raise AcquisitionError(
                        f"{name} is declared as lock-resolved but is absent from ripgrep's "
                        f"Cargo.lock at the pinned tag. Acquire the ripgrep repo first."
                    )
            else:
                version = crate_set["version"]
            resolved[name] = version

    for name, version in sorted(resolved.items()):
        key = f"rustdoc/{name}@{version}.json.zst"
        _cached(cache, key, DOCS_RS_JSON.format(name=name, version=version))

    (out / "CRATE_VERSIONS.json").write_text(
        json.dumps(resolved, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return resolved


def decompress_zstd(payload: bytes) -> bytes:
    """Expand a zstd payload, preferring the standard library on Python 3.14+."""
    try:
        from compression import zstd
    except ImportError:
        pass
    else:
        return zstd.decompress(payload)

    if shutil.which("zstd") is None:
        raise AcquisitionError(
            "zstd decompression needs Python 3.14+ (compression.zstd) or the zstd CLI on PATH"
        )
    done = subprocess.run(["zstd", "-dc"], input=payload, capture_output=True, check=False)
    if done.returncode != 0:
        raise AcquisitionError(f"zstd failed: {done.stderr.decode('utf-8', 'replace')[:200]}")
    return done.stdout


def read_rustdoc(cache: Path, name: str, version: str) -> bytes:
    """Return one cached rustdoc document as raw JSON bytes.

    docs.rs serves rustdoc JSON zstd-compressed. The bytes are cached exactly as served, so
    acquisition stays a pure download and the build owns every transformation.
    """
    return decompress_zstd((cache / "rustdoc" / f"{name}@{version}.json.zst").read_bytes())


# --------------------------------------------------------------------------- bindings


def acquire_bindings(manifest: dict, cache: Path, out: Path) -> list[dict]:
    """Fetch the shipped type declarations for the JS and Python bindings.

    The declarations are indexed rather than re-derived, and they are parsed later with ast-grep
    itself. Using the subject to index the subject keeps this build to the standard library plus
    the two pinned binaries.
    """
    acquired = []
    for binding in manifest.get("bindings", []):
        try:
            if binding["registry"] == "npm":
                files = _acquire_npm(binding, cache, out)
            else:
                files = _acquire_pypi(binding, cache, out)
        except AcquisitionError as exc:
            # A binding that cannot be fetched is recorded as unavailable, never silently
            # dropped: a missing binding must read as missing, not as a binding with no API.
            acquired.append({"name": binding["name"], "status": "unavailable", "reason": str(exc)})
            continue
        acquired.append({"name": binding["name"], "status": "ok", "files": files})
    return acquired


def _acquire_npm(binding: dict, cache: Path, out: Path) -> list[str]:
    basename = binding["package"].split("/")[-1]
    url = NPM_TARBALL.format(name=binding["package"], basename=basename, version=binding["version"])
    payload = _cached(cache, f"bindings/{basename}@{binding['version']}.tgz", url)
    written = []
    with tarfile.open(fileobj=io.BytesIO(payload), mode="r:gz") as archive:
        for member in archive.getmembers():
            if member.isfile() and member.name.endswith((".d.ts", ".js.flow")):
                relative = member.name.removeprefix("package/")
                target = out / "bindings" / binding["name"] / relative
                _write_member(archive, member, target)
                written.append(str(target.relative_to(out)))
    return sorted(written)


def _acquire_pypi(binding: dict, cache: Path, out: Path) -> list[str]:
    meta = json.loads(
        _cached(
            cache,
            f"bindings/{binding['package']}@{binding['version']}.json",
            PYPI_JSON.format(name=binding["package"], version=binding["version"]),
        )
    )
    wheels = [u for u in meta.get("urls", []) if u["packagetype"] == "bdist_wheel"]
    if not wheels:
        raise AcquisitionError(f"{binding['package']} {binding['version']} publishes no wheel")
    wheel = sorted(wheels, key=lambda u: u["filename"])[0]
    payload = _cached(cache, f"bindings/{wheel['filename']}", wheel["url"])
    written = []
    with zipfile.ZipFile(io.BytesIO(payload)) as archive:
        for name in archive.namelist():
            if name.endswith((".pyi", "py.typed")):
                target = out / "bindings" / binding["name"] / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(archive.read(name))
                written.append(str(target.relative_to(out)))
    return sorted(written)


# --------------------------------------------------------------------------- entry point


def main() -> int:
    build_dir = Path(__file__).resolve().parent
    manifest = json.loads(
        (build_dir / "manifests" / "ast-grep-ripgrep.json").read_text(encoding="utf-8")
    )
    cache = build_dir / ".cache"
    out = build_dir / "acquired"
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    record: dict[str, object] = {"repos": [], "crates": {}, "bindings": []}
    for spec in manifest["repos"]:
        sys.stdout.write(f"  repo    {spec['repo']}@{spec['ref']}" + "\n")
        record["repos"].append(acquire_repo(spec, cache, out))

    sys.stdout.write("  crates  resolving versions and downloading rustdoc JSON" + "\n")
    record["crates"] = acquire_crates(manifest, cache, out)

    sys.stdout.write("  binding type declarations" + "\n")
    record["bindings"] = acquire_bindings(manifest, cache, out)

    (out / "ACQUISITION.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    repos = len(record["repos"])
    crates = len(record["crates"])
    sys.stdout.write(
        f"acquired {repos} repositories, {crates} crates, {len(record['bindings'])} bindings" + "\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

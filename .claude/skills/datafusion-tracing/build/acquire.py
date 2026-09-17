"""Acquire every pinned input. The only stage that touches the network, cargo or a toolchain.

Two destinations, on one criterion -- whether anyone else could re-serve the bytes.

    build/.cache/     raw downloads, gitignored. Re-fetchable from the pin at any time.
    build/acquired/   the inputs build.py actually reads, committed. This is what makes
                      `build.py` offline and the determinism check meaningful: a rebuild reads
                      bytes that are in the tree, not bytes a server chose to send today.

`build.py` never imports this module. If acquisition could run inside the build, a rebuild
could pick up different bytes from the network and still call itself reproducible.

What is different here from the sibling repositories: each SUBJECT crate is captured twice.
Once from docs.rs -- the surface a reader of the documentation can see -- and once locally with
`--document-private-items`, which is the only way to observe the two option builders, because
they are public types inside private modules and no published artifact carries them. The second
capture is not a supplement that fills in impls on items the first already admitted, as it is in
the deltalake repository; here it ADDS items, and `visibility` in the index is what keeps the
difference legible rather than collapsing two different kinds of reachability into one.

Standard library only, plus cargo and the pinned nightly for the private capture.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from datetime import date
from pathlib import Path

import fetch

HERE = Path(__file__).resolve().parent
MANIFEST = HERE / "manifests" / "datafusion-tracing.json"
CACHE = HERE / ".cache"
ACQUIRED = HERE / "acquired"


class AcquisitionError(RuntimeError):
    """A pinned input could not be acquired, or arrived describing something else."""


def _log(message: str) -> None:
    sys.stdout.write(f"{message}\n")
    sys.stdout.flush()


def load_manifest() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def crate_specs(manifest: dict) -> list[dict]:
    """Flatten every crate set into one record per crate, carrying its set's pin."""
    specs: list[dict] = []
    for crate_set in manifest["crate_sets"]:
        for entry in crate_set["crates"]:
            spec = dict(entry)
            spec.setdefault("version", crate_set.get("version"))
            if not spec["version"]:
                raise AcquisitionError(
                    f"{spec['package']} has no version, and its crate set declares none"
                )
            spec["crate_set"] = crate_set["name"]
            spec["subject"] = crate_set["subject"]
            spec["capture"] = crate_set["capture"]
            spec["lib"] = spec.get("lib") or spec["package"].replace("-", "_")
            specs.append(spec)
    return specs


# --------------------------------------------------------------------------- capsule

def capsule_root() -> Path:
    """Where the private-items capture compiles. Never a literal path.

    The transferability check forbids an absolute home path in this directory, and a build that
    wrote into the repository under study would breach the one boundary every skill in this
    family keeps: a repository is never a build directory.
    """
    for variable in ("DATAFUSION_TRACING_CAPSULE", "RUST_SKILL_ACQUIRE_CAPSULE"):
        chosen = os.environ.get(variable)
        if chosen:
            return Path(chosen).expanduser()
    cache_home = os.environ.get("XDG_CACHE_HOME")
    base = Path(cache_home).expanduser() if cache_home else Path.home() / ".cache"
    return base / "rust-skill-acquire" / "datafusion-tracing"


def cargo_env(toolchain: str, target_dir: Path) -> dict[str, str]:
    """A cargo environment that inherits nothing it could be surprised by."""
    env = dict(os.environ)
    for variable in ("RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_TARGET"):
        env.pop(variable, None)
    env["RUSTUP_TOOLCHAIN"] = toolchain
    env["CARGO_TARGET_DIR"] = str(target_dir)
    env["CONFIG_SITE"] = "/dev/null"
    return env


def assert_toolchain(local: dict) -> str:
    """Refuse to run unless the pinned nightly is the one installed.

    Checked before anything expensive, because a document emitted by a different rustdoc is not
    a cheaper version of the right answer -- it is a different format that parses far enough to
    produce confident nonsense.
    """
    toolchain = local["toolchain"]
    try:
        result = subprocess.run(
            ["rustc", f"+{toolchain}", "--version", "--verbose"],
            capture_output=True, text=True, check=False,
        )
    except FileNotFoundError as error:
        raise AcquisitionError("rustc is not on PATH; the private capture needs it") from error
    if result.returncode != 0:
        raise AcquisitionError(
            f"rustc +{toolchain} is not installed. Install it with\n"
            f"    rustup toolchain install {toolchain}\n"
            f"or re-pin `tools.local_rustdoc` deliberately."
        )
    reading = result.stdout
    missing = [token for token in local["rustc_expected"] if token not in reading]
    if missing:
        raise AcquisitionError(
            f"rustc +{toolchain} reports:\n{reading.strip()}\n"
            f"which does not contain {missing}. The manifest records a producer identity; a "
            f"toolchain that no longer matches it must be re-pinned, not accepted."
        )
    return reading.strip()


def document_privately(spec: dict, local: dict, capsule: Path) -> bytes:
    """Unpack the published crate and document it with --document-private-items.

    The published .crate is used rather than a git checkout so the bytes documented are exactly
    the bytes crates.io served for the pin -- the same artifact docs.rs built from, which is what
    makes the diff between the two captures a statement about rustdoc's visibility rules rather
    than about two different source trees.
    """
    package, version = spec["package"], spec["version"]
    source = fetch.extract_crate(fetch.Cache(CACHE), package, version, capsule / "src")
    target_dir = capsule / "target"
    env = cargo_env(local["toolchain"], target_dir)
    env["RUSTDOCFLAGS"] = "-Z unstable-options --output-format json"

    _log(f"  {package} {version}: cargo doc --document-private-items (this compiles DataFusion)")
    result = subprocess.run(
        ["cargo", "doc", "--no-deps", "--lib", "--document-private-items",
         "--target", local["target"]],
        cwd=source, env=env, capture_output=True, text=True, check=False,
    )
    if result.returncode != 0:
        tail = "\n".join(result.stderr.strip().splitlines()[-25:])
        raise AcquisitionError(f"cargo doc failed for {package} {version}:\n{tail}")

    produced = target_dir / local["target"] / "doc" / f"{spec['lib']}.json"
    if not produced.exists():
        raise AcquisitionError(
            f"cargo doc reported success but {produced} was not written. A missing document is "
            f"a stop rather than a warning: build.py would otherwise see a crate with no "
            f"private surface and report the absence as a fact."
        )
    return produced.read_bytes()


# The fields model.py reads. A format version is a label; this is the thing that actually has
# to hold, and checking it is what lets the allowlist span 56 to 61 without hand-waving.
REQUIRED_TOP_LEVEL = ("index", "paths", "root", "external_crates", "format_version")
REQUIRED_INNER = ("struct", "enum", "trait", "function", "impl", "module")


def assert_document_shape(package: str, document: dict) -> None:
    """Confirm a served document carries the vocabulary the model depends on.

    rustdoc JSON is not self-describing and an unsupported version parses far enough to produce
    confident nonsense, so `format_version` is checked first. But the version is only a label:
    what the model actually needs is a set of field names, and those move independently of the
    number. Checking them is what makes accepting a span of five format versions a measurement
    rather than an assumption.
    """
    missing = [key for key in REQUIRED_TOP_LEVEL if key not in document]
    if missing:
        raise AcquisitionError(f"{package}: document lacks top-level {missing}")

    index = document["index"]
    seen_inner: set[str] = set()
    for item in index.values():
        seen_inner.update((item.get("inner") or {}).keys())
    # A small crate need not contain every kind; only contradict the model if something it
    # reads is spelled differently, which is how the import -> use rename would have shown up.
    if "import" in seen_inner and "use" not in seen_inner:
        raise AcquisitionError(
            f"{package}: re-exports are spelled `import`, not `use`. model.py reads `use`, so "
            f"every re-export in this crate would be silently dropped."
        )

    sample = next(
        (i for i in index.values() if "struct" in (i.get("inner") or {})),
        None,
    )
    if sample is not None:
        struct = sample["inner"]["struct"]
        if not isinstance(struct.get("kind"), dict):
            raise AcquisitionError(
                f"{package}: struct.kind is {type(struct.get('kind')).__name__}, not the "
                f"tagged object model.py destructures"
            )
    impl = next((i for i in index.values() if "impl" in (i.get("inner") or {})), None)
    if impl is not None:
        for field in ("items", "blanket_impl", "is_synthetic"):
            if field not in impl["inner"]["impl"]:
                raise AcquisitionError(f"{package}: impl records carry no {field!r}")


# --------------------------------------------------------------------------- stages

def acquire_crates(manifest: dict, cache: fetch.Cache, skip_private: bool) -> dict[str, dict]:
    """Fetch hosted rustdoc JSON and the published Cargo.toml for every pinned crate.

    The compressed docs.rs payload is stored verbatim rather than re-compressed: re-compressing
    would make the committed bytes depend on the local zstd build, which is exactly the kind of
    environmental coupling the determinism check exists to catch. The private capture has no
    such upstream form, so it is compressed here and its digest recorded.
    """
    local = manifest["tools"]["local_rustdoc"]
    supported = manifest["tools"]["rustdoc_format_supported"]
    records: dict[str, dict] = {}
    rustdoc_dir = ACQUIRED / "rustdoc"
    manifest_dir = ACQUIRED / "manifests"
    rustdoc_dir.mkdir(parents=True, exist_ok=True)
    manifest_dir.mkdir(parents=True, exist_ok=True)

    wants_private = [s for s in crate_specs(manifest) if "private" in s["capture"]]
    capsule = capsule_root()
    if wants_private and not skip_private:
        reading = assert_toolchain(local)
        _log(f"  toolchain: {reading.splitlines()[0]}")
        _log(f"  capsule:   {capsule}")

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
        if document.get("format_version") not in supported:
            raise AcquisitionError(
                f"{package}: docs.rs served rustdoc format {document.get('format_version')}, "
                f"which model.py does not parse. Supported: {supported}."
            )
        assert_document_shape(package, document)

        (rustdoc_dir / f"{package}@{version}.json.zst").write_bytes(compressed)
        (manifest_dir / f"{package}.Cargo.toml").write_text(
            fetch.crate_manifest(cache, package, version), encoding="utf-8"
        )

        record = {
            "package": package,
            "lib": spec["lib"],
            "version": version,
            "crate_set": spec["crate_set"],
            "subject": spec["subject"],
            # Recorded per crate, never assumed. docs.rs builds on its own schedule and its
            # fleet is not uniform: the crate that DEFINES format 61 is itself served at 60.
            "format_version": document.get("format_version"),
            "index_items": len(document.get("index", {})),
            "sha256": fetch.digest(payload),
            "compressed_sha256": fetch.digest(compressed),
        }

        if "private" in spec["capture"] and not skip_private:
            raw = document_privately(spec, local, capsule)
            private_doc = json.loads(raw)
            if private_doc.get("format_version") not in supported:
                raise AcquisitionError(
                    f"{package}: the local rustdoc emitted format "
                    f"{private_doc.get('format_version')}, not in {supported}"
                )
            assert_document_shape(f"{package} (private)", private_doc)
            blob = fetch.compress_zstd(raw)
            (rustdoc_dir / f"{package}@{version}.private.json.zst").write_bytes(blob)
            record["private"] = {
                "format_version": private_doc.get("format_version"),
                "index_items": len(private_doc.get("index", {})),
                "sha256": fetch.digest(raw),
                "compressed_sha256": fetch.digest(blob),
                "toolchain": local["toolchain"],
                "target": local["target"],
            }
            gained = record["private"]["index_items"] - record["index_items"]
            _log(
                f"  {package} {version}: docs.rs format {record['format_version']}, "
                f"{record['index_items']} items; private {record['private']['index_items']} "
                f"({gained:+d})"
            )
        else:
            _log(
                f"  {package} {version}: format {record['format_version']}, "
                f"{record['index_items']} index items"
            )
        records[package] = record
    return records


def acquire_corpora(manifest: dict, cache: fetch.Cache) -> dict[str, dict]:
    """Copy each pinned corpus out of the tagged tarball, recording the resolved commit.

    A tag is a pin only as long as nobody moves it, and a corpus whose provenance lives only in
    the manifest cannot be checked from the published tree. Both are recorded here and carried
    into PROVENANCE.json, because the sibling repository that omitted them cannot say which
    commit of its own corpus it describes.
    """
    records: dict[str, dict] = {}
    resolved: dict[tuple[str, str], str] = {}
    for corpus in manifest["corpora"]:
        repo, ref = corpus["repo"], corpus["ref"]
        key = (repo, ref)
        if key not in resolved:
            resolved[key] = fetch.resolve_commit(cache, repo, ref)
        files = fetch.repo_files(
            cache, repo, ref, corpus["source_prefix"], tuple(corpus["suffixes"])
        )
        # A source_prefix naming a whole repository root matches its nested directories too;
        # the suffix filter is what narrows it, and an empty result is a manifest error.
        if not files:
            raise AcquisitionError(
                f"corpus {corpus['name']}: {corpus['source_prefix']!r} with suffixes "
                f"{corpus['suffixes']} matched nothing at {repo}@{ref}"
            )
        target = ACQUIRED / "corpus" / corpus["name"]
        if target.exists():
            shutil.rmtree(target)
        digests: dict[str, str] = {}
        for relative, payload in sorted(files.items()):
            destination = target / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(payload)
            digests[relative] = fetch.digest(payload)
        records[corpus["name"]] = {
            "repo": repo,
            "ref": ref,
            "ref_kind": corpus["ref_kind"],
            "commit": resolved[key],
            "dest": corpus["dest"],
            "files": digests,
        }
        _log(f"  {corpus['name']}: {len(digests)} file(s) from {repo}@{ref} ({resolved[key][:12]})")
    return records


def acquire_release_history(manifest: dict, cache: fetch.Cache) -> dict:
    """Every published release, plus the example manifest each one pinned.

    This is the compatibility matrix's raw material. It is the one input here whose cache key is
    not a pin -- the registry answer changes when a release is published -- so the retrieval date
    is recorded beside it and reported in the catalog.
    """
    spec = manifest["compatibility"]
    versions = fetch.crates_io_versions(cache, spec["crate"])
    releases: dict[str, dict] = {}
    for entry in versions:
        number = entry["num"]
        try:
            example = fetch.raw_file(
                cache, spec["repo"], number, spec["example_manifest"]
            ).decode("utf-8", "replace")
        except fetch.FetchError:
            # A yanked or pre-tag release may have no tag to read from. Recorded as absent
            # rather than skipped, so the catalog can say so.
            example = None
        try:
            own = fetch.crate_manifest(cache, spec["crate"], number)
        except fetch.FetchError:
            own = None
        releases[number] = {
            "published": entry["created_at"][:10],
            "yanked": entry["yanked"],
            "rust_version": entry.get("rust_version"),
            "edition": entry.get("edition"),
            "example_manifest": example,
            "own_manifest": own,
        }
    _log(f"  {spec['crate']}: {len(releases)} published release(s)")
    return {"crate": spec["crate"], "retrieved": date.today().isoformat(), "releases": releases}


# --------------------------------------------------------------------------- entry point

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--skip-private",
        action="store_true",
        help="fetch everything except the --document-private-items capture. Useful only for "
             "checking the docs.rs half; the build refuses to run without the private capture.",
    )
    arguments = parser.parse_args()

    manifest = load_manifest()
    cache = fetch.Cache(CACHE)

    if ACQUIRED.exists():
        shutil.rmtree(ACQUIRED)
    ACQUIRED.mkdir(parents=True)

    _log("rustdoc JSON and crate manifests:")
    crates = acquire_crates(manifest, cache, arguments.skip_private)
    _log("pinned corpora:")
    corpora = acquire_corpora(manifest, cache)
    _log("release history:")
    history = acquire_release_history(manifest, cache)

    record = {
        "repository": manifest["repository"],
        "tools": manifest["tools"],
        "crates": crates,
        "corpora": corpora,
        "compatibility": history,
        "format_versions_served": sorted({c["format_version"] for c in crates.values()}),
    }
    (ACQUIRED / "ACQUISITION.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    private = [p for p, c in crates.items() if "private" in c]
    _log(
        f"acquired {len(crates)} crate(s) ({len(private)} with a private capture), "
        f"{len(corpora)} corpus/corpora, {len(history['releases'])} release(s)"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AcquisitionError, fetch.FetchError) as error:
        sys.stderr.write(f"acquisition failed: {error}\n")
        raise SystemExit(1) from error

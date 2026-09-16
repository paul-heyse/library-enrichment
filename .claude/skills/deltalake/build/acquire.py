"""Produce rustdoc JSON for a git-pinned crate family, locally.

This is the one stage that needs a network, `git`, `cargo` and a dated nightly. It is a
separate entry point on purpose: `verify.py` re-runs `build.py` to prove determinism, so
acquisition must never be reachable from there. After one run, `build.py` and `verify.py`
need only the standard library and `ast-grep`.

Usage:
    python3 acquire.py [--manifest manifests/deltalake.json] [--check] [--clean]

Why this exists at all: docs.rs serves rustdoc JSON keyed by a *published* version. The delta-rs
commit this repository pins is unpublished -- the newest release on crates.io is three months
older and pins arrow 58 / datafusion 53, which is the whole reason for pinning a commit. So the
bytes have to be made here.

Five phases, each of which fails loudly rather than degrading:

  0. assert the toolchain is exactly the one the manifest names
  1. check out each repository at its pinned commit, into a capsule outside every repository
  2. resolve a lockfile and assert it pins the git dependencies the manifest declares
  3. document every crate in ONE cargo invocation, so resolver-3 unifies features once
  4. collect, validate `format_version`, and record what was actually resolved
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ACQUIRED = HERE / "acquired"

# Proc-macro crates compile for the host, so rustdoc writes them to `target/doc` rather than
# `target/<triple>/doc`. Passing `--target` therefore yields no document for them at all -- a
# silent hole that looks exactly like "this crate has no public API". They get their own pass.
PROC_MACRO_KIND = "proc-macro"


class AcquireError(RuntimeError):
    """Acquisition could not produce a trustworthy artefact."""


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def capsule_root() -> Path:
    """Locate the scratch capsule. Never a literal path: the transferability check forbids it."""
    for name in ("DELTA_ACQUIRE_CAPSULE", "RUST_SKILL_ACQUIRE_CAPSULE"):
        value = os.environ.get(name)
        if value:
            return Path(value)
    cache = os.environ.get("XDG_CACHE_HOME")
    base = Path(cache) if cache else Path.home() / ".cache"
    return base / "rust-skill-acquire"


def envelope_key(crate_set: dict) -> str:
    """A cache key over everything that changes the bytes, so a miss is the staleness signal."""
    git = crate_set["git"]
    payload = {
        "schema": 1,
        "url": git["url"],
        "rev": git["rev"],
        "lock_deps": git.get("lock_deps", {}),
        "toolchain": git["toolchain"],
        "target": git["target"],
        "features": sorted(git.get("cargo_features", [])),
        "no_default_features": bool(git.get("no_default_features")),
        "crates": sorted(spec_name(entry) for entry in crate_set["crates"]),
    }
    blob = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(blob).hexdigest()[:12]


def spec_name(entry: str | dict) -> str:
    return entry if isinstance(entry, str) else entry["package"]


def acquired_dir(crate_set: dict) -> Path:
    git = crate_set["git"]
    slug = git["url"].rstrip("/").rsplit("/", 1)[-1]
    return ACQUIRED / f"{slug}@{git['rev'][:8]}-{envelope_key(crate_set)}"


def run(command: list[str], *, cwd: Path, env: dict, timeout: int = 5400) -> str:
    done = subprocess.run(
        command, cwd=cwd, env=env, capture_output=True, text=True, timeout=timeout, check=False
    )
    if done.returncode != 0:
        tail = (done.stderr or done.stdout)[-2000:]
        raise AcquireError(f"{' '.join(command[:4])} ... failed ({done.returncode}):\n{tail}")
    return done.stdout


def cargo_env(toolchain: str, target_dir: Path) -> dict:
    """A deliberately narrowed environment.

    `RUSTFLAGS`/`RUSTDOCFLAGS` are dropped so an inherited `-D warnings` cannot turn a new
    nightly lint into a failed acquisition. `CARGO_TARGET_DIR` is overridden because a host
    session may point it inside a working repository. `CONFIG_SITE` replicates the setting in
    the checkout's own `.cargo/config.toml`, which is skipped because cargo runs from the
    capsule rather than from inside the tree.
    """
    env = dict(os.environ)
    for name in ("RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_BUILD_TARGET_DIR"):
        env.pop(name, None)
    env.update(
        {
            "RUSTUP_TOOLCHAIN": toolchain,
            "CARGO_TARGET_DIR": str(target_dir),
            "CONFIG_SITE": "/dev/null",
            "CARGO_TERM_COLOR": "never",
            "CARGO_NET_RETRY": "3",
        }
    )
    return env


# --------------------------------------------------------------------------- phase 0


def assert_toolchain(git: dict, capsule: Path) -> str:
    toolchain = git["toolchain"]
    env = cargo_env(toolchain, capsule / "target" / "probe")
    version = run(["rustc", f"+{toolchain}", "--version"], cwd=capsule, env=env).strip()
    for needle in git.get("rustc_expected", []):
        if needle not in version:
            raise AcquireError(f"toolchain mismatch: expected {needle!r} in {version!r}")
    say(f"  toolchain {toolchain}: {version}")
    return version


# --------------------------------------------------------------------------- phase 1


def checkout(git: dict, capsule: Path) -> Path:
    """A fresh blobless clone, detached at the pinned commit.

    Deliberately not cargo's own `~/.cargo/git/checkouts`: cargo owns and garbage-collects that
    tree, and writing a target directory into it would be clobbered by the next `cargo fetch`.
    """
    slug = git["url"].rstrip("/").rsplit("/", 1)[-1]
    rev = git["rev"]
    source = capsule / "src" / slug / rev[:8]
    if (source / ".git").exists():
        head = run(["git", "-C", str(source), "rev-parse", "HEAD"], cwd=capsule, env=os.environ)
        if head.strip() == rev:
            say(f"  {slug}: already at {rev[:8]}")
            return source
        shutil.rmtree(source)

    source.mkdir(parents=True, exist_ok=True)
    say(f"  cloning {git['url']} at {rev[:8]}")
    run(
        [
            "git",
            "-c",
            "advice.detachedHead=false",
            "clone",
            "--filter=blob:none",
            "--no-checkout",
            git["url"],
            str(source),
        ],
        cwd=capsule,
        env=dict(os.environ),
    )
    run(
        ["git", "-C", str(source), "fetch", "--depth", "1", "origin", rev],
        cwd=capsule,
        env=dict(os.environ),
    )
    run(["git", "-C", str(source), "checkout", "--detach", rev], cwd=capsule, env=dict(os.environ))

    head = run(["git", "-C", str(source), "rev-parse", "HEAD"], cwd=capsule, env=os.environ)
    if head.strip() != rev:
        raise AcquireError(f"checkout landed on {head.strip()}, not {rev}")
    return source


# --------------------------------------------------------------------------- phase 2


def resolve_lock(git: dict, source: Path, capsule: Path, target_dir: Path) -> tuple[Path, str]:
    """Generate a lockfile, then assert it pins the git dependencies the manifest declares.

    delta-rs does not commit `Cargo.lock` and takes its kernel from a *branch*. A branch is not
    a pin: the same commit of delta-rs resolves to a different kernel next week. So the lock is
    the reproducible artefact, and the manifest's `lock_deps` is the assertion against it. A
    mismatch means the branch moved -- that is a hard stop, not something to paper over.
    """
    toolchain = git["toolchain"]
    env = cargo_env(toolchain, target_dir)
    lock = source / "Cargo.lock"
    manifest_path = source / "Cargo.toml"

    say("  resolving lockfile")
    run(
        ["cargo", f"+{toolchain}", "generate-lockfile", "--manifest-path", str(manifest_path)],
        cwd=capsule,
        env=env,
    )

    text = lock.read_text()
    for package, expected in (git.get("lock_deps") or {}).items():
        marker = f'name = "{package}"'
        if marker not in text:
            raise AcquireError(f"{package} absent from the resolved lockfile")
        if expected not in text:
            raise AcquireError(
                f"{package} does not resolve to {expected} -- the tracked branch has moved. "
                f"Re-pin deliberately; do not build against an undeclared revision."
            )
    say(f"  lock pins {len(git.get('lock_deps') or {})} git dependencies as declared")

    say("  fetching the dependency graph")
    run(
        [
            "cargo",
            f"+{toolchain}",
            "fetch",
            "--locked",
            "--target",
            git["target"],
            "--manifest-path",
            str(source / "Cargo.toml"),
        ],
        cwd=capsule,
        env=env,
    )

    digest = hashlib.sha256(lock.read_bytes()).hexdigest()
    saved = capsule / "lock" / f"{source.parent.name}-{git['rev'][:8]}.Cargo.lock"
    saved.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(lock, saved)
    return saved, digest


# --------------------------------------------------------------------------- phase 3


def partition_crates(crate_set: dict) -> tuple[list[dict], list[dict]]:
    libs, macros = [], []
    for entry in crate_set["crates"]:
        spec = {"package": entry} if isinstance(entry, str) else dict(entry)
        (macros if spec.get("kind") == PROC_MACRO_KIND else libs).append(spec)
    return libs, macros


def document(crate_set: dict, source: Path, capsule: Path, target_dir: Path) -> dict:
    git = crate_set["git"]
    toolchain, target = git["toolchain"], git["target"]
    env = cargo_env(toolchain, target_dir)
    env["RUSTDOCFLAGS"] = "-Z unstable-options --output-format json"

    libs, macros = partition_crates(crate_set)

    # Stale documents from an earlier envelope must never survive into a collection pass: a
    # crate that fails this time would otherwise be collected from last time's output.
    doc_dir = target_dir / target / "doc"
    if doc_dir.exists():
        shutil.rmtree(doc_dir)

    base = [
        "cargo",
        f"+{toolchain}",
        "doc",
        "--manifest-path",
        str(source / "Cargo.toml"),
        "--no-deps",
        "--frozen",
    ]
    if git.get("no_default_features"):
        base.append("--no-default-features")

    # ONE invocation for every library crate. Under resolver 3 a per-crate loop would resolve
    # features N times and emit N mutually inconsistent documents describing the same graph.
    resolved: dict = {}
    if libs:
        command = [*base, "--lib", "--target", target]
        for spec in libs:
            command += ["-p", spec["package"]]
        features = git.get("cargo_features") or []
        if features:
            command += ["--features", ",".join(features)]
        say(f"  documenting {len(libs)} library crates in one invocation")
        run(command, cwd=capsule, env=env)
        resolved = unit_graph_features(command, capsule, env, {s["package"] for s in libs})

    # Second capture, with private items. rustdoc omits the impls of a type declared in a
    # private module, so without this a re-exported builder looks like it has no methods. It is
    # written to its own target directory so the two documents cannot overwrite each other.
    if libs and crate_set.get("document_private_items"):
        say("  documenting again with --document-private-items, for impls rustdoc otherwise drops")
        private_env = dict(env)
        private_env["CARGO_TARGET_DIR"] = f"{target_dir}-private"
        private_env["RUSTDOCFLAGS"] = env["RUSTDOCFLAGS"] + " --document-private-items"
        private = [*base, "--lib", "--target", target]
        for spec in libs:
            private += ["-p", spec["package"]]
        if features:
            private += ["--features", ",".join(features)]
        run(private, cwd=capsule, env=private_env)

    # Proc-macro crates build for the host, so they must be documented without `--target`.
    if macros:
        say(f"  documenting {len(macros)} proc-macro crates for the host")
        host = [*base]
        for spec in macros:
            host += ["-p", spec["package"]]
        run(host, cwd=capsule, env=env)

    return resolved


def unit_graph_features(command: list[str], capsule: Path, env: dict, wanted: set[str]) -> dict:
    """Ask cargo which features it actually turned on, rather than inferring from the manifest.

    Resolver-3 unification can enable features nobody named -- `buoyant_kernel` picks up
    `arrow-conversion`, `default-engine-base` and `reqwest` this way. Recording the declared
    selection instead of the resolved one would understate the documented surface, which is the
    same class of error as understating the excluded one.
    """
    probe = [*command, "-Z", "unstable-options", "--unit-graph"]
    done = subprocess.run(probe, cwd=capsule, env=env, capture_output=True, text=True, check=False)
    if done.returncode != 0 or not done.stdout.strip():
        say("  ! unit graph unavailable; feature coverage will record the declared set only")
        return {}
    graph = json.loads(done.stdout)
    found: dict[str, set[str]] = {}
    for unit in graph.get("units", []):
        name = package_of(unit.get("pkg_id", ""))
        if name in wanted:
            found.setdefault(name, set()).update(unit.get("features", []))
    return {name: sorted(values) for name, values in found.items()}


def package_of(pkg_id: str) -> str:
    """Package name out of a cargo package id, in either the legacy or the URL-ish spelling."""
    if "#" in pkg_id:
        tail = pkg_id.rsplit("#", 1)[-1]
        return tail.split("@")[0] if "@" in tail else tail
    return pkg_id.split()[0]


# --------------------------------------------------------------------------- phase 4


def compress(payload: bytes) -> bytes:
    try:
        from compression import zstd  # Python 3.14+
    except ImportError:
        pass
    else:
        return zstd.compress(payload)
    if shutil.which("zstd") is None:
        raise AcquireError("need Python 3.14+ (compression.zstd) or the zstd CLI on PATH")
    done = subprocess.run(["zstd", "-19", "-c"], input=payload, capture_output=True, check=False)
    if done.returncode != 0:
        raise AcquireError(f"zstd failed: {done.stderr.decode('utf-8', 'replace')[:200]}")
    return done.stdout


def manifest_paths(git: dict, source: Path, capsule: Path, target_dir: Path) -> dict[str, Path]:
    """Where each package's Cargo.toml lives, including packages outside the workspace.

    `collect_crate_facts` needs the feature table, and feature gating is the one fact rustdoc
    JSON never carries. For a git-sourced set there is no crates.io tarball to read it from, and
    the kernel crates are not in the delta-rs tree at all, so cargo is asked directly.
    """
    env = cargo_env(git["toolchain"], target_dir)
    out = run(
        [
            "cargo",
            f"+{git['toolchain']}",
            "metadata",
            "--manifest-path",
            str(source / "Cargo.toml"),
            "--frozen",
            # Without this, metadata resolves every platform's graph and demands wasm-only
            # packages that `cargo fetch --target` never downloaded, so --frozen fails.
            "--filter-platform",
            git["target"],
            "--format-version",
            "1",
        ],
        cwd=capsule,
        env=env,
    )
    graph = json.loads(out)
    return {p["name"]: Path(p["manifest_path"]) for p in graph.get("packages", [])}


def collect(
    crate_set: dict,
    source: Path,
    target_dir: Path,
    supported: list[int],
    resolved: dict,
    rustc: str,
    lock_digest: str,
    manifests: dict[str, Path],
) -> Path:
    git = crate_set["git"]
    out = acquired_dir(crate_set)
    out.mkdir(parents=True, exist_ok=True)
    (out / "manifests").mkdir(exist_ok=True)

    libs, macros = partition_crates(crate_set)
    files: dict[str, dict] = {}
    missing: list[str] = []
    missing_manifests: list[str] = []

    for spec in libs + macros:
        package = spec["package"]
        lib = spec.get("lib") or package.replace("-", "_")
        root = target_dir / (git["target"] if spec.get("kind") != PROC_MACRO_KIND else "")
        candidate = root / "doc" / f"{lib}.json"
        if not candidate.exists():
            missing.append(package)
            continue
        payload = candidate.read_bytes()
        document = json.loads(payload)
        version = document.get("format_version")
        if version not in supported:
            raise AcquireError(
                f"{package}: rustdoc format_version {version} is outside {supported}"
            )
        blob = compress(payload)
        (out / f"{lib}.json.zst").write_bytes(blob)
        files[f"{lib}.json.zst"] = {
            "package": package,
            "sha256": hashlib.sha256(blob).hexdigest(),
            "raw_bytes": len(payload),
            "format_version": version,
            "items": len(document.get("index") or {}),
        }

        supplement = Path(f"{target_dir}-private") / git["target"] / "doc" / f"{lib}.json"
        if supplement.exists():
            raw = supplement.read_bytes()
            blob = compress(raw)
            (out / f"{lib}.private.json.zst").write_bytes(blob)
            files[f"{lib}.private.json.zst"] = {
                "package": package,
                "sha256": hashlib.sha256(blob).hexdigest(),
                "raw_bytes": len(raw),
                "format_version": json.loads(raw).get("format_version"),
                "role": "supplement: impls of private-module types, never a source of items",
            }
        manifest_path = (
            source / spec["path"] / "Cargo.toml" if spec.get("path") else manifests.get(package)
        )
        if manifest_path and manifest_path.exists():
            shutil.copy2(manifest_path, out / "manifests" / f"{package}.Cargo.toml")
        else:
            missing_manifests.append(package)

    if missing_manifests:
        raise AcquireError(
            "no Cargo.toml located for: "
            + ", ".join(sorted(missing_manifests))
            + " -- without it the feature table, the one fact rustdoc never carries, is unknown"
        )

    if missing:
        # No ACQUISITION.json is written, so build.py sees a cache miss rather than an index
        # that is quietly short a crate -- which reads exactly like "that crate has no API".
        raise AcquireError("no rustdoc JSON produced for: " + ", ".join(sorted(missing)))

    workspace = source / "Cargo.toml"
    if workspace.exists():
        shutil.copy2(workspace, out / "manifests" / "WORKSPACE.Cargo.toml")

    record = {
        "schema": 1,
        "envelope_key": envelope_key(crate_set),
        "source": {
            "url": git["url"],
            "rev": git["rev"],
            "target": git["target"],
            "toolchain": git["toolchain"],
            "branch_tracked": git.get("branch_tracked_as"),
        },
        "resolved_git_deps": git.get("lock_deps", {}),
        "lock_sha256": lock_digest,
        "rustc": rustc,
        "declared_features": git.get("cargo_features", []),
        "resolved_features": resolved,
        "files": files,
    }
    (out / "ACQUISITION.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    say(f"  wrote {len(files)} documents to {out.name}")
    return out


# --------------------------------------------------------------------------- driver


def check_drift(manifest: dict) -> int:
    """Report whether a tracked branch has moved off its pin. Report only -- never re-pin."""
    status = 0
    for crate_set in manifest["crate_sets"]:
        git = crate_set.get("git")
        if not git or not git.get("branch_tracked_as"):
            continue
        out = subprocess.run(
            ["git", "ls-remote", git["url"], git["branch_tracked_as"]],
            capture_output=True,
            text=True,
            check=False,
        )
        head = out.stdout.split("\t")[0].strip() if out.stdout else "?"
        pinned = git["rev"]
        moved = head != pinned
        say(
            f"  {git['url']} {git['branch_tracked_as']}: {head[:12]} "
            f"({'MOVED from ' + pinned[:12] if moved else 'at pin'})"
        )
        status |= int(moved)
    return status


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Produce rustdoc JSON from pinned git commits.")
    parser.add_argument("--manifest", type=Path, default=HERE / "manifests" / "deltalake.json")
    parser.add_argument("--check", action="store_true", help="report branch drift and exit")
    parser.add_argument("--clean", action="store_true", help="drop target dirs after collecting")
    args = parser.parse_args(argv)

    manifest = json.loads(args.manifest.read_text())
    if args.check:
        return check_drift(manifest)

    capsule = capsule_root()
    (capsule / "src").mkdir(parents=True, exist_ok=True)
    (capsule / "lock").mkdir(parents=True, exist_ok=True)
    supported = manifest["tools"]["rustdoc_format_versions"]
    say(f"capsule: {capsule}")

    for crate_set in manifest["crate_sets"]:
        if crate_set.get("source") != "git":
            continue
        name = crate_set["name"]
        git = crate_set["git"]
        say(f"\n{name} @ {git['rev'][:8]}")
        target_dir = capsule / "target" / name

        rustc = assert_toolchain(git, capsule)
        source = checkout(git, capsule)
        _, lock_digest = resolve_lock(git, source, capsule, target_dir)
        resolved = document(crate_set, source, capsule, target_dir)
        manifests = manifest_paths(git, source, capsule, target_dir)
        collect(crate_set, source, target_dir, supported, resolved, rustc, lock_digest, manifests)

        if args.clean:
            shutil.rmtree(target_dir, ignore_errors=True)
            say(f"  removed {target_dir}")

    say("\nacquisition complete; build.py and verify.py now run offline")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AcquireError as error:
        say(f"\nacquisition failed: {error}")
        raise SystemExit(1) from error

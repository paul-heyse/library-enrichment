"""Acquire pinned upstream repository corpora.

Deliberately much smaller than the Rust skills' `fetch.py`. Those carry a docs.rs client, a
crates.io tarball reader and a Cargo.toml feature parser, none of which a Python subject has any
use for. Copying them would have shipped four dead functions, which is the failure the sibling
skill's own leftovers demonstrate -- `deltalake` prose in a rule note that can never fire.

What survives is the part that is actually shared: a content cache keyed by a pin, and a tarball
reader that pulls one directory out of a GitHub archive.

Standard library only, and cached under `.cache/` keyed by `(repo, ref)`, so a rebuild after the
first run performs no network I/O. A cached entry is never revalidated, because the key is a pin.

The `ref_kind` distinction is load-bearing. GitHub serves a tag through `/archive/refs/tags/<tag>`
and any other ref through `/archive/<ref>`, and a commit SHA only works on the second. This
matters here because the MCP specification corpora were declared against the `main` *branch* --
a ref that moves, which would make `verify.py`'s determinism check meaningless the first time
upstream pushed. They are pinned to a commit instead.
"""

from __future__ import annotations

import hashlib
import io
import tarfile
import urllib.error
import urllib.request
from pathlib import Path

USER_AGENT = "python-capability-skill-builder (+https://github.com/jlowin/fastmcp)"
GITHUB_TARBALL_TAG = "https://github.com/{repo}/archive/refs/tags/{ref}.tar.gz"
GITHUB_TARBALL_REF = "https://github.com/{repo}/archive/{ref}.tar.gz"

MAX_DOWNLOAD_BYTES = 512 * 1024 * 1024


class FetchError(RuntimeError):
    """A pinned input could not be acquired."""


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


def repo_files(
    cache: Cache,
    repo: str,
    ref: str,
    prefix: str,
    suffixes: tuple[str, ...],
    ref_kind: str = "tag",
    excludes: tuple[str, ...] = (),
) -> dict[str, bytes]:
    """Return `{path relative to prefix: bytes}` for one directory of a pinned repository.

    `excludes` is relative to `prefix` and matches a path *segment prefix*, so `python-sdk/`
    drops that whole subtree. It exists because the useful corpus here is a documentation site
    that also carries its own rendering assets and a generated API reference -- and that API
    reference is the one thing this repository already produces better, from the installed code
    rather than from a doc build.

    Raises rather than returning empty. A corpus that fetched nothing and a corpus with nothing
    to match are indistinguishable downstream, and the second is a lie an agent cannot detect:
    it searches, finds no hits, and concludes the capability does not exist.
    """
    slug = repo.replace("/", "_")
    template = GITHUB_TARBALL_TAG if ref_kind == "tag" else GITHUB_TARBALL_REF
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
            inner = relative[len(prefix) :]
            if any(inner.startswith(skip) for skip in excludes):
                continue
            if suffixes and not inner.endswith(suffixes):
                continue
            handle = bundle.extractfile(member)
            if handle is not None:
                found[inner] = handle.read()

    if not found:
        raise FetchError(
            f"{repo}@{ref} prefix {prefix!r} matched no files "
            f"(suffixes={suffixes}, excludes={excludes}). "
            f"Check the ref spelling -- this project's tags carry a leading 'v' while the "
            f"analysis toolchain's do not, and a wrong ref is a 404 rather than an empty set."
        )
    return found


def digest(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()

"""Preview-first installation with locked generations and an atomic public skill pointer.

A bounded transaction records new bytes before staging. Recovery checks complete files or
exact prefixes of interrupted writes; unknown content is never removed. This operational
installer does not register clients or manage library evidence.
"""

from __future__ import annotations

import argparse
import base64
import fcntl
import hashlib
import json
import os
import shutil
import stat
from collections.abc import Mapping
from contextlib import ExitStack
from pathlib import Path, PurePosixPath
from typing import BinaryIO

from cli import say, warn

NAME = "library-research"
MANAGED = ".library-enrichment-install"
FORMAT = "library-enrichment-skill-install/1"
LIMIT = 8 * 1024 * 1024
COUNT = 256
ROOT = Path(__file__).resolve().parent.parent


def encoded(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def read(path: Path, limit: int = LIMIT) -> bytes:
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, "rb") as stream:
        metadata = os.fstat(stream.fileno())
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_uid != os.getuid():
            raise ValueError(f"not an owned regular file: {path}")
        data = stream.read(limit + 1)
        if len(data) > limit:
            raise ValueError(f"installation file budget exceeded: {path}")
        return data


def sync(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def write(path: Path, data: bytes) -> None:
    with path.open("xb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())
    path.chmod(0o644)


def inventory(root: Path) -> tuple[dict[str, bytes], set[str]]:
    files: dict[str, bytes] = {}
    dirs: set[str] = set()
    entries = total = 0

    def visit(at: Path, depth: int) -> None:
        nonlocal entries, total
        metadata = at.lstat()
        if not stat.S_ISDIR(metadata.st_mode) or metadata.st_uid != os.getuid() or depth > 16:
            raise ValueError(f"invalid installation directory: {at}")
        for child in at.iterdir():
            entries += 1
            if entries > COUNT * 2:
                raise ValueError("installation entry budget exceeded")
            name = child.relative_to(root).as_posix()
            kind = child.lstat().st_mode
            if stat.S_ISDIR(kind):
                dirs.add(name)
                visit(child, depth + 1)
            elif stat.S_ISREG(kind):
                data = read(child)
                total += len(data)
                if len(files) >= COUNT or total > LIMIT:
                    raise ValueError("installation file or byte budget exceeded")
                files[name] = data
            else:
                raise ValueError(f"installation contains a link or special file: {child}")

    visit(root, 0)
    return files, dirs


def directories(files: dict[str, bytes]) -> set[str]:
    return {
        str(parent)
        for name in files
        for parent in PurePosixPath(name).parents
        if str(parent) != "."
    }


def check_tree(root: Path, expected: dict[str, bytes], *, partial: bool = False) -> None:
    actual, dirs = inventory(root)
    if (not partial and actual.keys() != expected.keys()) or not actual.keys() <= expected.keys():
        raise ValueError(f"unrecognized installation files: {root}")
    if not dirs <= directories(expected):
        raise ValueError(f"unrecognized installation directories: {root}")
    for name, data in actual.items():
        if (not partial and data != expected[name]) or not expected[name].startswith(data):
            raise ValueError(f"modified installation file: {root / name}")


def owner(destination: Path, source: Path) -> dict[str, object]:
    return {
        "format": FORMAT,
        "uid": os.getuid(),
        "destination": str(destination),
        "source": str(source),
    }


def generation(
    files: dict[str, bytes], identity: dict[str, object]
) -> tuple[str, dict[str, bytes]]:
    manifest = {
        **identity,
        "files": {name: hashlib.sha256(data).hexdigest() for name, data in sorted(files.items())},
    }
    data = encoded(manifest)
    key = "g-" + hashlib.sha256(data).hexdigest()
    output = {"manifest.json": data, **{f"skill/{name}": data for name, data in files.items()}}
    if len(output) > COUNT or sum(map(len, output.values())) > LIMIT:
        raise ValueError("complete installation generation exceeds budget")
    return key, output


def validate_generation(root: Path, key: str, identity: dict[str, object]) -> None:
    if (
        len(key) != 66
        or not key.startswith("g-")
        or any(c not in "0123456789abcdef" for c in key[2:])
    ):
        raise ValueError("invalid installation generation identity")
    at = root / key
    files, _ = inventory(at)
    data = files.get("manifest.json", b"")
    if hashlib.sha256(data).hexdigest() != key[2:]:
        raise ValueError(f"modified generation manifest: {at}")
    skill = {name[6:]: value for name, value in files.items() if name.startswith("skill/")}
    expected_key, expected = generation(skill, identity)
    if expected_key != key:
        raise ValueError(f"modified or unrelated installation: {at}")
    check_tree(at, expected)


def retire(root: Path, key: str, identity: dict[str, object], *, apply: bool) -> None:
    """Retire exact old bytes with a manifest that survives each individual deletion."""
    journal = root / f"retire-{key}.json"
    original = root / key
    retired = root / f"retired-{key}"
    if not os.path.lexists(journal):
        validate_generation(root, key, identity)
        if not apply:
            return
        os.link(original / "manifest.json", journal, follow_symlinks=False)
        sync(root)
    data = read(journal)
    value = json.loads(data)
    if (
        not isinstance(value, dict)
        or set(value) != {*identity, "files"}
        or any(value[name] != expected for name, expected in identity.items())
        or not isinstance(value["files"], dict)
        or hashlib.sha256(data).hexdigest() != key[2:]
    ):
        raise ValueError("invalid retirement ownership manifest")
    expected = {f"skill/{name}": digest for name, digest in value["files"].items()}
    expected["manifest.json"] = key[2:]
    if os.path.lexists(original):
        validate_generation(root, key, identity)
        if os.path.lexists(retired):
            raise ValueError("two conflicting retirement directories")
        if apply:
            os.rename(original, retired)
            sync(root)
    if os.path.lexists(retired):
        files, dirs = inventory(retired)
        if not files.keys() <= expected.keys() or not dirs <= directories(
            dict.fromkeys(expected, b"")
        ):
            raise ValueError("unknown retirement content")
        for name, contents in files.items():
            if hashlib.sha256(contents).hexdigest() != expected[name]:
                raise ValueError("modified retirement content")
        if apply:
            for name in files:
                (retired / name).unlink()
            for name in sorted(dirs, key=lambda name: len(PurePosixPath(name).parts), reverse=True):
                (retired / name).rmdir()
            retired.rmdir()
            sync(root)
    if apply:
        journal.unlink()
        sync(root)


def current(destination: Path, identity: dict[str, object]) -> str | None:
    target = destination / NAME
    if not os.path.lexists(target):
        return None
    if not target.is_symlink():
        raise ValueError(f"refusing an unrelated or old in-place installation: {target}")
    parts = PurePosixPath(os.readlink(target)).parts
    if len(parts) != 3 or parts[0] != MANAGED or parts[2] != "skill":
        raise ValueError(f"refusing an unrelated installation symlink: {target}")
    validate_generation(destination / MANAGED, parts[1], identity)
    return parts[1]


def transaction_files(value: object) -> dict[str, bytes]:
    if not isinstance(value, dict) or len(value) > COUNT:
        raise ValueError("invalid transaction file inventory")
    files: dict[str, bytes] = {}
    total = 0
    for name, payload in value.items():
        if (
            not isinstance(name, str)
            or not isinstance(payload, str)
            or not name
            or not PurePosixPath(name).parts
            or "\\" in name
            or str(PurePosixPath(name)) != name
            or PurePosixPath(name).is_absolute()
            or ".." in PurePosixPath(name).parts
        ):
            raise ValueError("unsafe transaction path")
        data = base64.b64decode(payload, validate=True)
        total += len(data)
        if total > LIMIT:
            raise ValueError("transaction payload budget exceeded")
        files[name] = data
    if "SKILL.md" not in files:
        raise ValueError("transaction lacks the product skill")
    if files.keys() & directories(files):
        raise ValueError("transaction file and directory paths overlap")
    return files


def recover(destination: Path, identity: dict[str, object], *, apply: bool) -> None:
    root = destination / MANAGED
    pending = root / "pending.json"
    if not os.path.lexists(pending):
        return
    record = json.loads(read(pending, LIMIT * 2))
    if not isinstance(record, dict) or set(record) != {"owner", "previous", "files"}:
        raise ValueError("invalid installation transaction")
    if record["owner"] != identity:
        raise ValueError("unrelated installation transaction")
    previous = record["previous"]
    if previous is not None:
        if not isinstance(previous, str):
            raise ValueError("invalid predecessor")
        validate_generation(root, previous, identity)
    files = transaction_files(record["files"]) if record["files"] is not None else None
    key, expected = generation(files, identity) if files is not None else (None, {})
    if current(destination, identity) not in {previous, key}:
        raise ValueError("installation pointer changed during interrupted operation")
    staging = root / "staging"
    if os.path.lexists(staging):
        if files is None:
            raise ValueError("unexpected staging during uninstall")
        check_tree(staging, expected, partial=True)
    next_link = root / "next"
    if os.path.lexists(next_link) and (
        key is None
        or not next_link.is_symlink()
        or os.readlink(next_link) != f"{MANAGED}/{key}/skill"
    ):
        raise ValueError("unrelated installation replacement link")
    if not apply:
        say(f"  {destination / NAME}: interrupted operation will be recovered")
        return
    if files is not None and key is not None:
        final = root / key
        if os.path.lexists(final):
            validate_generation(root, key, identity)
            if os.path.lexists(staging):
                shutil.rmtree(staging)
        else:
            if staging.exists():
                shutil.rmtree(staging)
            staging.mkdir(mode=0o700)
            for name, data in expected.items():
                path = staging / name
                path.parent.mkdir(parents=True, exist_ok=True)
                write(path, data)
            for name in sorted(
                directories(expected), key=lambda name: len(PurePosixPath(name).parts), reverse=True
            ):
                sync(staging / name)
            sync(staging)
            os.rename(staging, final)
            sync(root)
        if not os.path.lexists(next_link):
            next_link.symlink_to(f"{MANAGED}/{key}/skill")
        os.replace(next_link, destination / NAME)
    else:
        (destination / NAME).unlink(missing_ok=True)
    sync(destination)
    pending.unlink()
    sync(root)


def validate_root(destination: Path, source: Path) -> dict[str, object]:
    identity = owner(destination, source)
    root = destination / MANAGED
    if os.path.lexists(root):
        metadata = root.lstat()
        if not stat.S_ISDIR(metadata.st_mode) or metadata.st_uid != os.getuid():
            raise ValueError(f"unrelated installer state: {root}")
        if initialization(destination, identity, apply=False):
            return identity
        entries = list(root.iterdir())
        if len(entries) > 64:
            raise ValueError("installer state entry budget exceeded")
        for path in entries:
            if path.name.startswith("g-"):
                validate_generation(root, path.name, identity)
            elif path.name.startswith("retire-g-") and path.name.endswith(".json"):
                retire(root, path.name[7:-5], identity, apply=False)
            elif path.name.startswith("retired-g-"):
                if not os.path.lexists(root / f"retire-{path.name[8:]}.json"):
                    raise ValueError("retirement without ownership manifest")
            elif path.name not in {"owner.json", "pending.json", "intent", "staging", "next"}:
                raise ValueError(f"unknown installer state: {path}")
        discard_intent(root, identity, apply=False)
        if not os.path.lexists(root / "pending.json") and any(
            os.path.lexists(root / name) for name in ("staging", "next")
        ):
            raise ValueError("staging without a durable ownership transaction")
    current(destination, identity)
    return identity


def initialization(destination: Path, identity: dict[str, object], *, apply: bool) -> bool:
    """An interrupted initializer has only the exact owner prefix and no public pointer."""
    root = destination / MANAGED
    path = root / "owner.json"
    expected = encoded(identity)
    actual = read(path) if os.path.lexists(path) else b""
    if actual == expected:
        return False
    if (
        not expected.startswith(actual)
        or any(p.name != "owner.json" for p in root.iterdir())
        or os.path.lexists(destination / NAME)
    ):
        raise ValueError(f"unrelated or altered installation initialization: {root}")
    if apply:
        # The owner prefix was validated while holding the destination lock. No generation
        # or public pointer can exist until this complete marker is durable.
        path.unlink(missing_ok=True)
        write(path, expected)
        sync(root)
        sync(destination)
    return True


def intent_bytes(record: Mapping[str, object]) -> bytes:
    """Put the fixed ownership header first, before a possibly interrupted input payload."""
    return (
        b'{"owner":'
        + encoded(record["owner"]).rstrip(b"\n")
        + b","
        + encoded({key: value for key, value in record.items() if key != "owner"})[1:]
    )


def discard_intent(root: Path, identity: dict[str, object], *, apply: bool) -> None:
    """Remove only an unpublished, bounded input buffer inside the validated owned root.

    Intent bytes never authorize generation or pointer changes. Only atomic promotion to
    pending.json admits a complete transaction. Unknown children, links and foreign ownership
    still fail; an interrupted payload may be discarded without interpreting its paths.
    """
    path = root / "intent"
    if not os.path.lexists(path):
        return
    data = read(path, LIMIT * 2)
    prefix = b'{"owner":' + encoded(identity).rstrip(b"\n") + b","
    if not (prefix.startswith(data) or data.startswith(prefix)) or os.path.lexists(
        root / "pending.json"
    ):
        raise ValueError("unrelated or conflicting unpublished installation intent")
    if apply:
        path.unlink()
        sync(root)


def lock(destination: Path) -> BinaryIO:
    path = destination / ".library-enrichment-install.lock"
    fd = os.open(path, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600)
    stream = os.fdopen(fd, "r+b")
    metadata = os.fstat(fd)
    if not stat.S_ISREG(metadata.st_mode) or metadata.st_uid != os.getuid() or metadata.st_size:
        stream.close()
        raise ValueError(f"invalid installation lock: {path}")
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BaseException:
        stream.close()
        raise
    return stream


def install(source: Path, destinations: list[Path], *, apply: bool, uninstall: bool) -> None:
    source = source.resolve(strict=True)
    files, _ = inventory(source)
    payload = {name: base64.b64encode(data).decode() for name, data in files.items()}
    transaction_files(payload)
    destinations = sorted({path.resolve() for path in destinations})
    for destination in destinations:
        generation(files, owner(destination, source))
    with ExitStack() as stack:
        if apply:
            for destination in destinations:
                destination.mkdir(parents=True, exist_ok=True)
                stack.enter_context(lock(destination))
        identities = [validate_root(destination, source) for destination in destinations]
        for destination, identity in zip(destinations, identities, strict=True):
            root = destination / MANAGED
            if root.exists():
                initialization(destination, identity, apply=apply)
                discard_intent(root, identity, apply=apply)
            recover(destination, identity, apply=apply)
            if root.exists():
                for journal in root.glob("retire-g-*.json"):
                    key = journal.name[7:-5]
                    if current(destination, identity) == key:
                        raise ValueError("refusing retirement of the selected skill")
                    retire(root, key, identity, apply=apply)
            say(f"  {destination / NAME}: {'remove' if uninstall else 'install/update'}")
            if not apply:
                continue
            if not root.exists():
                if uninstall:
                    continue
                root.mkdir(mode=0o700)
                initialization(destination, identity, apply=True)
            record = {
                "owner": identity,
                "previous": current(destination, identity),
                "files": None if uninstall else payload,
            }
            # A partial intent has caused no managed mutation. It is written privately and
            # only the complete, synced descriptor is atomically made recoverable.
            path = root / "intent"
            write(path, intent_bytes(record))
            os.rename(path, root / "pending.json")
            sync(root)
            recover(destination, identity, apply=True)
            selected = current(destination, identity)
            for path in root.iterdir():
                if path.name.startswith("g-") and path.name != selected:
                    retire(root, path.name, identity, apply=True)
            sync(root)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument(
        "--uninstall", action="store_true", help="select removal; preview by default"
    )
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    if args.apply and args.dry_run:
        parser.error("--apply and --dry-run are mutually exclusive")
    try:
        say(f"mode: {'apply' if args.apply else 'preview'}")
        install(
            ROOT / "skills" / NAME,
            [Path.home() / ".claude/skills", Path.home() / ".agents/skills"],
            apply=args.apply,
            uninstall=args.uninstall,
        )
    except (OSError, ValueError) as error:
        warn(f"install-skill: {error}")
        return 1
    if not args.apply:
        say("Preview only. Re-run with --apply to perform this operation.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

"""Installer ownership and actual interrupted-process recovery in isolated destinations."""

from __future__ import annotations

import importlib.util
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

import pytest

SCRIPTS = Path(__file__).resolve().parents[2] / "scripts"
sys.path.insert(0, str(SCRIPTS))
SPEC = importlib.util.spec_from_file_location("tested_skill_install", SCRIPTS / "skill_install.py")
assert SPEC and SPEC.loader
installer = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(installer)


@pytest.fixture
def layout(tmp_path):
    source = tmp_path / "source"
    source.mkdir()
    (source / "SKILL.md").write_text("first version\n")
    (source / "references").mkdir()
    (source / "references/contract.md").write_text("exact support\n")
    return source, tmp_path / "destination"


def apply(source, destination, *, uninstall=False):
    installer.install(source, [destination], apply=True, uninstall=uninstall)


def test_preview_alias_dedup_update_and_explicit_uninstall(layout, tmp_path):
    source, destination = layout
    installer.install(source, [destination], apply=False, uninstall=False)
    assert not destination.exists()
    destination.mkdir()
    alias = tmp_path / "alias"
    alias.symlink_to(destination, target_is_directory=True)
    installer.install(source, [destination, alias], apply=True, uninstall=False)
    target = destination / "library-research"
    assert target.is_symlink()
    old = target.resolve()
    assert (target / "SKILL.md").read_text() == "first version\n"
    (source / "SKILL.md").write_text("second version\n")
    apply(source, alias)
    assert (target / "SKILL.md").read_text() == "second version\n"
    assert not old.exists()
    installer.install(source, [destination], apply=False, uninstall=True)
    assert target.exists()
    apply(source, destination, uninstall=True)
    assert not os.path.lexists(target)
    assert not list((destination / installer.MANAGED).glob("g-*"))
    apply(source, destination, uninstall=True)


@pytest.mark.parametrize("change", ["content", "extra", "link", "manifest"])
def test_modified_generation_refused_without_removal(layout, change):
    source, destination = layout
    apply(source, destination)
    target = destination / installer.NAME
    original = os.readlink(target)
    if change == "content":
        (target / "SKILL.md").write_text("operator edits")
    elif change == "extra":
        (target / "personal.md").write_text("operator addition")
    elif change == "link":
        (target / "external").symlink_to(source / "SKILL.md")
    else:
        (target.resolve().parent / "manifest.json").write_text("{}")
    with pytest.raises(ValueError):
        apply(source, destination, uninstall=True)
    assert os.readlink(target) == original


def test_unrelated_destination_and_lock_are_refused(layout):
    source, destination = layout
    target = destination / installer.NAME
    target.mkdir(parents=True)
    (target / "mine").write_text("keep")
    with pytest.raises(ValueError):
        apply(source, destination)
    assert (target / "mine").read_text() == "keep"
    (target / "mine").unlink()
    target.rmdir()
    with installer.lock(destination), pytest.raises(BlockingIOError):
        apply(source, destination)
    assert not os.path.lexists(target)


CRASH_DRIVER = r"""
import os, signal, sys
from pathlib import Path
sys.path.insert(0, sys.argv[1])
import skill_install as installer
source, destination, marker = map(Path, sys.argv[2:5])
barrier = sys.argv[5]
def stop():
    marker.write_text('reached')
    os.kill(os.getpid(), signal.SIGSTOP)
original_write = installer.write
def interrupted_write(path, data):
    if ((barrier == 'partial-file' and path.name == 'SKILL.md')
        or (barrier == 'initialization' and path.name == 'owner.json')
        or (barrier == 'intent' and path.name == 'intent')):
        original_write(path, data[:len(data)//2])
        stop()
    original_write(path, data)
installer.write = interrupted_write
original_replace = installer.os.replace
def interrupted_replace(source, target):
    original_replace(source, target)
    if barrier == 'pointer' and target.name == installer.NAME:
        stop()
installer.os.replace = interrupted_replace
original_unlink = Path.unlink
def interrupted_unlink(path, *args, **kwargs):
    original_unlink(path, *args, **kwargs)
    if barrier == 'retirement' and any(p.startswith('retired-g-') for p in path.parts):
        stop()
    if barrier == 'uninstall' and path == destination / installer.NAME:
        stop()
Path.unlink = interrupted_unlink
installer.install(source, [destination], apply=True, uninstall=barrier == 'uninstall')
"""


@pytest.mark.parametrize(
    "barrier", ["initialization", "intent", "partial-file", "pointer", "retirement", "uninstall"]
)
def test_sigkill_update_and_uninstall_recover_exact_bytes(layout, tmp_path, barrier):
    source, destination = layout
    if barrier != "initialization":
        apply(source, destination)
    (source / "SKILL.md").write_text("second version with retained details\n")
    marker = tmp_path / "barrier"
    process = subprocess.Popen(
        [
            sys.executable,
            "-c",
            CRASH_DRIVER,
            str(SCRIPTS),
            str(source),
            str(destination),
            str(marker),
            barrier,
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
    )
    try:
        deadline = time.monotonic() + 10
        while not marker.exists() and process.poll() is None and time.monotonic() < deadline:
            time.sleep(0.01)
        assert marker.exists(), process.communicate(timeout=1)
        process.kill()
        assert process.wait(timeout=5) == -signal.SIGKILL
    finally:
        if process.poll() is None:
            process.kill()
        process.communicate(timeout=5)
    target = destination / installer.NAME
    if barrier not in {"uninstall", "initialization"}:
        assert (target / "SKILL.md").read_text() in {
            "first version\n",
            "second version with retained details\n",
        }
    # Preview recognizes the interrupted operation but does not complete it.
    installer.install(source, [destination], apply=False, uninstall=barrier == "uninstall")
    apply(source, destination, uninstall=barrier == "uninstall")
    if barrier == "uninstall":
        assert not os.path.lexists(target)
    else:
        assert (target / "SKILL.md").read_text() == "second version with retained details\n"
    root = destination / installer.MANAGED
    assert not (root / "pending.json").exists()
    assert not (root / "intent").exists()
    assert not list(destination.glob(".libenr-install-*"))
    assert not (root / "staging").exists()
    assert not list(root.glob("retire*"))


def test_source_links_and_unsafe_transactions_are_refused(layout):
    source, destination = layout
    (source / "linked").symlink_to(source / "SKILL.md")
    with pytest.raises(ValueError):
        apply(source, destination)
    assert not destination.exists()
    for name in ("../outside", "/absolute", ".", "a/../b"):
        with pytest.raises(ValueError):
            installer.transaction_files({"SKILL.md": "YQ==", name: "YQ=="})


def test_shell_entrypoint_from_unrelated_cwd_uses_preview_and_explicit_apply(tmp_path):
    home = tmp_path / "isolated-user"
    home.mkdir()
    unrelated = tmp_path / "unrelated"
    unrelated.mkdir()
    env = {**os.environ, "HOME": str(home), "UV_CACHE_DIR": str(tmp_path / "uv-cache")}
    command = [str(SCRIPTS / "install-skill.sh")]
    preview = subprocess.run(command, env=env, cwd=unrelated, capture_output=True, text=True)
    assert preview.returncode == 0, preview.stderr
    assert not (home / ".claude").exists()
    installed = subprocess.run(
        [*command, "--apply"], env=env, cwd=unrelated, capture_output=True, text=True
    )
    assert installed.returncode == 0, installed.stderr
    for name in (".claude", ".agents"):
        assert (home / name / "skills/library-research/SKILL.md").read_bytes() == (
            SCRIPTS.parent / "skills/library-research/SKILL.md"
        ).read_bytes()
    preview = subprocess.run(
        [*command, "--uninstall"], env=env, cwd=unrelated, capture_output=True, text=True
    )
    assert preview.returncode == 0, preview.stderr
    assert (home / ".claude/skills/library-research").exists()
    removed = subprocess.run(
        [*command, "--uninstall", "--apply"], env=env, cwd=unrelated, capture_output=True, text=True
    )
    assert removed.returncode == 0, removed.stderr
    assert not os.path.lexists(home / ".claude/skills/library-research")
    assert not list(unrelated.iterdir())


@pytest.mark.parametrize("child", ["foreign", "intent"])
def test_unknown_or_foreign_uncommitted_state_is_preserved(layout, child):
    source, destination = layout
    apply(source, destination)
    path = destination / installer.MANAGED / child
    path.write_bytes(b"foreign operator bytes")
    with pytest.raises(ValueError):
        apply(source, destination)
    assert path.read_bytes() == b"foreign operator bytes"

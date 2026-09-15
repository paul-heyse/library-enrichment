"""Client isolation and timeout ownership, without authenticating or calling a model."""

import subprocess
import sys
from pathlib import Path

import pytest
from client_sandbox import Sandbox


def test_owned_service_home_is_allowed_but_real_client_roots_are_refused(tmp_path, monkeypatch):
    monkeypatch.setenv("LIBENR_HOME", str(tmp_path / "service"))
    sandbox = Sandbox.create(tmp_path / "service/clients/run")
    assert sandbox.env()["HOME"] == str(sandbox.root)
    assert sandbox.env()["CODEX_SQLITE_HOME"].startswith(str(sandbox.root))
    assert sandbox.untouched() == (True, [])
    with pytest.raises(ValueError, match="already exists"):
        Sandbox.create(sandbox.root)
    monkeypatch.setenv("LIBENR_HOME", str(Path.home() / ".codex"))
    with pytest.raises(ValueError, match="isolated service"):
        Sandbox.create(Path.home() / ".codex/clients/forbidden")


def test_client_timeout_stops_its_native_descendants_before_home_cleanup(tmp_path, monkeypatch):
    monkeypatch.setenv("LIBENR_HOME", str(tmp_path))
    sandbox = Sandbox.create(tmp_path / "clients/timeout")
    marker = sandbox.root / "child-pid"
    child = (
        "import os,time;from pathlib import Path;"
        "Path('child-pid').write_text(str(os.getpid()));time.sleep(60)"
    )
    parent = (
        "import subprocess,sys,time;"
        "subprocess.Popen([sys.executable,'-c',sys.argv[1]]);time.sleep(60)"
    )
    with pytest.raises(subprocess.TimeoutExpired):
        sandbox.run([sys.executable, "-c", parent, child], timeout=1)
    pid = int(marker.read_text())
    state = Path(f"/proc/{pid}/stat")
    # A terminated orphan can briefly remain a zombie until the system reaps it.
    assert not state.exists() or state.read_text().split()[2] == "Z"
    assert sandbox.untouched() == (True, [])

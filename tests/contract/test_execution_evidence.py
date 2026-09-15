"""Acceptance evidence must describe executed tests on the actual source tree."""

import json
import runpy
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def test_comparison_gate_phases_follow_blueprint():
    registry = tomllib.loads((ROOT / "tests/gates.toml").read_text())["gates"]
    assert registry["R08"]["phase"] == registry["P06"]["phase"] == 3
    assert registry["P08"]["superseded"]


def test_receipts_reject_changed_logs_and_changed_source(tmp_path, monkeypatch):
    monkeypatch.syspath_prepend(str(ROOT / "scripts"))
    api = runpy.run_path(str(ROOT / "scripts/evidence_run.py"))
    log = tmp_path / "test.json"
    log.write_text('{"tests": []}')
    receipt = {
        "source_before": "source-a",
        "source_after": "source-a",
        "log_sha256": api["log_digest"](log),
        "exit_code": 0,
        "executable_sha256": "exe",
        "tool_versions": {"python": "test"},
        "native_before": {"fixture-native": "digest"},
        "native_after": {"fixture-native": "digest"},
        "argv": ["pytest"],
        "started_at": "start",
        "finished_at": "finish",
    }
    log.with_suffix(".json.execution.json").write_text(json.dumps(receipt))
    assert api["valid_receipt"](log, "source-a")
    receipt["native_after"] = {"fixture-native": "changed"}
    log.with_suffix(".json.execution.json").write_text(json.dumps(receipt))
    assert not api["valid_receipt"](log, "source-a")
    receipt["native_after"] = receipt["native_before"]
    log.with_suffix(".json.execution.json").write_text(json.dumps(receipt))
    assert not api["valid_receipt"](log, "source-b")
    log.write_text('{"tests": [{"outcome": "passed"}]}')
    assert not api["valid_receipt"](log, "source-a")


def test_source_fingerprint_includes_untracked_code_and_lockfiles(tmp_path):
    api = runpy.run_path(str(ROOT / "scripts/evidence_run.py"))
    before = api["source_digest"](tmp_path)
    (tmp_path / "crates").mkdir()
    (tmp_path / "crates/new.rs").write_text("fn main() {}")
    changed = api["source_digest"](tmp_path)
    assert before != changed
    (tmp_path / "Cargo.lock").write_text("new lock")
    assert api["source_digest"](tmp_path) != changed


def test_failed_command_cannot_produce_pass_and_later_run_clears_skip(tmp_path, monkeypatch):
    monkeypatch.syspath_prepend(str(ROOT / "scripts"))
    api = runpy.run_path(str(ROOT / "scripts/acceptance-report.py"))
    from evidence_run import log_digest

    log = tmp_path / "pytest.json"
    receipt = {
        "source_before": api["SOURCE_DIGEST"],
        "source_after": api["SOURCE_DIGEST"],
        "exit_code": 1,
        "argv": ["pytest"],
        "started_at": "start",
        "finished_at": "end",
        "executable_sha256": "exe",
        "tool_versions": {"python": "test"},
        "native_before": {"fixture-native": "digest"},
        "native_after": {"fixture-native": "digest"},
    }

    def write(outcome):
        log.write_text(json.dumps({"tests": [{"nodeid": "gate", "outcome": outcome}]}))
        receipt["log_sha256"] = log_digest(log)
        log.with_suffix(".json.execution.json").write_text(json.dumps(receipt))

    write("passed")
    assert api["load_pytest"](log)["gate"][0] is False
    receipt["exit_code"] = 0
    write("skipped")
    assert api["load_pytest"](log) == {}
    assert "gate" in api["SKIPPED"]
    write("passed")
    assert api["load_pytest"](log)["gate"][0] is True
    assert "gate" not in api["SKIPPED"]


def test_client_replay_records_only_the_explicit_boolean_and_skill_changes(tmp_path, monkeypatch):
    monkeypatch.syspath_prepend(str(ROOT / "scripts"))
    api = runpy.run_path(str(ROOT / "scripts/evidence_run.py"))
    monkeypatch.setenv("LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS", "1")
    monkeypatch.setenv("LIBENR_AUTH_TOKEN", "do-not-record")
    env = api["reproducing_environment"]()
    assert env["LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS"] == "1"
    assert "LIBENR_AUTH_TOKEN" not in env
    before = api["source_digest"](tmp_path)
    (tmp_path / "skills").mkdir()
    (tmp_path / "skills/SKILL.md").write_text("changed workflow")
    assert api["source_digest"](tmp_path) != before


def test_client_output_receipts_reject_lost_or_changed_transcripts(tmp_path):
    api = runpy.run_path(str(ROOT / "scripts/evidence_run.py"))
    out = tmp_path / "traces"
    out.mkdir()
    (out / "summary.json").write_text('{"results": []}')
    trace = out / "native.json"
    trace.write_text("native events")
    manifest = {
        "root": str(out),
        "files": {p.name: api["log_digest"](p) for p in out.iterdir()},
    }
    log = tmp_path / "client.json"
    log.write_text(json.dumps({"tests": [{"metadata": {"client_evidence": manifest}}]}))
    assert api["client_outputs_match"](log)
    trace.write_text("different events")
    assert not api["client_outputs_match"](log)
    trace.unlink()
    try:
        matched = api["client_outputs_match"](log)
    except OSError:
        matched = False
    assert not matched

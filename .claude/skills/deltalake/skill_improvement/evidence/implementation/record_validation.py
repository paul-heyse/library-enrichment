"""Collect retained executed checks; this verifies receipts/logs, not a fresh test execution."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from datetime import UTC, datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    records = []

    def record(name: str, files: list[str], command: str, scope: str) -> None:
        records.append(
            {
                "check": name,
                "state": "passed",
                "executed_command": command,
                "scope": scope,
                "artifacts": {
                    f: {
                        "sha256": digest(HERE / f),
                        "modified_at": datetime.fromtimestamp(
                            (HERE / f).stat().st_mtime, UTC
                        ).isoformat(),
                    }
                    for f in files
                },
            }
        )

    raw = json.loads((HERE / "contracts-validation.json").read_text())
    assert raw["preservation"]["state"] == raw["reader"]["state"] == "passed"
    assert raw["preservation"]["raw_records_compared"] == 6944
    record(
        "contracts and bounded reader",
        ["contracts-validation.json"],
        "python3 build/test_contracts.py",
        "All admitted raw records, alias/route/byte/link assertions",
    )
    units = (HERE / "unit-tests.log").read_text()
    assert "Ran 7 tests" in units and units.rstrip().endswith("OK")
    record(
        "identity access routing replay",
        ["unit-tests.log"],
        "python3 -m unittest discover -s build -p 'test_*.py'",
        "Seven implementation controls",
    )
    legacy = json.loads((HERE / "legacy-validation.json").read_text())
    assert len(legacy) == 5 and "15 rule cases passed" in legacy["rule tests"]
    assert "27 capability probes" in legacy["navigation probes"]
    record(
        "legacy rebuild/navigation/rules",
        ["legacy-validation.json", "legacy-validation.log"],
        "python3 build/verify.py",
        "Full rebuild plus all five existing verification groups",
    )
    runtime = json.loads((HERE / "probe-results.json").read_text())
    assert runtime["state"] == "passed"
    for path, expected in runtime["sources"].items():
        assert digest(ROOT / path) == expected, path
    assert digest(ROOT / "scripts/run_probes.py") == runtime["runner_sha256"]
    assert digest(HERE / "runtime-profile.json") == runtime["profile_sha256"]
    expected = {
        name
        for p in (HERE / "probes/tests").glob("*.rs")
        for name in re.findall(r"#\[(?:tokio::)?test\]\s*(?:async )?fn (\w+)", p.read_text())
    }
    assert set(runtime["passed_tests"]) == expected and len(expected) == 24
    record(
        "runtime",
        ["probe-results.json", "logs/runtime-1.log", "logs/runtime-2.log"],
        "python3 scripts/run_probes.py",
        "24 local named tests; source/fixture/lock/runner/profile verified",
    )
    access = json.loads((HERE / "compile-fail-results.json").read_text())
    assert access["state"] == "passed" and len(access["cases"]) == 2
    assert access["lock_sha256"] == digest(HERE / "probes/Cargo.lock")
    for case in access["cases"]:
        assert digest(ROOT / case["source"]) == case["source_sha256"]
        assert case["expected_error"] in (ROOT / case["log"]).read_text()
    record(
        "negative caller access",
        ["compile-fail-results.json"],
        "python3 scripts/check_access.py",
        "Two specific compiler errors; source and lock still match",
    )
    assert "Finished `dev` profile" in (HERE / "clippy.log").read_text()
    record(
        "probe Clippy",
        ["clippy.log"],
        "cargo +1.98.1 clippy --locked --offline --manifest-path "
        "skill_improvement/evidence/implementation/probes/Cargo.toml "
        "--target-dir skill_improvement/evidence/.build-target -j4 --all-targets -- -D warnings",
        "Probe crate all targets; upstream future-incompatibility notice retained",
    )
    assert (HERE / "ruff.log").read_text().strip() == "All checks passed!"
    record(
        "Python lint",
        ["ruff.log"],
        "uv run ruff check .claude/skills/deltalake/build .claude/skills/deltalake/scripts "
        ".claude/skills/deltalake/skill_improvement/evidence/implementation/verify_links.py "
        ".claude/skills/deltalake/skill_improvement/evidence/implementation/record_validation.py",
        "Active build/reader/verification Python; historical evaluator programs excluded",
    )
    links = json.loads((HERE / "links.json").read_text())
    assert links["state"] == "passed" and not links["failures"]
    record(
        "authored links",
        ["links.json"],
        "python3 skill_improvement/evidence/implementation/verify_links.py",
        f"{links['links']} links across {links['files']} authored documents; "
        "raw upstream links separate",
    )
    invalidation = json.loads((HERE / "invalidation.json").read_text())
    assert not invalidation["affected"] and not invalidation["changed_crates"]
    record(
        "current dependency bindings",
        ["invalidation.json"],
        "python3 scripts/invalidation.py",
        "No stale dependencies; mutation controls are in external distribution qualification",
    )
    sources = set(ROOT.glob("build/*.py")) | set(ROOT.glob("scripts/*.py"))
    sources.update(p for p in (ROOT / "authoring").rglob("*") if p.is_file())
    result = {
        "state": "passed",
        "recorded_at": datetime.now(UTC).isoformat(),
        "receipt_kind": "Verified aggregation of executed logs; not a new runtime execution",
        "command_note": "Commands are relative to the skill root unless prefixed .claude; "
        "Python commands were launched via uv run --no-project python.",
        "checks": records,
        "source_hashes": {str(p.relative_to(ROOT)): digest(p) for p in sorted(sources)},
        "distribution": "Adjacent qualification.json records the later distribution checks; "
        "excluded from this aggregation to avoid self-reference.",
        "not_run": [
            "live cloud/native services",
            "crash recovery and general distributed concurrency",
            "workload RSS/spill/performance",
            "fresh rustdoc production",
            "fresh-machine offline Cargo compilation",
        ],
    }
    (HERE / "validation.json").write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(
        json.dumps(
            {"state": result["state"], "checks": len(records), "source_hashes": len(sources)}
        )
        + "\n"
    )


if __name__ == "__main__":
    main()

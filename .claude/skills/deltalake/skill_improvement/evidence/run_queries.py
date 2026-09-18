"""Retain structural and text searches as replayable, explicitly limited evidence."""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL = HERE.parent.parent


def main() -> None:
    commands = [
        (
            "impls",
            [
                "ast-grep",
                "run",
                "-l",
                "rust",
                "-p",
                "impl $TRAIT for $TYPE { $$$BODY }",
                "--json=compact",
                "sources/delta-rs/crates/core/src",
            ],
        ),
        (
            "await-outputs",
            [
                "rg",
                "-n",
                "type Output =|pub (async )?fn (scan_table|scan_cdf|table_provider|write)"
                "|pub enum SessionFallbackPolicy",
                "sources/delta-rs/crates/core/src",
            ],
        ),
        (
            "transaction-phases",
            [
                "rg",
                "-n",
                "with_application_transaction|transaction_version|check_for_updated_application|write_commit_entry|before_post_commit_hook",
                "sources/delta-rs/crates/core/src",
            ],
        ),
        (
            "protocol-admission",
            [
                "rg",
                "-n",
                "reader_features.insert|writer_features.insert|can_read_from|can_write_to|cfg\\(feature",
                "sources/delta-rs/crates/core/src/kernel/transaction/protocol.rs",
            ],
        ),
        (
            "syntax-controls",
            [
                "ast-grep",
                "scan",
                "--rule",
                "queries/baseline-write-rule.yml",
                "--json=compact",
                "query-fixture.rs",
            ],
        ),
    ]
    runs = []
    (HERE / "queries").mkdir(exist_ok=True)
    for name, command in commands:
        done = subprocess.run(command, cwd=HERE, capture_output=True, text=True, check=False)
        path = (
            HERE / "queries" / f"{name}.json"
            if command[0] == "ast-grep"
            else HERE / "queries" / f"{name}.txt"
        )
        path.write_text(done.stdout)
        (HERE / "queries" / f"{name}.stderr.txt").write_text(done.stderr)
        row = {
            "id": name,
            "argv": [a.replace(str(SKILL), "<skill>") for a in command],
            "exit_code": done.returncode,
            "output": str(path.relative_to(HERE)),
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        }
        if command[0] == "ast-grep" and done.returncode == 0:
            matches = json.loads(done.stdout)
            row["matches"] = len(matches)
            if name == "syntax-controls":
                if len(matches) != 2:
                    raise RuntimeError(f"Expected two syntax matches, got {len(matches)}")
                row["interpretation"] = (
                    "One Delta lead and one unrelated same-name match; "
                    "with_commit_properties(default) escapes the rule "
                    "without establishing replay protection."
                )
        runs.append(row)
        if done.returncode:
            raise RuntimeError(done.stderr)
    (HERE / "query-runs.json").write_text(json.dumps(runs, indent=2) + "\n")
    sys.stdout.write(str([(r["id"], r.get("matches", "text")) for r in runs]) + "\n")


if __name__ == "__main__":
    main()

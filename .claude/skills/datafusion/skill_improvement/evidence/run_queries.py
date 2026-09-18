"""Retain structural query candidates and a false-attribution control."""

from __future__ import annotations

import json

from collect_evidence import HERE, SKILL, command, write_json


def main() -> None:
    recipes = [
        ("contract-query-tests", ["ast-grep", "test", "-c", "queries/sgconfig.yml"], HERE),
        (
            "contract-source-query",
            ["ast-grep", "scan", "-c", "queries/sgconfig.yml", "--json=compact", "sources"],
            HERE,
        ),
        (
            "collect-attribution-control",
            [
                "ast-grep",
                "scan",
                "-c",
                "queries/sgconfig.yml",
                "--filter",
                "^project-collect-over-stream$",
                "--json=compact",
                "skill_improvement/evidence/queries/non_datafusion_collect.rs",
            ],
            SKILL,
        ),
    ]
    results = [command(name, argv, cwd) for name, argv, cwd in recipes]
    for result in results:
        if result["exit_code"] != 0:
            raise RuntimeError(f"Query failed: {result}")
    matches = json.loads((HERE / "logs/contract-source-query.log").read_text())
    control = json.loads((HERE / "logs/collect-attribution-control.log").read_text())
    if not matches or len(control) != 1:
        raise AssertionError("Expected source candidates and one non-DataFusion collect match")
    tests = (HERE / "logs/contract-query-tests.log").read_text()
    if "1 passed; 0 failed" not in tests:
        raise AssertionError("Expected one executed rule group with no failures")
    write_json(
        HERE / "query-runs.json",
        {
            "commands": results,
            "source_candidate_count": len(matches),
            "non_datafusion_collect_matches": len(control),
            "interpretation": "AST matches locate declarations; the collect hint does not resolve a receiver type.",
        },
    )
    print(f"Retained {len(matches)} source candidates and one false-attribution control.")


if __name__ == "__main__":
    main()

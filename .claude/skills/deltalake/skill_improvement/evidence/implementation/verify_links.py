"""Verify authored reference links; captured upstream links have separate raw diagnostics."""

from __future__ import annotations

import json
import re
import sys
from datetime import UTC, datetime
from pathlib import Path
from urllib.parse import unquote

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def main() -> None:
    paths = {ROOT / p for p in ["SKILL.md", "reference.md", "MAINTENANCE.md", "scripts/README.md"]}
    paths.update((ROOT / "skill_improvement").glob("*.md"))
    paths.update((ROOT / "skill_improvement/examples").glob("*.md"))
    paths.update([HERE / "README.md", HERE.parent / "README.md"])
    for folder in ["capabilities", "routes", "integration"]:
        paths.update((ROOT / "content" / folder).glob("*.md"))
    paths.update((HERE / "evaluation").rglob("REPORT.md"))
    count = 0
    failures = []
    for path in sorted(paths):
        if not path.is_file():
            failures.append(str(path.relative_to(ROOT)))
            continue
        body = re.sub(r"```.*?```", "", path.read_text(), flags=re.S)
        for target in re.findall(r"\[[^\]\n]*\]\(([^)\s]+)\)", body):
            if re.match(r"(?:https?|mailto|app):", target) or target.startswith("#"):
                continue
            location = unquote(target.split("#", 1)[0].strip("<>"))
            if not (path.parent / location).is_file():
                failures.append({"from": str(path.relative_to(ROOT)), "target": target})
            count += 1
    result = {
        "state": "failed" if failures else "passed",
        "checked_at": datetime.now(UTC).isoformat(),
        "files": len(paths),
        "links": count,
        "failures": failures,
    }
    (HERE / "links.json").write_text(json.dumps(result, indent=2) + "\n")
    sys.stdout.write(json.dumps(result) + "\n")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()

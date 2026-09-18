"""Compare retained rustdoc documentation to the generated reading surface."""

from __future__ import annotations

import hashlib
import json
from compression import zstd
from pathlib import Path

HERE = Path(__file__).resolve().parent
SKILL = HERE.parent.parent
QUERIES = {
    "datafusion-55.1.0": {"collect", "execute_stream", "cache"},
    "datafusion-session-55.1.0": {"scan", "supports_filters_pushdown"},
    "arrow-row-59.3.0": {"convert_columns", "convert_rows"},
    "arrow-array-59.3.0": {"try_new", "with_schema"},
}


def main() -> None:
    report = []
    fragments = []
    for release, names in QUERIES.items():
        path = SKILL / "build/.cache/rustdoc" / f"{release}.json.zst"
        raw = json.loads(zstd.decompress(path.read_bytes()))
        local = [item for item in raw["index"].values() if item.get("crate_id") == 0]
        report.append(
            {
                "release": release,
                "artifact_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                "format_version": raw["format_version"],
                "documented_modules_in_raw": sum(
                    "module" in item["inner"] and bool(item.get("docs")) for item in local
                ),
                "matching_documented_functions": sum(
                    "function" in item["inner"]
                    and item.get("name") in names
                    and bool(item.get("docs"))
                    for item in local
                ),
                "scope": "Selected raw rustdoc documents, not a full public-method coverage count",
            }
        )
        for item in local:
            if "function" in item["inner"] and item.get("name") in names and item.get("docs"):
                fragments.append(
                    {
                        "release": release,
                        "artifact_local_id": item["id"],
                        "name": item["name"],
                        "span": item.get("span"),
                        "docs": item["docs"],
                        "function": item["inner"]["function"],
                    }
                )
    (HERE / "documentation-audit.json").write_text(json.dumps(report, indent=2) + "\n")
    (HERE / "rustdoc-fragments.json").write_text(json.dumps(fragments, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()

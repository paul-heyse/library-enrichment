"""Check evidence integrity, local document links, crate coverage, and preservation."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from urllib.parse import unquote, urlparse

HERE = Path(__file__).resolve().parent
BUNDLE = HERE.parent
SKILL = BUNDLE.parent


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def anchors(path: Path) -> set[str]:
    result = set()
    occurrences: dict[str, int] = {}
    for line in path.read_text().splitlines():
        if not re.match(r"^#{1,6} ", line):
            continue
        text = re.sub(r"^#+ +", "", line).lower()
        text = re.sub(r"[^\w\- ]", "", text).replace(" ", "-")
        count = occurrences.get(text, 0)
        occurrences[text] = count + 1
        result.add(f"{text}-{count}" if count else text)
    return result


def main() -> None:
    problems = []
    baseline = json.loads((HERE / "inventory.json").read_text())
    for relative, expected in baseline["input_hashes"].items():
        path = SKILL / relative
        if not path.exists() or sha256(path) != expected:
            problems.append(f"Existing skill changed since inventory: {relative}")
    manifest = json.loads((HERE / "source-manifest.json").read_text())
    source_count = 0
    for package in manifest:
        for item in package["files"]:
            path = HERE / item["path"]
            if not path.exists() or sha256(path) != item["sha256"]:
                problems.append(f"Source digest mismatch: {item['path']}")
            source_count += 1
    route_lines = (BUNDLE / "examples/crate-roles.tsv").read_text().splitlines()[1:]
    routes = [line.split("\t") for line in route_lines]
    if any(len(row) != 4 for row in routes):
        problems.append("A crate route does not have four columns")
    names = [row[0] for row in routes]
    if len(names) != len(set(names)) or set(names) != set(baseline["symbols_by_crate"]):
        problems.append("Crate roles do not cover the inventory exactly once")
    documents = [
        p
        for p in BUNDLE.rglob("*.md")
        if "sources" not in p.parts and ".build-target" not in p.parts
    ]
    checked_links = 0
    for document in documents:
        raw = document.read_text()
        if any(line.rstrip() != line for line in raw.splitlines()):
            problems.append(f"Trailing whitespace: {document.relative_to(BUNDLE)}")
        text = re.sub(r"```.*?```", "", raw, flags=re.DOTALL)
        for href in re.findall(r"\[[^\]]+\]\(([^)\n]+)\)", text):
            if urlparse(href).scheme:
                continue
            relative, _, anchor = unquote(href).partition("#")
            target = (document.parent / relative).resolve() if relative else document
            if not target.exists():
                problems.append(f"Missing link in {document.name}: {href}")
            elif (
                anchor
                and target.is_file()
                and target.suffix == ".md"
                and anchor not in anchors(target)
            ):
                problems.append(f"Missing anchor in {document.name}: {href}")
            checked_links += 1
    run = json.loads((HERE / "rust-probe-run.json").read_text())
    if run["status"] != "passed" or run["exit_code"] != 0:
        problems.append("Latest Rust probe run did not pass")
    if sha256(HERE / run["log"]) != run["log_sha256"]:
        problems.append("Rust log digest mismatch")
    if sha256(HERE / "probes/Cargo.lock") != run["lock_sha256"]:
        problems.append("Rust lock digest mismatch")
    result = {
        "status": "failed" if problems else "passed",
        "original_skill_files_unchanged": len(baseline["input_hashes"]),
        "source_files_checked": source_count,
        "crate_roles": len(names),
        "authored_markdown_documents": len(documents),
        "local_links_checked": checked_links,
        "problems": problems,
        "limits": "Checks reference integrity; does not establish comparative agent decision quality.",
    }
    (HERE / "bundle-validation.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
    raise SystemExit(bool(problems))


if __name__ == "__main__":
    main()

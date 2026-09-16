"""Write the model out as prose pages, machine records and a provenance digest.

Two rules carried over from the Rust skills, both load-bearing:

* **One fact is stored once.** Prose lives in `api/`, structure lives in `model/`, and the model
  record points at the page rather than repeating it. Nothing is duplicated, so nothing can
  disagree with itself.
* **File names are a rule, not a lookup.** A module path with `.` kept becomes the file name, so
  a reader who knows `fastmcp.server.server` knows the page is `api/fastmcp.server.server.md`
  without consulting an index.
"""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from datetime import UTC, datetime
from pathlib import Path

from model import NOISE_MEMBERS, PYDANTIC_MEMBERS, Item

MEMBERS_SHOWN = 60


def module_slug(module_path: str) -> str:
    return module_path


def model_file(module_path: str) -> str:
    return f"model/{module_slug(module_path)}.json"


def api_file(module_path: str) -> str:
    return f"api/{module_slug(module_path)}.md"


def doc_anchor(item: Item) -> str:
    return f"{api_file(item.module)}#{item.name.lower()}"


def exported_aliases(item: Item, exports: dict[str, set[str]]) -> list[str]:
    """The spellings a reader should be offered: declared re-exports, not import sites."""
    return [
        alias for alias in item.aliases if item.name in exports.get(alias.rsplit(".", 1)[0], ())
    ]


def group_by_module(items: dict[str, Item]) -> dict[str, list[Item]]:
    grouped: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        grouped[item.module].append(item)
    return {
        module: sorted(members, key=lambda i: (i.kind, i.name))
        for module, members in sorted(grouped.items())
    }


def _interesting_members(item: Item) -> list[dict]:
    return [
        member
        for member in item.members
        if member["name"] not in NOISE_MEMBERS
        and member["name"] not in PYDANTIC_MEMBERS
        and not member["name"].startswith("_")
    ]


def _split_members(item: Item) -> tuple[list[dict], dict[str, list[dict]]]:
    """Members this class declares, and the rest grouped by where they come from."""
    declared: list[dict] = []
    inherited: dict[str, list[dict]] = {}
    for member in _interesting_members(item):
        origin = member.get("declared_on", item.path)
        if origin == item.path:
            declared.append(member)
        else:
            inherited.setdefault(origin, []).append(member)
    return declared, inherited


def _item_as_dict(item: Item) -> dict:
    record: dict[str, object] = {
        "path": item.path,
        "preferred": item.preferred,
        "name": item.name,
        "kind": item.kind,
        "dist": item.dist,
        "signature": item.signature,
        "summary": item.summary,
        "nameable": item.nameable,
        "doc": doc_anchor(item) if not item.boundary else None,
    }
    for key, value in (
        ("labels", item.labels),
        ("aliases", item.aliases),
        ("bases", item.bases),
        ("overloads", item.overloads),
        ("decorators", item.decorators),
        ("types_used", item.types_used),
    ):
        if value:
            record[key] = value
    if item.inferred:
        record["inferred"] = item.inferred
        record["inferred_by"] = "ty"
    if item.protocol_era:
        record["protocol_era"] = item.protocol_era
    if item.boundary:
        record["boundary"] = True
    if item.dispatch_only:
        record["dispatch_only"] = True
    declared, inherited = _split_members(item)
    if declared:
        record["members"] = [
            {
                key: member[key]
                for key in ("name", "kind", "signature", "summary", "labels")
                if member.get(key)
            }
            for member in declared
        ]
    if inherited:
        # By origin, names only. The full flattened set lives in `index/members.tsv`; repeating
        # every base's signatures on every descendant tripled the emitted tree for no new fact.
        record["inherits"] = {
            origin: [member["name"] for member in members]
            for origin, members in sorted(inherited.items())
        }
    return record


def write_model(grouped: dict[str, list[Item]], root: Path) -> int:
    root.joinpath("model").mkdir(parents=True, exist_ok=True)
    written = 0
    for module, members in grouped.items():
        document = {
            "module": module,
            "dist": members[0].dist if members else "",
            "api": api_file(module),
            "items": [_item_as_dict(item) for item in members],
        }
        root.joinpath(model_file(module)).write_text(
            json.dumps(document, indent=1, sort_keys=True) + "\n"
        )
        written += 1
    return written


def _member_lines(item: Item) -> list[str]:
    declared, inherited = _split_members(item)
    lines: list[str] = []
    if declared:
        lines += ["", f"**Declared members ({len(declared)})**", ""]
        for member in declared[:MEMBERS_SHOWN]:
            labels = ", ".join(member.get("labels", []))
            suffix = f"  _{labels}_" if labels else ""
            lines.append(f"- `{member['signature']}`{suffix}")
            if member.get("summary"):
                lines.append(f"  {member['summary']}")
        if len(declared) > MEMBERS_SHOWN:
            lines.append(f"- … and {len(declared) - MEMBERS_SHOWN} more, see `model/`")
    if inherited:
        total = sum(len(members) for members in inherited.values())
        lines += ["", f"**Inherited ({total})**", ""]
        for origin, members in sorted(inherited.items()):
            names = ", ".join(
                f"`{member['name']}`" for member in sorted(members, key=lambda m: m["name"])
            )
            lines.append(f"- from `{origin}`: {names}")
        lines.append("")
        lines.append(
            "Signatures for inherited members are on the base's own page, and every one of them "
            "is a row in `content/index/members.tsv`."
        )
    return lines


def write_api(grouped: dict[str, list[Item]], root: Path, exports: dict[str, set[str]]) -> int:
    root.joinpath("api").mkdir(parents=True, exist_ok=True)
    written = 0
    for module, members in grouped.items():
        lines = [f"# `{module}`", ""]
        dist = members[0].dist if members else ""
        if dist:
            lines += [f"Distribution: `{dist}`", ""]
        for item in members:
            lines.append(f"## {item.name}")
            lines.append("")
            if item.preferred and item.preferred != item.path:
                lines.append(f"Import as `{item.preferred}`  ·  defined at `{item.path}`")
            else:
                lines.append(f"`{item.path}`")
            if not item.nameable:
                lines.append("")
                lines.append(
                    "> Not nameable: every path to this item passes through a private module. "
                    "It is real and reachable at runtime, but you cannot import or annotate it."
                )
            lines += ["", "```python", item.signature, "```"]
            if item.overloads:
                lines += ["", "**Overloads** (the signature above is the runtime dispatcher):", ""]
                lines += [f"- `{signature}`" for signature in item.overloads]
            if item.inferred:
                lines += [
                    "",
                    f"**Inferred type** (`ty`, not declared in the source): `{item.inferred}`",
                ]
            public = exported_aliases(item, exports)
            if public:
                shown = ", ".join(f"`{alias}`" for alias in public[:8])
                more = f" +{len(public) - 8}" if len(public) > 8 else ""
                lines += ["", f"**Also exported as** {shown}{more}"]
            incidental = len(item.aliases) - len(public)
            if incidental:
                lines += [
                    "",
                    f"_{incidental} further import-site paths reach this item; they work but "
                    "are not declared API. See `content/index/aliases.tsv`._",
                ]
            if item.bases:
                lines += ["", f"**Bases** {', '.join(f'`{b}`' for b in item.bases)}"]
            lines += _member_lines(item)
            if item.docs:
                lines += ["", item.docs.strip(), ""]
            lines.append("")
        root.joinpath(api_file(module)).write_text("\n".join(lines) + "\n")
        written += 1
    return written


def digest_tree(root: Path) -> dict[str, str]:
    digests: dict[str, str] = {}
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.name == "PROVENANCE.json":
            continue
        relative = path.relative_to(root).as_posix()
        digests[relative] = hashlib.sha256(path.read_bytes()).hexdigest()
    return digests


def write_provenance(
    root: Path,
    manifest: dict,
    acquisition: dict,
    counts: dict[str, int],
    tools: dict[str, str],
) -> None:
    record = {
        "repository": manifest["repository"]["name"],
        # Date only: a rebuild on the same day must be byte-identical, and a timestamp would
        # make the determinism check fail for the wrong reason.
        "generated_at": datetime.now(tz=UTC).date().isoformat(),
        "pinned": acquisition["resolved"],
        "envelope_key": acquisition["envelope_key"],
        "acquisition_tools": acquisition["tools"],
        "semantic": {
            "status": acquisition["semantic"]["status"],
            "version": acquisition["semantic"].get("version", ""),
            "asked": acquisition["semantic"].get("asked", 0),
            "answered": len(acquisition["semantic"].get("rows", [])),
        },
        "tools": tools,
        "counts": dict(sorted(counts.items())),
        "files": digest_tree(root),
    }
    root.joinpath("PROVENANCE.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")

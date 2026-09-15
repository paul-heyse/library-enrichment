"""Restructure the upstream generated catalogs into navigable indexes.

Three catalogs, each solving a different lookup that the raw upstream file makes awkward.

`configs.md` is a 204 KB flat table of 155 settings. Useful once you know the setting name;
useless for "what can I tune", and it never says how to reach a setting from Rust. This joins each
setting to the `SessionConfig` builder method that sets it, which is the missing half.

The SQL function docs are 327 functions across three files. This produces one categorized index so
a single grep answers "is there a function for X".

The crate map answers the question the facade crate makes hard: which of the 60 crates owns what.
"""

from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

from model import Item

CONFIG_ROW = re.compile(r"^\|\s*(datafusion\.[a-z0-9_.]+)\s*\|(.*?)\|(.*)\|\s*$")
FUNCTION_HEADING = re.compile(r"^###\s+`?([A-Za-z0-9_ ]+?)`?\s*$")
SECTION_HEADING = re.compile(r"^##\s+(.+?)\s*$")

FUNCTION_SOURCES = (
    ("Scalar", "corpus/guides/user-guide/sql/scalar_functions.md"),
    ("Aggregate", "corpus/guides/user-guide/sql/aggregate_functions.md"),
    ("Window", "corpus/guides/user-guide/sql/window_functions.md"),
    ("Operators", "corpus/guides/user-guide/sql/operators.md"),
)

SUBSYSTEMS = {
    "datafusion.catalog": "Catalog and schema defaults",
    "datafusion.execution": "Execution: batching, parallelism, file formats, memory",
    "datafusion.explain": "EXPLAIN output",
    "datafusion.format": "Output formatting",
    "datafusion.optimizer": "Optimizer and physical planning",
    "datafusion.runtime": "RuntimeEnv: memory pool and temporary storage",
    "datafusion.spark": "Spark compatibility",
    "datafusion.sql_parser": "SQL dialect and parsing",
}


def _setters(items: dict[str, Item], owner_suffix: str) -> dict[str, str]:
    """Map a builder method name to its rendered signature, for one owning type."""
    found: dict[str, str] = {}
    for item in items.values():
        if not item.path.endswith(owner_suffix):
            continue
        for method in item.methods:
            if method.name.startswith("with_") and not method.via_trait:
                found.setdefault(method.name, method.signature)
    return found


def write_config_options(items: dict[str, Item], content: Path) -> int:
    source = content.joinpath("corpus/guides/user-guide/configs.md")
    if not source.exists():
        return 0

    rows: list[tuple[str, str, str]] = []
    for line in source.read_text().splitlines():
        match = CONFIG_ROW.match(line)
        if match:
            rows.append((match.group(1).strip(), match.group(2).strip(), match.group(3).strip()))

    session = _setters(items, "::SessionConfig")
    runtime = _setters(items, "::RuntimeEnvBuilder")

    grouped: dict[str, list[tuple[str, str, str]]] = defaultdict(list)
    for key, default, description in rows:
        prefix = ".".join(key.split(".")[:2])
        grouped[prefix].append((key, default, description))

    lines = [
        "# Configuration settings",
        "",
        f"{len(rows)} settings, grouped by subsystem, each joined to the Rust builder method that",
        "sets it where one exists. Upstream's own table is the source and stays authoritative for",
        "defaults and descriptions: [`corpus/guides/user-guide/configs.md`]"
        "(../corpus/guides/user-guide/configs.md).",
        "",
        "Three ways to reach a setting, in order of how specific they are:",
        "",
        "```rust",
        "SessionConfig::new().with_batch_size(4096)                  // typed builder method",
        'SessionConfig::new().set_bool("datafusion.execution.collect_statistics", true)',
        'ctx.sql("SET datafusion.execution.batch_size = 4096")       // at runtime, from SQL',
        "```",
        "",
        "A setting with no builder method is reachable only through the string form.",
        "",
    ]

    for prefix in sorted(grouped):
        lines.append(f"## `{prefix}`")
        lines.append("")
        lines.append(SUBSYSTEMS.get(prefix, ""))
        lines.append("")
        lines.append("| Setting | Default | Rust setter | Description |")
        lines.append("|---|---|---|---|")
        for key, default, description in sorted(grouped[prefix]):
            leaf = key.split(".")[-1]
            candidate = f"with_{leaf}"
            if candidate in session:
                setter = f"`SessionConfig::{candidate}`"
            elif candidate in runtime:
                setter = f"`RuntimeEnvBuilder::{candidate}`"
            else:
                setter = "—"
            clean = description.replace("|", "\\|").strip()
            if len(clean) > 300:
                clean = clean[:299].rstrip() + "…"
            lines.append(f"| `{key}` | {default or '—'} | {setter} | {clean} |")
        lines.append("")

    content.joinpath("catalogs").mkdir(parents=True, exist_ok=True)
    content.joinpath("catalogs/config-options.md").write_text("\n".join(lines))
    return len(rows)


def write_sql_functions(content: Path) -> int:
    """One categorized index across the scalar, aggregate, window and operator docs."""
    entries: list[tuple[str, str, str, str]] = []
    for family, relative in FUNCTION_SOURCES:
        source = content.joinpath(relative)
        if not source.exists():
            continue
        section = ""
        for line in source.read_text().splitlines():
            heading = SECTION_HEADING.match(line)
            if heading:
                section = heading.group(1)
                continue
            function = FUNCTION_HEADING.match(line)
            if function:
                name = function.group(1).strip()
                anchor = name.lower().replace(" ", "-").replace("_", "_")
                entries.append((family, section, name, f"{relative}#{anchor}"))

    grouped: dict[tuple[str, str], list[tuple[str, str]]] = defaultdict(list)
    for family, section, name, link in entries:
        grouped[(family, section)].append((name, link))

    lines = [
        "# SQL functions and operators",
        "",
        f"{len(entries)} entries across scalar, aggregate and window functions and operators, from",
        "upstream's generated documentation. Each links to the full description, signature and",
        "examples in the source page.",
        "",
        "These are the SQL-callable surface. The Rust `Expr` equivalents live under the `expr_fn`",
        "modules — search `content/index/symbols.tsv` for `expr_fn` to find them, and note that",
        "`datafusion::prelude` re-exports many, so the access path differs from the defining path.",
        "",
        "Registering your own is `SessionContext::register_udf`, `register_udaf` or",
        "`register_udwf`; see [`../traits/ScalarUDFImpl.md`](../traits/ScalarUDFImpl.md).",
        "",
    ]

    for family, _ in FUNCTION_SOURCES:
        sections = sorted(key for key in grouped if key[0] == family)
        if not sections:
            continue
        total = sum(len(grouped[key]) for key in sections)
        lines.append(f"## {family} ({total})")
        lines.append("")
        for key in sections:
            names = sorted(grouped[key])
            lines.append(f"**{key[1]}** — {len(names)}")
            lines.append("")
            lines.append(", ".join(f"[`{name}`](../{link})" for name, link in names))
            lines.append("")

    content.joinpath("catalogs").mkdir(parents=True, exist_ok=True)
    content.joinpath("catalogs/sql-functions.md").write_text("\n".join(lines))
    return len(entries)


def write_crate_map(items: dict[str, Item], content: Path) -> int:
    """What each of the pinned crates owns, which the facade crate deliberately hides."""
    by_crate: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        by_crate[item.crate].append(item)

    lines = [
        "# Crate map",
        "",
        f"{len(by_crate)} crates hold the pinned surface. The `datafusion` facade documents 1,376",
        "items of its own and re-exports the rest, so the crate an item is *reached* through is",
        "usually not the crate that defines it. Resolve access paths through",
        "[`../index/aliases.tsv`](../index/aliases.tsv) before attributing an item to a crate.",
        "",
        "| Crate | Items | Traits | Notable extension points |",
        "|---|---:|---:|---|",
    ]

    for crate in sorted(by_crate):
        members = by_crate[crate]
        traits = [item for item in members if item.kind == "trait"]
        # Rank by how many types implement it: the most-implemented trait in a crate is the one
        # most likely to be the thing that crate exists to let you plug into.
        ranked = sorted(traits, key=lambda item: -len(item.implementors))
        notable = [item.name for item in ranked if item.implementors][:4]
        lines.append(
            f"| `{crate}` | {len(members)} | {len(traits)} | "
            f"{', '.join(f'`{n}`' for n in sorted(notable)) or '—'} |"
        )

    lines.append("")
    content.joinpath("catalogs").mkdir(parents=True, exist_ok=True)
    content.joinpath("catalogs/crate-map.md").write_text("\n".join(lines))
    return len(by_crate)

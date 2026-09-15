"""Write one page per extension point.

The module pages answer "what is in this module". This answers the different question an agent
actually has: "can I plug into this, how much work is it, and who already did it".

Required and provided methods are separated deliberately. That split decides how much work an
implementation is, and it is invisible in a flat method list -- which is exactly why a reader
working from rustdoc alone underestimates what a trait offers. The provided methods are where the
capability hides: the default is almost always the conservative answer rather than the good one.
"""

from __future__ import annotations

from collections import defaultdict
from pathlib import Path

from emit import api_file, model_file
from model import Item

EXTENSION_POINTS = (
    "Accumulator",
    "AnalyzerRule",
    "CatalogProvider",
    "CatalogProviderList",
    "DataSink",
    "ExecutionPlan",
    "ExprPlanner",
    "ExtensionPlanner",
    "FileFormat",
    "FileOpener",
    "FunctionFactory",
    "GroupsAccumulator",
    "ObjectStore",
    "OptimizerRule",
    "PartitionEvaluator",
    "PhysicalExtensionCodec",
    "PhysicalOptimizerRule",
    "PruningStatistics",
    "PruningStatistics",
    "QueryPlanner",
    "RelationPlanner",
    "ScalarUDFImpl",
    "SchemaProvider",
    "Session",
    "TableFunctionImpl",
    "TableProvider",
    "TypePlanner",
    "UserDefinedLogicalNode",
    "UserDefinedLogicalNodeCore",
    "WindowUDFImpl",
    "AggregateUDFImpl",
)


def write_traits(items: dict[str, Item], root: Path, examples: dict[str, list[str]]) -> int:
    traits_dir = root.joinpath("traits")
    traits_dir.mkdir(parents=True, exist_ok=True)

    by_name: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        if item.kind == "trait" and item.name in EXTENSION_POINTS:
            by_name[item.name].append(item)

    written = 0
    for name in sorted(by_name):
        # A name can be defined more than once across the pinned set; prefer the documented one.
        item = max(by_name[name], key=lambda entry: len(entry.docs))
        required_names = set(item.required_methods)
        provided_names = set(item.provided_methods)

        lines = [
            f"# {item.name}",
            "",
            f"`{item.path}`",
            "",
            "```rust",
            item.signature,
            "```",
            "",
        ]
        if item.aliases:
            lines.append("Also reachable as " + ", ".join(f"`{a}`" for a in item.aliases))
            lines.append("")

        prose = api_file(item.module)
        records = model_file(item.module)
        lines.append(
            f"Prose: [`{prose}`](../{prose}#{name.lower()}) · records: [`{records}`](../{records})"
        )
        lines.append("")

        required = sorted(
            (m for m in item.methods if m.name in required_names), key=lambda m: m.name
        )
        if required:
            lines += ["## Required", "", "Every implementation must supply these.", "", "```rust"]
            lines += [m.signature for m in required]
            lines += ["```", ""]

        provided = sorted(
            (m for m in item.methods if m.name in provided_names), key=lambda m: m.name
        )
        if provided:
            lines += [
                "## Provided",
                "",
                "Defaulted, and this is where the capability hides. The default is the "
                "conservative answer -- no pushdown, no statistics, no specialization -- so an "
                "implementation that overrides none of these works correctly and performs badly.",
                "",
                "```rust",
            ]
            lines += [m.signature for m in provided]
            lines += ["```", ""]

        if item.implementors:
            lines += [
                f"## Implementors ({len(item.implementors)})",
                "",
                "Read one before writing your own.",
                "",
            ]
            lines += [f"- `{implementor}`" for implementor in item.implementors]
            lines.append("")

        demos = sorted(examples.get(name) or [])
        if demos:
            lines += [f"## Demonstrated by {len(demos)} upstream example(s)", ""]
            lines += [f"- [`{path}`](../{path})" for path in demos]
            lines.append("")

        if item.docs:
            lines += ["## Documentation", "", item.docs.rstrip(), ""]

        traits_dir.joinpath(f"{name}.md").write_text("\n".join(lines))
        written += 1
    return written

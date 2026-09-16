"""Restructure the model into lookup catalogs.

A catalog answers a question directly rather than by navigation: which operations exist, which
table properties a writer honours, which protocol features are modelled, which errors a caller
must handle, and where this library plugs into somebody else's traits.

Every catalog here is derived from the model. None parses upstream prose, because upstream
prose about a moving commit is a lead, not a source -- `docs/feature-table.md` sits in the
corpus as a cross-check, and where the two disagree the model is what shipped.
"""

from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

from model import Item

# The one idiom the whole operation surface is built on: every builder is a future you
# configure and then await. Finding them structurally beats maintaining a list by hand.
FUTURE_TRAIT_SUFFIX = "IntoFuture"


def write_all(
    items: dict[str, Item],
    facts: dict[str, dict],
    hidden_impls: list[tuple[str, str]],
    content: Path,
) -> dict[str, int]:
    content.joinpath("catalogs").mkdir(parents=True, exist_ok=True)
    return {
        "operations": write_operations(items, content),
        "table_properties": write_table_properties(items, content),
        "table_features": write_table_features(items, content),
        "errors": write_errors(items, content),
        "foreign_impls": write_foreign_impls(items, hidden_impls, content),
        "crates": write_crate_map(items, facts, content),
    }


def _write(content: Path, name: str, lines: list[str]) -> None:
    content.joinpath("catalogs", name).write_text("\n".join(lines) + "\n")


def _find(items: dict[str, Item], *, name: str, kind: str | None = None) -> list[Item]:
    return [i for i in items.values() if i.name == name and (kind is None or i.kind == kind)]


def _leaf(path: str) -> str:
    return path.rsplit("::", 1)[-1]


# --------------------------------------------------------------------------- operations


def write_operations(items: dict[str, Item], content: Path) -> int:
    """Every operation builder, the entry point that constructs it, and how to configure it.

    Builders are found by their `IntoFuture` implementation rather than by name, so an
    operation added upstream appears here without anyone editing a list.
    """
    builders = sorted(
        (
            item
            for item in items.values()
            if item.kind == "struct"
            and any(_leaf(t) == FUTURE_TRAIT_SUFFIX for t in item.implements)
        ),
        key=lambda item: item.name,
    )

    # Which entry-point method hands back each builder, derived from return types rather than
    # assumed. That matters more than it sounds: the `DeltaOps(table).delete()` front door that
    # every older write-up teaches no longer exists at this pin -- the operations are inherent
    # methods on `DeltaTable` now -- and only a generated map notices that.
    constructors: dict[str, list[str]] = defaultdict(list)
    for owner in items.values():
        for method in owner.methods:
            # `clone` and `default` return Self; they construct nothing a caller starts from.
            if method.name in {"clone", "default", "new"} or method.via_trait:
                continue
            returns = method.signature.split("->")[-1]
            for builder in builders:
                # Word-bounded: a plain substring test makes `CdfLoadBuilder` look like a
                # constructor of `LoadBuilder`, and the resulting row is quietly wrong.
                if re.search(rf"\b{re.escape(builder.name)}\b", returns):
                    constructors[builder.path].append(f"{owner.name}::{method.name}")

    lines = [
        "# Operations",
        "",
        f"{len(builders)} operation builders. Every one implements `IntoFuture`, so the shape is",
        "always the same: take a builder from an entry point, chain `with_*` calls, then await it.",
        "Nothing runs until the await.",
        "",
        "```rust",
        "let (table, metrics) = table.delete().with_predicate(expr).await?;",
        "```",
        "",
        "A builder with no `with_*` call still runs -- with defaults chosen for safety rather",
        "than for your workload. The configuration column is where the capability is.",
        "",
        "| Operation | Constructed by | Configuration | Defined in |",
        "|---|---|---:|---|",
    ]

    for builder in builders:
        options = sorted({m.name for m in builder.methods if m.name.startswith("with_")})
        made_by = sorted(set(constructors.get(builder.path, [])))
        entry = ", ".join(f"`{c}`" for c in made_by[:3]) or "—"
        lines.append(
            f"| `{builder.name}` | {entry} | {len(options)} | "
            f"[`{builder.crate}`](../api/{_api_ref(builder)}) |"
        )

    # A builder defined in a private module with no re-export is reachable by method call but
    # cannot be named, imported, or written in a signature. rustdoc emits the type without its
    # impls, so it also appears here with zero configuration -- which is a reporting artefact,
    # not a builder that takes no options. Saying so beats letting the zero speak for itself.
    unnameable = [b for b in builders if not b.aliases]
    if unnameable:
        lines += [
            "",
            "## Reachable but not nameable",
            "",
            "These builders are returned by a public method but live in a private module with no",
            "re-export. You can call and await one; you cannot `use` it or name it in a signature.",
            "rustdoc omits the impls of such a type, so their configuration count above reads 0",
            "whatever the source says -- read the source before concluding one takes no options.",
            "",
        ]
        for builder in unnameable:
            lines.append(f"- `{builder.path}`")

    lines += ["", "## Configuration by operation", ""]
    for builder in builders:
        options = sorted({m.name for m in builder.methods if m.name.startswith("with_")})
        if not options:
            continue
        lines.append(f"**`{builder.name}`** — {builder.summary or 'no summary'}")
        lines.append("")
        lines.append("".join(f"`{name}` " for name in options).strip())
        lines.append("")

    _write(content, "operations.md", lines)
    return len(builders)


def _api_ref(item: Item) -> str:
    return item.module.replace("::", ".") + ".md#" + item.name.lower()


# --------------------------------------------------------------------------- properties


def write_table_properties(items: dict[str, Item], content: Path) -> int:
    """The `delta.*` table properties this implementation models, and how to read them."""
    owners = _find(items, name="TableProperty", kind="enum")
    config = _find(items, name="TablePropertiesExt")
    variants = sorted({v for owner in owners for v in owner.variants})
    accessors = sorted(
        {m.name for item in config for m in item.methods if not m.name.startswith("_")}
    )

    lines = [
        "# Table properties",
        "",
        f"{len(variants)} properties are modelled as `TableProperty` variants. They are stored in",
        "the table metadata as `delta.*` strings, so a property this build does not model is not",
        "an error at write time -- it is simply ignored, which is the failure worth knowing about.",
        "",
        "Set them with `DeltaTable::set_tbl_properties`, or at creation with",
        "`CreateBuilder::with_configuration`. Read them back through `TablePropertiesExt`",
        f"({len(accessors)} accessors), which is an extension trait on the kernel's",
        "`TableProperties` -- so the accessors are invisible until you import it.",
        "",
        "| Property variant |",
        "|---|",
    ]
    lines += [f"| `{v}` |" for v in variants]

    if accessors:
        lines += ["", "## Reading them back", "", "`TableConfig` accessors:", ""]
        lines.append(", ".join(f"`{a}`" for a in accessors))

    lines += [
        "",
        "Cross-check against upstream's own view in",
        "[`../corpus/guides/feature-table.md`](../corpus/guides/feature-table.md).",
        "Where the two disagree, this table is what the pinned commit compiles.",
    ]
    _write(content, "table-properties.md", lines)
    return len(variants)


# --------------------------------------------------------------------------- features


def write_table_features(items: dict[str, Item], content: Path) -> int:
    """Protocol reader/writer features, which decide whether a table is readable at all."""
    groups: list[tuple[Item, list[str]]] = []
    for name in ("TableFeatures", "TableFeature", "ReaderFeature", "WriterFeature"):
        for item in _find(items, name=name):
            if item.variants:
                groups.append((item, sorted(item.variants)))

    total = sum(len(v) for _, v in groups)
    lines = [
        "# Table features",
        "",
        "Protocol features are the compatibility contract: a reader that does not support a",
        "table's required feature must refuse the table rather than read it wrongly. Which is",
        "why this is a catalog and not a footnote -- the set below is what this pin can honour.",
        "",
        "Add one with `DeltaTable::add_feature`. Adding a feature raises the protocol",
        "version and can make the table unreadable by older clients; that is the point of it.",
        "",
    ]
    if not groups:
        lines += [
            "**No feature enumeration resolved in this build.** That is a finding, not an absence:",
            "the types are modelled in the kernel, so a missing table here means the kernel crate",
            "set did not resolve. Check [`../index/unresolved.tsv`](../index/unresolved.tsv).",
        ]
    for item, variants in groups:
        lines += [f"## `{item.path}`", "", f"{len(variants)} variants.", ""]
        lines.append(", ".join(f"`{v}`" for v in variants))
        lines.append("")

    _write(content, "table-features.md", lines)
    return total


# --------------------------------------------------------------------------- errors


def write_errors(items: dict[str, Item], content: Path) -> int:
    """The error vocabulary a caller has to handle, enumerated rather than discovered."""
    errors = sorted(
        (i for i in items.values() if i.kind == "enum" and i.name.endswith("Error") and i.variants),
        key=lambda i: (-len(i.variants), i.name),
    )
    total = sum(len(i.variants) for i in errors)

    lines = [
        "# Errors",
        "",
        f"{len(errors)} error enums, {total} variants. Matching on a variant is the only way to",
        "tell a retryable failure from a permanent one -- a stringified error cannot be matched,",
        "and the retry decision is the whole reason the variants exist.",
        "",
        "| Error | Variants | Defined in |",
        "|---|---:|---|",
    ]
    for item in errors:
        lines.append(f"| `{item.name}` | {len(item.variants)} | `{item.crate}` |")

    for item in errors[:6]:
        lines += ["", f"## `{item.path}`", ""]
        lines.append(", ".join(f"`{v}`" for v in sorted(item.variants)))
    lines.append("")

    _write(content, "errors.md", lines)
    return total


# --------------------------------------------------------------------------- foreign impls


def _is_noise(trait_path: str) -> bool:
    return trait_path.startswith(("core::", "std::", "alloc::"))


def write_foreign_impls(
    items: dict[str, Item],
    hidden_impls: list[tuple[str, str]],
    content: Path,
) -> int:
    """Every trait from another library that this one implements.

    This is the integration surface, and it is the axis a hand-written reference gets wrong
    most reliably: it is easy to remember the one famous trait and miss the other dozen. Here
    it is enumerated, so the count is whatever is true rather than whatever was recalled.

    Both halves are enumerated. Counting only implementations on nameable types understates
    this seam badly -- `ExecutionPlan` falls from eight implementors to one, and
    `ScalarUDFImpl` and `UserDefinedLogicalNodeCore` disappear entirely -- because the
    interesting implementors are physical operators and UDFs that live in private modules. A
    reader asking whether this library plugs into DataFusion's planner is asking about the
    impl, not about whether they can spell the type, so the internal ones are reported as
    such rather than dropped.
    """
    local_crates = {item.crate for item in items.values()}

    def is_foreign(trait_path: str) -> bool:
        root = trait_path.split("::", 1)[0]
        return root.replace("_", "-") not in local_crates and trait_path not in items

    public: dict[str, list[str]] = defaultdict(list)
    for item in items.values():
        for trait_path in item.implements:
            if is_foreign(trait_path):
                public[trait_path].append(item.name)

    # Internal implementors are qualified by their parent module. Two distinct types can share
    # a leaf name -- `table_provider::DeltaScan` is the retired wrapper, `next::DeltaScan` the
    # current one -- and a bare name would present them as one type appearing in both columns.
    internal: dict[str, list[str]] = defaultdict(list)
    for trait_path, implementor in hidden_impls:
        if is_foreign(trait_path):
            internal[trait_path].append("::".join(implementor.rsplit("::", 2)[-2:]))

    every = sorted(set(public) | set(internal))
    seam = [path for path in every if not _is_noise(path)]
    noise = len(every) - len(seam)
    seam.sort(key=lambda path: (-(len(public[path]) + len(internal[path])), path))

    def cell(names: list[str]) -> str:
        """Count every implementation; name each distinct implementor once."""
        if not names:
            return "--"
        ordered = sorted(set(names))
        shown = ", ".join(f"`{name}`" for name in ordered[:4])
        return shown + (f" +{len(ordered) - 4}" if len(ordered) > 4 else "")

    lines = [
        "# Foreign trait implementations",
        "",
        f"{len(seam)} traits defined elsewhere are implemented here (plus {noise} std/core",
        "traits, omitted as noise). This is the whole integration seam, generated -- not a",
        "curated highlight of the famous ones.",
        "",
        "Two columns, because the distinction is load-bearing. **Public** implementors are types",
        "you can name, store in a struct field and write in a signature. **Internal** ones live in",
        "private modules: the impl is real and runs, and it is why the integration works, but you",
        "cannot name the type -- you reach the behaviour through whatever public function returns",
        "it. Do not read an internal implementor as an extension point you can substitute.",
        "",
        "The trait is not in this index; only the implementations are. To read a foreign trait's",
        "own contract, resolve its canonical path in that library's own reference.",
        "",
        "Each row names at most four implementors. `content/index/foreign-impls.tsv` is the",
        "complete table -- one row per implementation, with the implementor's full canonical path",
        "and its `nameable` flag -- and is what to grep when a row here ends in `+n`.",
        "",
        "| Foreign trait | Public | Internal | Public implementors | Internal implementors |",
        "|---|---:|---:|---|---|",
    ]
    for path in seam:
        visible, hidden = public[path], internal[path]
        lines.append(
            f"| `{path}` | {len(visible)} | {len(hidden)} | {cell(visible)} | {cell(hidden)} |"
        )
    lines.append("")

    _write(content, "foreign-impls.md", lines)
    return len(seam)


# --------------------------------------------------------------------------- crate map


def write_crate_map(items: dict[str, Item], facts: dict[str, dict], content: Path) -> int:
    """What each crate owns, and under which feature envelope it was documented."""
    by_crate: dict[str, list[Item]] = defaultdict(list)
    for item in items.values():
        by_crate[item.crate].append(item)

    lines = [
        "# Crate map",
        "",
        f"{len(by_crate)} crates hold the pinned surface. The facade crate re-exports most of it,",
        "so the crate an item is *reached* through is usually not the crate that defines it.",
        "Resolve access paths through [`../index/aliases.tsv`](../index/aliases.tsv) before",
        "attributing an item to a crate.",
        "",
        "The envelope column matters: a crate documented under a partial feature set has a",
        "partial surface here, and [`../index/coverage.tsv`](../index/coverage.tsv) says which",
        "features were off and why.",
        "",
        "| Crate | Items | Traits | Envelope | Notable extension points |",
        "|---|---:|---:|---|---|",
    ]

    for crate in sorted(by_crate):
        members = by_crate[crate]
        traits = [item for item in members if item.kind == "trait"]
        # Rank by how many types implement it: the most-implemented trait in a crate is the one
        # most likely to be the thing that crate exists to let you plug into.
        ranked = sorted(traits, key=lambda item: -len(item.implementors))
        notable = [item.name for item in ranked if item.implementors][:4]
        entry = facts.get(crate) or {}
        on = entry.get("features_on") or []
        off = entry.get("features_off") or {}
        envelope = f"{len(on)} on / {len(off)} off" if on or off else "—"
        lines.append(
            f"| `{crate}` | {len(members)} | {len(traits)} | {envelope} | "
            f"{', '.join(f'`{n}`' for n in sorted(notable)) or '—'} |"
        )

    lines.append("")
    _write(content, "crate-map.md", lines)
    return len(by_crate)

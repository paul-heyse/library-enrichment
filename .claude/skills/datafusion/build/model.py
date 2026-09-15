"""Walk rustdoc JSON into a canonical item table, then stitch re-exports across crates.

Two passes, because a facade crate cannot resolve its own re-exports. `datafusion`'s document
carries 1,376 items and 145 `use` statements whose targets live in other crates' documents; the
real surface only appears once every crate is loaded and the `use` targets are joined by canonical
path string. Glob re-exports are expanded in the second pass for the same reason.

The unit of identity is the canonical defining path. An access path such as
`datafusion::prelude::Expr` is recorded as an alias of `datafusion_expr::expr::Expr`, never as a
second item.
"""

from __future__ import annotations

import json
from collections.abc import Callable
from dataclasses import dataclass, field

MAX_DEPTH = 128
MAX_GLOB_ROUNDS = 16
SUMMARY_CHARS = 240

ITEM_KINDS = {
    "struct",
    "enum",
    "union",
    "trait",
    "trait_alias",
    "function",
    "type_alias",
    "constant",
    "static",
    "macro",
    "proc_macro",
    "primitive",
}


class FormatError(RuntimeError):
    """The document's format version is absent or unsupported."""


def probe_format(payload: bytes, supported: list[int]) -> int:
    """Read `format_version` before interpreting anything else.

    rustdoc's JSON is not self-describing: an unsupported version parses far enough to produce
    confident nonsense. Checking the version first is what turns a silent misparse into an error.
    """
    header = json.loads(payload)
    version = header.get("format_version")
    if version is None:
        raise FormatError("document carries no format_version")
    if version not in supported:
        raise FormatError(f"format_version {version} is not in {supported}")
    return version


@dataclass
class Method:
    name: str
    signature: str
    summary: str
    via_trait: str | None
    owner: str


@dataclass
class Item:
    path: str
    name: str
    kind: str
    crate: str
    module: str
    signature: str
    summary: str
    docs: str
    deprecated: str | None = None
    aliases: list[str] = field(default_factory=list)
    methods: list[Method] = field(default_factory=list)
    implements: list[str] = field(default_factory=list)
    implementors: list[str] = field(default_factory=list)
    required_methods: list[str] = field(default_factory=list)
    provided_methods: list[str] = field(default_factory=list)
    variants: list[str] = field(default_factory=list)
    fields: list[str] = field(default_factory=list)


@dataclass
class CrateModel:
    name: str
    version: str
    format_version: int
    items: dict[str, Item]
    # alias access-path -> canonical target path, for explicit (non-glob) re-exports
    aliases: dict[str, str]
    # (access module path, target module canonical path) for `pub use target::*`
    globs: list[tuple[str, str]]
    modules: dict[str, list[str]]
    # (trait canonical path, implementor canonical path); resolved globally, because the impl
    # of a trait almost never lives in the crate that defines the trait.
    impl_edges: list[tuple[str, str]]
    unresolved: list[str]


def _summary(docs: str | None) -> str:
    if not docs:
        return ""
    first = docs.strip().split("\n\n", 1)[0].replace("\n", " ").strip()
    if len(first) > SUMMARY_CHARS:
        first = first[: SUMMARY_CHARS - 1].rstrip() + "…"
    return first


def _deprecation(item: dict) -> str | None:
    block = item.get("deprecation")
    if not block:
        return None
    since = block.get("since")
    note = block.get("note")
    if since and note:
        return f"since {since}: {note}"
    return note or (f"since {since}" if since else "deprecated")


def build_crate(name: str, version: str, payload: bytes, supported: list[int]) -> CrateModel:
    """Parse one crate document into items, explicit aliases and unexpanded glob edges."""
    from render import render_function, render_item_header

    format_version = probe_format(payload, supported)
    document = json.loads(payload)
    index: dict = document["index"]
    paths: dict = document["paths"]

    def canonical(identifier: object) -> str | None:
        entry = paths.get(str(identifier))
        if not entry:
            return None
        return "::".join(entry["path"])

    items: dict[str, Item] = {}
    aliases: dict[str, str] = {}
    impl_edges: list[tuple[str, str]] = []
    globs: list[tuple[str, str]] = []
    modules: dict[str, list[str]] = {}
    unresolved: list[str] = []

    # Pass A: enumerate every item this crate defines, straight from the index.
    #
    # Walking the public module tree is not sufficient. A crate that defines a type in a private
    # module and re-exports it publicly — the dominant DataFusion idiom — never surfaces that type
    # as a child of a public module, so a walk-only enumeration silently loses it. The index plus
    # the `paths` table is the complete, authoritative list; `crate_id == 0` keeps out the external
    # items that belong to some other crate's document.
    # Associated items carry `function` in `inner` exactly like a free function, and `paths` lists
    # them under their owner: `…::PhysicalOptimizerRule::optimize`. Enumerating them as top-level
    # items would invent thousands of free functions that do not exist. The exact discriminator is
    # membership -- an associated item is always listed inside its owner's `items` -- rather than
    # the shape of its path, which only holds when rustdoc happened to emit the parent module.
    member_ids: set[str] = set()
    for entry in index.values():
        inner = entry.get("inner") or {}
        for key in ("trait", "impl", "enum", "struct", "union"):
            block = inner.get(key)
            if not isinstance(block, dict):
                continue
            for collection in ("items", "variants"):
                member_ids.update(str(i) for i in block.get(collection) or [])
            layout = block.get("kind")
            if isinstance(layout, dict) and "plain" in layout:
                member_ids.update(str(i) for i in layout["plain"].get("fields") or [])

    for identifier, entry in index.items():
        if str(identifier) in member_ids:
            continue
        inner = entry.get("inner") or {}
        kind = next(iter(inner), None)
        item_name = entry.get("name")
        if kind not in ITEM_KINDS or not item_name:
            continue
        location = paths.get(str(identifier))
        if not location or location.get("crate_id") != 0:
            continue

        path = "::".join(location["path"])
        if path in items:
            continue
        module_path = path.rsplit("::", 1)[0] if "::" in path else path

        docs = entry.get("docs") or ""
        body = inner[kind]
        shape = body if isinstance(body, dict) else {}
        if kind == "function":
            signature = render_function(item_name, shape)
        else:
            signature = render_item_header(item_name, kind, shape)

        record = Item(
            path=path,
            name=item_name,
            kind=kind,
            crate=name,
            module=module_path,
            signature=signature,
            summary=_summary(docs),
            docs=docs,
            deprecated=_deprecation(entry),
        )

        if kind == "enum" and shape:
            for variant_id in shape.get("variants") or []:
                variant = index.get(str(variant_id))
                if variant and variant.get("name"):
                    record.variants.append(variant["name"])
        if kind in ("struct", "union") and shape:
            layout = shape.get("kind")
            field_ids = []
            if isinstance(layout, dict) and "plain" in layout:
                field_ids = layout["plain"].get("fields") or []
            elif isinstance(shape.get("fields"), list):
                field_ids = shape["fields"]
            for field_id in field_ids:
                member = index.get(str(field_id))
                if member and member.get("name"):
                    record.fields.append(member["name"])
        if kind == "trait" and shape:
            for member_id in shape.get("items") or []:
                member = index.get(str(member_id))
                if not member:
                    continue
                function = (member.get("inner") or {}).get("function")
                if function is None or not member.get("name"):
                    continue
                bucket = (
                    record.provided_methods if function.get("has_body") else record.required_methods
                )
                bucket.append(member["name"])
                record.methods.append(
                    Method(
                        name=member["name"],
                        signature=render_function(member["name"], function),
                        summary=_summary(member.get("docs")),
                        via_trait=None,
                        owner=path,
                    )
                )

        items[path] = record
        modules.setdefault(module_path, []).append(path)

    # Pass B: walk the public module tree purely to learn access paths.
    #
    # This is where `pub use` becomes an alias rather than a duplicate item, and where the module
    # membership needed to expand a glob re-export is recorded.
    root_id = str(document.get("root"))
    crate_path = name.replace("-", "_")
    seen_modules: set[str] = set()

    def walk_module(module_id: str, module_path: str, depth: int) -> None:
        if depth > MAX_DEPTH or module_id in seen_modules:
            return
        seen_modules.add(module_id)
        entry = index.get(module_id)
        if not entry:
            return
        module = (entry.get("inner") or {}).get("module")
        if module is None:
            return

        for child_id in module.get("items") or []:
            child = index.get(str(child_id))
            if child is None:
                continue
            child_inner = child.get("inner") or {}
            kind = next(iter(child_inner), None)
            child_name = child.get("name")

            if kind == "module" and child_name:
                walk_module(str(child_id), f"{module_path}::{child_name}", depth + 1)
            elif kind == "use":
                use = child_inner["use"]
                target = canonical(use["id"])
                access = f"{module_path}::{use['name']}"
                if target is None:
                    unresolved.append(access)
                elif use.get("is_glob"):
                    globs.append((module_path, target))
                else:
                    aliases[access] = target
                    # A re-exported item is reachable at this module, so it counts as a member.
                    # Without this, a glob over a facade module — `pub use datafusion_catalog::*`,
                    # where the target's own root is entirely re-exports — expands to nothing.
                    modules.setdefault(module_path, []).append(target)
            elif kind in ITEM_KINDS and child_name:
                definition = canonical(child_id)
                access = f"{module_path}::{child_name}"
                if definition and definition != access:
                    aliases[access] = definition
                modules.setdefault(module_path, []).append(definition or access)

    if index.get(root_id) is not None:
        walk_module(root_id, crate_path, 0)

    for module_path in modules:
        modules[module_path] = sorted(set(modules[module_path]))

    _attach_impls(index, items, canonical, impl_edges)

    return CrateModel(
        name=name,
        version=version,
        format_version=format_version,
        items=items,
        aliases=aliases,
        globs=globs,
        modules=modules,
        impl_edges=impl_edges,
        unresolved=sorted(set(unresolved)),
    )


def _attach_impls(
    index: dict,
    items: dict[str, Item],
    canonical: Callable[[object], str | None],
    impl_edges: list[tuple[str, str]],
) -> None:
    """Attach inherent and trait methods, and record trait/implementor edges.

    Blanket, synthetic and auto-derived impls are dropped. Left in, `Debug`, `Clone` and
    `PartialEq` account for more than half of every crate's impl items and bury the traits that
    actually represent extension points.
    """
    from render import render_function

    for entry in index.values():
        inner = entry.get("inner") or {}
        block = inner.get("impl")
        if block is None:
            continue
        if block.get("is_synthetic") or block.get("blanket_impl"):
            continue

        target = (block.get("for") or {}).get("resolved_path")
        owner = canonical(target["id"]) if target else None
        if owner is None:
            continue

        trait_reference = block.get("trait")
        trait_path = canonical(trait_reference["id"]) if trait_reference else None

        if trait_path:
            impl_edges.append((trait_path, owner))

        record = items.get(owner)
        if record is None:
            continue
        for member_id in block.get("items") or []:
            member = index.get(str(member_id))
            if not member:
                continue
            function = (member.get("inner") or {}).get("function")
            if function is None or not member.get("name"):
                continue
            record.methods.append(
                Method(
                    name=member["name"],
                    signature=render_function(member["name"], function),
                    summary=_summary(member.get("docs")),
                    via_trait=trait_path,
                    owner=owner,
                )
            )


StitchResult = tuple[dict[str, Item], dict[str, list[str]], list[str]]


def stitch(models: dict[str, CrateModel]) -> StitchResult:
    """Join every crate's items and resolve access paths into aliases on canonical items.

    Returns the merged item table, the alias index (canonical path -> access paths), and the
    access paths that could not be resolved to any known item.
    """
    merged: dict[str, Item] = {}
    for model in models.values():
        for path, item in model.items.items():
            existing = merged.get(path)
            if existing is None or len(item.docs) > len(existing.docs):
                merged[path] = item

    module_members: dict[str, set[str]] = {}
    for model in models.values():
        for module_path, contained in model.modules.items():
            module_members.setdefault(module_path, set()).update(contained)

    glob_edges: list[tuple[str, str]] = []
    for model in models.values():
        glob_edges.extend(model.globs)

    # Glob re-exports chain. `datafusion::prelude` globs `datafusion_functions::expr_fn`, whose
    # own contents are eight further globs over per-domain modules. A single expansion pass sees
    # an empty module and drops the whole prelude, so propagate to a fixed point.
    for _ in range(MAX_GLOB_ROUNDS):
        changed = False
        for access_module, target_module in glob_edges:
            incoming = module_members.get(target_module)
            if not incoming:
                continue
            bucket = module_members.setdefault(access_module, set())
            before = len(bucket)
            bucket.update(incoming)
            changed = changed or len(bucket) != before
        if not changed:
            break

    alias_index: dict[str, list[str]] = {}
    unresolved: list[str] = []

    def record(access: str, canonical_path: str) -> None:
        if access == canonical_path:
            return
        if canonical_path in merged:
            alias_index.setdefault(canonical_path, []).append(access)
        elif canonical_path in module_members:
            # A module re-export: every item it owns is reachable under the access path.
            for owned in module_members[canonical_path]:
                if owned in merged:
                    leaf = owned.rsplit("::", 1)[-1]
                    alias_index.setdefault(owned, []).append(f"{access}::{leaf}")
        else:
            unresolved.append(f"{access} -> {canonical_path}")

    for model in models.values():
        # Explicit re-exports are recorded by access path because `pub use X as Y` renames, and
        # only the `use` statement knows the new name.
        for access, target in model.aliases.items():
            record(access, target)

    for access_module, target_module in glob_edges:
        owned = module_members.get(target_module)
        if not owned:
            unresolved.append(f"{access_module}::* -> {target_module}")
            continue
        for path in owned:
            if path in merged:
                leaf = path.rsplit("::", 1)[-1]
                alias_index.setdefault(path, []).append(f"{access_module}::{leaf}")

    for model in models.values():
        for trait_path, implementor in model.impl_edges:
            if implementor in merged:
                merged[implementor].implements.append(trait_path)
            if trait_path in merged:
                merged[trait_path].implementors.append(implementor)

    for path, access_paths in alias_index.items():
        merged[path].aliases = sorted(set(access_paths))

    for item in merged.values():
        item.implements = sorted(set(item.implements))
        item.implementors = sorted(set(item.implementors))

    return merged, alias_index, sorted(set(unresolved))

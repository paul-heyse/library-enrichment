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
    # Module paths that are public the whole way down, and so nameable by a caller.
    public_modules: set[str]
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


def build_crate(
    name: str,
    version: str,
    payload: bytes,
    supported: list[int],
    supplement: bytes | None = None,
) -> CrateModel:
    """Parse one crate document into items, explicit aliases and unexpanded glob edges.

    `supplement` is an optional second rustdoc document of the same crate, produced with
    `--document-private-items`. It is used for one thing only: rustdoc emits a type declared in
    a private module *without any of its impls*, so such a type appears here real but empty --
    zero methods, no traits -- and nothing distinguishes that from a type that genuinely has
    none. delta-rs hits this on `LoadBuilder`, which `DeltaTable::scan_table` returns: 34 impls
    including `IntoFuture`, none of them in the public document.

    The supplement never adds items. Its index is twice the size of the public one, and letting
    it decide membership would publish private types as API. It fills in impls for items the
    public document already admitted, and nothing else.
    """
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
    seen_modules: set[tuple[str, str]] = set()
    public_modules: set[str] = {crate_path}

    def walk_module(module_id: str, module_path: str, depth: int) -> None:
        # Keyed by (id, path), not id: a private module re-exported from two places really is
        # reachable under two names, and collapsing them loses one of the access paths.
        if depth > MAX_DEPTH or (module_id, module_path) in seen_modules:
            return
        seen_modules.add((module_id, module_path))
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
                child_path = f"{module_path}::{child_name}"
                # Track which module paths are public the whole way down. rustdoc lists a
                # private module among its parent's items, so being reachable by the walk is
                # not the same as being nameable by a caller -- and minting an import path
                # through a private module produces a path that does not compile.
                if child.get("visibility") == "public" and module_path in public_modules:
                    public_modules.add(child_path)
                walk_module(str(child_id), child_path, depth + 1)
            elif kind == "use":
                use = child_inner["use"]
                target = canonical(use["id"])
                access = f"{module_path}::{use['name']}"
                if target is None and use.get("is_glob"):
                    # `pub use private_mod::*`. rustdoc records no canonical path for a private
                    # module, so the glob target cannot be resolved by path -- but the module
                    # itself is in `index`, because its contents are re-exported. Walking it
                    # under the *accessing* path is what the glob actually means, and skipping
                    # it would drop whatever the module holds without leaving a trace.
                    #
                    # This is not an edge case here: `kernel::schema::schema`,
                    # `kernel::models::actions` and `kernel::snapshot` are all private modules
                    # re-exported this way, and between them they hold the schema model, the
                    # log actions and the snapshot API.
                    nested = index.get(str(use["id"]))
                    if nested is not None and "module" in (nested.get("inner") or {}):
                        walk_module(str(use["id"]), module_path, depth + 1)
                    else:
                        unresolved.append(f"{access}::* -> <unresolvable glob target>")
                elif target is None:
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

    if supplement is not None:
        probe_format(supplement, supported)
        extra = json.loads(supplement)
        extra_index, extra_paths = extra["index"], extra["paths"]

        def extra_canonical(identifier: object) -> str | None:
            entry = extra_paths.get(str(identifier))
            return "::".join(entry["path"]) if entry else None

        # Ids are per-document, so the two are joined by canonical path. Only items the public
        # document already admitted can receive anything.
        _attach_impls(extra_index, items, extra_canonical, impl_edges, public_only=True)
        for record in items.values():
            seen: set[tuple[str, str | None, str]] = set()
            deduped: list[Method] = []
            for method in record.methods:
                key = (method.name, method.via_trait, method.signature)
                if key in seen:
                    continue
                seen.add(key)
                deduped.append(method)
            record.methods = deduped

    return CrateModel(
        name=name,
        version=version,
        format_version=format_version,
        items=items,
        aliases=aliases,
        globs=globs,
        modules=modules,
        public_modules=public_modules,
        impl_edges=impl_edges,
        unresolved=sorted(set(unresolved)),
    )


def _attach_impls(
    index: dict,
    items: dict[str, Item],
    canonical: Callable[[object], str | None],
    impl_edges: list[tuple[str, str]],
    public_only: bool = False,
) -> None:
    """Attach inherent and trait methods, and record trait/implementor edges.

    Blanket, synthetic and auto-derived impls are dropped. Left in, `Debug`, `Clone` and
    `PartialEq` account for more than half of every crate's impl items and bury the traits that
    actually represent extension points.

    `public_only` is for the supplementary private-items document, where the index contains
    everything the crate defines rather than everything it exports. A `pub(crate)` method on a
    public type is real code but not callable surface, and admitting it would be the same class
    of error as omitting a public one.
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
            if trait_reference:
                trait_entry = index.get(str(trait_reference["id"]))
                if (
                    trait_entry
                    and trait_entry.get("crate_id") == 0
                    and trait_entry.get("visibility") != "public"
                ):
                    continue
            elif member.get("visibility") != "public":
                continue
            if public_only and member.get("visibility") not in ("public", "default", None):
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


StitchResult = tuple[
    dict[str, Item],
    dict[str, list[str]],
    list[str],
    list[tuple[str, str]],
]


def stitch(models: dict[str, CrateModel]) -> StitchResult:
    """Join every crate's items and resolve access paths into aliases on canonical items.

    Returns the merged item table, the alias index (canonical path -> access paths), the access
    paths that could not be resolved to any known item, and the trait implementations whose
    implementing type is real but not nameable.

    That last one exists because dropping those edges makes the integration surface look
    smaller than it is. A `MetricObserverExec` you cannot name still implements `ExecutionPlan`,
    and a reader asking "does this library plug into DataFusion's physical planning?" is asking
    about the impl, not about whether they can spell the type. Counting only nameable
    implementors turns that question's answer from eight into one.
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

    # A glob over a module re-exports that module's public *submodules* too, so everything
    # underneath stays reachable one level up. `pub use deltalake_core::*` in the facade is what
    # makes `deltalake::kernel::StructType` -- the spelling upstream's own documentation uses --
    # a real path, even though the facade crate defines nothing.
    #
    # The rewrite is applied to already-recorded ACCESS paths rather than to canonical paths.
    # That distinction is load-bearing: a canonical path may run through a private module, so
    # rewriting canonical paths would mint import paths that do not compile.
    # Only modules the walk actually entered are public. An item whose canonical path runs
    # through a private module is reachable only under the re-export the walk recorded, so it
    # must not be rewritten -- that would mint an import path that does not compile.
    public_modules = {path for model in models.values() for path in model.public_modules}

    for _ in range(MAX_GLOB_ROUNDS):
        added = 0
        for access_module, target_module in glob_edges:
            prefix = f"{target_module}::"
            for canonical_path, access_paths in list(alias_index.items()):
                for existing in list(access_paths):
                    if not existing.startswith(prefix):
                        continue
                    suffix = existing[len(prefix) :]
                    if "::" not in suffix:
                        continue  # the leaf case is already handled above
                    derived = f"{access_module}::{suffix}"
                    if derived not in alias_index[canonical_path]:
                        alias_index[canonical_path].append(derived)
                        added += 1
            for canonical_path, item in merged.items():
                if not canonical_path.startswith(prefix) or item.module not in public_modules:
                    continue
                derived = f"{access_module}::{canonical_path[len(prefix) :]}"
                if derived not in alias_index.setdefault(canonical_path, []):
                    alias_index[canonical_path].append(derived)
                    added += 1
        if not added:
            break

    indexed_crates = {model.name.replace("-", "_") for model in models.values()}
    hidden_impls: set[tuple[str, str]] = set()
    for model in models.values():
        for trait_path, implementor in model.impl_edges:
            if implementor in merged:
                merged[implementor].implements.append(trait_path)
            elif implementor.split("::", 1)[0] in indexed_crates:
                # Defined by a crate this repository indexes, but behind a private module, so
                # it never became an Item. The edge is still a fact about the crate's surface.
                hidden_impls.add((trait_path, implementor))
            if trait_path in merged:
                merged[trait_path].implementors.append(implementor)

    for path, access_paths in alias_index.items():
        merged[path].aliases = sorted(set(access_paths))

    for item in merged.values():
        item.implements = sorted(set(item.implements))
        item.implementors = sorted(set(item.implementors))

    return merged, alias_index, sorted(set(unresolved)), sorted(hidden_impls)

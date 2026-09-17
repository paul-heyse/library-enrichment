"""Join the per-distribution documents into one canonical item table.

The contract is the one the Rust skills established, and it survives the language change intact:

> **The canonical defining path is the identity. Every other spelling is an alias, never a second
> item.**

`fastmcp.FastMCP`, `fastmcp.server.FastMCP` and `fastmcp.server.server.FastMCP` are one item with
three access paths. What Python adds is that the *defining* path is frequently one a caller must
never write -- `mcp_types.Tool` is defined at `mcp_types._types.Tool` -- so this module also
computes, for every item, the best spelling to put in front of a reader.

Three joins happen here that nothing upstream can do:

* **Alias chains resolve to a fixed point.** `fastmcp.Client` reaches `fastmcp.client.Client`
  which reaches `fastmcp.client.client.Client`. One hop leaves a dangling path.
* **Annotations join to items.** Griffe resolves a type name one hop, to an access path
  (`mcp_types.ToolAnnotations`), not to the defining path. Without finishing that join against
  the alias table, no signature in the repository links to any row in the index.
* **Base classes close transitively.** `FastMCP` reaches `Provider` through `AggregateProvider`,
  and "a FastMCP server is itself a Provider" is the fact the whole composition story rests on.
  A direct-children table omits it.
"""

from __future__ import annotations

import json
from collections import defaultdict, deque
from dataclasses import dataclass, field
from pathlib import Path

import subjects

MAX_ALIAS_HOPS = 16
MAX_CLOSURE_DEPTH = 32

# Every Python class carries these, and left in they bury the members that mean something.
# Same role as the Rust skill's ubiquitous-trait filter, and the same reason: `Tool` has 24
# inherited `model_*` methods from pydantic that are identical for all 400 models in the index.
NOISE_MEMBERS = frozenset(
    {
        "__init__",
        "__new__",
        "__repr__",
        "__str__",
        "__eq__",
        "__ne__",
        "__hash__",
        "__lt__",
        "__le__",
        "__gt__",
        "__ge__",
        "__bool__",
        "__len__",
        "__iter__",
        "__enter__",
        "__exit__",
        "__aenter__",
        "__aexit__",
        "__getattr__",
        "__setattr__",
        "__delattr__",
        "__getitem__",
        "__setitem__",
        "__call__",
        "__contains__",
        "__copy__",
        "__deepcopy__",
        "__reduce__",
        "__getstate__",
        "__setstate__",
        "__class_getitem__",
        "__init_subclass__",
        "__subclasshook__",
        "__post_init__",
    }
)
PYDANTIC_MEMBERS = frozenset(
    {
        "model_config",
        "model_fields",
        "model_computed_fields",
        "model_extra",
        "model_fields_set",
        "model_construct",
        "model_copy",
        "model_dump",
        "model_dump_json",
        "model_json_schema",
        "model_parametrized_name",
        "model_post_init",
        "model_rebuild",
        "model_validate",
        "model_validate_json",
        "model_validate_strings",
        "copy",
        "dict",
        "json",
        "schema",
        "schema_json",
        "parse_obj",
        "parse_raw",
        "parse_file",
        "from_orm",
        "construct",
        "update_forward_refs",
    }
)


@dataclass
class Item:
    path: str
    name: str
    kind: str
    dist: str
    module: str
    signature: str
    summary: str
    docs: str
    labels: list[str] = field(default_factory=list)
    file: str = ""
    lineno: int | None = None
    endlineno: int | None = None
    aliases: list[str] = field(default_factory=list)
    preferred: str = ""
    nameable: bool = False
    in_all: bool = False
    underscore_free: bool = False
    bases: list[str] = field(default_factory=list)
    base_paths: list[str] = field(default_factory=list)
    members: list[dict] = field(default_factory=list)
    overloads: list[str] = field(default_factory=list)
    decorators: list[str] = field(default_factory=list)
    doc_sections: dict = field(default_factory=dict)
    types_used: list[str] = field(default_factory=list)
    inferred: str = ""
    unannotated: bool = False
    boundary: bool = False
    protocol_era: str = ""
    dispatch_only: bool = False
    subject: str = ""
    reachability: str = "internal"
    reached_via: list[str] = field(default_factory=list)


@dataclass
class Model:
    items: dict[str, Item]
    access: dict[str, str]
    unresolved: list[tuple[str, str]]
    descendants: dict[str, list[tuple[str, int]]]
    overrides: list[tuple[str, str, str, str]]
    conditional: list[dict]
    not_indexed: list[dict]
    modules: dict[str, list[str]]
    dists: dict[str, str]
    exports: dict[str, set[str]]
    type_checking: dict[str, set[str]] = field(default_factory=dict)

    def exported(self, alias: str, name: str) -> bool:
        """Is this spelling an intended re-export, or just where somebody imported it?

        Every `from X import Y` anywhere in the indexed set creates a reachable path. `FastMCP`
        collects 28 of them, most of which are module bodies that happen to use it. They are
        real -- importing from them works -- but presenting them as the API misleads, so the
        distinction is carried rather than resolved by discarding.
        """
        return name in self.exports.get(alias.rsplit(".", 1)[0], ())


def load_documents(directory: Path) -> dict[str, dict]:
    documents: dict[str, dict] = {}
    # Two sidecars share this directory and are not distribution documents: the integrity
    # manifest, and the batch-analysis payload. Matching on the positive shape rather than the
    # names would be neater, but a document that failed to write would then be skipped silently
    # instead of raising -- and "no acquired documents" is the message that belongs there.
    sidecars = {"ACQUISITION.json", "ANALYSIS.json", "PROBES.json"}
    for path in sorted(directory.glob("*.json")):
        if path.name in sidecars:
            continue
        document = json.loads(path.read_text())
        documents[document["dist"]] = document
    if not documents:
        raise FileNotFoundError(f"no acquired documents under {directory}")
    return documents


def _resolve(target: str, access: dict[str, str], items: set[str]) -> str | None:
    """Follow an access path to the item it names, or to nothing."""
    current = target
    for _ in range(MAX_ALIAS_HOPS):
        if current in items:
            return current
        if current not in access:
            return None
        current = access[current]
    return None


def _writable(path: str) -> bool:
    return not any(part.startswith("_") for part in path.split(".")[1:])


def _preferred_spelling(
    canonical: str,
    aliases: list[str],
    name: str,
    exports: dict[str, set[str]],
    type_checking: dict[str, set[str]] | None = None,
) -> tuple[str, bool]:
    """The spelling to show a reader, and whether any spelling is writable at all.

    A caller cannot import `mcp_types._types.Tool`; they import `mcp_types.Tool`. So leading with
    the defining path would be true and useless.

    Path length alone is the wrong ranking, and picked two visibly wrong answers before this:
    `mcp.Tool` beat `mcp_types.Tool` because it is shorter, and `Provider` came out as
    `fastmcp.apps.app.Provider` -- a plain `import` inside a module body, not a public export at
    all. Every `from X import Y` in any indexed module creates an access path, and most of them
    are somebody's implementation detail.

    So rank by what makes a spelling *intended*: declared in the containing module's `__all__`
    first, then rooted in the distribution that defines the item, then shallowest.

    And a spelling that exists only under `if TYPE_CHECKING:` is not a candidate at all. Measured
    against rich 15.0.0: `rich/__init__.py` imports `Console` inside that guard and nowhere else,
    so Griffe offers `rich.Console`, which is shorter than `rich.console.Console` and wins every
    tiebreak -- and `from rich import Console` raises ImportError. Both `ty` and `pyrefly` accept
    the line with zero errors. A preferred spelling is a promise that the import runs, so the
    guarded ones are excluded here and carried in `aliases.tsv` as their own kind instead.
    """
    home = canonical.split(".")[0]
    guarded = type_checking or {}

    def type_checking_only(spelling: str) -> bool:
        module, _, leaf = spelling.rpartition(".")
        return leaf in guarded.get(module, ())

    def rank(spelling: str) -> tuple[int, int, int, int, str]:
        module = spelling.rsplit(".", 1)[0]
        return (
            0 if name in exports.get(module, ()) else 1,
            0 if spelling.split(".")[0] == home else 1,
            spelling.count("."),
            len(spelling),
            spelling,
        )

    # The canonical path competes rather than merely standing in when no alias qualifies.
    # fastmcp can treat it as a fallback because its canonical paths are mostly private
    # (`mcp_types._types.Tool`), so any alias beats them. Rich is the opposite: `rich.table.Table`
    # IS the documented spelling, and every module that imports it at runtime creates an alias.
    # Measured, fallback-only ranking answered `rich.console.Console` with `rich.bar.Console` --
    # a real import site, importable, and nobody's idea of the right answer. Ranked together,
    # `Console` is in `rich.console`'s exports and not in `rich.bar`'s, so the first term decides
    # it before length ever applies.
    candidates = [a for a in aliases if _writable(a) and not type_checking_only(a)]
    if _writable(canonical) and not type_checking_only(canonical):
        candidates.append(canonical)
    clean = sorted(set(candidates), key=rank)
    if clean:
        return clean[0], True
    return canonical, False


def stitch(
    documents: dict[str, dict],
    semantic: dict | None = None,
    manifest: dict | None = None,
) -> Model:
    """Merge every distribution into one table and finish the joins Griffe leaves open."""
    items: dict[str, Item] = {}
    access: dict[str, str] = {}
    modules: dict[str, list[str]] = {}
    conditional: list[dict] = []
    not_indexed: list[dict] = []
    dists: dict[str, str] = {}
    exports: dict[str, set[str]] = {}
    type_checking: dict[str, set[str]] = {}

    for dist, document in sorted(documents.items()):
        dists[dist] = document["version"]
        conditional.extend(document.get("conditional", []))
        not_indexed.extend(document.get("not_indexed", []))
        for name, values in document.get("exports", {}).items():
            exports.setdefault(name, set()).update(values)
        for name, values in document.get("type_checking", {}).items():
            type_checking.setdefault(name, set()).update(values)
        for module in document["modules"]:
            modules.setdefault(module, [])
        for raw in document["items"]:
            path = raw["path"]
            if path in items and len(items[path].docs) >= len(raw.get("docs", "")):
                continue
            items[path] = Item(
                path=path,
                name=raw["name"],
                kind=raw["kind"],
                dist=raw["dist"],
                module=raw["module"],
                signature=raw["signature"],
                summary=raw["summary"],
                docs=raw.get("docs", ""),
                labels=raw.get("labels", []),
                file=raw.get("file", ""),
                lineno=raw.get("lineno"),
                endlineno=raw.get("endlineno"),
                dispatch_only=bool(raw.get("dispatch_only")),
                bases=raw.get("bases", []),
                base_paths=raw.get("base_paths", []),
                members=raw.get("members", []),
                overloads=raw.get("overloads", []),
                decorators=raw.get("decorators", []),
                doc_sections=raw.get("doc_sections", {}),
                types_used=raw.get("types_used", []),
                underscore_free=raw.get("underscore_free", False),
                unannotated=bool(raw.get("unannotated") or raw.get("unannotated_return")),
            )
        access.update(document["access"])
        # `McpError = MCPError` never became an Alias in Griffe, so it is folded in here or it
        # reaches no table at all.
        access.update(document.get("assignment_aliases", {}))

    known = set(items)
    unresolved: list[tuple[str, str]] = []
    alias_index: dict[str, list[str]] = defaultdict(list)
    for spelling, target in sorted(access.items()):
        landed = _resolve(target, access, known)
        if landed is None:
            unresolved.append((spelling, target))
        elif spelling != landed:
            alias_index[landed].append(spelling)

    for path, item in items.items():
        item.aliases = sorted(set(alias_index.get(path, [])))
        item.preferred, item.nameable = _preferred_spelling(
            path, item.aliases, item.name, exports, type_checking
        )
        item.in_all = any(
            item.name in exports.get(spelling.rsplit(".", 1)[0], ())
            for spelling in (*item.aliases, path)
        )
        if "._v" in path:
            era = next((p for p in path.split(".") if p.startswith("_v")), "")
            item.protocol_era = era.removeprefix("_v").replace("_", "-")

    items.update(_boundary_items(unresolved, exports))
    known = set(items)
    _join_types(items, access, known)
    _attach_inferred(items, semantic)
    descendants = _descendants(items, access, known)
    overrides = _overrides(items)

    # Subject and reachability are the two facts that make three subjects one index. Both are
    # joins over what is already computed above; neither costs another pass over the source.
    if manifest is not None:
        subject_of = subjects.classifier(manifest)
        for path, item in items.items():
            item.subject = subject_of(path)
        registries = [
            entry for subject in manifest["subjects"] for entry in subject.get("registries", ())
        ]
        declarations = [
            entry
            for subject in manifest["subjects"]
            for entry in subject.get("declared_reachability", ())
        ]
        _reachability(items, descendants, registries, declarations)

    for path, item in items.items():
        modules.setdefault(item.module, []).append(path)
    for module in modules:
        modules[module] = sorted(set(modules[module]))

    return Model(
        items=items,
        access=access,
        unresolved=sorted(set(unresolved)),
        descendants=descendants,
        overrides=overrides,
        conditional=sorted(conditional, key=lambda row: (row["module"], row["name"])),
        not_indexed=not_indexed,
        modules=modules,
        dists=dists,
        exports=exports,
        type_checking=type_checking,
    )


def _boundary_items(
    unresolved: list[tuple[str, str]], exports: dict[str, set[str]]
) -> dict[str, Item]:
    """Names an indexed package deliberately re-exports from a package we do not index.

    `fastmcp.dependencies.__all__` lists thirteen names, five of which come from `uncalled_for`
    -- `Depends` among them, the dependency-injection entry point. They resolve to nothing here
    because that distribution is a declared boundary, so without this they would be absent from
    every table and `rg Depends symbols.tsv` would answer that FastMCP has no such thing.

    A boundary row is honest on both counts: the name is findable at the spelling a caller
    writes, and it is marked as defined outside the index with no documentation of its own.
    """
    admitted: dict[str, Item] = {}
    for spelling, target in unresolved:
        module, _, name = spelling.rpartition(".")
        if name not in exports.get(module, ()):
            continue
        if spelling in admitted:
            continue
        admitted[spelling] = Item(
            path=spelling,
            name=name,
            kind="reexport",
            dist=target.split(".")[0],
            module=module,
            signature=f"{name}  # re-exported from {target}",
            summary=f"Re-exported from {target}, which this repository does not index.",
            docs="",
            preferred=spelling,
            # Not hardcoded True. fastmcp can assume it, because the modules that re-export
            # across a boundary there are public. Here `typer._typing` declares an `__all__`
            # inside a private module, so a boundary row is admitted at a spelling nobody can
            # write -- and claiming it is nameable contradicts `underscore_free` on the same
            # record, which is exactly what `check_honesty` looks for.
            nameable=_writable(spelling),
            in_all=True,
            underscore_free=_writable(spelling),
            boundary=True,
        )
    return admitted


def _join_types(items: dict[str, Item], access: dict[str, str], known: set[str]) -> None:
    """Rewrite one-hop type names into canonical paths, dropping what leaves the index."""
    for item in items.values():
        resolved: list[str] = []
        for spelling in item.types_used:
            landed = spelling if spelling in known else _resolve(spelling, access, known)
            if landed:
                resolved.append(landed)
        item.types_used = sorted(set(resolved))
        for member in item.members:
            if "types_used" not in member:
                continue
            member_resolved = []
            for spelling in member["types_used"]:
                landed = spelling if spelling in known else _resolve(spelling, access, known)
                if landed:
                    member_resolved.append(landed)
            member["types_used"] = sorted(set(member_resolved))


def _attach_inferred(items: dict[str, Item], semantic: dict | None) -> None:
    """A declared annotation is never overwritten by an inferred one."""
    if not semantic or semantic.get("status") != "passed":
        return
    for row in semantic.get("rows", []):
        item = items.get(row["path"])
        if item is not None and item.unannotated:
            item.inferred = row["inferred"]


def _descendants(
    items: dict[str, Item], access: dict[str, str], known: set[str]
) -> dict[str, list[tuple[str, int]]]:
    """Invert the base-class edges and close them transitively.

    This replaces what the plan first intended to take from the language server. Measured, ty's
    `typeHierarchy/subtypes` returns 10 direct children of `Provider`; this closure returns 26
    and includes `FastMCP`, which reaches `Provider` through `AggregateProvider`.
    """
    children: dict[str, set[str]] = defaultdict(set)
    for path, item in items.items():
        if item.kind != "class":
            continue
        for base in item.base_paths:
            landed = base if base in known else _resolve(base, access, known)
            if landed:
                children[landed].add(path)

    closure: dict[str, list[tuple[str, int]]] = {}
    for base in sorted(children):
        seen: dict[str, int] = {}
        queue: deque[tuple[str, int]] = deque((child, 1) for child in sorted(children[base]))
        while queue:
            node, depth = queue.popleft()
            if node in seen or depth > MAX_CLOSURE_DEPTH:
                continue
            seen[node] = depth
            queue.extend((grand, depth + 1) for grand in sorted(children.get(node, ())))
        closure[base] = sorted(seen.items())
    return closure


def _overrides(items: dict[str, Item]) -> list[tuple[str, str, str, str]]:
    """One row per (subclass, base, member) saying overridden, inherited or new.

    This is the honest analogue of the Rust index's required-versus-provided split. `Provider`
    is not abstract: its hooks return `[]` or `None`, and you extend it by overriding them. So
    "what must I write to make a Provider" is answered by this table and by nothing else.
    """
    rows: list[tuple[str, str, str, str]] = []
    for path, item in sorted(items.items()):
        if item.kind != "class":
            continue
        for member in item.members:
            name = member["name"]
            if name in NOISE_MEMBERS or name in PYDANTIC_MEMBERS:
                continue
            declared = member.get("declared_on", path)
            if declared == path:
                relation = "new" if not item.base_paths else "declared"
            elif member.get("inherited"):
                relation = "inherited"
            else:
                relation = "overridden"
            rows.append((path, declared, name, relation))
    return sorted(set(rows))


REACHABILITY = (
    "importable",
    "received",
    "subclassed-via",
    "subtype-of",
    "registered-via",
    "declared",
    "internal",
)


def _reachability(
    items: dict[str, Item],
    descendants: dict[str, list[tuple[str, int]]],
    registries: list[dict],
    declarations: list[dict],
) -> None:
    """Why a caller will meet an item they cannot import.

    `nameable: no` is a true fact and a useless stopping point. Typer vendors Click at
    `typer/_click/`, so 5,548 lines -- `Context`, `Parameter`, `ParamType`, the whole exception
    hierarchy, shell completion -- sit behind a private module path. Every one of them is
    correctly `nameable: no`, and an index that says only that has deleted the answer: you never
    import `typer._click.core.Context`, you are *handed* one as `ctx`, and `typer.Context`
    subclasses it.

    So nameability stays honest and reachability is recorded beside it. Both are joins over
    tables already computed here -- invert `types_used`, invert `descendants` -- with one
    declared case for the registry that no inversion can see.

    Precedence is deliberate: `importable` beats everything, because if a writable spelling
    exists that is the answer and the rest is trivia.
    """
    delivered: dict[str, set[str]] = defaultdict(set)

    # received -- appears in the resolved annotations of something you can write.
    for path, item in items.items():
        if not item.nameable:
            continue
        spelling = item.preferred or path
        for used in item.types_used:
            if used != path:
                delivered[used].add(f"{spelling} (parameter)")
        for member in item.members:
            for used in member.get("types_used", ()):
                if used != path:
                    delivered[used].add(f"{spelling}.{member['name']} (parameter)")

    # subclassed-via -- a base of something you can write.
    inherited: dict[str, set[str]] = defaultdict(set)
    for base, rows in descendants.items():
        for child, _depth in rows:
            child_item = items.get(child)
            if child_item is not None and child_item.nameable:
                inherited[base].add(f"{child_item.preferred or child} (subclass)")

    # subtype-of -- you never name this class, you handle it through an ancestor you can name.
    # Six of typer's vendored click exceptions land here: `NoSuchOption`, `MissingParameter`,
    # `BadArgumentUsage`, `BadOptionUsage`, `FileError` and `NoArgsIsHelpError` are raised at
    # runtime and caught with `except typer.TyperException`, but only `BadParameter` is
    # re-exported, so a nameability-only reading calls the other six internal and stops.
    ancestors: dict[str, set[str]] = defaultdict(set)
    for base, rows in descendants.items():
        base_item = items.get(base)
        if base_item is None or not base_item.nameable:
            continue
        spelling = base_item.preferred or base
        for child, _depth in rows:
            child_item = items.get(child)
            if child_item is not None and not child_item.nameable:
                ancestors[child].add(f"{spelling} (ancestor)")

    registered: dict[str, set[str]] = defaultdict(set)
    for entry in registries:
        base, registrar = entry["base"], entry["registrar"]
        registered[base].add(f"{registrar} (registry)")
        for child, _depth in descendants.get(base, ()):
            registered[child].add(f"{registrar} (registry)")

    # declared -- reachability no graph edge can carry, with its evidence in the manifest.
    # `typer._click.types.IntRange` is what `Annotated[int, typer.Option(min=1)]` BECOMES; typer
    # constructs it from the annotation, so it appears in nobody's signature and inherits from
    # nothing nameable. It is still the answer to "what does my annotation do on the command
    # line", which is the question typer exists to answer.
    declared: dict[str, set[str]] = defaultdict(set)
    for entry in declarations:
        via = entry["reached_via"]
        for base in entry.get("subclasses_of", ()):
            for child, _depth in descendants.get(base, ()):
                declared[child].add(via)
        for path in entry.get("paths", ()):
            declared[path].add(via)

    for path, item in items.items():
        if item.nameable:
            item.reachability = "importable"
            item.reached_via = [item.preferred or path]
        elif path in inherited:
            item.reachability = "subclassed-via"
            item.reached_via = sorted(inherited[path])
        elif path in delivered:
            item.reachability = "received"
            item.reached_via = sorted(delivered[path])
        elif path in ancestors:
            item.reachability = "subtype-of"
            item.reached_via = sorted(ancestors[path])
        elif path in declared:
            item.reachability = "declared"
            item.reached_via = sorted(declared[path])
        elif path in registered:
            item.reachability = "registered-via"
            item.reached_via = sorted(registered[path])
        else:
            item.reachability = "internal"
            item.reached_via = []

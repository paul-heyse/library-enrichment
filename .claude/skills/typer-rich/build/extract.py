"""Griffe object tree -> the normalized JSON this repository is built from.

This is the Python analogue of rustdoc JSON, and it is deliberately *our* schema rather than
`griffe dump`'s. Two reasons. `griffe dump` is lossy where it matters -- `labels` null,
`overloads` zero, `filepath` null, and annotations emitted as expression trees rather than the
text you would type. And pinning our own schema keeps `build.py` on the standard library, so the
offline half of the build never imports Griffe.

Positions are recorded as a *range*, not a point. `lineno` alone cannot answer "which
definition encloses this offset", and that is the question every cross-reference fact arrives
as. `endlineno` costs one attribute read and is the join key for all of them.

Five things here have no Rust counterpart, and each exists because leaving it out produced a
silently wrong index rather than an error:

* **Definitions live in private modules.** `mcp_types.Tool` is defined at `mcp_types._types.Tool`
  and re-exported through a 211-entry `__all__`. A walk that prunes on a leading underscore
  reaches 71 of that package's 462 classes. So the walk descends everywhere, and *nameability*
  is decided from the access paths that reach an item, never from where it was defined.
* **Publicity is two facts, not one.** `fastmcp.Settings` is importable and documented but absent
  from `__all__`, because the *instance* `settings` is exported and the *class* is not. An
  `is_exported` filter would delete it, along with every submodule of `fastmcp`.
* **Assignment aliases are Attributes.** `McpError = MCPError` records as an attribute with a
  value, not as an `Alias`, so it reaches no alias table. It is the spelling a pre-2.0 codebase
  greps for.
* **Namespace packages are unreachable from the root.** `fastmcp/contrib/` has no `__init__.py`,
  so Griffe cannot see it at all, while `from fastmcp.contrib.mcp_mixin import MCPMixin` works.
* **Annotations are source-local spellings.** `ExprName.canonical_path` resolves one hop, to the
  access path (`mcp_types.ToolAnnotations`), not to the defining path. The join to `symbols.tsv`
  has to be finished downstream against the alias table, so every annotation records the one-hop
  names it mentions rather than pretending to have resolved them.
"""

from __future__ import annotations

import ast
import json
import sys
from pathlib import Path
from typing import Any

import griffe

SCHEMA = 1
SUMMARY_CHARS = 240
ITEM_KINDS = ("class", "function", "attribute")


class ExtractError(RuntimeError):
    """A distribution could not be modelled honestly."""


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


# --------------------------------------------------------------------------- text helpers


def _summary(docs: str | None) -> str:
    if not docs:
        return ""
    head = docs.strip().split("\n\n", 1)[0].replace("\n", " ").strip()
    if len(head) <= SUMMARY_CHARS:
        return head
    return head[: SUMMARY_CHARS - 1].rstrip() + "…"


def _text(value: object) -> str:
    """Griffe expressions stringify to the source spelling; None must not become 'None'."""
    return "" if value is None else str(value)


def underscore_free(path: str) -> bool:
    """A path a caller can write: no private segment after the distribution root."""
    return not any(part.startswith("_") for part in path.split(".")[1:])


def _relative(filepath: Path | None, site_packages: Path) -> str:
    """Absolute capsule paths must never reach the emitted tree."""
    if filepath is None:
        return ""
    try:
        return Path(filepath).resolve().relative_to(site_packages).as_posix()
    except ValueError:
        return Path(filepath).name


def _type_names(annotation: griffe.Expr | str | None) -> list[str]:
    """Every named type an annotation mentions, as one-hop canonical paths."""
    if annotation is None or isinstance(annotation, str) or not hasattr(annotation, "iterate"):
        return []
    return sorted(
        {
            node.canonical_path
            for node in annotation.iterate(flat=True)
            if isinstance(node, griffe.ExprName)
        }
    )


def _parameter_text(parameter: griffe.Parameter) -> str:
    text = parameter.name
    # `parameter.annotation is None` is the only correct test. `str(None)` is the literal string
    # "None", which is itself a legitimate annotation, so testing the text would conflate
    # "unannotated" with "annotated as None".
    if parameter.annotation is not None:
        text += f": {_text(parameter.annotation)}"
    if parameter.default is not None:
        text += f" = {_text(parameter.default)}"
    return text


def _signature(obj: griffe.Object) -> str:
    kind = obj.kind.value
    if kind == "function":
        params = ", ".join(_parameter_text(p) for p in obj.parameters)
        returns = _text(obj.returns)
        prefix = "async def" if "async" in (obj.labels or ()) else "def"
        return f"{prefix} {obj.name}({params})" + (f" -> {returns}" if returns else "")
    if kind == "class":
        bases = ", ".join(_text(b) for b in (obj.bases or []))
        return f"class {obj.name}({bases})" if bases else f"class {obj.name}"
    annotation = _text(getattr(obj, "annotation", None))
    value = _text(getattr(obj, "value", None))
    text = obj.name
    if annotation:
        text += f": {annotation}"
    if value:
        text += f" = {value}"
    return text


def _doc_sections(docstring: griffe.Docstring | None) -> dict[str, Any]:
    if docstring is None:
        return {}
    sections: dict[str, Any] = {}
    for section in docstring.parsed:
        kind = section.kind.value
        if kind == "parameters":
            sections["parameters"] = sorted(
                {f"{item.name}: {_summary(item.description)}" for item in section.value}
            )
        elif kind == "returns":
            sections["returns"] = [_summary(item.description) for item in section.value]
        elif kind == "raises":
            sections["raises"] = sorted(
                f"{_text(item.annotation)}: {_summary(item.description)}" for item in section.value
            )
        elif kind == "examples":
            sections["examples"] = True
    return sections


# --------------------------------------------------------------------------- item records


def _overload_signatures(member: griffe.Object) -> list[str]:
    variants = sorted(getattr(member, "overloads", None) or [], key=lambda v: v.lineno or 0)
    return [_signature(v) for v in variants]


def _members(owner: griffe.Class, site_packages: Path) -> list[dict]:
    """Members declared on this class, plus everything the MRO contributes.

    `declared_on` is the direct analogue of the Rust index's `via_trait`, and it is what makes an
    inheriting class legible: `FunctionTool` declares four names and inherits the rest, so a
    declared-only table would report it as nearly empty.
    """
    rows: dict[tuple[str, str], dict] = {}
    chain: list[griffe.Class] = [owner]
    try:
        chain.extend(owner.mro())
    except Exception as error:  # a base outside the indexed set cannot be resolved
        say(f"    mro unavailable for {owner.canonical_path}: {type(error).__name__}")
    for depth, holder in enumerate(chain):
        for name, member in sorted(holder.members.items()):
            if member.is_alias or member.kind.value not in ("function", "attribute"):
                continue
            key = (name, member.kind.value)
            if key in rows:
                continue
            row: dict[str, Any] = {
                "name": name,
                "kind": member.kind.value,
                "labels": sorted(member.labels or ()),
                "signature": _signature(member),
                "summary": _summary(member.docstring.value if member.docstring else ""),
                "declared_on": holder.canonical_path,
                "inherited": depth > 0,
                "lineno": member.lineno,
                "endlineno": member.endlineno,
                "file": _relative(member.filepath, site_packages),
            }
            # `@abstractmethod` is the difference between "you must write this" and "a default
            # exists and is probably the conservative answer". Recording it per member is what
            # lets an extension-point page split required from provided instead of listing
            # everything a subclass could touch.
            marks = getattr(member, "decorators", None) or []
            decorated = sorted({_text(mark.value) for mark in marks})
            if decorated:
                row["decorators"] = decorated
            overloads = _overload_signatures(member)
            if overloads:
                row["overloads"] = overloads
            annotated = getattr(member, "returns", None) or getattr(member, "annotation", None)
            if annotated is None:
                row["unannotated"] = True
            types = _type_names(annotated)
            if types:
                row["types_used"] = types
            rows[key] = row
    return [rows[key] for key in sorted(rows)]


def _item(member: griffe.Object, canonical: str, dist: str, site_packages: Path) -> dict:
    kind = member.kind.value
    docs = member.docstring.value if member.docstring else ""
    record: dict[str, Any] = {
        "path": canonical,
        "name": member.name,
        "kind": kind,
        "dist": dist,
        "module": canonical.rsplit(".", 1)[0] if "." in canonical else canonical,
        "signature": _signature(member),
        "summary": _summary(docs),
        "docs": docs,
        "labels": sorted(member.labels or ()),
        "lineno": member.lineno,
        "endlineno": member.endlineno,
        "file": _relative(member.filepath, site_packages),
        "underscore_free": underscore_free(canonical),
    }
    sections = _doc_sections(member.docstring)
    if sections:
        record["doc_sections"] = sections
    decorators = sorted({_text(d.value) for d in (getattr(member, "decorators", None) or [])})
    if decorators:
        record["decorators"] = decorators
    if kind == "class":
        record["bases"] = [_text(b) for b in (member.bases or [])]
        record["base_paths"] = sorted(
            {b.canonical_path for b in (member.bases or []) if hasattr(b, "canonical_path")}
        )
        record["members"] = _members(member, site_packages)
    if kind == "function":
        overloads = _overload_signatures(member)
        if overloads:
            # The implementation signature is the runtime dispatcher, not the callable contract.
            # Emitting only it -- which one-signature-per-item would do -- emits the wrong thing:
            # `FastMCP.tool`'s implementation says it returns a `FunctionTool`, while both
            # overloads and the runtime say it returns the decorated function unchanged.
            record["overloads"] = overloads
            record["dispatch_only"] = True
        if member.returns is None:
            record["unannotated_return"] = True
        types = _type_names(member.returns)
        if types:
            record["types_used"] = types
    if kind == "attribute" and getattr(member, "annotation", None) is None:
        record["unannotated"] = True
    return record


# --------------------------------------------------------------------------- traversal


def _collect(
    root: griffe.Module,
    dist: str,
    site_packages: Path,
    signals: dict,
    excluded: tuple[str, ...] = (),
) -> dict:
    """Walk one root into a flat fragment, so rebasing it is a single substitution."""
    items: dict[str, dict] = {}
    access: dict[str, str] = {}
    modules: set[str] = set()
    assignment_aliases: dict[str, str] = {}
    exports: dict[str, list[str]] = {}
    type_checking: dict[str, list[str]] = {}
    seen: set[str] = set()

    def excluded_module(path: str) -> bool:
        return any(path == skip or path.startswith(f"{skip}.") for skip in excluded)

    def visit(obj: griffe.Object, prefix: str) -> None:
        if prefix in seen:
            return
        seen.add(prefix)
        in_module = obj.kind.value == "module"
        # Generated data tables are excluded at the walk, not filtered downstream, so they never
        # reach the item set, the coverage expectation or the digest tree. Excluding them in only
        # one of those places is what makes assert_coverage pass vacuously.
        if in_module and excluded_module(prefix):
            return
        if in_module:
            modules.add(prefix)
            exports[prefix] = _module_exports(obj, prefix, site_packages, signals)
            module_file = getattr(obj, "filepath", None)
            if module_file is not None and Path(module_file).is_file():
                guarded = _type_checking_names(Path(module_file))
                if guarded:
                    type_checking[prefix] = sorted(guarded)
        for name, member in sorted(obj.members.items()):
            path = f"{prefix}.{name}"
            if member.is_alias:
                access[path] = member.target_path
                continue
            kind = member.kind.value
            canonical = member.canonical_path
            # A method is a member of its class, never a second top-level item. Admitting both
            # would double-count the surface, and worse, would make every class look like a
            # module -- `api/....FastMCP.md` alongside the module page that already documents
            # it. Nested *classes* stay items, because `Outer.Inner` is a type you can name.
            admissible = kind == "class" or (in_module and kind in ITEM_KINDS)
            if admissible:
                access.setdefault(path, canonical)
                if canonical not in items:
                    items[canonical] = _item(member, canonical, dist, site_packages)
                # `McpError = MCPError` is an Attribute whose value is a bare name. Griffe does
                # not treat it as an alias, so without this it reaches no alias table at all.
                if kind == "attribute":
                    value = getattr(member, "value", None)
                    if isinstance(value, griffe.ExprName):
                        assignment_aliases[canonical] = value.canonical_path
            # A class member's spelling is deliberately NOT recorded as an access path.
            # `Class.method` is answered by `members.tsv`, and admitting it here would fill
            # `unresolved.tsv` with thousands of methods -- drowning the rows that table exists
            # for, which are re-exports that genuinely leave the indexed set.
            if kind in ("module", "class"):
                visit(member, path)

    visit(root, root.name)
    return {
        "items": items,
        "access": access,
        "modules": sorted(modules),
        "assignment_aliases": assignment_aliases,
        "exports": exports,
        "type_checking": type_checking,
    }


def _rebase(fragment: dict, old_root: str, new_root: str) -> dict:
    """Rewrite every path in a fragment from one root to another.

    A namespace subpackage has to be loaded with its own directory on the search path, which
    makes Griffe believe it is top level: `fastmcp/contrib/mcp_mixin` comes back rooted at
    `mcp_mixin`. Rebasing is a whole-document substitution rather than a per-field one because
    the root appears in item paths, module names, both sides of the access map, `declared_on`,
    `base_paths` and `types_used` -- and missing any one of those leaves a reference that
    resolves nowhere.
    """
    encoded = json.dumps(fragment)
    encoded = encoded.replace(f'"{old_root}.', f'"{new_root}.')
    encoded = encoded.replace(f'"{old_root}"', f'"{new_root}"')
    return json.loads(encoded)


def _bound_names(body: list[ast.stmt]) -> list[str]:
    names: set[str] = set()
    for node in body:
        if isinstance(node, ast.Import | ast.ImportFrom):
            names.update(alias.asname or alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ClassDef | ast.FunctionDef | ast.AsyncFunctionDef):
            names.add(node.name)
        elif isinstance(node, ast.Assign):
            names.update(t.id for t in node.targets if isinstance(t, ast.Name))
    return sorted(names)


def _redundant_aliases(path: Path) -> set[str]:
    """Names re-exported by the PEP 484 `from .x import Y as Y` idiom.

    Griffe's `Module.exports` is `__all__` and nothing else. That is the whole public surface of
    a library that writes `__all__`, and almost none of it for a library that does not. Typer's
    `__init__.py` is thirty lines of `from ._click.termui import secho as secho` and declares no
    `__all__` at all, so the `__all__`-only reading returns an empty set for the one module whose
    exports matter most.

    A redundant alias is not an accident of style: PEP 484 gives it exactly this meaning, and
    type checkers already treat it as an explicit re-export. Reading it here is reading what the
    author wrote, not guessing.
    """
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except (OSError, SyntaxError):
        return set()
    names: set[str] = set()
    for node in tree.body:
        if isinstance(node, ast.Import | ast.ImportFrom):
            for alias in node.names:
                if alias.asname is not None and alias.asname == alias.name:
                    names.add(alias.asname)
    return names


def _type_checking_names(path: Path) -> set[str]:
    """Names bound only under `if TYPE_CHECKING:` -- importable to a checker, absent at runtime.

    Measured against rich 15.0.0: `rich/__init__.py` imports `Console` inside a TYPE_CHECKING
    guard and nowhere else. Griffe records the alias, so an index that trusts it reports
    `rich.Console` as the importable spelling. It is not one -- `from rich import Console` raises
    ImportError -- and both `ty` and `pyrefly` accept the line with zero errors, because that is
    precisely what the guard is for.

    So this is not a Griffe bug to route around; it is the sharpest thing this repository can
    tell an agent. A preferred spelling must be runtime-importable, and the guarded spelling is
    kept as its own alias kind rather than discarded, because the trap is worth naming.
    """
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except (OSError, SyntaxError):
        return set()

    def is_guard(test: ast.expr) -> bool:
        if isinstance(test, ast.Name) and test.id == "TYPE_CHECKING":
            return True
        return isinstance(test, ast.Attribute) and test.attr == "TYPE_CHECKING"

    guarded: set[str] = set()
    runtime: set[str] = set()
    for node in tree.body:
        if isinstance(node, ast.If) and is_guard(node.test):
            for statement in ast.walk(node):
                if isinstance(statement, ast.Import | ast.ImportFrom):
                    guarded.update(a.asname or a.name.split(".")[0] for a in statement.names)
        elif isinstance(node, ast.Import | ast.ImportFrom):
            runtime.update(a.asname or a.name.split(".")[0] for a in node.names)
    return guarded - runtime


def _main_guard_names(path: Path) -> set[str]:
    """Names bound under `if __name__ == "__main__":`.

    Nearly every rich module ends in a self-demo, and Griffe walks the guard body statically, so
    `rich/panel.py`'s throwaway `p = Panel(...)` and `c = Console()` arrive as module members
    indistinguishable from `Panel` itself. They are not API -- they never execute on import --
    and left in they would rank as exports and pollute the preferred-spelling ordering.
    """
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except (OSError, SyntaxError):
        return set()
    names: set[str] = set()
    for node in tree.body:
        if not isinstance(node, ast.If):
            continue
        test = node.test
        if not (
            isinstance(test, ast.Compare)
            and isinstance(test.left, ast.Name)
            and test.left.id == "__name__"
            and any(isinstance(c, ast.Constant) and c.value == "__main__" for c in test.comparators)
        ):
            continue
        for statement in ast.walk(node):
            if isinstance(statement, ast.Assign):
                names.update(t.id for t in statement.targets if isinstance(t, ast.Name))
            elif isinstance(statement, ast.AnnAssign) and isinstance(statement.target, ast.Name):
                names.add(statement.target.id)
            elif isinstance(statement, ast.Import | ast.ImportFrom):
                names.update(a.asname or a.name.split(".")[0] for a in statement.names)
            elif isinstance(statement, ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef):
                names.add(statement.name)
    return names


def _public_definitions(obj: griffe.Object, prefix: str) -> set[str]:
    """Non-underscore names *defined in* a non-underscore module.

    Rich's convention. It documents `rich.table.Table` and never re-exports it, so neither of the
    other two signals sees it. The "defined in" half is what keeps the signal honest: an alias is
    skipped, which is why `rich/__init__.py`'s module-level `from typing import Any, Callable,
    Optional, Union` does not turn the typing module into rich's public API.

    Names bound under the module's `if __name__ == "__main__":` demo are subtracted for the same
    reason: measured, this signal alone reported `rich.panel` as exporting `c` and `p`.
    """
    if not underscore_free(prefix):
        return set()
    path = getattr(obj, "filepath", None)
    demo = _main_guard_names(Path(path)) if path is not None and Path(path).is_file() else set()
    names: set[str] = set()
    for name, member in obj.members.items():
        if name.startswith("_") or member.is_alias or name in demo:
            continue
        if member.kind.value in ITEM_KINDS or member.kind.value == "class":
            names.add(name)
    return names


def _module_exports(
    obj: griffe.Object, prefix: str, site_packages: Path, signals: dict
) -> list[str]:
    """Merge the enabled export signals for one module into one sorted set."""
    names: set[str] = set()
    if signals.get("dunder_all", True):
        names.update(obj.exports or ())
    if signals.get("redundant_alias"):
        path = getattr(obj, "filepath", None)
        if path is not None and Path(path).is_file():
            names.update(_redundant_aliases(Path(path)))
    if signals.get("public_module_definition"):
        names.update(_public_definitions(obj, prefix))
    return sorted(names)


def _conditional_imports(roots: list[griffe.Module], site_packages: Path) -> list[dict]:
    """Names bound under a module-level `try` with an `ImportError` handler.

    This is the Python answer to a Cargo feature, and it is sharper than "the import fails".
    `fastmcp.exceptions` does `try: from mcp import MCPError` with a handler that defines a local
    stand-in, and Griffe records whichever branch is live. So an optional dependency does not
    merely gate availability -- it changes what a canonical item *is*. Recording which branch
    this build saw is what stops the index asserting a fallback as though it were the library.
    """
    found: list[dict] = []
    seen_files: set[str] = set()
    for root in roots:
        for module in [root, *root.modules.values()]:
            path = getattr(module, "filepath", None)
            if path is None or not Path(path).is_file():
                continue
            relative = _relative(Path(path), site_packages)
            if relative in seen_files:
                continue
            seen_files.add(relative)
            try:
                tree = ast.parse(Path(path).read_text(encoding="utf-8"))
            except SyntaxError:
                continue
            for node in tree.body:
                if not isinstance(node, ast.Try):
                    continue
                if not any(
                    isinstance(h.type, ast.Name) and h.type.id == "ImportError"
                    for h in node.handlers
                ):
                    continue
                fallback = {n for h in node.handlers for n in _bound_names(h.body)}
                found.extend(
                    {
                        "module": module.path,
                        "file": relative,
                        "name": name,
                        "guard": "try-except-ImportError",
                        "fallback_defines": name in fallback,
                    }
                    for name in _bound_names(node.body)
                )
    return sorted(found, key=lambda row: (row["module"], row["name"]))


def extract(
    module_name: str,
    dist: str,
    version: str,
    search_paths: list[str],
    *,
    namespace_packages: tuple[str, ...] = (),
    export_signals: dict | None = None,
    exclude_modules: tuple[str, ...] = (),
) -> dict:
    """Model one distribution into the normalized document."""
    signals = export_signals or {
        "dunder_all": True,
        "redundant_alias": False,
        "public_module_definition": False,
    }
    site_packages = Path(search_paths[0]).resolve()
    roots: list[tuple[griffe.Module, str]] = []
    loaded: list[str] = []
    not_indexed: list[dict] = []

    def load(name: str, paths: list[str]) -> griffe.Module:
        return griffe.load(
            name,
            search_paths=paths,
            allow_inspection=False,
            resolve_aliases=False,
            docstring_parser="google",
        )

    roots.append((load(module_name, search_paths), ""))
    loaded.append(module_name)

    # `fastmcp/contrib/` has no `__init__.py`, so it is an implicit namespace package: Griffe
    # cannot reach it from the package root even though `from fastmcp.contrib.mcp_mixin import
    # MCPMixin` works at runtime. Loading each child with the namespace directory on the search
    # path finds it; rebasing puts it back where callers spell it. Left alone, three subpackages
    # -- including the documented way to declare tools on a class -- produce no symbol, no
    # unresolved row and no warning.
    for dotted in namespace_packages:
        parent, _, leaf = dotted.rpartition(".")
        directory = site_packages.joinpath(*parent.split(".")) if parent else site_packages
        try:
            roots.append((load(leaf, [str(directory), *search_paths]), dotted))
            loaded.append(dotted)
        except Exception as error:
            not_indexed.append({"module": dotted, "reason": f"{type(error).__name__}: {error}"})

    items: dict[str, dict] = {}
    access: dict[str, str] = {}
    modules: set[str] = set()
    assignment_aliases: dict[str, str] = {}
    exports_by_module: dict[str, list[str]] = {}
    type_checking_by_module: dict[str, list[str]] = {}

    for root, rebase_to in roots:
        fragment = _collect(root, dist, site_packages, signals, exclude_modules)
        if rebase_to:
            fragment = _rebase(fragment, root.name, rebase_to)
        items.update(fragment["items"])
        access.update(fragment["access"])
        modules.update(fragment["modules"])
        assignment_aliases.update(fragment["assignment_aliases"])
        exports_by_module.update(fragment["exports"])
        type_checking_by_module.update(fragment["type_checking"])

    return {
        "schema": SCHEMA,
        "dist": dist,
        "module": module_name,
        "version": version,
        "loaded_roots": sorted(loaded),
        "modules": sorted(modules),
        "items": [items[path] for path in sorted(items)],
        "access": dict(sorted(access.items())),
        "assignment_aliases": dict(sorted(assignment_aliases.items())),
        "exports": {name: sorted(values) for name, values in sorted(exports_by_module.items())},
        "conditional": _conditional_imports([root for root, _ in roots], site_packages),
        "not_indexed": sorted(not_indexed, key=lambda row: row["module"]),
        "type_checking": {
            name: sorted(values) for name, values in sorted(type_checking_by_module.items())
        },
        "export_signals": dict(sorted(signals.items())),
        "excluded_modules": sorted(exclude_modules),
    }

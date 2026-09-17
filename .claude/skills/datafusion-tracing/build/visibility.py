"""Classify how a caller can actually reach each item and method of a subject crate.

This is the module the repository exists for. `datafusion-tracing` declares `mod options;` and
`mod rule_options;` private and then selectively re-exports two types out of them. Everything
else in those modules is `pub` and unreachable by path -- which rustdoc treats as "not part of
the documented surface" and a caller experiences as "the methods the README chains do not exist".

Four values, and the discriminator between the middle two is not rustdoc's `visibility` field:

    supported               in the hosted document, and not doc(hidden) in the source
    doc-hidden              in the hosted document, but the source marks it #[doc(hidden)]
    reachable-undocumented  rustdoc calls it public, no published artifact carries it, and a
                            supported item hands it to you
    internal                everything else, including types rustdoc calls public that nothing
                            public ever yields

`InstrumentedExec` is the case that forces the last distinction. rustdoc records it `public`, so
a classification that trusted that field would file it beside the option builders -- yet upstream
documents it as deliberately private, nothing public returns it, and there is no spelling that
reaches it. The builders are the opposite: also invisible, but `InstrumentationOptions::builder()`
returns one, so the methods on it are the API you are meant to use. Collapsing those two into one
value would be worse than not classifying at all, because it would recommend a type that cannot
be named.

Two facts here cannot come from rustdoc and are read from the pinned source instead:

    #[doc(hidden)]  is not recorded in `attrs` at format 61 -- measured, not assumed
    a re-export rustdoc omits entirely, which is how `instrument_session_state` behaves: it is
    `pub use`d in lib.rs and appears in NEITHER document, even with --document-private-items

Standard library plus the ast-grep binary.
"""

from __future__ import annotations

import json
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

SUPPORTED = "supported"
DOC_HIDDEN = "doc-hidden"
REACHABLE = "reachable-undocumented"
INTERNAL = "internal"

ORDER = (SUPPORTED, DOC_HIDDEN, REACHABLE, INTERNAL)

# `pub use <path>;` under a `#[doc(hidden)]` attribute. A code shape, so it is an ast-grep rule
# rather than a regex over the file: in tree-sitter's Rust grammar an attribute is a SIBLING of
# the item it decorates, not a child, which a line-oriented pattern gets wrong in both
# directions -- it misses an attribute separated by a comment and claims one that decorates the
# previous item.
DOC_HIDDEN_RULE = """
id: source-doc-hidden-reexport
language: rust
severity: hint
message: re-export marked doc(hidden)
rule:
  kind: use_declaration
  has: { kind: visibility_modifier, regex: '^pub$' }
  follows:
    kind: attribute_item
    stopBy: neighbor
    has: { kind: attribute, regex: 'doc\\(hidden\\)' }
"""

REEXPORT = re.compile(r"\bpub\s+use\s+([A-Za-z_][A-Za-z0-9_:]*)\s*;")


class VisibilityError(RuntimeError):
    """The classification could not be established, so no row may claim one."""


@dataclass
class Classification:
    items: dict[str, str] = field(default_factory=dict)
    public_fields: dict[str, list[str]] = field(default_factory=dict)
    reached_via: dict[str, str] = field(default_factory=dict)
    methods: dict[tuple[str, str], str] = field(default_factory=dict)
    absent_from_rustdoc: list[str] = field(default_factory=list)
    doc_hidden: list[str] = field(default_factory=list)

    def of_item(self, path: str) -> str:
        return self.items.get(path, INTERNAL)

    def of_method(self, owner: str, name: str) -> str:
        return self.methods.get((owner, name), self.of_item(owner))


# --------------------------------------------------------------------------- source facts

def doc_hidden_reexports(source: Path) -> list[str]:
    """Names the pinned source re-exports under `#[doc(hidden)]`.

    Read from the source because rustdoc does not carry it. Measured on datafusion-tracing
    55.0.0: `new_instrument_rule` is in the hosted document with `attrs: []`, while its source
    declaration is preceded by `#[doc(hidden)]`. Inferring the attribute from the document is
    therefore not merely unreliable, it is impossible.
    """
    if not source.is_dir():
        raise VisibilityError(f"{source} is not a directory; the source corpus is required")
    result = subprocess.run(
        ["ast-grep", "scan", "--inline-rules", DOC_HIDDEN_RULE, "--json=stream", str(source)],
        capture_output=True, text=True, check=False,
    )
    if result.returncode not in (0, 1):
        raise VisibilityError(
            f"ast-grep exited {result.returncode} scanning {source}: "
            f"{result.stderr.strip()[:300]}"
        )
    names: list[str] = []
    for line in result.stdout.splitlines():
        if not line.strip():
            continue
        match = REEXPORT.search(json.loads(line)["text"])
        if match:
            names.append(match.group(1).rsplit("::", 1)[-1])
    return sorted(set(names))


def declared_reexports(source: Path) -> list[str]:
    """Every `pub use` leaf name in the crate root.

    Used only to find the re-exports rustdoc drops. `lib.rs` is the one file where the
    difference matters, because a name re-exported there is the crate's advertised surface.
    """
    root = source / "lib.rs"
    if not root.exists():
        raise VisibilityError(f"{root} is absent; cannot read the advertised surface")
    return sorted({m.rsplit("::", 1)[-1] for m in REEXPORT.findall(root.read_text())})


# --------------------------------------------------------------------------- documents

def _own_paths(document: dict) -> dict[str, str]:
    """{rustdoc id: canonical path} for items this crate defines."""
    return {
        str(identifier): "::".join(entry["path"])
        for identifier, entry in document["paths"].items()
        if entry["crate_id"] == 0
    }


def _visibility_of(item: dict | None) -> str:
    raw = (item or {}).get("visibility")
    return raw if isinstance(raw, str) else "restricted"


def _inherent_methods(document: dict, crate_prefix: str) -> dict[str, dict[str, dict]]:
    """{owner canonical path: {method name: method item}} over inherent impls of this crate."""
    index = {str(k): v for k, v in document["index"].items()}
    paths = _own_paths(document)
    found: dict[str, dict[str, dict]] = {}
    for entry in index.values():
        block = (entry.get("inner") or {}).get("impl")
        if not block or block.get("blanket_impl") or block.get("is_synthetic"):
            continue
        if block.get("trait"):
            continue
        target = (block.get("for") or {}).get("resolved_path") or {}
        owner = paths.get(str(target.get("id")))
        if not owner or not owner.startswith(crate_prefix):
            continue
        for member_id in block.get("items", []):
            member = index.get(str(member_id))
            if member and member.get("name"):
                found.setdefault(owner, {})[member["name"]] = member
    return found


def _signature_surface(document: dict, supported_paths: set[str]) -> str:
    """The rendered records of every nameable item, for the reachability search.

    A type is reachable when something you can name hands it to you. Rather than walking
    rustdoc's type trees -- which would have to model every wrapper a return type can nest a
    name inside -- the nameable items' rendered JSON is searched for the leaf name. That is
    deliberately generous: it can call a type reachable because a nameable declaration merely
    mentions it. Reporting one type as reachable that is not is a smaller error than hiding
    sixteen methods a caller has no other description of.
    """
    index = {str(k): v for k, v in document["index"].items()}
    paths = _own_paths(document)
    keep = [
        json.dumps(entry)
        for identifier, entry in index.items()
        if paths.get(identifier) in supported_paths
    ]
    # A struct's fields are separate index entries, not inline in the struct's own record, so
    # a public field's type is invisible to a search over the struct alone. That would file
    # `PreviewFn` as internal, when in fact it is the declared type of `InstrumentationOptions
    # .preview_fn` -- a name a caller meets in the signature they are trying to satisfy and
    # cannot write, which is exactly what the reachable-undocumented row is for.
    for identifier, entry in index.items():
        if paths.get(identifier) not in supported_paths:
            continue
        shape = ((entry.get("inner") or {}).get("struct") or {}).get("kind") or {}
        for field_id in (shape.get("plain") or {}).get("fields", []):
            member = index.get(str(field_id))
            if member is not None:
                keep.append(json.dumps(member))

    # Impl blocks carry the methods, and an impl has no `paths` entry of its own, so include
    # those whose `for` type is nameable. Without this, `InstrumentationOptions::builder()` --
    # the one declaration that yields the builder -- is not in the surface at all.
    for entry in index.values():
        block = (entry.get("inner") or {}).get("impl")
        if not block or block.get("blanket_impl") or block.get("is_synthetic"):
            continue
        target = (block.get("for") or {}).get("resolved_path") or {}
        if paths.get(str(target.get("id"))) in supported_paths:
            keep.append(json.dumps(entry))
            for member_id in block.get("items", []):
                member = index.get(str(member_id))
                if member is not None:
                    keep.append(json.dumps(member))
    return "\n".join(keep)


def nameable_paths(document: dict) -> set[str]:
    """Canonical paths a caller can actually write, by walking the crate root.

    This is the discriminator the whole classification rests on, and it is not rustdoc's
    `visibility` field. `InstrumentationOptionsBuilder` is recorded `public` and has a `paths`
    entry -- it is a return type, so rustdoc must name it somewhere -- yet docs.rs answers 404
    for its page and it is absent from `all.html`, because the module holding it is private and
    nothing re-exports it. Walking from the root through public modules and explicit re-exports
    reproduces exactly the fifteen items the rendered documentation shows.
    """
    index = {str(k): v for k, v in document["index"].items()}
    paths = _own_paths(document)
    root = str(document["root"])
    seen: set[str] = set()
    queue = [root]
    while queue:
        identifier = queue.pop()
        entry = index.get(identifier)
        if entry is None:
            continue
        canonical = paths.get(identifier)
        if canonical:
            if canonical in seen:
                continue
            seen.add(canonical)
        inner = entry.get("inner") or {}
        if "module" in inner:
            queue.extend(str(child) for child in inner["module"].get("items", []))
        elif "use" in inner:
            target = inner["use"].get("id")
            if target is not None:
                target_path = paths.get(str(target))
                if target_path:
                    seen.add(target_path)
                queue.append(str(target))
    return seen


# --------------------------------------------------------------------------- classification

def classify(
    package: str,
    public_document: bytes,
    private_document: bytes,
    source: Path,
) -> Classification:
    """Assign a visibility to every item and inherent method of one subject crate."""
    public = json.loads(public_document)
    private = json.loads(private_document)
    crate_prefix = package.replace("-", "_")

    hidden = set(doc_hidden_reexports(source))
    advertised = set(declared_reexports(source))

    nameable = nameable_paths(public)
    public_paths = set(_own_paths(public).values())
    private_index = {str(k): v for k, v in private["index"].items()}
    private_paths = _own_paths(private)
    public_index_by_path = {
        path: {str(k): v for k, v in public["index"].items()}.get(identifier)
        for identifier, path in _own_paths(public).items()
    }

    result = Classification(doc_hidden=sorted(hidden))

    for path in sorted(nameable):
        leaf = path.rsplit("::", 1)[-1]
        result.items[path] = DOC_HIDDEN if leaf in hidden else SUPPORTED

    supported_paths = {p for p, v in result.items.items() if v == SUPPORTED}
    surface = _signature_surface(public, supported_paths)

    def mentioned(leaf: str) -> bool:
        return bool(re.search(rf'"{re.escape(leaf)}"', surface))

    # Everything else this crate defines, from whichever document carries it. rustdoc's own
    # `visibility` is necessary but not sufficient: it calls `InstrumentedExec` public, and
    # upstream documents that type as deliberately unreachable.
    for path in sorted(public_paths | set(private_paths.values())):
        if path in result.items:
            continue
        leaf = path.rsplit("::", 1)[-1]
        if leaf in hidden:
            # Callable, and the macros expand to it -- but upstream says it is not the API.
            result.items[path] = DOC_HIDDEN
            continue
        identifier = next((i for i, p in private_paths.items() if p == path), None)
        declared = _visibility_of(private_index.get(identifier)) if identifier else None
        if declared is None and path in public_paths:
            declared = _visibility_of(public_index_by_path.get(path))
        if declared == "public" and mentioned(leaf):
            result.items[path] = REACHABLE
        else:
            result.items[path] = INTERNAL

    # Methods. An inherent method present only in the private document is reachable exactly when
    # its owner is -- which is what makes the option builders legible: the struct is in the
    # hosted document's `paths` with no impl block at all, so the sixteen methods are invisible
    # while the type that carries them is not.
    public_methods = _inherent_methods(public, crate_prefix)
    private_methods = _inherent_methods(private, crate_prefix)
    for owner, members in private_methods.items():
        owner_visibility = result.of_item(owner)
        for name in members:
            if name in public_methods.get(owner, {}):
                result.methods[(owner, name)] = result.items.get(owner, SUPPORTED)
            elif owner_visibility in (SUPPORTED, DOC_HIDDEN, REACHABLE):
                result.methods[(owner, name)] = REACHABLE
            else:
                result.methods[(owner, name)] = INTERNAL

    for path, value in result.items.items():
        if value in (REACHABLE, DOC_HIDDEN):
            via = _reached_via(path, public, supported_paths)
            if via:
                result.reached_via[path] = via
    for (owner, name), value in result.methods.items():
        if value != REACHABLE:
            continue
        via = result.reached_via.get(owner) or _reached_via(owner, public, supported_paths)
        if via:
            result.reached_via[f"{owner}::{name}"] = via

    # Which of a struct's fields a caller can actually read or set. Necessary because the
    # subject crates are modelled from the PRIVATE capture, where rustdoc lists private fields
    # like any other -- so an unfiltered field list claims `RuleInstrumentationOptions` has four
    # public fields when all four are `pub(crate)`. That is the same error this module exists to
    # correct, pointing the other way: it would send a reader to a struct literal that cannot
    # compile. `InstrumentationOptions` really does have four, and they really are the trap.
    result.public_fields = _public_fields(private)

    # A name the crate root advertises that neither document carries. Recorded rather than
    # dropped: silence in an index is not evidence, and this is the one class of item where the
    # index would otherwise disagree with the crate's own lib.rs.
    known = {p.rsplit("::", 1)[-1] for p in result.items}
    result.absent_from_rustdoc = sorted(advertised - known)
    return result


def _public_fields(document: dict) -> dict[str, list[str]]:
    """{struct canonical path: field names rustdoc records as public}."""
    index = {str(k): v for k, v in document["index"].items()}
    paths = _own_paths(document)
    found: dict[str, list[str]] = {}
    for identifier, path in paths.items():
        entry = index.get(identifier)
        if entry is None:
            continue
        shape = ((entry.get("inner") or {}).get("struct") or {}).get("kind")
        # `kind` is the string "unit" for a unit struct and a tagged object otherwise, so it
        # cannot be indexed unconditionally. A tuple struct's fields have no names and are not
        # what this column is about.
        if not isinstance(shape, dict):
            continue
        names = []
        for field_id in (shape.get("plain") or {}).get("fields", []):
            member = index.get(str(field_id))
            if member and member.get("name") and _visibility_of(member) == "public":
                names.append(member["name"])
        if names:
            found[path] = sorted(names)
    return found


def _reached_via(path: str, public: dict, supported_paths: set[str]) -> str:
    """Name a nameable declaration that yields this type, for the `reached_via` column.

    The shortest spelling wins, because the column is read as an instruction: what a caller
    wants is the one call that puts the value in their hand, not an inventory of every
    signature the name appears in.
    """
    leaf = path.rsplit("::", 1)[-1]
    index = {str(k): v for k, v in public["index"].items()}
    paths = _own_paths(public)
    candidates: list[str] = []
    for entry in index.values():
        block = (entry.get("inner") or {}).get("impl")
        if not block or block.get("blanket_impl") or block.get("is_synthetic") or block.get("trait"):
            continue
        target = (block.get("for") or {}).get("resolved_path") or {}
        owner_path = paths.get(str(target.get("id")))
        if owner_path not in supported_paths:
            continue
        owner_leaf = owner_path.rsplit("::", 1)[-1]
        for member_id in block.get("items", []):
            member = index.get(str(member_id))
            if member and member.get("name") and re.search(
                rf'"{re.escape(leaf)}"', json.dumps(member)
            ):
                candidates.append(f"{owner_leaf}::{member['name']}()")
    if candidates:
        return min(sorted(set(candidates)), key=len)
    for identifier, owner_path in paths.items():
        if owner_path in supported_paths and re.search(
            rf'"{re.escape(leaf)}"', json.dumps(index.get(identifier, {}))
        ):
            return f"{owner_path.rsplit('::', 1)[-1]} (named in its declaration)"
    return ""

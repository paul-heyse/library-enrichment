"""Render rustdoc JSON type trees back into Rust signature text.

rustdoc emits structure, not source. Every signature an agent reads is reconstructed here, so this
module is the one place where a mistake becomes thousands of wrong signatures. The shapes it
handles were confirmed against real `format_version` 61 documents rather than recalled: see the
variant list in `render_type`.

Rendering is deliberately source-shaped rather than fully qualified. rustdoc records the path as
written at the definition site, which is what a reader recognizes; canonical paths live in the
model records instead.
"""

from __future__ import annotations

import re

MAX_DEPTH = 64


def _path(raw: str) -> str:
    """Normalize a rustdoc path for display."""
    for prefix in ("$crate::", "crate::"):
        if raw.startswith(prefix):
            return raw[len(prefix) :]
    return raw


def render_generic_args(args: dict | None, depth: int) -> str:
    """Render the `<...>` or `(...)` that follows a path."""
    if not args:
        return ""
    if "angle_bracketed" in args:
        block = args["angle_bracketed"]
        pieces = [render_generic_arg(a, depth + 1) for a in block.get("args") or []]
        pieces += [render_constraint(c, depth + 1) for c in block.get("constraints") or []]
        pieces = [p for p in pieces if p]
        return f"<{', '.join(pieces)}>" if pieces else ""
    if "parenthesized" in args:
        block = args["parenthesized"]
        inputs = ", ".join(render_type(t, depth + 1) for t in block.get("inputs") or [])
        output = block.get("output")
        tail = f" -> {render_type(output, depth + 1)}" if output else ""
        return f"({inputs}){tail}"
    return ""


def render_generic_arg(arg: object, depth: int) -> str:
    if not isinstance(arg, dict):
        return "_"
    if "lifetime" in arg:
        return str(arg["lifetime"])
    if "type" in arg:
        return render_type(arg["type"], depth)
    if "const" in arg:
        constant = arg["const"] or {}
        return str(constant.get("expr") or constant.get("value") or "_")
    return "_"


def render_constraint(constraint: dict, depth: int) -> str:
    """Render an associated-item constraint, e.g. `Item = &'a Expr` or `Output: Debug`."""
    name = constraint.get("name", "")
    args = render_generic_args(constraint.get("args"), depth)
    binding = constraint.get("binding") or {}
    if "equality" in binding:
        equality = binding["equality"]
        if "type" in equality:
            return f"{name}{args} = {render_type(equality['type'], depth + 1)}"
        constant = equality.get("const") or {}
        return f"{name}{args} = {constant.get('expr') or constant.get('value') or '_'}"
    if "constraint" in binding:
        bounds = " + ".join(render_bound(b, depth + 1) for b in binding["constraint"] or [])
        return f"{name}{args}: {bounds}"
    return f"{name}{args}"


def render_bound(bound: object, depth: int) -> str:
    if not isinstance(bound, dict):
        return "_"
    if "trait_bound" in bound:
        block = bound["trait_bound"]
        trait = block.get("trait") or {}
        text = _path(trait.get("path", "")) + render_generic_args(trait.get("args"), depth)
        modifier = block.get("modifier", "none")
        if modifier == "maybe":
            text = f"?{text}"
        for_params = block.get("generic_params") or []
        if for_params:
            bound_names = ", ".join(p.get("name", "") for p in for_params)
            text = f"for<{bound_names}> {text}"
        return text
    if "outlives" in bound:
        return str(bound["outlives"])
    if "use" in bound:
        captured = ", ".join(str(u) for u in bound["use"] or [])
        return f"use<{captured}>"
    return "_"


def render_type(node: object, depth: int = 0) -> str:
    """Render one rustdoc `Type` node.

    Every variant the format defines is handled; an unrecognized shape renders as `_` rather than
    raising, because a single odd node must not lose an otherwise correct signature.
    """
    if depth > MAX_DEPTH:
        return "..."
    if node is None:
        return "()"
    if isinstance(node, str):
        return node
    if not isinstance(node, dict):
        return "_"

    if "resolved_path" in node:
        block = node["resolved_path"]
        return _path(block.get("path", "")) + render_generic_args(block.get("args"), depth)

    if "generic" in node:
        return str(node["generic"])

    if "primitive" in node:
        return str(node["primitive"])

    if "borrowed_ref" in node:
        block = node["borrowed_ref"]
        lifetime = f"{block['lifetime']} " if block.get("lifetime") else ""
        mutable = "mut " if block.get("is_mutable") else ""
        return f"&{lifetime}{mutable}{render_type(block.get('type'), depth + 1)}"

    if "raw_pointer" in node:
        block = node["raw_pointer"]
        kind = "mut" if block.get("is_mutable") else "const"
        return f"*{kind} {render_type(block.get('type'), depth + 1)}"

    if "slice" in node:
        return f"[{render_type(node['slice'], depth + 1)}]"

    if "array" in node:
        block = node["array"]
        length = block.get("len", "_")
        return f"[{render_type(block.get('type'), depth + 1)}; {length}]"

    if "tuple" in node:
        members = node["tuple"] or []
        if not members:
            return "()"
        if len(members) == 1:
            return f"({render_type(members[0], depth + 1)},)"
        return "(" + ", ".join(render_type(m, depth + 1) for m in members) + ")"

    if "dyn_trait" in node:
        block = node["dyn_trait"]
        traits = []
        for entry in block.get("traits") or []:
            trait = entry.get("trait") or {}
            args = render_generic_args(trait.get("args"), depth)
            traits.append(_path(trait.get("path", "")) + args)
        if block.get("lifetime"):
            traits.append(str(block["lifetime"]))
        return f"dyn {' + '.join(traits)}" if traits else "dyn _"

    if "impl_trait" in node:
        bounds = " + ".join(render_bound(b, depth + 1) for b in node["impl_trait"] or [])
        return f"impl {bounds}" if bounds else "impl _"

    if "qualified_path" in node:
        block = node["qualified_path"]
        self_type = render_type(block.get("self_type"), depth + 1)
        trait = block.get("trait") or {}
        trait_path = _path(trait.get("path", "")) if trait else ""
        name = block.get("name", "")
        args = render_generic_args(block.get("args"), depth)
        if trait_path:
            return f"<{self_type} as {trait_path}>::{name}{args}"
        return f"{self_type}::{name}{args}"

    if "function_pointer" in node:
        block = node["function_pointer"]
        signature = block.get("sig") or {}
        inputs = ", ".join(render_type(t, depth + 1) for _, t in signature.get("inputs") or [])
        output = signature.get("output")
        tail = f" -> {render_type(output, depth + 1)}" if output else ""
        header = block.get("header") or {}
        unsafe = "unsafe " if header.get("is_unsafe") else ""
        return f"{unsafe}fn({inputs}){tail}"

    if "pat" in node:
        # Pattern types are unstable; render the base type and drop the refinement.
        return render_type((node["pat"] or {}).get("type"), depth + 1)

    if "infer" in node:
        return "_"

    return "_"


def render_generics(generics: dict | None) -> tuple[str, str]:
    """Return the `<...>` parameter list and the `where ...` clause for a definition."""
    if not generics:
        return "", ""

    params = []
    for param in generics.get("params") or []:
        name = param.get("name", "")
        kind = param.get("kind") or {}
        if "lifetime" in kind:
            outlives = kind["lifetime"].get("outlives") or []
            params.append(f"{name}: {' + '.join(outlives)}" if outlives else name)
        elif "type" in kind:
            block = kind["type"]
            if block.get("is_synthetic"):
                # `impl Trait` in argument position; already rendered at the use site.
                continue
            bounds = " + ".join(render_bound(b, 1) for b in block.get("bounds") or [])
            text = f"{name}: {bounds}" if bounds else name
            if block.get("default"):
                text += f" = {render_type(block['default'], 1)}"
            params.append(text)
        elif "const" in kind:
            block = kind["const"]
            text = f"const {name}: {render_type(block.get('type'), 1)}"
            if block.get("default"):
                text += f" = {block['default']}"
            params.append(text)

    clauses = []
    for predicate in generics.get("where_predicates") or []:
        if "bound_predicate" in predicate:
            block = predicate["bound_predicate"]
            subject = render_type(block.get("type"), 1)
            bounds = " + ".join(render_bound(b, 1) for b in block.get("bounds") or [])
            for_params = block.get("generic_params") or []
            if for_params:
                names = ", ".join(p.get("name", "") for p in for_params)
                subject = f"for<{names}> {subject}"
            if bounds:
                clauses.append(f"{subject}: {bounds}")
        elif "lifetime_predicate" in predicate:
            block = predicate["lifetime_predicate"]
            outlives = " + ".join(block.get("outlives") or [])
            if outlives:
                clauses.append(f"{block.get('lifetime', '')}: {outlives}")
        elif "eq_predicate" in predicate:
            block = predicate["eq_predicate"]
            lhs = render_type(block.get("lhs"), 1)
            rhs = render_type(block.get("rhs"), 1)
            clauses.append(f"{lhs} == {rhs}")

    param_text = f"<{', '.join(params)}>" if params else ""
    where_text = f" where {', '.join(clauses)}" if clauses else ""
    return param_text, where_text


ASYNC_TRAIT_LIFETIME = "'async_trait"
SYNTHETIC_LIFETIME = re.compile(r"'(?:life\d+|async_trait)\b\s*\+?\s*")


def _async_trait_output(output: object) -> object | None:
    """Recover `T` from the `Pin<Box<dyn Future<Output = T> + Send + 'async_trait>>` desugaring.

    `#[async_trait]` rewrites an async method into a boxed future before rustdoc ever sees it, so
    the recorded signature of `TableProvider::scan` is a five-lifetime monster that no reader would
    recognize. Returns the awaited type when the shape matches, otherwise None.
    """
    if not isinstance(output, dict):
        return None
    pin = output.get("resolved_path")
    if not pin or not pin.get("path", "").endswith("Pin"):
        return None
    pin_args = ((pin.get("args") or {}).get("angle_bracketed") or {}).get("args") or []
    if not pin_args:
        return None
    boxed = (pin_args[0] or {}).get("type") or {}
    box = boxed.get("resolved_path")
    if not box or not box.get("path", "").endswith("Box"):
        return None
    box_args = ((box.get("args") or {}).get("angle_bracketed") or {}).get("args") or []
    if not box_args:
        return None
    dyn_block = ((box_args[0] or {}).get("type") or {}).get("dyn_trait")
    if not dyn_block:
        return None
    for entry in dyn_block.get("traits") or []:
        trait = entry.get("trait") or {}
        if not trait.get("path", "").endswith("Future"):
            continue
        constraints = ((trait.get("args") or {}).get("angle_bracketed") or {}).get("constraints")
        for constraint in constraints or []:
            if constraint.get("name") == "Output":
                equality = (constraint.get("binding") or {}).get("equality") or {}
                if "type" in equality:
                    return equality["type"]
    return None


def _strip_async_trait_generics(generics: dict | None) -> dict | None:
    """Drop the synthetic lifetimes `#[async_trait]` introduces."""
    if not generics:
        return generics
    params = [
        param
        for param in generics.get("params") or []
        if not (
            param.get("name", "").startswith("'life") or param.get("name") == ASYNC_TRAIT_LIFETIME
        )
    ]
    clauses = []
    for predicate in generics.get("where_predicates") or []:
        if "lifetime_predicate" in predicate:
            block = predicate["lifetime_predicate"]
            if ASYNC_TRAIT_LIFETIME in (block.get("outlives") or []):
                continue
        if "bound_predicate" in predicate:
            bounds = predicate["bound_predicate"].get("bounds") or []
            synthetic = any(
                isinstance(b, dict) and b.get("outlives") == ASYNC_TRAIT_LIFETIME for b in bounds
            )
            if synthetic:
                continue
        clauses.append(predicate)
    return {"params": params, "where_predicates": clauses}


def render_function(name: str, inner: dict) -> str:
    """Render a full `fn` signature, including header qualifiers and the where clause."""
    header = inner.get("header") or {}
    signature = inner.get("sig") or {}

    awaited = _async_trait_output(signature.get("output"))
    desugared = awaited is not None
    if desugared:
        header = {**header, "is_async": True}
        signature = {**signature, "output": awaited}
        inner = {**inner, "generics": _strip_async_trait_generics(inner.get("generics"))}
    prefix = ""
    if header.get("is_const"):
        prefix += "const "
    if header.get("is_async"):
        prefix += "async "
    if header.get("is_unsafe"):
        prefix += "unsafe "
    abi = header.get("abi")
    if abi and abi != "Rust":
        abi_name = abi if isinstance(abi, str) else next(iter(abi), "C")
        prefix += f'extern "{abi_name}" '

    params, where_clause = render_generics(inner.get("generics"))

    arguments = []
    for argument_name, argument_type in signature.get("inputs") or []:
        rendered = render_type(argument_type, 1)
        if argument_name == "self":
            # rustdoc records the receiver as a normal argument named `self`.
            arguments.append(
                "self" if rendered in ("Self", "_") else rendered.replace("Self", "self")
            )
        else:
            arguments.append(f"{argument_name}: {rendered}")
    if signature.get("is_c_variadic"):
        arguments.append("...")

    output = signature.get("output")
    returns = f" -> {render_type(output, 1)}" if output else ""
    text = f"{prefix}fn {name}{params}({', '.join(arguments)}){returns}{where_clause}"
    if desugared:
        # The synthetic lifetimes survive inside argument types, where they are noise: the
        # source reads `&self`, not `&'life0 self`. These names cannot occur in real code.
        text = SYNTHETIC_LIFETIME.sub("", text)
    return text


def render_item_header(name: str, kind: str, inner: dict) -> str:
    """Render the one-line declaration header for a non-function item."""
    if kind == "function":
        return render_function(name, inner)

    params, where_clause = render_generics(inner.get("generics"))

    if kind == "struct":
        return f"struct {name}{params}{where_clause}"
    if kind == "enum":
        return f"enum {name}{params}{where_clause}"
    if kind == "union":
        return f"union {name}{params}{where_clause}"
    if kind == "trait":
        prefix = "unsafe " if inner.get("is_unsafe") else ""
        bounds = " + ".join(render_bound(b, 1) for b in inner.get("bounds") or [])
        supertraits = f": {bounds}" if bounds else ""
        return f"{prefix}trait {name}{params}{supertraits}{where_clause}"
    if kind == "trait_alias":
        bounds = " + ".join(render_bound(b, 1) for b in inner.get("params") or [])
        return f"trait {name}{params} = {bounds}{where_clause}"
    if kind == "type_alias":
        return f"type {name}{params} = {render_type(inner.get('type'), 1)}{where_clause}"
    if kind == "constant":
        constant = inner.get("const") or {}
        return f"const {name}: {render_type(inner.get('type'), 1)} = {constant.get('expr', '_')}"
    if kind == "static":
        mutable = "mut " if inner.get("is_mutable") else ""
        return f"static {mutable}{name}: {render_type(inner.get('type'), 1)}"
    if kind == "macro":
        return f"macro_rules! {name}"
    if kind == "proc_macro":
        return f"macro {name}"
    if kind == "module":
        return f"mod {name}"
    if kind == "primitive":
        return f"primitive {name}"
    return name

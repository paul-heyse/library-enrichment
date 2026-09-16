"""Restructure the model into lookup catalogs.

A catalog answers a question directly rather than by navigation: what can I register and with
what options, what was removed, where do I plug in, which transport, which auth provider, what
must I catch.

Every catalog is derived from the model. None parses upstream prose, because prose about a
library that moved this fast is a lead, not a source.

One exception is declared rather than hidden. `registration.md` carries hand-seeded rows about
what the decorators *do at runtime*, because no static signature states it correctly -- and it
is guarded by an assertion in `verify.py` rather than left to rot.
"""

from __future__ import annotations

import ast
import re
from pathlib import Path

import model

# Seeds are access paths, resolved through the alias table. An unresolved seed fails the build:
# a catalog that silently drops its subject asserts by omission that a capability is gone.
EXTENSION_POINTS = (
    ("fastmcp.server.providers.Provider", "Where components come from"),
    ("fastmcp.server.transforms.Transform", "Rewriting the component catalog, observably"),
    ("fastmcp.server.middleware.Middleware", "Intercepting requests"),
    ("fastmcp.server.auth.AuthProvider", "Authenticating callers"),
    ("fastmcp.client.transports.ClientTransport", "How a client reaches a server"),
)

REMOVED_KWARGS_SOURCE = "fastmcp.server.server._REMOVED_KWARGS"
DICT_LITERAL = re.compile(r"=\s*(\{.*\})\s*$", re.DOTALL)


class CatalogError(RuntimeError):
    """A catalog could not be derived honestly."""


def _write(content: Path, name: str, lines: list[str]) -> None:
    content.joinpath("catalogs", name).write_text("\n".join(lines) + "\n")


def resolve(built: model.Model, spelling: str) -> model.Item:
    """An access path to the item it names, or a hard failure."""
    if spelling in built.items:
        return built.items[spelling]
    current = spelling
    for _ in range(model.MAX_ALIAS_HOPS):
        if current in built.items:
            return built.items[current]
        if current not in built.access:
            break
        current = built.access[current]
    raise CatalogError(
        f"catalog seed {spelling!r} resolves to nothing. Either it moved upstream or the "
        "alias table lost it; do not ship a catalog that silently omits its subject."
    )


def write_all(built: model.Model, manifest: dict, content: Path) -> dict[str, int]:
    content.joinpath("catalogs").mkdir(parents=True, exist_ok=True)
    return {
        "removed_api": write_removed_api(built, content),
        "registration": write_registration(built, content),
        "extension_points": write_extension_points(built, content),
        "transports": write_transports(built, content),
        "auth_providers": write_auth_providers(built, content),
        "middleware": write_middleware(built, content),
        "exceptions": write_exceptions(built, content),
        "context": write_context(built, content),
        "protocol_eras": write_protocol_eras(built, content),
        "distributions": write_distribution_map(built, manifest, content),
    }


# --------------------------------------------------------------------------- removed API


def write_removed_api(built: model.Model, content: Path) -> int:
    """Constructor keywords that now raise, straight from the guard upstream ships.

    This is the highest-value page here. Every entry is source that still looks right and fails
    only when the server is constructed, and upstream wrote the replacement text for us.
    """
    item = built.items.get(REMOVED_KWARGS_SOURCE)
    if item is None:
        raise CatalogError(f"{REMOVED_KWARGS_SOURCE} is absent; the removal guard moved")
    match = DICT_LITERAL.search(item.signature)
    if not match:
        raise CatalogError(f"could not read a dict literal out of {REMOVED_KWARGS_SOURCE}")
    removed = ast.literal_eval(match.group(1))

    lines = [
        "# Removed and relocated API",
        "",
        f"{len(removed)} keyword arguments that `FastMCP()` accepted before 4.0 now raise",
        "`TypeError`. Each row is the replacement upstream names in its own error message.",
        "",
        "These matter more than a renamed import: the call still looks correct, still passes a",
        "type checker in most setups, and fails only when the server is constructed.",
        "",
        "| Removed keyword | What to do instead |",
        "|---|---|",
    ]
    lines += [f"| `{key}` | {value} |" for key, value in sorted(removed.items())]

    gone = [
        (
            "FastMCP.import_server(...)",
            "Use `mount()` or add a provider; composition is now the Provider architecture.",
        ),
        ("FastMCP.as_proxy(...)", "Use `fastmcp.server.create_proxy(target, ...)`."),
    ]
    lines += [
        "",
        "## Methods that no longer exist",
        "",
        "| Removed | What to do instead |",
        "|---|---|",
    ]
    for name, replacement in gone:
        leaf = name.split("(")[0].split(".")[-1]
        # If a same-named item reappears upstream, say so rather than keep asserting it is gone.
        present = any(leaf == item.name for item in built.items.values())
        note = replacement
        if present:
            note += " (NOTE: a same-named item is in the index again -- re-check)"
        lines.append(f"| `{name}` | {note} |")

    lines += [
        "",
        "## The other FastMCP",
        "",
        "`mcp.server.fastmcp` is a tombstone module: importing it raises `ModuleNotFoundError`",
        "with a migration message. The official SDK renamed its own server class to",
        "`mcp.server.mcpserver.MCPServer`. That is a different library from `fastmcp.FastMCP`",
        "with a different feature set -- not an alias, not a fork to fall back on.",
    ]
    _write(content, "removed-api.md", lines)
    return len(removed)


# --------------------------------------------------------------------------- registration


def write_registration(built: model.Model, content: Path) -> int:
    """The decorator surface, and what each decorator actually returns.

    The runtime behaviour here cannot be read off any signature, and the signature that looks
    most authoritative is the one that is wrong -- see the note in the page.
    """
    server = resolve(built, "fastmcp.FastMCP")
    decorators = [
        member
        for member in server.members
        if member["name"] in ("tool", "resource", "prompt", "completion", "custom_route")
    ]
    lines = [
        "# Registration surface",
        "",
        "How components get into a server, and what each decorator leaves behind.",
        "",
        "## What the decorators return",
        "",
        "`@mcp.tool` returns **the function unchanged**, stamped with a `__fastmcp__` marker.",
        "It does not return a `FunctionTool`. The `FunctionTool` is constructed later, inside",
        "`add_tool`. This matters because the implementation signature says otherwise:",
        "",
        "```",
        "implementation:  -> Callable[[AnyFunction], FunctionTool] | FunctionTool | partial[...]",
        "both overloads:  -> F        (the decorated function, unchanged)",
        "runtime:         <class 'function'>, with __fastmcp__ set",
        "```",
        "",
        "Three sources, three answers. The overloads and the runtime agree; the implementation",
        "annotation is the odd one out, and it is the one a naive index would publish.",
        "",
        "Consequence for your code: the decorated name stays callable as an ordinary function,",
        "so unit-testing it directly works, and type checkers see the original signature.",
        "",
        "## Decorators on `FastMCP`",
        "",
        "| Decorator | Signature |",
        "|---|---|",
    ]
    for member in sorted(decorators, key=lambda m: m["name"]):
        lines.append(f"| `@mcp.{member['name']}` | `{member['signature']}` |")

    lines += [
        "",
        "Each has an imperative twin (`add_tool`, `add_resource`, `add_template`, `add_prompt`,",
        "`add_middleware`, `add_provider`, `add_transform`, `add_extension`) for registering",
        "something you did not write with a decorator.",
        "",
        "## One URI rule worth knowing",
        "",
        '`@mcp.resource("res://x")` and `@mcp.resource("res://{id}")` produce **different**',
        "runtime objects -- a resource and a resource template -- discriminated by whether the",
        "URI carries a parameter. Nothing in the signature says so.",
        "",
        "## Full keyword detail",
        "",
        "Read `content/api/fastmcp.server.server.md` for every keyword, type and default, and",
        "`content/index/overloads.tsv` for the overload set of any dispatching callable.",
    ]
    _write(content, "registration.md", lines)
    return len(decorators)


# --------------------------------------------------------------------------- extension points


def write_extension_points(built: model.Model, content: Path) -> int:
    lines = [
        "# Extension points",
        "",
        "The base classes you subclass, with every in-index descendant. Counts are the",
        "**transitive** closure, which is the only way `FastMCP` shows up as a `Provider` --",
        "it reaches one through `AggregateProvider`, and that is the central fact of the 4.0",
        "composition model.",
        "",
        "| Base | Role | Descendants | Import as |",
        "|---|---|---:|---|",
    ]
    total = 0
    for spelling, role in EXTENSION_POINTS:
        item = resolve(built, spelling)
        children = built.descendants.get(item.path, [])
        total += 1
        lines.append(f"| `{item.name}` | {role} | {len(children)} | `{item.preferred}` |")

    lines += [
        "",
        "Three of these are distinct axes that are easy to conflate:",
        "",
        "- **Provider** decides *where components come from*. `mount()` and `create_proxy()`",
        "  are both thin wrappers over it.",
        "- **Transform** rewrites the component catalog and is observable by the system.",
        "- **Middleware** intercepts requests in flight.",
        "",
        "Per-base descendant lists are in `content/index/descendants.tsv`; what you must",
        "override is in `content/index/overrides.tsv`.",
        "",
    ]
    for spelling, _ in EXTENSION_POINTS:
        item = resolve(built, spelling)
        children = [c for c, _ in built.descendants.get(item.path, [])]
        lines += [f"## {item.name}", "", f"`{item.preferred}` -- {len(children)} descendants", ""]
        nameable = [c for c in children if built.items[c].nameable]
        for child in sorted(nameable)[:30]:
            lines.append(f"- `{built.items[child].preferred}`")
        hidden = len(children) - len(nameable)
        if hidden:
            lines.append(f"- _plus {hidden} defined behind private modules_")
        lines.append("")
    _write(content, "extension-points.md", lines)
    return total


# --------------------------------------------------------------------------- transports


def write_transports(built: model.Model, content: Path) -> int:
    base = resolve(built, "fastmcp.client.transports.ClientTransport")
    children = [c for c, _ in built.descendants.get(base.path, [])]
    lines = [
        "# Transports",
        "",
        "## Server side",
        "",
        "`mcp.run(transport=...)` accepts `stdio`, `http`, `sse` and `streamable-http`.",
        "Host, port and path are **not** constructor arguments -- pass them to",
        "`run_http_async()` / `http_app()`, or set `FASTMCP_*` environment variables.",
        "See `catalogs/removed-api.md`.",
        "",
        "| Method | Purpose |",
        "|---|---|",
        "| `run(...)` | synchronous wrapper |",
        "| `run_async(...)` | the async entry point |",
        "| `run_stdio_async(...)` | stdio explicitly |",
        "| `run_http_async(...)` | HTTP, where host/port/path/security options live |",
        "| `http_app(...)` | a Starlette app to mount in your own ASGI stack |",
        "",
        "## Client side",
        "",
        f"{len(children)} transport classes descend from `{base.preferred}`. `Client(...)`",
        "infers one from what you pass -- a URL, a path, a dict, or a server instance.",
        "",
        "| Transport | Import as |",
        "|---|---|",
    ]
    for child in sorted(children):
        item = built.items[child]
        if item.nameable:
            lines.append(f"| `{item.name}` | `{item.preferred}` |")
    _write(content, "transports.md", lines)
    return len(children)


# --------------------------------------------------------------------------- auth


def write_auth_providers(built: model.Model, content: Path) -> int:
    base = resolve(built, "fastmcp.server.auth.AuthProvider")
    children = [c for c, _ in built.descendants.get(base.path, [])]
    lines = [
        "# Authentication providers",
        "",
        f"{len(children)} classes descend from `{base.preferred}`. Most wrap one identity",
        "service; the rest are the generic verifiers and the proxy layers they build on.",
        "",
        "| Provider | Import as | Summary |",
        "|---|---|---|",
    ]
    rows = 0
    for child in sorted(children):
        item = built.items[child]
        if not item.nameable:
            continue
        rows += 1
        lines.append(f"| `{item.name}` | `{item.preferred}` | {item.summary[:90]} |")
    lines += [
        "",
        "Client-side authentication is separate: `BearerAuth`, `OAuth`,",
        "`ClientCredentialsOAuthProvider` and `PrivateKeyJWTOAuthProvider` live under",
        "`fastmcp.client.auth`.",
    ]
    _write(content, "auth-providers.md", lines)
    return rows


# --------------------------------------------------------------------------- middleware


def write_middleware(built: model.Model, content: Path) -> int:
    base = resolve(built, "fastmcp.server.middleware.Middleware")
    hooks = [
        member["name"]
        for member in base.members
        if member["name"].startswith("on_") and not member.get("inherited")
    ]
    children = [c for c, _ in built.descendants.get(base.path, [])]
    lines = [
        "# Middleware",
        "",
        f"`{base.preferred}` declares {len(hooks)} hooks. Override the ones you need; the rest",
        "pass through.",
        "",
        "| Hook |",
        "|---|",
    ]
    lines += [f"| `{hook}` |" for hook in sorted(hooks)]
    lines += [
        "",
        f"## Built-in middleware ({len(children)} descendants)",
        "",
        "| Class | Import as |",
        "|---|---|",
    ]
    for child in sorted(children):
        item = built.items[child]
        if item.nameable:
            lines.append(f"| `{item.name}` | `{item.preferred}` |")
    lines += [
        "",
        "Every hook is `async`. Writing one as `def` produces a coroutine where a value was",
        "expected, and the failure surfaces far from the definition.",
    ]
    _write(content, "middleware.md", lines)
    return len(hooks)


# --------------------------------------------------------------------------- exceptions


def write_exceptions(built: model.Model, content: Path) -> int:
    base = resolve(built, "fastmcp.exceptions.FastMCPError")
    children = [c for c, _ in built.descendants.get(base.path, [])]
    lines = [
        "# Exceptions",
        "",
        f"What a caller branches on. `{base.preferred}` has {len(children)} descendants.",
        "",
        "| Exception | Import as | Summary |",
        "|---|---|---|",
    ]
    for child in sorted(children):
        item = built.items[child]
        lines.append(f"| `{item.name}` | `{item.preferred}` | {item.summary[:80]} |")
    conditional = [row for row in built.conditional if row["name"] == "MCPError"]
    if conditional:
        lines += [
            "",
            "## One conditional definition",
            "",
            "`fastmcp.exceptions.MCPError` is bound by a `try`/`except ImportError` pair: the",
            "preferred branch imports it from `mcp`, and the fallback defines a local stand-in.",
            "This index was built with `mcp` installed, so the row you see is the real one --",
            "but in an install without it, the same name is a different class. See",
            "`content/index/conditional.tsv`.",
        ]
    _write(content, "exceptions.md", lines)
    return len(children)


# --------------------------------------------------------------------------- context


def write_context(built: model.Model, content: Path) -> int:
    item = resolve(built, "fastmcp.Context")
    members = [
        member
        for member in item.members
        if not member["name"].startswith("_")
        and member["name"] not in model.NOISE_MEMBERS
        and member["name"] not in model.PYDANTIC_MEMBERS
    ]
    asynchronous = sum(1 for member in members if "async" in member.get("labels", []))
    lines = [
        "# The Context object",
        "",
        f"What a tool body can do. `{item.preferred}` exposes {len(members)} members, of which",
        f"{asynchronous} are `async`.",
        "",
        "Obtain one by annotating a parameter `ctx: Context` on any tool, resource or prompt,",
        "or by calling `fastmcp.server.dependencies.get_context()`.",
        "",
        "| Member | Kind | Signature |",
        "|---|---|---|",
    ]
    for member in sorted(members, key=lambda m: m["name"]):
        labels = ",".join(member.get("labels", [])) or member["kind"]
        lines.append(f"| `{member['name']}` | {labels} | `{member['signature'][:96]}` |")
    _write(content, "context.md", lines)
    return len(members)


# --------------------------------------------------------------------------- protocol eras


def write_protocol_eras(built: model.Model, content: Path) -> int:
    eras: dict[str, int] = {}
    for item in built.items.values():
        if item.protocol_era:
            eras[item.protocol_era] = eras.get(item.protocol_era, 0) + 1
    lines = [
        "# Protocol eras",
        "",
        "`mcp_types` carries more than one dated copy of the wire model, because a FastMCP",
        "connection negotiates a protocol era and a proxy can mirror the era of whatever is",
        "in front of it. Same class names, different modules, different shapes.",
        "",
        "| Era | Types |",
        "|---|---:|",
    ]
    lines += [f"| `{era}` | {count} |" for era, count in sorted(eras.items())]
    lines += [
        "",
        "Never match a wire type by leaf name alone: `CallToolResult` exists in every era and",
        "in the era-neutral module. `content/index/symbols.tsv` carries the era in its own",
        "column; `-` means era-neutral.",
    ]
    _write(content, "protocol-eras.md", lines)
    return len(eras)


# --------------------------------------------------------------------------- distributions


def write_distribution_map(built: model.Model, manifest: dict, content: Path) -> int:
    per_dist: dict[str, dict[str, int]] = {}
    for item in built.items.values():
        bucket = per_dist.setdefault(item.dist, {"items": 0, "classes": 0, "nameable": 0})
        bucket["items"] += 1
        bucket["classes"] += item.kind == "class"
        bucket["nameable"] += item.nameable
    notes = {entry["dist"]: entry.get("note", "") for entry in manifest["distributions"]}
    lines = [
        "# Distribution map",
        "",
        "| Distribution | Items | Classes | Nameable | Role |",
        "|---|---:|---:|---:|---|",
    ]
    for dist, bucket in sorted(per_dist.items()):
        role = notes.get(dist, "")
        lines.append(
            f"| `{dist}` | {bucket['items']} | {bucket['classes']} | "
            f"{bucket['nameable']} | {role[:80]} |"
        )
    lines += [
        "",
        "`fastmcp` on PyPI is a metapackage that ships no code; every module lives in",
        "`fastmcp-slim`, which is what its extras resolve through. Ask metadata questions of",
        "`fastmcp-slim`.",
    ]
    _write(content, "distribution-map.md", lines)
    return len(per_dist)

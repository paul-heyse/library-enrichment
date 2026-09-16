"""One page per extension point: what you must write, what is already written, who did it.

The module pages answer "what is in this module". The catalog answers "what are the extension
points". Neither answers the question an agent actually arrives with: *can I plug into this, how
much work is it, and who already did it*.

Required and provided are separated deliberately, and for Python the split is not the same fact
it is in Rust. There is no `trait` keyword to read it off. Three things make a member required:

  * it carries `@abstractmethod`, which `extract.py` now records per member;
  * the class is a `Protocol`, where every declared member is an obligation;
  * nothing else. A concrete method on a concrete base is *provided*, and the default is almost
    always the conservative answer rather than the good one -- a `Provider` that overrides none
    of its hooks is a correct Provider that supplies nothing.

The trailing "demonstrated by" section comes from `link.py`, which matches `class X(Base)` as
syntax rather than searching for the base's name in text. A page that silently reported "no
example demonstrates this" would be indistinguishable from one whose search was broken, so the
build says out loud when the edge count collapses.

The list below was seeded from `descendants.tsv` ranked by implementor count, not from what the
author remembered, then narrowed to the points a *caller* implements. `WireModel` (301
descendants) and `MCPModel` (155) are excluded on purpose: they are the wire model's own base
classes, and nobody subclasses them to extend FastMCP.
"""

from __future__ import annotations

from pathlib import Path

from model import Item, Model

# (canonical path, what implementing it gets you)
EXTENSION_POINTS: list[tuple[str, str]] = [
    (
        "fastmcp.server.providers.base.Provider",
        "Where components come from. A server is itself a Provider, and `mount()` and "
        "`create_proxy()` are thin wrappers over one.",
    ),
    (
        "fastmcp.server.transforms.Transform",
        "Rewriting the component catalog, observably. The system can see what you changed, "
        "which is what separates this from a Provider that lies.",
    ),
    (
        "fastmcp.server.middleware.middleware.Middleware",
        "Intercepting requests in flight. Twelve hooks, all async, all pass-through unless "
        "you override them.",
    ),
    (
        "fastmcp.server.auth.auth.AuthProvider",
        "Authenticating callers. Upstream never subclasses this directly -- read "
        "`TokenVerifier` or `OAuthProxy` first.",
    ),
    (
        "fastmcp.server.auth.auth.TokenVerifier",
        "Verifying a bearer token. The smallest useful auth surface, and the one the corpus "
        "actually extends.",
    ),
    (
        "fastmcp.server.auth.auth.OAuthProvider",
        "A full OAuth authorization server, rather than verification alone.",
    ),
    (
        "fastmcp.client.transports.base.ClientTransport",
        "How a client reaches a server. Eleven ship; write one for a transport that does not.",
    ),
    (
        "fastmcp.utilities.components.FastMCPComponent",
        "The shared base of tools, resources and prompts -- what every registered thing is.",
    ),
    (
        "fastmcp.server.providers.aggregate.AggregateProvider",
        "Composing several Providers into one. `FastMCP` reaches `Provider` through this.",
    ),
]


def _required_and_provided(item: Item) -> tuple[list[dict], list[dict]]:
    """Split declared members into obligations and defaults.

    A Protocol's members are all obligations whether or not they carry a decorator: the class
    exists to state a shape. Elsewhere `@abstractmethod` is the only reliable marker, because
    Python has no syntax that distinguishes "you should override this" from "you may".
    """
    protocol = any("Protocol" in base for base in item.bases)
    required: list[dict] = []
    provided: list[dict] = []
    for member in item.members:
        if member.get("inherited") or member["name"].startswith("_"):
            continue
        decorators = member.get("decorators") or []
        if protocol or any("abstractmethod" in text for text in decorators):
            required.append(member)
        else:
            provided.append(member)
    return required, provided


def _signature_block(members: list[dict]) -> list[str]:
    return ["```python", *[member["signature"] for member in members], "```", ""]


def write_all(
    built: Model,
    content: Path,
    demonstrated: dict[str, list[str]],
) -> int:
    """Write one page per extension point. Returns the number written."""
    out = content / "extension-points"
    out.mkdir(parents=True, exist_ok=True)
    written = 0
    index: list[tuple[str, str, int, int]] = []

    for canonical, role in EXTENSION_POINTS:
        item = built.items.get(canonical)
        if item is None:
            raise SystemExit(
                f"extension point {canonical!r} is not in the model; "
                f"it was renamed or removed upstream, which is a re-pin decision, not a "
                f"page to drop silently"
            )
        required, provided = _required_and_provided(item)
        # `descendants` holds (path, depth) pairs -- the closure records how far each
        # subtype sits from the base, which the implementor list does not need.
        subtypes = sorted(path for path, _ in built.descendants.get(canonical, ()))

        # Join the syntactic edges over the transitive closure: an example that subclasses
        # `FastMCPApp` is demonstrating `Provider` too, and only `descendants.tsv` knows that.
        leaves = {canonical.rsplit(".", 1)[-1]}
        leaves.update(path.rsplit(".", 1)[-1] for path in subtypes)
        examples = sorted({file for leaf in leaves for file in demonstrated.get(leaf, ())})

        lines = [
            f"# {item.name}",
            "",
            role,
            "",
            f"Import as `{item.preferred}`" if item.nameable else "**Not importable.**",
            f"Defined at `{canonical}`.",
            "",
            "```python",
            item.signature,
            "```",
            "",
        ]
        if required:
            lines += [
                "## Required",
                "",
                "You must write these. Nothing works until you do.",
                "",
                *_signature_block(required),
            ]
        else:
            lines += [
                "## Required",
                "",
                "Nothing. Every member has a default, so a subclass that overrides none of them "
                "is valid -- and does nothing useful.",
                "",
            ]
        if provided:
            lines += [
                "## Provided",
                "",
                "Defaulted, and this is where the capability hides. The default is almost always "
                "the conservative answer, so an implementation that overrides none of these "
                "works correctly and supplies nothing.",
                "",
                *_signature_block(provided),
            ]
        lines += [
            f"## Implementors ({len(subtypes)})",
            "",
            "Transitive. Read one before writing your own.",
            "",
        ]
        lines += [f"- `{path}`" for path in subtypes[:40]]
        if len(subtypes) > 40:
            lines.append(f"- … and {len(subtypes) - 40} more, see `index/descendants.tsv`")
        lines.append("")

        if examples:
            lines += [
                f"## Demonstrated by {len(examples)} upstream file(s)",
                "",
                "Matched as syntax -- `class X(Base)` -- not by searching for the name.",
                "",
                *[f"- [`{path}`](../{path})" for path in examples[:12]],
                "",
            ]
        lines += [
            "## Documentation",
            "",
            f"Prose: [`api/{item.module}.md`](../api/{item.module}.md) · "
            f"records: [`model/{item.module}.json`](../model/{item.module}.json)",
            "",
        ]
        (out / f"{item.name}.md").write_text("\n".join(lines))
        index.append((item.name, role, len(required), len(subtypes)))
        written += 1

    header = [
        "# Extension points",
        "",
        "What you subclass, how much you must write, and how many already exist.",
        "`Required` counts obligations: `@abstractmethod` members, or every member when the "
        "base is a Protocol.",
        "",
        "| Extension point | Required | Implementors | Role |",
        "|---|---:|---:|---|",
    ]
    header += [
        f"| [`{name}`]({name}.md) | {required} | {subtypes} | {role} |"
        for name, role, required, subtypes in index
    ]
    (out / "00-map.md").write_text("\n".join(header) + "\n")
    return written

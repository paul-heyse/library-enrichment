"""Turn the twelve `macro_rules!` macros into a table of accepted invocation forms.

Twelve of the fifteen items this crate publishes are macros, so the API a caller writes is
invocation grammar, not signatures. rustdoc elides a macro's body -- every arm reads
`=> { ... }` -- but it keeps the arms themselves, and the arms ARE the grammar:

    (target: $target:expr, options: $options:expr, $($field:tt)*) => { ... };
    (options: $options:expr, $($field:tt)*) => { ... };
    (target: $target:expr, options: $options:expr) => { ... };
    (options: $options:expr) => { ... };

Nothing else in the repository carries this. `options:` is a mandatory keyword, not a positional
argument; `target:` is optional and changes what a subscriber filter must match; and the
trailing `$($field:tt)*` is where the `field = field::Empty` declarations go, without which
`add_custom_field` sets a value that reaches no span. Getting any of those wrong is a macro
expansion error whose message names none of them.

What this module cannot tell you is what an arm expands to. See `reference.md` limit 5: rustdoc
emits no macro bodies at any format version, so the expansion is established by the source
corpus and by probes, never by the index.
"""

from __future__ import annotations

import re
from pathlib import Path

ARM = re.compile(r"^\s*\((?P<arm>.*?)\)\s*=>", re.MULTILINE)
LEVEL = re.compile(r"_(trace|debug|info|warn|error)_spans$")


def _family(name: str) -> str:
    return "rules" if name.startswith("instrument_rules_") else "exec"


def _level(name: str) -> str:
    """The level an arm emits at, which is baked into the macro's name.

    `instrument_with_spans!` takes the level as a positional argument; the other five bake it
    in. That is the one difference between them, and it is invisible in a signature because
    there is no signature.
    """
    match = LEVEL.search(name)
    if match:
        return match.group(1).upper()
    return "caller-supplied"


def arms_of(source: str) -> list[str]:
    """Every accepted invocation form of one macro, in declaration order."""
    return [match.group("arm").strip() for match in ARM.finditer(source)]


def sources_from(document: dict) -> dict[str, str]:
    """{macro name: its `macro_rules!` text} for one crate's rustdoc document.

    Read from `inner.macro` rather than from the model, because `render.py` renders a macro as
    its header alone -- `macro_rules! instrument_with_info_spans` -- which is true and carries
    none of the information a caller needs. The arms are only in the raw document.
    """
    found: dict[str, str] = {}
    for entry in document["index"].values():
        body = (entry.get("inner") or {}).get("macro")
        if body and entry.get("name"):
            found[entry["name"]] = body
    return found


def write_all(content: Path, items: dict, sources: dict[str, str]) -> int:
    """Write `index/macros.tsv` and `catalogs/macros.md`. Returns the arm count."""
    index = content / "index"
    catalogs = content / "catalogs"
    index.mkdir(parents=True, exist_ok=True)
    catalogs.mkdir(parents=True, exist_ok=True)

    macros = sorted(
        (item for item in items.values() if item.kind == "macro" and "tracing" in item.crate),
        key=lambda item: item.name,
    )
    rows: list[str] = []
    per_macro: dict[str, list[str]] = {}
    if not sources:
        raise RuntimeError(
            "no macro sources were supplied. The arms are the only description of this API "
            "that exists, so an empty table is a stop rather than a quiet zero."
        )
    for item in macros:
        arms = arms_of(sources.get(item.name, ""))
        per_macro[item.name] = arms
        for ordinal, arm in enumerate(arms):
            rows.append("\t".join((
                item.name,
                str(ordinal),
                arm or "(no arguments)",
                "yes" if "options:" in arm else "no",
                "yes" if "target:" in arm else "no",
                "yes" if "$field" in arm or "$fields" in arm else "no",
                "yes" if "state:" in arm else "no",
                _level(item.name),
                _family(item.name),
            )))
    (index / "macros.tsv").write_text("".join(f"{row}\n" for row in sorted(set(rows))))
    _write_catalog(catalogs, macros, per_macro)
    return len(rows)


def _write_catalog(catalogs: Path, macros: list, per_macro: dict[str, list[str]]) -> None:
    exec_macros = [m for m in macros if _family(m.name) == "exec"]
    rule_macros = [m for m in macros if _family(m.name) == "rules"]

    lines = [
        "# Macro invocation grammar",
        "",
        f"{len(macros)} macros, {sum(len(a) for a in per_macro.values())} accepted invocation "
        "forms. Read an arm as the literal shape you type: `options:` is a **keyword**, not a "
        "positional argument, and omitting it is a macro expansion error that names nothing "
        "useful.",
        "",
        "## Two families",
        "",
        "| Family | Macros | Wraps | Returns |",
        "|---|---:|---|---|",
        f"| `instrument_with_*_spans!` | {len(exec_macros)} | every `ExecutionPlan` node, at "
        "run time | a `PhysicalOptimizerRule` you register |",
        f"| `instrument_rules_with_*_spans!` | {len(rule_macros)} | the analyzer, optimizer and "
        "physical-optimizer passes, at plan time | a `SessionState` |",
        "",
        "The second takes `state:` as well as `options:`, which is the shape difference: one "
        "produces a rule you add to a builder, the other consumes and returns the state.",
        "",
        "## Arms",
        "",
    ]
    for item in macros:
        lines += [f"### `{item.name}!`", ""]
        if item.summary:
            lines += [item.summary, ""]
        lines.append(f"Level: **{_level(item.name)}**")
        lines.append("")
        for arm in per_macro[item.name]:
            lines.append(f"- `{item.name}!({arm})`")
        lines.append("")

    lines += [
        "## The custom-field coupling",
        "",
        "`InstrumentationOptions::builder().add_custom_field(\"env\", \"production\")` sets a "
        "value. It does **not** create a field. A `tracing` span's field set is fixed when the "
        "span is created, so the macro must declare the key too:",
        "",
        "```rust",
        "let rule = instrument_with_info_spans!(",
        "    options: options,",
        "    env = field::Empty,      // declared here",
        "    region = field::Empty,",
        ");",
        "```",
        "",
        "Upstream says the same thing in `corpus/examples/integration-utils/src/lib.rs`, in a "
        "comment beside those two lines: *custom fields keys must be defined at compile time*. "
        "Declare without setting and the field is absent from the span; set without declaring "
        "and the value goes nowhere. Neither case is an error at compile time or at run time.",
        "",
        "## What an arm expands to is not in this index",
        "",
        "rustdoc emits no macro bodies, at any format version -- every arm above ends `=> "
        "{ ... }` in the source document too. The expansion is in "
        "`corpus/source/exec_instrument_macros.rs` and "
        "`corpus/source/rule_instrumentation_macros.rs`, and what it does at run time is in "
        "`index/behaviors.tsv`. Do not infer it from the arm.",
    ]
    (catalogs / "macros.md").write_text("\n".join(lines) + "\n")

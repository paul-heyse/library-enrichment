"""Direct-lookup catalogs: the options surface, the crate map, and the negative space.

`options.md` is the one a reader reaches for most, and it is the reason this repository exists.
Both option builders are public types in private modules, so no published artifact documents
them: docs.rs answers 404, they are absent from `all.html`, and the hosted rustdoc JSON carries
the structs with no impl block at all. Sixteen methods a caller must use live only in the
`--document-private-items` capture. This catalog is where they are written down.

`not-reachable.md` is the negative space: what exists and cannot be used, and what to reach for
instead. An invisible limitation is indistinguishable from an absent capability, which is why it
is a page rather than an omission.
"""

from __future__ import annotations

from collections import defaultdict
from pathlib import Path

import visibility

SUBJECT_CRATES = ("datafusion-tracing", "instrumented-object-store")


def write_all(content: Path, items: dict, classifications: dict, acquisition: dict) -> dict:
    catalogs = content / "catalogs"
    catalogs.mkdir(parents=True, exist_ok=True)
    counts = {
        "catalog_options": _options(catalogs, items),
        "catalog_crates": _crate_map(catalogs, items, acquisition),
        "catalog_not_reachable": _not_reachable(catalogs, items, classifications),
    }
    return counts


# --------------------------------------------------------------------------- options

def _options(catalogs: Path, items: dict) -> int:
    owners = [
        item for item in items.values()
        if item.crate == "datafusion-tracing" and item.kind == "struct"
        and item.name.endswith(("Options", "OptionsBuilder"))
    ]
    owners.sort(key=lambda item: (not item.name.endswith("Builder"), item.name))

    lines = [
        "# The options surface",
        "",
        "Two option types, each with a builder. **The builders are documented nowhere else.** "
        "`mod options;` and `mod rule_options;` are private in `lib.rs`, which selectively "
        "re-exports only the two `*Options` types; the builders are `pub` inside those private "
        "modules, so:",
        "",
        "- `docs.rs/.../struct.InstrumentationOptionsBuilder.html` answers **404**",
        "- neither builder appears in `all.html`",
        "- the hosted rustdoc JSON carries both structs with **zero impl blocks**",
        "",
        "Every row below marked `reachable-undocumented` came from a local "
        "`--document-private-items` capture and appears in no published artifact. Three of the "
        "names — `record_metrics`, `preview_limit`, `preview_fn` — also exist as public *fields* "
        "on `InstrumentationOptions`, so searching for the name finds something and lands you on "
        "struct-literal construction, which silently has no `add_custom_field`.",
        "",
    ]
    written = 0
    for item in owners:
        lines += [
            f"## `{item.name}`",
            "",
            f"`{item.path}` — reached via "
            f"`{item.reached_via}`" if item.reached_via else f"`{item.path}`",
            "",
        ]
        if item.summary:
            lines += [item.summary, ""]
        # Only for an item the hosted document carries. A builder's fields come from the
        # private capture, where rustdoc lists private fields too -- calling those "public"
        # would be the same class of error this catalog exists to correct, pointing the other
        # way. `InstrumentationOptions` genuinely has four public fields, and they are the
        # struct-literal trap; the builder's are nobody's business.
        if item.fields and item.visibility == visibility.SUPPORTED:
            lines += [
                "Public fields: " + ", ".join(f"`{f}`" for f in item.fields)
                + " — settable directly, which is the trap: a struct literal compiles and has "
                  "no `add_custom_field`.",
                "",
            ]
        methods = [
            m for m in sorted(item.methods, key=lambda m: m.name)
            if not (m.via_trait and m.via_trait.startswith("core::"))
        ]
        if methods:
            lines += ["| Method | Visibility | Signature |", "|---|---|---|"]
            for method in methods:
                flag = "**undocumented**" if method.visibility == visibility.REACHABLE \
                    else method.visibility
                signature = method.signature.replace("|", "\\|")
                lines.append(f"| `{method.name}` | {flag} | `{signature}` |")
                written += 1
            lines.append("")

    lines += [
        "## Which rule-instrumentation constructor",
        "",
        "`RuleInstrumentationOptions` offers three documented entry points and a builder with "
        "ten undocumented ones. The documented three are shorthands:",
        "",
        "| You want | Write |",
        "|---|---|",
        "| everything | `RuleInstrumentationOptions::full()` |",
        "| phase spans only, no per-rule spans | `RuleInstrumentationOptions::phase_only()` |",
        "| everything plus plan diffs | `RuleInstrumentationOptions::full().with_plan_diff()` |",
        "| one phase only | `RuleInstrumentationOptions::builder().optimizer().build()` |",
        "| two phases, one of them coarse | "
        "`…builder().analyzer().optimizer_phase_only().build()` |",
        "",
        "The last two rows are the reason the builder matters: there is no documented way to "
        "instrument the physical optimizer and nothing else, and per-rule spans on a large plan "
        "are the bulk of the trace volume.",
        "",
        "## Reading this back",
        "",
        "```bash",
        "rg -P '\\treachable-undocumented\\t' content/index/methods.tsv | cut -f1,2,5",
        "```",
    ]
    (catalogs / "options.md").write_text("\n".join(lines) + "\n")
    return written


# --------------------------------------------------------------------------- crate map

def _crate_map(catalogs: Path, items: dict, acquisition: dict) -> int:
    per_crate: dict[str, list] = defaultdict(list)
    for item in items.values():
        per_crate[item.crate].append(item)

    by_subject: dict[str, list[str]] = defaultdict(list)
    for package, record in acquisition["crates"].items():
        by_subject[record["subject"]].append(package)

    lines = [
        "# Crate map",
        "",
        "Three groups. Only the first is this repository's subject; the other two are indexed "
        "because you cannot use the subject without them, and their versions are the ones "
        "`datafusion-tracing 55.0.0` is known to compose with rather than the newest published.",
        "",
        "| Crate | Group | Version | rustdoc format | Items | Methods |",
        "|---|---|---|---:|---:|---:|",
    ]
    for subject in ("datafusion-tracing", "tracing", "opentelemetry"):
        for package in sorted(by_subject.get(subject, [])):
            record = acquisition["crates"][package]
            members = per_crate.get(package, [])
            group = {
                "datafusion-tracing": "subject",
                "tracing": "wiring",
                "opentelemetry": "otel",
            }[subject]
            private = " + private" if "private" in record else ""
            lines.append(
                f"| `{package}` | {group} | {record['version']} | "
                f"{record['format_version']}{private} | {len(members)} | "
                f"{sum(len(m.methods) for m in members)} |"
            )

    lines += [
        "",
        "docs.rs serves this set at five different rustdoc format versions — it builds on its "
        "own schedule and its fleet is not uniform. All four OpenTelemetry crates come back at "
        "format 56, older than any other repository in this family parses. The build asserts the "
        "field vocabulary it depends on rather than trusting the version number.",
        "",
        "## Not indexed",
        "",
        "| Crate | Why | Use instead |",
        "|---|---|---|",
        "| `datafusion` | the thing being instrumented, not the instrumentation | the DataFusion "
        "reference |",
        "| `arrow`, `parquet` | reached only through DataFusion | the DataFusion reference |",
        "| `object_store` | `instrument_object_store` wraps it; the trait is not ours | the "
        "DataFusion reference |",
        "| `tonic` | `opentelemetry-otlp` re-exports four types from it | tonic's own docs |",
        "",
        "Those access paths land in `index/unresolved.tsv`. That is a boundary, not a gap.",
    ]
    (catalogs / "crate-map.md").write_text("\n".join(lines) + "\n")
    return len(per_crate)


# --------------------------------------------------------------------------- negative space

def _not_reachable(catalogs: Path, items: dict, classifications: dict) -> int:
    internal = [
        item for item in items.values()
        if item.crate in SUBJECT_CRATES and item.visibility == visibility.INTERNAL
        and item.kind in ("struct", "enum", "trait", "type_alias")
    ]
    internal.sort(key=lambda item: item.name)
    absent = [
        (name, package)
        for package, classification in classifications.items()
        for name in classification.absent_from_rustdoc
    ]
    hidden = [
        item for item in items.values() if item.visibility == visibility.DOC_HIDDEN
    ]

    lines = [
        "# What exists and cannot be used",
        "",
        "Three kinds of thing, and they need different answers. An invisible limitation reads "
        "exactly like an absent capability, so each row says what to do instead.",
        "",
        "## Real, public to rustdoc, and unreachable",
        "",
        "`rustdoc` records these as `public` — they are `pub` items inside private modules — but "
        "nothing you can name ever yields one, so there is no spelling that reaches them. "
        "`InstrumentedExec` is the one that matters: upstream's README says it is *intentionally "
        "private so downstream code cannot depend on its internals*, and the supported surface "
        "is the optimizer rule plus the ordinary `ExecutionPlan` trait.",
        "",
        "| Item | Kind | Instead |",
        "|---|---|---|",
    ]
    advice = {
        "InstrumentedExec": "register the rule; introspect through `ExecutionPlan` as usual",
        "PreviewFn": "pass a closure to `preview_fn`; inference supplies the type",
        "PreviewRecorderBuilder": "set `preview_limit` and `preview_fn` on the options builder",
        "InstrumentationLevel": "choose the macro whose name carries the level",
    }
    for item in internal:
        lines.append(
            f"| `{item.name}` | {item.kind} | "
            f"{advice.get(item.name, 'no public route; this is an implementation detail')} |"
        )

    lines += [
        "",
        "Note that `PreviewFn` is not merely unreachable — rustdoc expands the alias away "
        "entirely, so the public signature of `InstrumentationOptions.preview_fn` reads "
        "`Option<Arc<dyn Fn(&RecordBatch) -> Result<String> + Send + Sync>>`. Neither the name "
        "in the source nor the name in the documentation is one you can write.",
        "",
        "## Advertised by `lib.rs`, emitted by neither rustdoc capture",
        "",
        "| Name | Crate | What it is | Instead |",
        "|---|---|---|---|",
    ]
    for name, package in sorted(absent):
        lines.append(
            f"| `{name}` | `{package}` | a `#[doc(hidden)]` `pub use` that appears in neither "
            f"the hosted document nor the `--document-private-items` capture | call the "
            f"`instrument_rules_with_*_spans!` macro, which expands to it |"
        )

    lines += [
        "",
        "This one is worth dwelling on: the crate root re-exports the name, so it is genuinely "
        "public and genuinely callable, and **no rustdoc document in existence mentions it**. "
        "The index would have contradicted the crate's own `lib.rs` by silence. It is recorded "
        "in `index/unreachable.tsv` instead.",
        "",
        "## Public, documented, and not for you",
        "",
        "| Item | Why it is public | Instead |",
        "|---|---|---|",
    ]
    for item in hidden:
        lines.append(
            f"| `{item.name}` | `#[doc(hidden)]`; upstream's comment says *only public because "
            f"they need to be accessed by the macros* | the macro that expands to it |"
        )

    lines += [
        "",
        "`#[doc(hidden)]` is **not recorded in rustdoc JSON** at format 61 — measured: "
        "`new_instrument_rule` comes back with `attrs: []` while its source declaration carries "
        "the attribute. The classification is read from `content/corpus/source/lib.rs` with an "
        "ast-grep rule, because the attribute is a sibling node of the item it decorates and a "
        "line-oriented pattern gets that wrong in both directions.",
    ]
    (catalogs / "not-reachable.md").write_text("\n".join(lines) + "\n")
    return len(internal) + len(absent) + len(hidden)

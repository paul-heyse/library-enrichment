"""Derive the emitted span contract from upstream's pinned trace snapshots.

The question an agent has about this library is almost never "does this function exist". It is
"what will I see in Jaeger" -- span names, levels, targets, field names, nesting. No signature
carries any of that, and the crate's own metrics field names are built as

    format!("datafusion.metrics.{}", metric.value().name())

from whichever DataFusion node happened to run, so the field vocabulary is OPEN. It cannot be
enumerated from the source at all. It can only be observed.

Upstream observes it for us. `tests/snapshots/*_trace.snap` are insta captures of the JSON span
stream under nine scenarios whose option sets are declared in `tests/integration_tests.rs`, and
those nine scenarios differ from each other one option at a time. That makes the corpus a
natural experiment: a field that appears in exactly the scenarios carrying
`with_metrics_collection()` is conditional on metrics, and the table can say so rather than
listing every field as if it were unconditional.

These rows are `recorded`, never `confirmed`. They are upstream's observations under upstream's
harness; only a probe with a control that comes out the other way isolates a cause, and
`probes.py` promotes the rows it reproduces.

One snapshot is not valid JSON. See `REPAIR` below -- the cause is exact, upstream's own, and
quotable, so the repair is a named substitution rather than a heuristic, and every row derived
from that file is flagged.
"""

from __future__ import annotations

import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path

RECORDED = "recorded"
CONFIRMED = "confirmed"

# `tests/test_utils/insta_settings.rs` registers this filter:
#
#     settings.add_filter(
#         r#"e_tag: Some\(\\"([0-9a-fA-F-]+)\\"\)"#,
#         r#"e_tag: Some\("ffffffff-fffffffffffff-fff"\)"#,
#     );
#
# An insta replacement is literal text, so the `\(` and `\)` the author carried over from the
# pattern are emitted verbatim -- and the `\\"` the pattern matched is replaced by a bare `"`.
# The result is a JSON string containing `Some\("…"\)`, which is two invalid escapes and an
# unescaped quote. The replacement should have read `e_tag: Some(\\"ffffffff-…-fff\\")`.
#
# So the repair is not a guess about malformed input: it is substituting the exact literal
# upstream emits for the one it would have emitted had the replacement been written correctly.
# Anything else in the corpus that fails to parse is a stop, not a second repair.
REPAIR = (
    r'Some\("ffffffff-fffffffffffff-fff"\)',
    r'Some(\"ffffffff-fffffffffffff-fff\")',
)

SCENARIO_RULE = """
id: source-test-scenario
language: rust
severity: hint
message: an integration scenario and the options it sets
rule:
  kind: call_expression
  has:
    field: function
    regex: '^execute_test_case$'
"""

OPTION_CALLS = re.compile(r"\.(with_[a-z_]+|collapse_[a-z_]+|ignore_[a-z_]+)\(")
SCENARIO_NAME = re.compile(r'"([0-9]{2}_[a-z_]+)"')
QUERY_NAME = re.compile(r'QueryTestCase::new\("([a-z_]+)"\)')

# How a field name maps to the option that produces it. Derived from the scenario table at
# build time, not asserted here; these are only the labels put on what the experiment shows.
METRIC_PREFIX = "datafusion.metrics."
OTEL_PREFIX = "otel."


class SnapshotError(RuntimeError):
    """A pinned snapshot could not be read, so no row may be derived from it."""


def _body(text: str) -> str:
    """Strip insta's YAML header. The body is everything after the second `---` line."""
    parts = text.split("---\n", 2)
    if len(parts) != 3:
        raise SnapshotError("snapshot has no insta header")
    return parts[2]


def load_trace(path: Path) -> tuple[list[dict], bool]:
    """Return the span records of one trace snapshot, and whether the repair was applied."""
    body = _body(path.read_text(encoding="utf-8"))
    try:
        return json.loads(body), False
    except json.JSONDecodeError:
        pass
    repaired = body.replace(*REPAIR)
    try:
        return json.loads(repaired), True
    except json.JSONDecodeError as error:
        raise SnapshotError(
            f"{path.name} is not valid JSON and is not the one malformed snapshot this build "
            f"knows how to repair: {error}. A second repair would be a heuristic, and a "
            f"heuristic that silently succeeds is how an index starts describing something "
            f"other than its subject."
        ) from error


def scenarios(tests: Path) -> dict[str, dict]:
    """Read each integration scenario's option set from the pinned test source.

    Parsed with ast-grep rather than a regex over the file, because the thing being read is a
    builder chain and its extent is a syntactic fact: a regex would have to guess where one
    call ends, and guesses wrongly exactly when a scenario spans several lines -- which most of
    them do. Within one matched call expression, scanning for `.with_*(` is safe, because the
    match is already delimited.
    """
    target = tests / "integration_tests.rs"
    if not target.exists():
        raise SnapshotError(f"{target} is absent; scenario options cannot be read")
    result = subprocess.run(
        ["ast-grep", "scan", "--inline-rules", SCENARIO_RULE, "--json=stream", str(target)],
        capture_output=True, text=True, check=False,
    )
    if result.returncode not in (0, 1):
        raise SnapshotError(f"ast-grep exited {result.returncode}: {result.stderr[:300]}")
    found: dict[str, dict] = {}
    for line in result.stdout.splitlines():
        if not line.strip():
            continue
        text = json.loads(line)["text"]
        name = SCENARIO_NAME.search(text)
        if not name:
            continue
        query = QUERY_NAME.search(text)
        found[name.group(1)] = {
            "query": query.group(1) if query else "-",
            "options": sorted(set(OPTION_CALLS.findall(text))),
        }
    if not found:
        raise SnapshotError(
            "no integration scenarios matched. The option table is what makes a field's "
            "conditionality a measurement rather than a guess, so an empty result is a stop."
        )
    return found


def _classify(field: str, custom: set[str]) -> str:
    if field.startswith(METRIC_PREFIX):
        return "metric"
    if field.startswith(OTEL_PREFIX):
        return "otel"
    if field.startswith("object_store."):
        return "object-store"
    if field.startswith("datafusion."):
        return "instrumentation"
    if field in custom:
        return "custom"
    return "harness"


def _basis(kind: str, scenarios_seen: set[str], table: dict[str, dict]) -> tuple[str, str]:
    """Say WHY a field is present, and only name an option when an option is the reason.

    A metric field needs `with_metrics_collection()` -- and then needs a node that reports it.
    Twenty-eight of the metric fields observed here appear in one scenario only, and the naive
    reading of that is "conditional on `with_object_store_collection`", which is false: they
    appear there because that scenario is the one that reads Parquet. Attributing a node's
    metric to an unrelated option would be a confident wrong answer of exactly the kind this
    repository exists to prevent, so metric fields are stratified over the scenarios that
    enabled metrics at all, and anything narrower than that stratum is reported `node-dependent`
    rather than pinned on whatever option happens to correlate.
    """
    if kind == "metric":
        enabled = {
            name for name, entry in table.items()
            if "with_metrics_collection" in entry["options"]
        }
        if scenarios_seen >= enabled:
            return "option", "with_metrics_collection"
        return "node-dependent", "with_metrics_collection"
    implied = _implied_by(scenarios_seen, table)
    if implied == "-":
        return "unconditional" if len(scenarios_seen) == len(table) else "unexplained", "-"
    return "option", implied


def collect(corpus: Path) -> dict:
    """Walk every trace snapshot and aggregate the span and field tables."""
    traces = sorted((corpus / "traces").glob("*_trace.snap"))
    if not traces:
        raise SnapshotError(f"{corpus / 'traces'} holds no *_trace.snap files")
    table = scenarios(corpus / "tests")

    spans: dict[tuple[str, str], dict] = {}
    fields: dict[tuple[str, str], dict] = {}
    repaired_files: list[str] = []
    custom_fields: set[str] = set()

    for path in traces:
        scenario = path.name.removesuffix("_trace.snap")
        records, repaired = load_trace(path)
        if repaired:
            repaired_files.append(path.name)
        for record in records:
            span = record.get("span") or {}
            name = span.get("name")
            if not name:
                continue
            target = record.get("target", "-")
            level = record.get("level", "-")
            key = (name, target)
            entry = spans.setdefault(
                key,
                {"level": set(), "parents": set(), "scenarios": set(), "occurrences": 0,
                 "fields": set(), "clean": set()},
            )
            entry["level"].add(level)
            entry["scenarios"].add(scenario)
            entry["occurrences"] += 1
            if not repaired:
                entry["clean"].add(scenario)
            for parent in record.get("spans") or []:
                if parent.get("name") and parent["name"] != name:
                    entry["parents"].add(parent["name"])
            for field_name, value in span.items():
                if field_name == "name":
                    continue
                entry["fields"].add(field_name)
                field_entry = fields.setdefault(
                    (name, field_name),
                    {"scenarios": set(), "occurrences": 0, "example": "", "clean": set()},
                )
                field_entry["scenarios"].add(scenario)
                field_entry["occurrences"] += 1
                if not repaired:
                    field_entry["clean"].add(scenario)
                if not field_entry["example"]:
                    field_entry["example"] = _sample(value)

    # A field the caller declared rather than the library: present in the span but named by no
    # datafusion.* or otel.* convention. `05_basic_all_options` adds env and region this way,
    # which is the only evidence in the corpus that add_custom_field reaches a span at all.
    for (name, field_name) in fields:
        if not field_name.startswith((METRIC_PREFIX, OTEL_PREFIX, "datafusion.")):
            if field_name not in {"query", "sql", "logical_plan", "physical_plan",
                                  "query_name", "test_name"}:
                custom_fields.add(field_name)

    return {
        "spans": spans,
        "fields": fields,
        "scenarios": table,
        "repaired_files": sorted(repaired_files),
        "custom_fields": custom_fields,
        "trace_files": [p.name for p in traces],
    }


def _sample(value: object) -> str:
    text = value if isinstance(value, str) else json.dumps(value)
    text = text.replace("\t", " ").replace("\n", "\\n")
    return text[:110] + ("…" if len(text) > 110 else "")


def _implied_by(scenarios_seen: set[str], table: dict[str, dict]) -> str:
    """The option every scenario showing this field sets, and no other scenario sets.

    This is the whole reason the corpus is worth more than a flat field list. Nine scenarios
    differ from one another one option at a time, so a field confined to the scenarios carrying
    one option is conditional on it -- and a field present everywhere is unconditional. Where
    the evidence does not separate the options, the column says `-` rather than picking one.
    """
    if not scenarios_seen or len(scenarios_seen) == len(table):
        return "-"
    candidates = set.intersection(
        *(set(table[s]["options"]) for s in scenarios_seen if s in table)
    ) if all(s in table for s in scenarios_seen) else set()
    elsewhere = {
        option
        for name, entry in table.items()
        if name not in scenarios_seen
        for option in entry["options"]
    }
    discriminating = sorted(candidates - elsewhere)
    return ",".join(discriminating) if discriminating else "-"


def write_all(
    content: Path, corpus: Path, confirmed: set[str] | None = None
) -> dict[str, int]:
    """`confirmed` names the spans an executed probe reproduced with a control.

    Passed in rather than read here, and applied while the table is written rather than by
    editing it afterwards, so the same `behaviors.tsv` always yields the same `spans.tsv`. A
    promotion applied as a later mutation would make the build non-reproducible: running it
    twice would differ depending on whether probes had run in between.
    """
    confirmed = confirmed or set()
    data = collect(corpus)
    index = content / "index"
    index.mkdir(parents=True, exist_ok=True)
    # Only scenarios with a trace snapshot can discriminate. `07_scrabble_all_options` sets
    # `.ignore_full_trace()` upstream because its span ordering is not deterministic, so it
    # contributes no records -- and leaving it in the table made every option it sets look
    # non-discriminating, which silently emptied the conditionality column.
    observed = {name.removesuffix("_trace.snap") for name in data["trace_files"]}
    table = {k: v for k, v in data["scenarios"].items() if k in observed}

    span_rows = []
    for (name, target), entry in sorted(data["spans"].items()):
        span_rows.append("\t".join((
            name,
            target,
            ",".join(sorted(entry["level"])),
            str(len(entry["fields"])),
            ",".join(sorted(entry["fields"])) or "-",
            ",".join(sorted(entry["parents"])) or "-",
            str(entry["occurrences"]),
            ",".join(sorted(entry["scenarios"])),
            _implied_by(entry["scenarios"], table),
            CONFIRMED if name in confirmed else RECORDED,
            "repaired" if not entry["clean"] else "-",
        )))
    (index / "spans.tsv").write_text("".join(f"{row}\n" for row in span_rows))

    field_rows = []
    for (name, field_name), entry in sorted(data["fields"].items()):
        kind = _classify(field_name, data["custom_fields"])
        basis, gate = _basis(kind, entry["scenarios"], table)
        field_rows.append("\t".join((
            name,
            field_name,
            kind,
            str(entry["occurrences"]),
            ",".join(sorted(entry["scenarios"])),
            basis,
            gate,
            entry["example"] or "-",
            RECORDED,
            "repaired" if not entry["clean"] else "-",
        )))
    (index / "span-fields.tsv").write_text("".join(f"{row}\n" for row in field_rows))

    scenario_rows = [
        "\t".join((
            name,
            entry["query"],
            ",".join(entry["options"]) or "-",
            "trace" if name in observed else "no-trace",
        ))
        for name, entry in sorted(data["scenarios"].items())
    ]
    (index / "scenarios.tsv").write_text("".join(f"{row}\n" for row in scenario_rows))

    previews = _previews(corpus)
    (index / "previews.tsv").write_text("".join(f"{row}\n" for row in previews))

    _write_catalog(content, data, span_rows, previews)
    return {
        "spans": len(span_rows),
        "fields": len(field_rows),
        "scenarios": len(scenario_rows),
        "previews": len(previews),
    }


def _previews(corpus: Path) -> list[str]:
    """The per-node preview renders, which are what `preview_fn` actually produced.

    A preview snapshot is plain text, not JSON: it is the string the caller's preview function
    returned, captured verbatim. It is the only evidence in the repository of what
    `pretty_format_compact_batch` does to a column that does not fit -- which is the question
    about it nobody can answer from its signature.
    """
    rows = []
    for path in sorted((corpus / "traces").glob("*.snap")):
        if path.name.endswith("_trace.snap"):
            continue
        stem = path.name.removesuffix(".snap")
        parts = stem.split("_")
        node = parts[-1]
        ordinal = parts[-2] if len(parts) > 2 else "-"
        scenario = "_".join(parts[:-2])
        body = _body(path.read_text(encoding="utf-8"))
        lines = [line for line in body.splitlines() if line.strip()]
        width = max((len(line) for line in lines), default=0)
        compact = "|===" in body
        rows.append("\t".join((
            node, scenario, ordinal, str(len(lines)), str(width),
            "compact" if compact else "plain",
            f"corpus/traces/{path.name}",
        )))
    return rows


def _write_catalog(content: Path, data: dict, span_rows: list[str], previews: list[str]) -> None:
    catalogs = content / "catalogs"
    catalogs.mkdir(parents=True, exist_ok=True)
    table = data["scenarios"]

    lines = [
        "# The emitted span contract",
        "",
        f"Derived from {len(data['trace_files'])} upstream trace snapshots across "
        f"{len(table)} scenarios. Every row is **`recorded`**: upstream's observation under "
        "upstream's harness. `probes.py` promotes to `confirmed` only what a probe reproduces "
        "with a control that comes out the other way.",
        "",
        "## Spans",
        "",
        "| Span | Target | Level | Fields | Seen in | Conditional on |",
        "|---|---|---|---:|---:|---|",
    ]
    for row in span_rows:
        name, target, level, count, _fields, _parents, _occ, scenarios_seen, implied, _v, rep = \
            row.split("\t")
        flag = " ⚠" if rep == "repaired" else ""
        lines.append(
            f"| `{name}`{flag} | `{target}` | {level} | {count} | "
            f"{len(scenarios_seen.split(','))}/{len(table)} | "
            f"{'`' + implied + '`' if implied != '-' else 'unconditional'} |"
        )

    lines += [
        "",
        "## The target is the CALLER's crate, not this one",
        "",
        "Every `instrument_with_*_spans!` arm that omits `target:` expands to",
        "",
        "```rust",
        "$crate::instrument_with_spans!(target: module_path!(), $lvl, options: $options, ...)",
        "```",
        "",
        "and `module_path!()` expands at the **call site**. So the spans this library emits "
        "carry the target of whichever crate invoked the macro -- which is why every "
        "execution-plan row above reads `integration_utils` rather than `datafusion_tracing`. "
        "A subscriber filtered with `EnvFilter::new(\"datafusion_tracing=info\")` therefore "
        "receives **nothing**, silently. Filter on your own crate, or pass `target:` "
        "explicitly.",
        "",
        "`instrumented-object-store` behaves the opposite way: it calls `tracing` directly "
        "rather than through a macro, so its spans do carry "
        "`instrumented_object_store::instrumented_object_store`. The two halves of this "
        "library do not agree about targets, and the table above is the evidence.",
        "",
        "## The metrics field vocabulary is open",
        "",
        "`datafusion-tracing/src/metrics.rs` names every metric field as",
        "",
        "```rust",
        'format!("datafusion.metrics.{}", metric.value().name())',
        "```",
        "",
        "so the set of `datafusion.metrics.*` fields is whatever the DataFusion nodes in your "
        "plan report, not a list this library defines. What `span-fields.tsv` holds is what was "
        "observed under the pinned queries. A field absent from it is **unobserved, not "
        "unavailable** -- look at your node's `MetricsSet`, which is DataFusion's surface, not "
        "this one's.",
        "",
        "## Scenarios",
        "",
        "The nine trace snapshots differ one option at a time, which is what lets the "
        "`Conditional on` column above be a measurement. Options are read from "
        "`tests/integration_tests.rs` at build time.",
        "",
        "| Scenario | Query | Options |",
        "|---|---|---|",
    ]
    for name, entry in sorted(table.items()):
        options = ", ".join(f"`{o}()`" for o in entry["options"]) or "none"
        lines.append(f"| `{name}` | `{entry['query']}` | {options} |")

    if data["repaired_files"]:
        lines += [
            "",
            "## One snapshot upstream ships is not valid JSON",
            "",
            "".join(f"`{name}` " for name in data["repaired_files"]) + "carries "
            "`Some\\(\"…\"\\)` inside a JSON string: two invalid escapes and an unescaped quote. "
            "The cause is upstream's own insta filter in `tests/test_utils/insta_settings.rs`, "
            "whose replacement text carries the regex escapes `\\(` and `\\)` verbatim and drops "
            "the `\\\\\"` its pattern matched. The replacement should have read "
            "`e_tag: Some(\\\\\"ffffffff-fffffffffffff-fff\\\\\")`.",
            "",
            "This build substitutes that exact literal and nothing else, and flags every row "
            "derived from the file with `repaired` in the last column. Any other snapshot that "
            "fails to parse stops the build rather than receiving a second repair.",
        ]

    if previews:
        lines += [
            "",
            "## Preview renders",
            "",
            f"{len(previews)} per-node captures of what a `preview_fn` returned. "
            "`previews.tsv` carries node, scenario, line count, widest line and whether the "
            "render is `compact` -- the `|===|` header rule that "
            "`pretty_format_compact_batch` draws and `pretty_format_batches` does not.",
        ]

    (catalogs / "spans.md").write_text("\n".join(lines) + "\n")

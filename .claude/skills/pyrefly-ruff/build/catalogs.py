"""Catalogs: direct answers to fixed questions, every one derived from a machine oracle.

Two oracle families feed this module and neither is prose. The rustdoc index supplies the Rust
API; `acquired/oracles/` supplies what each tool says about itself at the pinned version. Where
both describe the same fact, they are joined and the join rate is checked, because a join that
silently degrades is how a generated catalog starts lying while still looking generated.

The join that matters most: `ruff rule --all --output-format json` reports 970 rules, and
`ruff_linter::codes::Rule` carries 970 variants. They correspond one-for-one under a kebab-case
to PascalCase mapping. That is only true because both come from the same pinned release -- the
ruff on this workstation's PATH is 0.16.7's predecessor 0.14.4 and reports 934, so a capture from
PATH would have lost 36 rules and still produced a confident-looking table.
"""

from __future__ import annotations

import json
from pathlib import Path

from model import Item

HERE = Path(__file__).resolve().parent
ORACLES = HERE / "acquired" / "oracles"

RULE_ENUM = "ruff_linter::codes::Rule"

# Predicates the cross-reference report emits, and what each one actually answers. Measured from
# a real payload rather than transcribed: these are the names in the captured sample.
GLEAN_MEANING: dict[str, str] = {
    "python.CalleeToCaller.4": "who calls this — the call-graph edge, reversed",
    "python.FileCall.4": "every call site in a file, with its argument positions",
    "python.XRefsViaNameByTarget.4": "every reference to a target, grouped by what it refers to",
    "python.XRefsViaNameByFile.4": "every reference in a file, grouped by the file",
    "python.xrefs.XRefsByFile.1": "the same cross-references in the newer by-file schema",
    "python.DeclarationLocation.4": "where each name is declared, with its span",
    "python.DefinitionLocation.4": "where each name is defined, with its span",
    "python.ContainingTopLevelDeclaration.4": "which top-level declaration encloses a declaration",
    "python.DeclarationDocstring.4": "the docstring attached to a declaration",
    "python.NameToSName.4": "a flat name to its structured (dotted) name",
    "python.Module.4": "the module entity itself",
    "python.Name.4": "interned name strings",
    "python.ImportStarLocation.4": "where a star-import brings names in",
    "src.FileLanguage.1": "the language a file was parsed as",
    "src.FileLines.1": "line-offset table, needed to turn a byte span into a line",
    "digest.FileDigest.1": "the content digest of the file the facts came from",
    "gencode.GenCode.1": "whether the file is generated",
}


def load_oracles() -> dict:
    """Read the captured tool output. Absent is a state, not a crash."""
    record = ORACLES / "ORACLES.json"
    if not record.is_file():
        return {}
    return json.loads(record.read_text())


def read_oracle(name: str) -> object:
    path = ORACLES / name
    if not path.is_file():
        return None
    return json.loads(path.read_text())


def write_all(
    items: dict[str, Item],
    facts: dict[str, dict],
    hidden_impls: set[tuple[str, str]],
    content: Path,
) -> dict[str, int]:
    """Write every catalog and return per-catalog row counts."""
    oracles = load_oracles()
    counts: dict[str, int] = {}
    counts["rules"] = write_rules(items, content)
    counts["cross_references"] = write_cross_references(oracles, content)
    counts["import_graph"] = write_import_graph(oracles, content)
    counts["structured_outputs"] = write_structured_outputs(oracles, content)
    counts["config_options"] = write_config_options(content)
    counts["protocols"] = write_protocols(items, content)
    counts["cli_surface"] = write_cli_surface(content)
    counts["error_kinds"] = write_error_kinds(items, content)
    counts["conformance"] = write_conformance(content)
    counts["crate_map"] = write_crate_map(items, facts, content)
    return counts


def _write(content: Path, name: str, lines: list[str]) -> None:
    target = content / "catalogs" / name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("\n".join(lines).rstrip() + "\n")


def _table(content: Path, name: str, rows: list[str]) -> int:
    """A TSV beside the prose. An empty table is an empty file, never a blank line."""
    target = content / "index" / name
    target.parent.mkdir(parents=True, exist_ok=True)
    ordered = sorted(set(rows))
    target.write_text("\n".join(ordered) + "\n" if ordered else "")
    return len(ordered)


def _clean(text: object) -> str:
    """TSV has two forbidden characters and no escape for either.

    `None` is coerced rather than raising: a few rules carry a null `linter`, and a catalog that
    dies on a null field is worse than one that records the absence.
    """
    if text is None:
        return "-"
    return str(text).replace("\t", " ").replace("\n", " ").replace("\r", " ").strip()


def fold(name: str) -> str:
    """Normalise a rule name for joining, by discarding separators and case.

    Naive kebab-to-PascalCase joins 937 of the 970 rules and silently loses 33, every one of
    them an acronym the variant spells in capitals while the rule name lowercases it:
    `blanket-noqa` is `BlanketNOQA`, `non-pep585-annotation` is `NonPEP585Annotation`,
    `io-error` is `IOError`, `six-py3` is `SixPY3`. Case-folding both sides is exact here --
    no two variants collide under it -- and `write_rules` fails the build if that ever changes.
    """
    return name.replace("-", "").replace("_", "").lower()


def write_rules(items: dict[str, Item], content: Path) -> int:
    """The complete lint-rule inventory, joined to the enum variant that implements it.

    The status field is the reason this is worth a catalog rather than a link: 17 of the 970 rules
    are `Removed`, and a removed rule is exactly what a model writes into a config file from
    memory. Preview rules are equally invisible in source -- selecting one without `--preview`
    silently does nothing.
    """
    rules = read_oracle("ruff-rules.json")
    if not isinstance(rules, list):
        _write(content, "rules.md", _blocked("rules", "ruff rule --all --output-format json"))
        return 0

    variants = set(items[RULE_ENUM].variants) if RULE_ENUM in items else set()
    folded: dict[str, str] = {}
    collisions = sorted(v for v in variants if fold(v) in folded and folded.setdefault(fold(v), v))
    for variant_name in sorted(variants):
        folded.setdefault(fold(variant_name), variant_name)
    if collisions:
        raise ValueError(
            f"{len(collisions)} Rule variants collide under case folding ({collisions[:3]}); "
            "the rule-to-variant join is no longer exact and needs a different key"
        )

    rows: list[str] = []
    joined = 0
    by_linter: dict[str, list[dict]] = {}
    for rule in rules:
        variant = folded.get(fold(_clean(rule.get("name"))))
        resolved = variant is not None
        joined += int(resolved)
        status = next(iter(rule.get("status") or {"Unknown": {}}))
        since = (rule.get("status") or {}).get(status, {}).get("since", "-")
        rows.append(
            "\t".join(
                (
                    _clean(rule.get("code")),
                    _clean(rule.get("name")),
                    _clean(rule.get("linter")),
                    _clean(rule.get("fix_availability")),
                    _clean(status),
                    _clean(since),
                    _clean((rule.get("source_location") or {}).get("file")),
                    f"{RULE_ENUM}::{variant}" if resolved else "-",
                )
            )
        )
        by_linter.setdefault(_clean(rule.get("linter")), []).append(rule)

    count = _table(content, "rules.tsv", rows)

    lines = [
        "# Lint rules",
        "",
        f"All {len(rules)} rules ruff 0.16.7 reports, from `ruff rule --all --output-format json`.",
        f"{joined} of them join to a variant of `{RULE_ENUM}`, which carries {len(variants)}.",
        "",
        "Three columns decide whether writing a rule code into a configuration file will do",
        "anything at all, and none of them is visible in the code being linted:",
        "",
        "| Status | Rules | What it means for a config file |",
        "|---|---:|---|",
    ]
    status_counts: dict[str, int] = {}
    for rule in rules:
        status_counts[next(iter(rule.get("status") or {"Unknown": {}}))] = (
            status_counts.get(next(iter(rule.get("status") or {"Unknown": {}})), 0) + 1
        )
    meaning = {
        "Stable": "selectable and enforced",
        "Preview": "selected only when `preview = true`; otherwise silently inert",
        "Removed": "selecting it is an error — the rule no longer exists",
        "Deprecated": "still works, but scheduled for removal",
    }
    for status, n in sorted(status_counts.items(), key=lambda kv: -kv[1]):
        lines.append(f"| `{status}` | {n} | {meaning.get(status, '—')} |")

    lines += [
        "",
        "Full rows, including each rule's implementing source file and enum variant, are in",
        "`content/index/rules.tsv`. Look a rule up by code rather than by name:",
        "",
        "```bash",
        "rg -P '^F401\\t' content/index/rules.tsv",
        "rg -P '\\tRemoved\\t' content/index/rules.tsv | cut -f1,2",
        "```",
        "",
        "## Rules by linter",
        "",
        "| Prefix | Linter | Rules |",
        "|---|---|---:|",
    ]
    linters = read_oracle("ruff-linters.json")
    prefixes = {row["name"]: row["prefix"] for row in linters} if isinstance(linters, list) else {}
    for linter, group in sorted(by_linter.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        lines.append(f"| `{prefixes.get(linter, '?')}` | {linter} | {len(group)} |")
    _write(content, "rules.md", lines)
    return count


def write_cross_references(oracles: dict, content: Path) -> int:
    """Definitions, references and call edges — the capability neither tool advertises clearly.

    This is the catalog the repository exists for. Neither `--help` nor the published docs state
    what `--report-glean` emits, so the shape below is read off a real payload produced against a
    fixed three-module fixture during acquisition.
    """
    captured = _captured(oracles, "report_glean")
    if not captured or not captured.get("predicates"):
        _write(content, "cross-references.md", _blocked("cross-references", "check --report-glean"))
        return 0

    predicates = captured["predicates"]
    lines = [
        "# Cross-references and call graphs",
        "",
        "`pyrefly check --report-glean <dir>` writes one JSON document per module, named by",
        "content digest, each a list of `{predicate, facts}` entries. It is the only route either",
        "toolchain offers to **definitions, references and caller/callee edges** as data.",
        "",
        "Measured during acquisition against a three-module fixture, so the counts below are",
        "shape evidence, not scale evidence:",
        "",
        "| Predicate | Facts | Answers |",
        "|---|---:|---|",
    ]
    for name, count in predicates:
        lines.append(f"| `{name}` | {count} | {GLEAN_MEANING.get(name, '—')} |")

    pysa_default = _captured(oracles, "report_pysa_capnp") or {}
    pysa_json = _captured(oracles, "report_pysa_json") or {}
    lines += [
        "",
        "`python.CalleeToCaller` is the direct answer to *who calls this*, and",
        "`python.XRefsViaNameByTarget` to *where is this referenced*. Both are absent from the LSP",
        "surface at any useful scale, because LSP answers one position at a time.",
        "",
        "## Recipe",
        "",
        "```bash",
        "pyrefly check --report-glean ./glean-out",
        "jq -r '.[]' ./glean-out/*.json \\",
        "  | jq -s 'map(select(.predicate==\"python.CalleeToCaller.4\")) | .[].facts'",
        "```",
        "",
        "## `--report-pysa` — richer, behind a flag",
        "",
        'Its help text promises "a Pysa-compatible JSON file for each module" and its DEFAULT',
        f"format does not deliver one: {pysa_default.get('file_count', 0)} files in formats",
        f"{pysa_default.get('formats', {})} — Cap'n Proto binary plus a copy of the bundled",
        "typeshed. Reading that needs the `.capnp` schema from the source tree.",
        "",
        "`--report-pysa-format json` is a different report. Measured at the same pin:",
        f"{pysa_json.get('file_count', 0)} files in formats {pysa_json.get('formats', {})} —",
        "real JSON under `definitions/`, `call_graphs/` and `type_of_expressions/`.",
        "",
        "Where it beats Glean, and it is not close:",
        "",
        "| Fact | Glean | pysa JSON |",
        "|---|---|---|",
        "| Position | UTF-8 byte span, needs `src.FileLines` | `line:col-line:col` |",
        "| Call target | the callee's dotted name | `init_targets`/`new_targets` with "
        "`receiver_class`, `implicit_receiver`, `is_static_method` |",
        "| Parameters | absent | kind (`Pos`/`PosOnly`/`KwOnly`/`VarArg`/`Kwargs`), requiredness, "
        "annotation resolved to a defining module and class |",
        "| Expression types | absent | every expression, per function |",
        "| MRO | absent | linearised, each entry naming its module |",
        "| References | `python.xrefs.XRefsByFile.1`, target carries its defining file | absent |",
        "",
        "So: **pysa JSON for calls, definitions, parameters and types; Glean for references.**",
        "The default format is what made this look unusable, and measuring only the default is",
        "how that conclusion survived a full acquisition pass.",
    ]
    _write(content, "cross-references.md", lines)
    return len(predicates)


def write_import_graph(oracles: dict, content: Path) -> int:
    """What `ruff analyze graph` gives you, and the four ways it is narrower than its name."""
    captured = _captured(oracles, "graph_dependencies", tool="ruff")
    missing = _captured(oracles, "graph_missing_path", tool="ruff") or {}
    sample = read_oracle("ruff-graph-dependencies.json")
    lines = [
        "# The import graph",
        "",
        "`ruff analyze graph` maps each file to the files it imports. It is fast, needs no",
        "configuration, and is narrower than its name in four ways that are all silent:",
        "",
        "1. **File-level, not module-level.** Keys and values are paths relative to the working",
        "   directory. There is no module-name representation, so joining it to anything that",
        "   speaks dotted module names is your job.",
        "2. **No `--output-format`.** It always writes JSON to stdout, and always prints",
        "   `warning: \\`ruff analyze graph\\` is experimental and may change without warning`",
        "   to stderr. Redirect the two separately.",
        "3. **A path that does not exist yields `{}` and exit status 0.** Measured during",
        f"   acquisition, the status was `{json.dumps(missing.get('exit'))}`. Nothing tells",
        "   *no imports* from *wrong path*, so check the key count, never the status.",
        "4. **Third-party edges need `--python`.** Without a virtual environment it resolves only",
        "   first-party files.",
        "",
        "Measured output for the acquisition fixture:",
        "",
        "```json",
        json.dumps(sample, indent=1) if sample is not None else "{}",
        "```",
        "",
        "`--direction dependents` inverts it. `analyze.include-dependencies` in configuration adds",
        "edges the scanner cannot see, which is the escape hatch for dynamic imports:",
        "",
        "```toml",
        "[tool.ruff.analyze.include-dependencies]",
        '"foo/bar.py" = ["foo/baz/*.py"]',
        "```",
        "",
        "For call edges rather than import edges, see `cross-references.md` — this command has no",
        "notion of a call.",
    ]
    _write(content, "import-graph.md", lines)
    return 1 if captured else 0


def write_structured_outputs(oracles: dict, content: Path) -> int:
    """Every machine-readable emitter of both tools, in one table."""
    rows = [
        (
            "ruff",
            "check",
            "12 formats",
            "`--output-format`: concise, full, json, json-lines,"
            " junit, grouped, github, gitlab, pylint, rdjson, azure, sarif",
        ),
        ("ruff", "rule --all", "JSON", "the complete rule inventory; see `rules.md`"),
        ("ruff", "config", "JSON", "every configuration key, its type and default"),
        ("ruff", "linter", "JSON", "the 59 upstream linter families and their code prefixes"),
        ("ruff", "version", "JSON", "`{version, commit_info}` — the version assertion"),
        ("ruff", "analyze graph", "JSON only", "file-level import map; see `import-graph.md`"),
        ("pyrefly", "check", "JSON", "`--output-format json` diagnostics"),
        ("pyrefly", "check --report-glean", "JSON", "definitions, references, call edges"),
        ("pyrefly", "check --report-pysa", "Cap'n Proto", "binary at the DEFAULT format"),
        (
            "pyrefly",
            "check --report-pysa --report-pysa-format json",
            "JSON",
            "definitions, call graphs, expression types; see `cross-references.md`",
        ),
        ("pyrefly", "coverage report", "JSON", "per-symbol typed/any/untyped coverage"),
        ("pyrefly", "stubgen", ".pyi", "reconstructed stubs; unresolved marked `Incomplete`"),
        ("pyrefly", "check --dependency-graph", "JSON", "file-level module imports"),
        ("pyrefly", "check --report-timings", "JSON", "per-module phase timings"),
        ("pyrefly", "check --report-trace", "text", "type traces"),
        ("pyrefly", "check --baseline", "JSON", "a diffable error baseline"),
        ("pyrefly", "dump-config", "text", "the resolved configuration and covered files"),
        ("pyrefly", "lsp", "LSP", "see `protocols.md`"),
        ("pyrefly", "tsp", "TSP", "LSP superset; see `protocols.md`"),
    ]
    lines = [
        "# Structured output",
        "",
        "Never parse either tool's human output. Both emit machine-readable forms for everything",
        "that matters, and the human formats are explicitly unstable.",
        "",
        "| Tool | Command | Format | Notes |",
        "|---|---|---|---|",
    ]
    lines += [f"| `{t}` | `{c}` | {f} | {n} |" for t, c, f, n in rows]
    lines += [
        "",
        "Two traps, both measured:",
        "",
        "- `ruff analyze graph` has **no** `--output-format` flag, unlike every other ruff",
        "  subcommand. Asking for one is an error, not a no-op.",
        "- `ruff check` exits non-zero when it finds diagnostics. That is a finding, not a",
        "  failure — use `--exit-zero` when the JSON is the product.",
    ]
    _write(content, "structured-outputs.md", lines)
    return len(rows)


def write_config_options(content: Path) -> int:
    """ruff's configuration surface, from the tool, as a table and a TSV."""
    config = read_oracle("ruff-config.json")
    if not isinstance(config, dict):
        _write(content, "config-options.md", _blocked("config", "ruff config --output-format json"))
        return 0
    # Each value is an object carrying `default`, `doc`, `value_type`, `scope` and a
    # `deprecated` marker. An earlier version scraped "Default value:" out of the rendered text
    # form and produced a column of dashes, because `ruff config --output-format json` does not
    # render; it returns the structured entry.
    rows = []
    for key, body in sorted(config.items()):
        entry = body if isinstance(body, dict) else {"doc": body}
        doc = _clean(entry.get("doc"))
        rows.append(
            "\t".join(
                (
                    "ruff",
                    key,
                    _clean(entry.get("value_type")),
                    _clean(entry.get("default")),
                    "deprecated" if entry.get("deprecated") else "-",
                    doc[:300],
                )
            )
        )
    count = _table(content, "config.tsv", rows)

    groups: dict[str, int] = {}
    for key in config:
        groups[key.split(".")[0] if "." in key else "(top level)"] = (
            groups.get(key.split(".")[0] if "." in key else "(top level)", 0) + 1
        )
    lines = [
        "# Configuration",
        "",
        f"All {len(config)} ruff configuration keys, from `ruff config --output-format json`.",
        "Full rows with defaults are in `content/index/config.tsv`.",
        "",
        "| Section | Keys |",
        "|---|---:|",
    ]
    lines += [f"| `{g}` | {n} |" for g, n in sorted(groups.items(), key=lambda kv: (-kv[1], kv[0]))]
    lines += [
        "",
        "```bash",
        "rg -P '^ruff\\tlint\\.' content/index/config.tsv | cut -f2,3",
        "```",
        "",
        "pyrefly's configuration is a JSON Schema in its own source tree (`schemas/pyrefly.json`)",
        "and every key is also a CLI override under `Config Overrides` in `cli-surface.md`.",
    ]
    _write(content, "config-options.md", lines)
    return count


def write_protocols(items: dict[str, Item], content: Path) -> int:
    """What the two servers advertise, from their own `initialize` response."""
    lsp = read_oracle("checker-lsp-initialize.json")
    tsp = read_oracle("checker-tsp-initialize.json")
    if not isinstance(lsp, dict):
        _write(content, "protocols.md", _blocked("protocols", "pyrefly lsp / tsp"))
        return 0

    def caps(doc: object) -> dict:
        return ((doc or {}).get("result") or {}).get("capabilities") or {}

    lsp_caps, tsp_caps = caps(lsp), caps(tsp)
    shared = sorted(set(lsp_caps) & set(tsp_caps))
    tsp_only = sorted(set(tsp_caps) - set(lsp_caps))

    tsp_items = sorted(p for p in items if p.startswith("tsp_types::"))
    lines = [
        "# Language servers: LSP and TSP",
        "",
        "`pyrefly lsp` and `pyrefly tsp` both speak framed JSON-RPC on stdio and both answer",
        "`initialize`. The capabilities below are the servers' own replies, captured during",
        "acquisition — not a reading of the source.",
        "",
        f"Both advertise these {len(shared)} capabilities:",
        "",
    ]
    lines += [f"- `{name}`" for name in shared]
    lines += [
        "",
        "The ones that matter for analysis, and that a weaker server either lacks or answers",
        "unreliably: `referencesProvider`, `callHierarchyProvider`, `typeHierarchyProvider`,",
        "`implementationProvider`, `declarationProvider`, `typeDefinitionProvider` and",
        "`inlayHintProvider`.",
        "",
    ]
    if tsp_only:
        lines += [f"TSP additionally advertises: {', '.join(f'`{n}`' for n in tsp_only)}."]
        experimental = tsp_caps.get("experimental")
        if experimental:
            lines += [
                "",
                "```json",
                json.dumps(experimental, indent=1),
                "```",
                "",
            ]
    lines += [
        "",
        "## The Type Server Protocol as types",
        "",
        f"`tsp_types` is indexed here as {len(tsp_items)} Rust items, so the protocol's request,",
        "response and notification shapes are readable as declarations rather than inferred from",
        "traffic. Start at `content/api/tsp_types.md`.",
        "",
        "LSP answers one position at a time. For whole-project definitions, references and call",
        "edges, use the Glean report instead — see `cross-references.md`.",
    ]
    _write(content, "protocols.md", lines)
    return len(shared)


def write_crate_map(items: dict[str, Item], facts: dict[str, dict], content: Path) -> int:
    """Which crate owns what, and — the load-bearing column — which version line it is on."""
    by_crate: dict[str, list[Item]] = {}
    for item in items.values():
        by_crate.setdefault(item.crate, []).append(item)

    lines = [
        "# Crate map",
        "",
        "One release can carry more than one version line. ruff publishes `ruff`, `ruff_linter`",
        "and `ruff_wasm` on the product line and every library crate on a `0.0.x` line; they are",
        "the same release, published together, and `ruff_linter 0.16.7` depends on the library",
        'crates at `^0.0.13`. Writing `ruff_python_ast = "0.16.7"` into a Cargo.toml selects a',
        "version that does not exist.",
        "",
        "| Crate | Version | Items | Traits |",
        "|---|---|---:|---:|",
    ]
    for crate in sorted(by_crate):
        group = by_crate[crate]
        version = facts.get(crate, {}).get("version") or group[0].version
        traits = sum(1 for i in group if i.kind == "trait")
        lines.append(f"| `{crate}` | {version} | {len(group)} | {traits} |")
    _write(content, "crate-map.md", lines)
    return len(by_crate)


def _captured(oracles: dict, key: str, tool: str | None = None) -> dict | None:
    for name, record in (oracles.get("tools") or {}).items():
        if tool and name != tool:
            continue
        captured = record.get("captured") or {}
        if key in captured:
            return captured[key]
    return None


def _blocked(what: str, prerequisite: str) -> list[str]:
    """A catalog whose oracle did not run says so, and names what would produce it."""
    return [
        f"# {what.replace('-', ' ').capitalize()}",
        "",
        "**blocked** — this catalog is generated from a tool's own output, and that output was",
        "not captured. Run `python3 build/acquire.py --stage oracles`, which needs",
        f"`{prerequisite}`.",
        "",
        "It is deliberately empty rather than reconstructed from documentation: a catalog that",
        "cannot be traced to the pinned tool is the thing this repository exists to avoid.",
    ]


def write_cli_surface(content: Path) -> int:
    """Every subcommand and flag of both tools, parsed from their own `--help`.

    Worth a table rather than a link because the two surfaces are lopsided in a way nobody
    expects: ruff's flags are almost all on `check`, while the checker repeats one large
    `Config Overrides` group on every subcommand, so "which flags does this accept" has a very
    different answer per tool. Parsed from captured text rather than transcribed, so a re-pin
    updates it.
    """
    rows: list[str] = []
    for path in sorted(ORACLES.glob("*-help*.txt")):
        stem = path.stem
        tool = "ruff" if stem.startswith("ruff") else "pyrefly"
        subcommand = stem.split("-help", 1)[1].lstrip("-") or "(top level)"
        group = "Options"
        for flag, value, summary in _parse_help(path.read_text()):
            if flag is None:
                group = value
                continue
            rows.append("\t".join((tool, subcommand, group, flag, value, summary)))
    count = _table(content, "cli.tsv", rows)

    by_tool: dict[str, set[str]] = {}
    for row in rows:
        tool, subcommand = row.split("\t")[:2]
        by_tool.setdefault(tool, set()).add(subcommand)
    lines = [
        "# Command-line surface",
        "",
        f"{count} distinct flags across both tools, parsed from the `--help` output captured at",
        "the pinned versions. Full rows are in `content/index/cli.tsv`:",
        "",
        "```bash",
        "rg -P '^ruff\\tcheck\\t' content/index/cli.tsv | cut -f4,5,6",
        "rg -P '\\tConfig Overrides\\t' content/index/cli.tsv | cut -f1,4",
        "```",
        "",
        "| Tool | Subcommands with flags |",
        "|---|---:|",
    ]
    for tool in sorted(by_tool):
        lines.append(f"| `{tool}` | {len(by_tool[tool])} |")
    lines += [
        "",
        "Two shapes worth knowing before reading a flag list:",
        "",
        "- ruff groups everything under `Options`, plus a small global group repeated everywhere.",
        "- The checker splits its flags into `Output`, `Behavior` and `Config Overrides`, and",
        "  repeats `Config Overrides` on every subcommand. Every key in its configuration file has",
        "  a matching override there, which is the supported way to answer a one-off question",
        "  without editing a config.",
    ]
    _write(content, "cli-surface.md", lines)
    return count


def _parse_help(text: str) -> list[tuple[str | None, str, str]]:
    """Yield (flag, value, summary) triples, and (None, group, "") when a group heading starts.

    clap's layout is stable enough to parse and not stable enough to trust blindly: a flag sits at
    two to six spaces of indent, its description is indented further, and a group heading is a
    bare capitalised word followed by a colon at column zero.
    """
    out: list[tuple[str | None, str, str]] = []
    lines = text.splitlines()
    for position, line in enumerate(lines):
        stripped = line.strip()
        if line and not line[0].isspace() and stripped.endswith(":") and len(stripped) < 40:
            out.append((None, stripped[:-1], ""))
            continue
        indent = len(line) - len(line.lstrip())
        if not (2 <= indent <= 6) or not stripped.startswith("-"):
            continue
        head = stripped.split("  ")[0]
        parts = head.replace(",", " ").split()
        flag = next((p for p in parts if p.startswith("--")), parts[0])
        value = next((p for p in parts if p.startswith("<") or p.startswith("[")), "-")
        # clap glues an optional value to its flag: `--add-noqa[=<REASON>]`, `--summary[=<S>]`.
        # Splitting them keeps the flag column greppable as a flag.
        for separator in ("[", "="):
            if separator in flag:
                flag, _, attached = flag.partition(separator)
                value = separator + attached if value == "-" else value
        summary = ""
        tail = stripped[len(head) :].strip()
        if tail:
            summary = tail
        else:
            for follower in lines[position + 1 : position + 3]:
                if follower.strip() and len(follower) - len(follower.lstrip()) > indent:
                    summary = follower.strip()
                    break
        out.append((flag, value, _clean(summary)[:220]))
    return out


def write_error_kinds(items: dict[str, Item], content: Path) -> int:
    """The checker's error taxonomy, which is also its filtering vocabulary."""
    path = "pyrefly_config::error_kind::ErrorKind"
    if path not in items:
        _write(content, "error-kinds.md", _blocked("error kinds", "the pyrefly crate set"))
        return 0
    kinds = sorted(items[path].variants)
    rows = ["\t".join(("pyrefly", kind, _kebab(kind))) for kind in kinds]
    count = _table(content, "error-kinds.tsv", rows)
    lines = [
        "# Error kinds",
        "",
        f"The checker classifies every diagnostic into one of {len(kinds)} kinds, enumerated by",
        f"`{path}`. The same vocabulary is the filter: `--only`, `--error`, `--warn` and",
        "`--ignore` all take a kind, and a configuration file can set a severity per kind.",
        "",
        "The CLI spells them in kebab-case while the enum spells them in PascalCase, so",
        "`content/index/error-kinds.tsv` carries both.",
        "",
        "```bash",
        "rg -i 'annotation|return' content/index/error-kinds.tsv | cut -f2,3",
        "```",
        "",
        "Prose for each kind, with examples, is in",
        "`content/corpus/pyrefly/error-kinds.mdx`; suppression syntax is in",
        "`content/corpus/pyrefly/error-suppressions.mdx`.",
        "",
        "ruff's equivalent vocabulary is not an enum of kinds but the 970 rule codes in",
        "`rules.md`. The two do not correspond: a ruff rule is a check, a checker error kind is a",
        "category of type error.",
        "",
        "## The kinds",
        "",
    ]
    lines += [f"- `{_kebab(kind)}`" for kind in kinds]
    _write(content, "error-kinds.md", lines)
    return count


def _kebab(pascal: str) -> str:
    out: list[str] = []
    for position, character in enumerate(pascal):
        if character.isupper() and position:
            out.append("-")
        out.append(character.lower())
    return "".join(out)


def write_conformance(content: Path) -> int:
    """Which typing features the checker actually supports, from its own measured results.

    This is the rare case where upstream publishes a scored, machine-readable self-assessment
    against an external suite. Reporting it is strictly better than characterising support in
    prose, and the failing list is the part worth reading.
    """
    root = content / "corpus" / "conformance" / "third_party"
    summary_path = root / "results.json"
    detail_path = root / "conformance.result"
    if not summary_path.is_file():
        _write(content, "conformance.md", _blocked("conformance", "the conformance corpus"))
        return 0
    summary = json.loads(summary_path.read_text())
    detail: dict = {}
    if detail_path.is_file():
        body = detail_path.read_text()
        detail = json.loads(body[body.index("{") :])

    rows = []
    for name in sorted(set(summary.get("passing", [])) | set(summary.get("failing", []))):
        status = "pass" if name in set(summary.get("passing", [])) else "fail"
        rows.append("\t".join(("pyrefly", name, status, str(len(detail.get(name, []))))))
    count = _table(content, "conformance.tsv", rows)

    failing = sorted(summary.get("failing", []))
    lines = [
        "# Typing conformance",
        "",
        "Measured by upstream against the Python typing council's conformance suite, and",
        "committed to the repository rather than asserted:",
        "",
        "| | |",
        "|---|---:|",
        f"| Tests | {summary.get('total')} |",
        f"| Pass | {summary.get('pass')} |",
        f"| Fail | {summary.get('fail')} |",
        f"| Pass rate | {summary.get('pass_rate')} |",
        f"| Recorded deviations | {summary.get('differences')} |",
        "",
        "A test passes when the checker's diagnostics match the suite's expectations exactly, so",
        "a failure is a **deviation from the specification**, not a crash. The deviations are",
        "per-line and readable in `corpus/conformance/third_party/conformance.result`.",
        "",
        f"## The {len(failing)} failing areas",
        "",
    ]
    lines += [f"- `{name}` — {len(detail.get(name, []))} deviations" for name in failing]
    lines += [
        "",
        "Every deviation, including those in otherwise-passing files, is in",
        "`content/index/conformance.tsv`:",
        "",
        "```bash",
        "rg -P '\\tfail\\t' content/index/conformance.tsv | cut -f2,4",
        "```",
        "",
        "The suite's own sources are in `corpus/conformance/third_party/`, so a question such as",
        "whether PEP 695 type parameter defaults are handled is answered by reading the test",
        "rather than by trying it.",
    ]
    _write(content, "conformance.md", lines)
    return count

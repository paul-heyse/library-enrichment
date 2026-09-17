"""Build the line-oriented indexes from the binary oracles and the acquired schemas.

Most of this is mechanical projection. One part is not, and it is the reason this module exists
rather than a few dict comprehensions in `build.py`.

**The two shipped kind catalogues disagree.** ast-grep publishes node kinds twice: once in
`schemas/languages.json` as `<lang>Nodes`, and once per language in `schemas/<lang>_rule.json`
as the `kind` enum. For Rust the first lists 161 kinds and the second 163, and neither is a
superset -- each holds kinds the other omits. Picking one silently would publish a catalogue
that is wrong in a way no reader could detect.

So disputed kinds are **adjudicated by the binary**. `ast-grep run -k <kind>` exits 8 when it
cannot parse the kind as a selector and 1 when the kind is valid but matches nothing, which is
exactly the distinction needed. On the build machine this ruled `type_parameter`,
`lifetime_parameter`, `use_bounds` and `generic_pattern` valid, and
`constrained_type_parameter` and `optional_type_parameter` invalid -- so the per-language rule
schema was right and `languages.json` carries two kinds this binary rejects.

Every kind therefore ships with the source that claimed it and, where the sources disagreed, the
verdict the binary gave.
"""

from __future__ import annotations

import json
from pathlib import Path

import oracles

# Exit 8 is ast-grep's "cannot parse kind as a valid selector". Exit 1 is a valid kind that
# matched nothing. The difference is the whole adjudication.
AST_GREP_BAD_SELECTOR = 8

BOTH = "both"
RULE_SCHEMA = "rule-schema"
LANGUAGES_SCHEMA = "languages-schema"


def _clean(value: object) -> str:
    """Scrub tabs and newlines so a value can live in one TSV field."""
    text = str(value if value is not None else "")
    return text.replace("\t", " ").replace("\r", " ").replace("\n", " ").strip()


def write_tsv(path: Path, rows: list[tuple]) -> int:
    """Write sorted, header-less, tab-separated rows."""
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = sorted("\t".join(_clean(cell) for cell in row) for row in rows)
    path.write_text("\n".join(lines) + ("\n" if lines else ""), encoding="utf-8")
    return len(lines)


# --------------------------------------------------------------------------- flags


def flag_rows() -> list[tuple]:
    """One row per flag of both tools, carrying upstream's own category."""
    rows = []
    for flag in oracles.ripgrep_flags() + oracles.ast_grep_flags():
        if not flag.get("long") and not flag.get("short"):
            continue
        rows.append(
            (
                flag["tool"],
                flag["command"],
                flag.get("long", ""),
                flag.get("short", ""),
                flag.get("category", ""),
                flag.get("arg", ""),
                flag.get("values", ""),
                flag.get("summary", ""),
            )
        )
    return rows


# --------------------------------------------------------------------------- kinds and fields


def _load_schema_kinds(schemas: Path) -> tuple[dict[str, set[str]], dict[str, set[str]]]:
    """Return (kinds by language, fields by language) from `languages.json`."""
    definitions = json.loads((schemas / "languages.json").read_text(encoding="utf-8"))[
        "definitions"
    ]
    kinds = {
        k[: -len("Nodes")]: set(v["enum"]) for k, v in definitions.items() if k.endswith("Nodes")
    }
    fields = {
        k[: -len("Fields")]: set(v["enum"]) for k, v in definitions.items() if k.endswith("Fields")
    }
    return kinds, fields


def _load_rule_schema_kinds(schemas: Path) -> dict[str, set[str]]:
    """Return kinds by language from each `<lang>_rule.json`."""
    kinds: dict[str, set[str]] = {}
    for path in sorted(schemas.glob("*_rule.json")):
        language = path.name[: -len("_rule.json")]
        document = json.loads(path.read_text(encoding="utf-8"))
        enum = (
            document.get("$defs", {})
            .get("SerializableRule", {})
            .get("properties", {})
            .get("kind", {})
            .get("enum")
        )
        if enum:
            kinds[language] = set(enum)
    return kinds


def _adjudicate(language: str, kind: str) -> bool:
    """Ask the binary whether it accepts this kind as a selector."""
    code, _, _ = oracles.run(
        "ast-grep",
        "run",
        "--stdin",
        "-l",
        language,
        "-k",
        kind,
        check=False,
        stdin="\n",
    )
    return code != AST_GREP_BAD_SELECTOR


def kind_rows(schemas: Path) -> tuple[list[tuple], dict[str, int]]:
    """One row per node kind, with its source and, where disputed, the binary's verdict."""
    languages_kinds, _ = _load_schema_kinds(schemas)
    rule_kinds = _load_rule_schema_kinds(schemas)

    rows: list[tuple] = []
    stats = {"disputed": 0, "rejected": 0, "languages": 0}

    for language in sorted(set(languages_kinds) | set(rule_kinds)):
        from_languages = languages_kinds.get(language, set())
        from_rules = rule_kinds.get(language, set())
        stats["languages"] += 1

        for kind in sorted(from_languages | from_rules):
            in_both = kind in from_languages and kind in from_rules
            if in_both:
                rows.append((language, kind, BOTH, "not-disputed"))
                continue
            stats["disputed"] += 1
            source = RULE_SCHEMA if kind in from_rules else LANGUAGES_SCHEMA
            accepted = _adjudicate(language, kind)
            if not accepted:
                stats["rejected"] += 1
            rows.append((language, kind, source, "accepted" if accepted else "rejected"))
    return rows, stats


def field_rows(schemas: Path) -> list[tuple]:
    """One row per tree-sitter field name, the vocabulary `has: {field: ...}` draws on."""
    _, fields = _load_schema_kinds(schemas)
    return [(language, name) for language, names in fields.items() for name in sorted(names)]


# --------------------------------------------------------------------------- rule surface


SCOPE_BY_DEF = {
    "SerializableRule": "ruleObject",
    "Relation": "relation",
    "Transformation": "transform",
    "Trans": "transform",
    "Substring": "transform.substring",
    "Replace": "transform.replace",
    "Convert": "transform.convert",
    "Rewrite": "transform.rewrite",
    "SerializableFixConfig": "fix",
    "SerializableRewriter": "rewriter",
    "SerializableNthChild": "nthChild",
    "SerializableRange": "range",
    "LabelConfig": "labels",
}


def rule_field_rows(schemas: Path) -> list[tuple]:
    """One row per field of the YAML rule surface, from ast-grep's own JSON Schema."""
    document = json.loads((schemas / "rust_rule.json").read_text(encoding="utf-8"))
    rows: list[tuple] = []
    required = set(document.get("required", []))

    for name, spec in document.get("properties", {}).items():
        rows.append(
            (
                "ruleFile",
                name,
                _type_of(spec),
                "required" if name in required else "optional",
                _summary_of(spec),
            )
        )

    for def_name, definition in document.get("$defs", {}).items():
        scope = SCOPE_BY_DEF.get(def_name)
        if not scope:
            continue
        def_required = set(definition.get("required", []))
        for name, spec in definition.get("properties", {}).items():
            rows.append(
                (
                    scope,
                    name,
                    _type_of(spec),
                    "required" if name in def_required else "optional",
                    _summary_of(spec),
                )
            )
    return rows


def _type_of(spec: dict) -> str:
    if "type" in spec:
        value = spec["type"]
        return ",".join(value) if isinstance(value, list) else str(value)
    if "$ref" in spec:
        return spec["$ref"].rsplit("/", 1)[-1]
    if "oneOf" in spec or "anyOf" in spec:
        return "oneOf"
    return ""


def _summary_of(spec: dict) -> str:
    return _clean(spec.get("description", "")).split(". ")[0]


# --------------------------------------------------------------------------- languages


def language_rows(schemas: Path, probed: dict[str, bool]) -> list[tuple]:
    """One row per language name, joining what the binary accepts to what the schemas describe."""
    languages_kinds, fields = _load_schema_kinds(schemas)
    rule_kinds = _load_rule_schema_kinds(schemas)
    names = set(probed) | set(languages_kinds) | set(rule_kinds)

    rows = []
    for name in sorted(names):
        accepted = probed.get(name)
        rows.append(
            (
                name,
                "accepted" if accepted else ("rejected" if accepted is False else "not-probed"),
                len(rule_kinds.get(name, languages_kinds.get(name, set()))),
                len(fields.get(name, set())),
                "yes" if name in rule_kinds else "no",
            )
        )
    return rows


# --------------------------------------------------------------------------- exit codes


def exit_code_rows() -> list[tuple]:
    """The exit vocabulary of both tools, including the places it carries no information.

    These are not the same vocabulary, and conflating them is a real source of silent wrong
    answers: a script that treats `== 2` as "error" misreads every ast-grep failure, and one
    that treats any nonzero status as "no matches" turns a broken invocation into a confident
    false negative.
    """
    return [
        ("rg", "rg", "0", "at least one match", ""),
        ("rg", "rg", "1", "ran successfully, no match", "A clean no-match, not an error."),
        (
            "rg",
            "rg",
            "2",
            "an error occurred",
            "Distinct from 1. Collapsing them turns a broken invocation into a false negative.",
        ),
        ("ast-grep", "ast-grep run", "0", "at least one match", ""),
        ("ast-grep", "ast-grep run", "1", "ran successfully, no match", ""),
        (
            "ast-grep",
            "ast-grep run",
            "8",
            "the pattern or kind could not be parsed",
            "Not 2. ast-grep does not share ripgrep's exit vocabulary.",
        ),
        (
            "ast-grep",
            "ast-grep outline",
            "0",
            "always, including an empty outline AND a missing file",
            "Probe A007: outline exits 0 even for a path that does not exist, reporting the "
            "error only on stderr. The status carries no information; branch on stdout.",
        ),
        (
            "ast-grep",
            "ast-grep test",
            "3",
            "--filter matched no rule",
            "A filter typo looks like a clean run unless you check for 3.",
        ),
        (
            "ast-grep",
            "ast-grep test",
            "0",
            "tests ran, or --update-all was used",
            "--update-all always exits 0, so it can never gate anything.",
        ),
    ]


# --------------------------------------------------------------------------- file types


def file_type_rows() -> list[tuple]:
    return [(name, globs) for name, globs in oracles.ripgrep_type_list()]

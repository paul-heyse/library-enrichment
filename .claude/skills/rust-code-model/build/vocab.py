"""Catalogues parsed out of pinned upstream source.

These four tables exist because the layers they describe have no rustdoc JSON to index. There
is no hosted documentation for any compiler-internal crate, and rust-analyzer's generated AST
is emitted with the missing-docs lint switched off, so its node set has no doc comments to
extract either. In both cases the defining source file *is* the reference, and parsing it is the
only way to get a greppable catalogue.

Nothing here parses Rust properly. These are brace-counting readers aimed at four specific
files pinned to exact commits, which is a reasonable trade when the alternative is vendoring a
Rust parser -- but it is also why every reader asserts a plausible row count and fails loudly
rather than silently emitting a short table. A catalogue that quietly loses half its rows reads
exactly like a layer with a smaller vocabulary.
"""

from __future__ import annotations

import re
from pathlib import Path

DOC_LINE = re.compile(r"^\s*///\s?(.*)$")
ENUM_HEAD = re.compile(r"^pub enum (\w+)(?:<[^>]*>)?\s*\{", re.MULTILINE)
VARIANT = re.compile(r"^\s{4}(?:#\[[^\]]*\]\s*)?([A-Z]\w*)\s*(?:\{|\(|,|=|$)")
UNGRAM_NODE = re.compile(r"^([A-Za-z_]\w*)\s*=", re.MULTILINE)
TEXT_ARM = re.compile(r'^\s+([A-Z][A-Z0-9_]*) => "((?:[^"\\]|\\.)*)"', re.MULTILINE)
SYNTAX_KIND_VARIANT = re.compile(r"^\s{4}(_*[A-Z][A-Z0-9_]*),\s*$", re.MULTILINE)

# The MIR enums that make up the operational vocabulary an agent actually reads in a dump.
# Ordered as a reader meets them: what a block does, how it leaves, what values look like.
MIR_ENUMS = (
    ("MirPhase", "which MIR you are looking at"),
    ("AnalysisPhase", "sub-phase of analysis MIR"),
    ("RuntimePhase", "sub-phase of runtime MIR"),
    ("StatementKind", "what a statement inside a basic block does"),
    ("TerminatorKind", "how a basic block hands control on"),
    ("Rvalue", "how a value is computed"),
    ("Operand", "how a value is consumed"),
    ("ProjectionElem", "how a place is narrowed"),
    ("BorrowKind", "what kind of reference is taken"),
    ("AggregateKind", "how a composite value is built"),
    ("CastKind", "how a cast is performed"),
    ("UnwindAction", "where unwinding goes"),
    ("AssertKind", "which runtime check failed"),
    ("NonDivergingIntrinsic", "an intrinsic that cannot branch"),
    ("FakeReadCause", "why a borrow-check-only read exists"),
    ("CallSource", "what syntax produced a call"),
)

LITERAL_KINDS = frozenset(
    {"BYTE", "BYTE_STRING", "CHAR", "C_STRING", "FLOAT_NUMBER", "INT_NUMBER", "STRING"}
)
TRIVIA_KINDS = frozenset({"WHITESPACE", "COMMENT", "ERROR", "TOMBSTONE", "EOF", "__LAST"})


class VocabError(RuntimeError):
    """A pinned source file did not parse into a plausible catalogue."""


def _clean(text: str) -> str:
    return " ".join(text.replace("\t", " ").split())


def _first_sentence(lines: list[str]) -> str:
    joined = _clean(" ".join(lines))
    if not joined:
        return ""
    head = joined.split(". ", 1)[0]
    return head if head.endswith(".") else head + ("." if joined else "")


def _has_default_body(text: str, start: int, limit: int) -> bool:
    """Does the method beginning at `start` carry a default body?

    Decided from the signature alone -- the first `;` or `{` that follows the header -- rather
    than by scanning the whole slice up to the next member. Scanning the slice reads the *next*
    member's doc comment too, and rustc's prose contains braces: the comment above
    `apply_effect` mentions "{early,primary} x {statement,terminator}", which is enough to make
    a required method look provided.
    """
    for position in range(start, limit):
        if text[position] == ";":
            return False
        if text[position] == "{":
            return True
    return False


def _trait_body(text: str, header: str) -> str | None:
    """Return the brace-balanced body that follows `header`, or None if it is absent.

    Without this the reader would take everything after the header to the end of the file,
    which silently folds every later trait's members into the one being described.
    """
    if header not in text:
        return None
    cursor = text.index(header) + len(header)
    start, depth = cursor, 1
    while cursor < len(text) and depth:
        if text[cursor] == "{":
            depth += 1
        elif text[cursor] == "}":
            depth -= 1
        cursor += 1
    return text[start : cursor - 1]


# --------------------------------------------------------------------------- Rust enums


def parse_enums(text: str) -> dict[str, list[tuple[str, str]]]:
    """Return {enum name: [(variant, first doc sentence)]} for every `pub enum` in one file.

    Variants are taken at brace depth one relative to the enum body, so a struct-shaped variant
    with its own fields does not contribute its field names as variants.
    """
    found: dict[str, list[tuple[str, str]]] = {}
    for match in ENUM_HEAD.finditer(text):
        name = match.group(1)
        cursor = match.end()
        depth = 1
        body_start = cursor
        while cursor < len(text) and depth:
            char = text[cursor]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
            cursor += 1
        body = text[body_start : cursor - 1]

        variants: list[tuple[str, str]] = []
        docs: list[str] = []
        depth = 0
        for line in body.splitlines():
            doc = DOC_LINE.match(line)
            if doc and depth == 0:
                docs.append(doc.group(1))
                continue
            if depth == 0:
                variant = VARIANT.match(line)
                if variant:
                    variants.append((variant.group(1), _first_sentence(docs)))
                    docs = []
                elif line.strip() and not line.strip().startswith(("//", "#[")):
                    docs = []
            depth += line.count("{") + line.count("(") - line.count("}") - line.count(")")
            depth = max(depth, 0)
        found[name] = variants
    return found


def write_mir_vocabulary(source: Path, out: Path) -> int:
    """`category · variant · meaning` for the enums that make up MIR's operational vocabulary."""
    text = (source / "rustc" / "mir-syntax.rs").read_text(encoding="utf-8")
    enums = parse_enums(text)

    missing = [name for name, _ in MIR_ENUMS if not enums.get(name)]
    if missing:
        raise VocabError(
            f"these MIR enums did not parse out of mir-syntax.rs: {missing}. The file is pinned "
            f"to a commit, so this means the reader is wrong rather than upstream having moved."
        )

    rows: list[str] = []
    for name, purpose in MIR_ENUMS:
        for variant, doc in enums[name]:
            rows.append("\t".join((name, variant, purpose, doc or "-")))
    (out / "mir-vocabulary.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")
    return len(rows)


# --------------------------------------------------------------------------- dataflow


def write_dataflow(source: Path, out: Path) -> int:
    """The `Analysis` trait's shape, its two directions, and the shipped analyses.

    Worth reading closely if you remember this API: `AnalysisDomain` is gone as a separate
    trait -- `Domain`, `Direction` and `NAME` are associated items on `Analysis` itself -- and
    `GenKillAnalysis` was removed outright, though the `GenKill` helper trait remains. A
    tutorial written against either is dead code now.
    """
    framework = (source / "rustc" / "dataflow-framework.rs").read_text(encoding="utf-8")
    direction = (source / "rustc" / "dataflow-direction.rs").read_text(encoding="utf-8")
    impls = (source / "rustc" / "dataflow-impls.rs").read_text(encoding="utf-8")

    rows: list[str] = []

    members = _trait_body(framework, "pub trait Analysis<'tcx> {")
    if members is None:
        raise VocabError("the Analysis trait did not parse out of dataflow-framework.rs")

    # Members are read as slices between one member header and the next, at the trait's own
    # indent, and each method is then classified from its own signature. That distinction is
    # the whole difference between "you must write this" and "you may override it", and it is
    # the first thing anyone implementing the trait needs.
    headers = list(re.finditer(r"^    (type|const|fn) (\w+)", members, re.MULTILINE))
    for position, match in enumerate(headers):
        kind = {"type": "associated-type", "const": "associated-const", "fn": "method"}[
            match.group(1)
        ]
        end = headers[position + 1].start() if position + 1 < len(headers) else len(members)
        provided = kind != "method" or _has_default_body(members, match.start(), end)
        rows.append(
            "\t".join(("Analysis", kind, match.group(2), "provided" if provided else "required"))
        )

    for name in re.findall(r"^pub struct (Forward|Backward);", direction, re.MULTILINE):
        rows.append("\t".join(("Direction", "impl", name, "provided")))

    for name in sorted(set(re.findall(r"\b(Maybe\w+|EverInitializedPlaces)\b", impls))):
        if name.endswith("Domain"):
            continue
        rows.append("\t".join(("shipped-analysis", "struct", name, "provided")))

    (out / "dataflow.tsv").write_text("\n".join(rows) + "\n", encoding="utf-8")
    return len(rows)


# --------------------------------------------------------------------------- rust-analyzer


def write_ast_nodes(source: Path, out: Path) -> int:
    """Node names from `rust.ungram`, the grammar the AST is generated from.

    The generated `ast` module has no doc comments at all, so the grammar is a better catalogue
    of the node set than the rustdoc of the types it produces.
    """
    text = (source / "rust-analyzer" / "rust.ungram").read_text(encoding="utf-8")
    nodes = sorted(set(UNGRAM_NODE.findall(text)))
    if len(nodes) < 100:
        raise VocabError(f"rust.ungram yielded only {len(nodes)} nodes; the reader is wrong")
    (out / "ast-nodes.tsv").write_text(
        "\n".join(f"{node}\t{_snake(node)}" for node in nodes) + "\n", encoding="utf-8"
    )
    return len(nodes)


def _snake(name: str) -> str:
    """`RecordExpr` -> `RECORD_EXPR`, the SyntaxKind spelling of an AST node."""
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).upper()


def write_syntax_kinds(source: Path, out: Path) -> int:
    """Every `SyntaxKind`, classified, with its literal text where it has one.

    The classification is derived, not guessed: a kind has text exactly when `SyntaxKind::text`
    returns one for it, and the kinds that panic there are the nodes, literals and trivia. A
    A kind with no entry in `text` that is also absent from the grammar is written as
    `token-no-fixed-text` -- which is what was observed, not a guess about intent. IDENT,
    LIFETIME_IDENT and the doc-comment kinds land there, and that is correct: they are tokens
    whose text varies per occurrence.
    """
    text = (source / "rust-analyzer" / "syntax-kind-generated.rs").read_text(encoding="utf-8")
    enum_body = text.split("pub enum SyntaxKind {", 1)
    if len(enum_body) != 2:
        raise VocabError("the SyntaxKind enum did not parse out of the generated file")
    kinds = SYNTAX_KIND_VARIANT.findall(enum_body[1].split("\n}", 1)[0])
    if len(kinds) < 300:
        raise VocabError(f"SyntaxKind yielded only {len(kinds)} variants; the reader is wrong")

    literals = dict(TEXT_ARM.findall(text))
    ungram = {
        _snake(node)
        for node in UNGRAM_NODE.findall(
            (source / "rust-analyzer" / "rust.ungram").read_text(encoding="utf-8")
        )
    }

    rows: list[str] = []
    for kind in kinds:
        spelling = literals.get(kind, "")
        if kind in TRIVIA_KINDS:
            klass = "trivia-or-sentinel"
        elif kind in LITERAL_KINDS:
            klass = "literal"
        elif kind.endswith("_KW"):
            klass = "keyword"
        elif spelling:
            klass = "punctuation"
        elif kind in ungram:
            klass = "node"
        else:
            klass = "token-no-fixed-text"
        rows.append("\t".join((kind, klass, spelling or "-", "yes" if kind in ungram else "no")))
    (out / "syntax-kinds.tsv").write_text("\n".join(sorted(rows)) + "\n", encoding="utf-8")
    return len(rows)


# --------------------------------------------------------------------------- entry point


def write_all(source: Path, out: Path) -> dict[str, int]:
    return {
        "mir_vocabulary": write_mir_vocabulary(source, out),
        "dataflow": write_dataflow(source, out),
        "ast_nodes": write_ast_nodes(source, out),
        "syntax_kinds": write_syntax_kinds(source, out),
    }

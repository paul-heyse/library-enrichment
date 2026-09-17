"""Build the regex construct map: what each engine supports, and how that was established.

This is the index an agent consults before choosing between `rg`, `rg -P`, and giving up on a
regex entirely. Three columns decide everything, and they do not come from the same place.

* `rust_regex` -- whether ripgrep's default engine accepts the construct.
* `pcre2` -- whether PCRE2 accepts it.
* `rg_reachable` -- whether it is reachable through the ripgrep CLI, which is a **narrower**
  question than whether PCRE2 supports it. ripgrep drives PCRE2 through a fixed builder and
  exposes no way to set arbitrary compile options, so genuine PCRE2 features can be unreachable.

Where a probe exists, the verdict comes from the probe and the probe id is carried in the row.
That matters more here than anywhere else in this repository, because the obvious alternative --
reading `rg --pcre2-version` -- is known to be wrong on the machine this targets: the string says
10.45 while the linked library is 10.48, three releases of drift.

`since_pcre2` is **history, not a gate**. PCRE2 10.48 is the baseline this repository is built
against and asserted at build time, so a construct marked reachable simply is reachable. The
column is kept only to answer the separate question "will this pattern also work somewhere I do
not control", and nothing here should be read as "you may not have this".

The rows with no probe are marked `unknown` rather than assumed. An index that guesses is worse
than one that admits a gap, because a guess is indistinguishable from a measurement once it is
written down.
"""

from __future__ import annotations

# Constructs that ripgrep's default engine accepts. Sourced from the REGEX SYNTAX section of the
# generated manual, which states the engine is Rust's `regex` with no look-around and no
# backreferences, and from the `regex-syntax` crate the build indexes alongside it.
DEFAULT_ENGINE_SUPPORTED = {
    "literal",
    "character-class",
    "negated-character-class",
    "unicode-property",
    "dot",
    "anchor-line",
    "anchor-haystack",
    "word-boundary",
    "alternation",
    "group-capturing",
    "group-non-capturing",
    "group-named",
    "quantifier-greedy",
    "quantifier-lazy",
    "repetition-bounded",
    "inline-flags",
    "escape-sequences",
    "class-nested-rust",
}

# Every row: construct, syntax, what it is for, and the probe that settles it where one exists.
# The version column is the release that introduced the construct upstream. It is history, never
# a gate: availability at the asserted baseline is decided by `reachable` and by the probe.
CONSTRUCTS = [
    ("literal", "abc", "match text exactly", "", "", "both"),
    ("character-class", "[a-z]", "match one of a set", "", "", "both"),
    ("negated-character-class", "[^a-z]", "match one outside a set", "", "", "both"),
    ("dot", ".", "any character except newline", "", "", "both"),
    ("anchor-line", "^ $", "start or end of a LINE, always, even under -U", "", "P020", "both"),
    ("anchor-haystack", "\\A \\z", "start or end of the whole haystack", "", "P020", "both"),
    ("word-boundary", "\\b", "zero-width word edge", "", "", "both"),
    ("alternation", "a|b", "either branch", "", "", "both"),
    ("group-capturing", "(...)", "group and capture", "", "", "both"),
    ("group-non-capturing", "(?:...)", "group without capturing", "", "", "both"),
    ("group-named", "(?<n>...)", "group under a name", "", "P006", "both"),
    ("quantifier-greedy", "a* a+ a?", "repeat, preferring more", "", "", "both"),
    ("quantifier-lazy", "a*? a+?", "repeat, preferring fewer", "", "", "both"),
    ("repetition-bounded", "a{2,5}", "repeat a bounded number of times", "", "", "both"),
    ("inline-flags", "(?i)", "set flags inside the pattern", "", "", "both"),
    ("unicode-property", "\\p{L}", "match by Unicode property", "", "P015", "both"),
    ("class-nested-rust", "[\\w&&[^0-9]]", "set algebra, Rust engine syntax", "", "", "default"),
    ("lookahead", "(?=...) (?!...)", "assert what follows, without consuming", "", "P002", "pcre2"),
    ("lookbehind", "(?<=...) (?<!...)", "assert what precedes", "", "P003", "pcre2"),
    (
        "variable-lookbehind",
        "(?<=.{0,20}x)",
        "lookbehind of varying width, bounded",
        "10.43",
        "P004",
        "pcre2",
    ),
    ("backreference", "\\1", "require a repeat of an earlier capture", "", "P005", "pcre2"),
    ("named-backreference", "\\k<n>", "require a repeat, by name", "", "P006", "pcre2"),
    ("match-reset", "\\K", "drop everything matched so far from the result", "", "P007", "pcre2"),
    ("possessive-quantifier", "a++ a*+", "repeat without giving back", "", "P008", "pcre2"),
    ("atomic-group", "(?>...)", "group that never backtracks into itself", "", "P009", "pcre2"),
    ("branch-reset", "(?|...)", "reuse capture numbers across alternatives", "", "P010", "pcre2"),
    ("conditional", "(?(1)a|b)", "branch on whether a group matched", "", "P011", "pcre2"),
    ("recursion", "(?R)", "match a balanced, nested construct", "", "P012", "pcre2"),
    ("subroutine", "(?&name)", "call a named group as a subpattern", "", "", "pcre2"),
    (
        "recursion-returned-captures",
        "(?&name(<cap>))",
        "return selected captures from a subroutine call to the caller",
        "10.47",
        "P013",
        "pcre2",
    ),
    (
        "control-verb-skip-fail",
        "(*SKIP)(*F)",
        "discard a region so later alternatives cannot match inside it",
        "",
        "P018",
        "pcre2",
    ),
    ("control-verb-commit", "(*COMMIT) (*PRUNE) (*THEN)", "control backtracking", "", "", "pcre2"),
    ("control-verb-accept", "(*ACCEPT) (*FAIL)", "end the match early", "", "", "pcre2"),
    ("script-run", "(*sr:...)", "require one Unicode script throughout", "", "P017", "pcre2"),
    (
        "extended-class-perl",
        "(?[A & B])",
        "set algebra over classes, and since 10.48 it composes with lookarounds",
        "10.45",
        "P015",
        "pcre2",
    ),
    (
        "extended-class-with-lookaround",
        "(?[\\d-[1]]).(?<!x)",
        "an extended class in the same pattern as a lookaround",
        "10.48",
        "P035",
        "pcre2",
    ),
    (
        "unicode-17-script",
        "\\p{Sidetic}",
        "match one of the four scripts Unicode 17.0 added",
        "10.48",
        "P036",
        "pcre2",
    ),
    (
        "extended-class-uts18",
        "[A&&[B]]",
        "set algebra, UTS#18 form",
        "10.45",
        "P016",
        "unreachable",
    ),
    (
        "scan-substring",
        "(*scs:(1)...)",
        "re-scan an already-captured substring",
        "10.45",
        "P014",
        "pcre2",
    ),
    ("newline-convention", "(*CRLF) (*ANYCRLF)", "set the newline convention", "", "", "pcre2"),
    ("bsr-control", "(*BSR_ANYCRLF)", "control what \\R matches", "", "", "pcre2"),
    ("no-jit", "(*NO_JIT)", "ask PCRE2 not to JIT this pattern", "", "", "pcre2"),
    (
        "resource-limits",
        "(*LIMIT_MATCH=n) (*LIMIT_DEPTH=n) (*LIMIT_HEAP=n)",
        "lower the engine's own limits from inside the pattern",
        "",
        "",
        "pcre2",
    ),
    ("callout", "(?C1)", "invoke a host callback mid-match", "", "", "unreachable"),
]

# PCRE2 capability that exists in the library but cannot be reached through the ripgrep CLI.
# This is the negative-space table: without it, an invisible limitation is indistinguishable
# from an absent capability.
UNREACHABLE = [
    (
        "UTS#18 extended classes",
        "pcre2",
        "no",
        "Requires the PCRE2_ALT_EXTENDED_CLASS compile option, which ripgrep never sets.",
        "Use the Perl-style (?[A & B]) form, which needs no option.",
    ),
    (
        "pcre2_substitute and its $+ replacement",
        "pcre2",
        "no",
        "ripgrep implements -r in its own printer layer and never calls PCRE2's substitution API.",
        "Use rg -r with $1 / $name, remembering it rewrites output only, never the file.",
    ),
    (
        "pcre2_next_match()",
        "pcre2",
        "no",
        "ripgrep owns match iteration.",
        "None needed at the CLI; relevant only when embedding PCRE2 directly.",
    ),
    (
        "variable-lookbehind maximum setter",
        "pcre2",
        "no",
        "The library default caps variable-length lookbehind at 255 characters and ripgrep "
        "exposes no setter.",
        "Rewrite the pattern using \\K, a capture, or a lookahead from an earlier anchor.",
    ),
    (
        "custom callout handler",
        "pcre2",
        "no",
        "ripgrep registers no callout callback, so (?C...) has nothing to call.",
        "Post-process rg --json output in the host program instead.",
    ),
    (
        "pcre2_set_optimize and the PCRE2_EXTRA_* options",
        "pcre2",
        "no",
        "Compile-time options such as PCRE2_EXTRA_TURKISH_CASING and PCRE2_EXTRA_PYTHON_OCTAL "
        "have no CLI mapping.",
        "Bind PCRE2 directly if one of these is genuinely required.",
    ),
    (
        "--regex-size-limit / --dfa-size-limit as PCRE2 guards",
        "rg",
        "partial",
        "Both flags bound the default engine only. Under -P they do not constrain PCRE2 at all.",
        "Use in-pattern (*LIMIT_MATCH=n) and (*LIMIT_DEPTH=n), plus process-level limits.",
    ),
    (
        "in-place file rewriting",
        "rg",
        "no",
        "-r rewrites printed output. ripgrep never modifies a file, by design.",
        "Use ast-grep -U for a structural rewrite, or a dedicated tool for a textual one.",
    ),
    (
        "partial-match substitution (PCRE2_ERROR_PARTIALSUBS)",
        "pcre2",
        "no",
        "New in 10.48: pcre2_substitute() gained partial-match support and a new error code. "
        "It is the only public API addition in the release, and ripgrep calls none of it.",
        "None at the CLI. rg -r rewrites printed output through its own printer layer.",
    ),
    (
        "semantic resolution",
        "both",
        "no",
        "Neither tool resolves imports, types, overloads or dispatch. ast-grep establishes "
        "syntax; ripgrep establishes lexical occurrence.",
        "Confirm candidates with a compiler, language server or type checker.",
    ),
]


def regex_rows(probe_results: list[dict]) -> list[tuple]:
    """Join the construct table to the probe verdicts."""
    by_id = {r["id"]: r for r in probe_results}
    rows = []
    for name, syntax, purpose, since_pcre2, probe_id, engines in CONSTRUCTS:
        probe = by_id.get(probe_id) if probe_id else None
        if probe is None:
            observed = "unknown" if engines != "both" else "not-probed"
        else:
            observed = probe["verdict"]

        rust_regex = "yes" if name in DEFAULT_ENGINE_SUPPORTED else "no"
        if engines == "default":
            pcre2, reachable = "no", "default-only"
        elif engines == "unreachable":
            pcre2, reachable = "yes", "no"
        elif engines == "both":
            pcre2, reachable = "yes", "yes"
        else:
            pcre2, reachable = "yes", "requires -P"

        rows.append(
            (name, syntax, rust_regex, pcre2, reachable, since_pcre2, probe_id, observed, purpose)
        )
    return rows


def unreachable_rows() -> list[tuple]:
    return list(UNREACHABLE)


def behavior_rows(probe_results: list[dict]) -> list[tuple]:
    """One row per executed probe: the question, the command, and what actually happened."""
    rows = []
    for record in probe_results:
        rows.append(
            (
                record["id"],
                record["tool"],
                record["topic"],
                record["verdict"],
                str(record["exit"]),
                record["question"],
                record["command"],
                record.get("stdout", "")[:200],
                record.get("control", ""),
                str(record.get("control_exit", "")),
            )
        )
    return rows

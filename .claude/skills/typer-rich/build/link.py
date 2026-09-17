"""Cross-reference the indexed API against the vendored corpus, structurally.

The naive way to link a class to an example is to search for its name. That conflates three
different things: a class actually being subclassed, a class being mentioned in a docstring, and
an unrelated identifier that happens to share a word. `Provider` alone appears in prose far more
often than it appears as a base.

ast-grep separates them, because it matches syntax rather than text. `class X(Provider):` is an
implementation; `Provider` inside a comment is not a node this pattern can match at all. So the
edges produced here are evidence, not coincidence.

Two limits, both deliberate and both worth knowing before trusting a page:

**A base is matched by name, not by identity.** ast-grep resolves no imports, so a class of
upstream's own that inherits an unrelated `Provider` would land here too. In this corpus that is
a theoretical rather than an observed problem -- the names are distinctive -- but a page saying
"demonstrated by N examples" means N *syntactic* demonstrations.

**Only direct bases are seen.** `class Approval(FastMCPApp)` is a `Provider` transitively, and
this returns it under `FastMCPApp`. The transitive closure lives in `descendants.tsv`; joining
the two is the caller's job, and `extension_points.py` does it.
"""

from __future__ import annotations

import json
import subprocess
from collections import Counter
from pathlib import Path

SUBCLASS = """class $NAME($$$BASES):
    $$$BODY"""

CALL = "$RECEIVER.$METHOD($$$ARGS)"

COMPOSITION = ("mount", "create_proxy", "add_provider", "add_transform", "add_middleware")


def _run(pattern: str, corpus: Path) -> list[dict]:
    """One ast-grep pass. Returns [] rather than raising when the corpus is absent."""
    if not corpus.is_dir():
        return []
    done = subprocess.run(
        ["ast-grep", "run", "--lang", "python", "--pattern", pattern, str(corpus), "--json=stream"],
        capture_output=True,
        text=True,
        check=False,
    )
    rows: list[dict] = []
    for line in done.stdout.splitlines():
        line = line.strip()
        if line:
            rows.append(json.loads(line))
    return rows


def _meta(row: dict, name: str) -> str:
    single = (row.get("metaVariables") or {}).get("single") or {}
    return (single.get(name) or {}).get("text", "")


def _multi(row: dict, name: str) -> list[str]:
    multiple = (row.get("metaVariables") or {}).get("multi") or {}
    return [entry.get("text", "") for entry in multiple.get(name, []) if entry.get("text")]


def subclass_sites(corpus: Path, content: Path) -> dict[str, list[str]]:
    """Return `{base leaf name: [corpus-relative files]}` for every subclass in the corpus.

    Grouping and deduplication happen here because ast-grep emits one record per match and
    cannot aggregate across them.
    """
    edges: dict[str, set[str]] = {}
    for row in _run(SUBCLASS, corpus):
        where = str(Path(row["file"]).resolve().relative_to(content.resolve()))
        for base in _multi(row, "BASES") or [_meta(row, "BASES")]:
            leaf = base.split("[")[0].split(".")[-1].strip()
            # `$$$BASES` reports the separating commas as matches too, so `,` arrived as a
            # base with three files on the first run. Only identifiers are bases.
            if leaf.isidentifier():
                edges.setdefault(leaf, set()).add(where)
    return {base: sorted(files) for base, files in sorted(edges.items())}


def registration_sites(corpus: Path, content: Path, queries: Path) -> dict[str, int]:
    """Return `{corpus rule id: match count}` for the shipped `corpus-*` rules.

    Running the rules rather than a second set of patterns is deliberate. The first version of
    this function carried its own decorator pattern, requiring a bare `@x.tool` on a plain
    `def`; upstream mostly writes `@mcp.tool(...)` on `async def`, so it reported 8 files where
    the shipped rule matches 2,028. A near-empty result that looks like a real one is precisely
    what these edges exist to avoid, and keeping one matcher per question is how that stays
    true as the rules improve.
    """
    config = queries / "sgconfig.yml"
    if not config.is_file() or not corpus.is_dir():
        return {}
    done = subprocess.run(
        [
            "ast-grep",
            "scan",
            "-c",
            str(config),
            "--filter",
            "^corpus-",
            str(corpus),
            "--json=stream",
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    counter: Counter[str] = Counter()
    for line in done.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        row = json.loads(line)
        rule = row.get("ruleId")
        if rule:
            counter[rule] += 1
    return dict(sorted(counter.items()))


def composition_counts(corpus: Path) -> dict[str, int]:
    """Return `{mount|create_proxy|add_*: count}`.

    Counts rather than files: the question these answer is "which composition route does
    upstream actually reach for", and one file calling `mount` forty times is forty answers.
    """
    counter: Counter[str] = Counter()
    for row in _run(CALL, corpus):
        method = _meta(row, "METHOD")
        if method in COMPOSITION:
            counter[method] += 1
    return dict(sorted(counter.items()))

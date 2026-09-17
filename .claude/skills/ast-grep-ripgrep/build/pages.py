"""Generate the topic pages, construct pages and catalogues from the indexes.

The curated half of each page -- the mental model, the decision rules, the anti-patterns -- is
seeded in `topics.json`, because no generator can write it. Everything else is joined from the
indexes at build time, so a flag table, a construct row or a probe result on a page cannot
disagree with what was actually built.

A seed that does not resolve fails the build rather than rendering a shorter page. A page that
quietly drops a flag it can no longer find would assert by omission that the capability is gone,
which is precisely the failure this repository exists to prevent.
"""

from __future__ import annotations

import json
from pathlib import Path


class SeedError(RuntimeError):
    """A seed referenced something the indexes do not contain."""


def read_tsv(path: Path) -> list[list[str]]:
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8")
    return [line.split("\t") for line in text.splitlines() if line]


def _table(headers: list[str], rows: list[list[str]]) -> str:
    if not rows:
        return "_None._\n"
    out = ["| " + " | ".join(headers) + " |", "|" + "|".join(["---"] * len(headers)) + "|"]
    for row in rows:
        cells = [(c or "").replace("|", "\\|") for c in row]
        out.append("| " + " | ".join(cells) + " |")
    return "\n".join(out) + "\n"


def _bullets(items: list[str]) -> str:
    return "\n".join(f"- {item}" for item in items) + "\n" if items else "_None recorded._\n"


# --------------------------------------------------------------------------- topics


def render_topics(build_dir: Path, content: Path) -> dict[str, int]:
    """Write `content/topics/` -- the router keyed by functional question."""
    seeds = json.loads((build_dir / "topics.json").read_text(encoding="utf-8"))["topics"]
    index = content / "index"
    flags = read_tsv(index / "flags.tsv")
    regex = read_tsv(index / "regex.tsv")
    behaviors = read_tsv(index / "behaviors.tsv")

    by_construct = {row[0]: row for row in regex}
    by_probe = {row[0]: row for row in behaviors}

    topics_dir = content / "topics"
    topics_dir.mkdir(parents=True, exist_ok=True)
    written = 0

    for seed in seeds:
        _validate_seed(seed, by_construct, by_probe)
        (topics_dir / f"{seed['slug']}.md").write_text(
            _render_topic(seed, flags, by_construct, by_probe), encoding="utf-8"
        )
        written += 1

    (topics_dir / "00-map.md").write_text(_render_map(seeds), encoding="utf-8")
    return {"topics": written + 1}


def _validate_seed(seed: dict, by_construct: dict, by_probe: dict) -> None:
    for name in seed.get("constructs", []):
        if name not in by_construct:
            raise SeedError(
                f"topic {seed['slug']!r} names construct {name!r}, which is absent from "
                f"regex.tsv. Either the construct was renamed upstream or the seed is stale; "
                f"a page that silently omitted it would imply the construct does not exist."
            )
    for probe_id in seed.get("probes", []):
        if probe_id not in by_probe:
            raise SeedError(
                f"topic {seed['slug']!r} cites probe {probe_id!r}, which did not run. "
                f"Citing evidence that does not exist is worse than citing none."
            )


def _render_topic(seed: dict, flags: list[list[str]], by_construct: dict, by_probe: dict) -> str:
    parts = [
        f"# {seed['title']}\n\n",
        f"**{seed['question']}**\n\n",
        seed["mental_model"] + "\n",
    ]

    categories = seed.get("flag_categories", [])
    if categories:
        rows = [
            [r[0], ("--" + r[2]) if r[2] else "", ("-" + r[3]) if r[3] else "", r[7]]
            for r in flags
            if r[4] in categories
        ]
        parts.append("\n## Flags\n\n")
        parts.append(
            f"Every flag upstream files under {', '.join(categories)}. "
            f"The grouping is ripgrep's own, taken from `rg --help`.\n\n"
        )
        parts.append(_table(["tool", "long", "short", "what it does"], sorted(rows)))

    constructs = seed.get("constructs", [])
    if constructs:
        rows = []
        for name in constructs:
            row = by_construct[name]
            rows.append([row[0], f"`{row[1]}`", row[2], row[4], row[5] or "-", row[7]])
        parts.append("\n## Constructs\n\n")
        parts.append(
            "`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can "
            "actually get at it, which is narrower than whether PCRE2 supports it. `observed` "
            "is the verdict of an executed probe, not a reading of a version string.\n\n"
        )
        parts.append(
            _table(["construct", "syntax", "default", "reachable", "since pcre2", "observed"], rows)
        )

    probe_ids = seed.get("probes", [])
    if probe_ids:
        rows = [[p, by_probe[p][3], by_probe[p][5], f"`{by_probe[p][6]}`"] for p in probe_ids]
        parts.append("\n## Evidence\n\n")
        parts.append(
            "Each row was executed against the fixture tree at build time. `confirmed` means "
            "the probe came out as expected **and** its control came out the other way.\n\n"
        )
        parts.append(_table(["probe", "verdict", "question", "command"], rows))

    parts.append("\n## Decision rules\n\n")
    parts.append(_bullets(seed.get("decision_rules", [])))
    parts.append("\n## Anti-patterns\n\n")
    parts.append(_bullets(seed.get("anti_patterns", [])))
    parts.append("\n## Checklist\n\n")
    parts.append(_bullets(seed.get("checklist", [])))
    return "".join(parts)


def _render_map(seeds: list[dict]) -> str:
    parts = [
        "# Capability map\n",
        "\nRouting is by the question you arrived with, not by tool. Find the row that matches "
        "what you are trying to do; the page names the flags, constructs and recorded "
        "behaviour for it.\n",
    ]
    for group, heading in (
        ("cross-cutting", "Either tool, or both"),
        ("lexical", "Text search — ripgrep"),
        ("structural", "Syntax search and rewriting — ast-grep"),
    ):
        rows = [
            [f"[{s['title']}]({s['slug']}.md)", s["question"]] for s in seeds if s["group"] == group
        ]
        parts.append(f"\n## {heading}\n\n")
        parts.append(_table(["Topic", "Answers"], rows))
    return "".join(parts)


# --------------------------------------------------------------------------- constructs


def render_constructs(content: Path) -> dict[str, int]:
    """Write one page per regex construct: what it constrains, and what it does not."""
    index = content / "index"
    regex = read_tsv(index / "regex.tsv")
    behaviors = {row[0]: row for row in read_tsv(index / "behaviors.tsv")}

    target = content / "constructs"
    target.mkdir(parents=True, exist_ok=True)
    for row in regex:
        name, syntax, rust_regex, pcre2, reachable, since_pcre2, probe_id, observed, purpose = row
        probe = behaviors.get(probe_id)
        body = [
            f"# `{name}`\n",
            f"\n`{syntax}`\n",
            f"\n{purpose.capitalize()}.\n",
            "\n## Availability\n\n",
            _table(
                ["engine", "answer"],
                [
                    ["ripgrep default (Rust regex)", rust_regex],
                    ["PCRE2", pcre2],
                    ["reachable through the rg CLI", reachable],
                    ["introduced in PCRE2", since_pcre2 or "not version-gated"],
                    ["established by", f"probe {probe_id}" if probe_id else "not probed"],
                    ["observed", observed],
                ],
            ),
        ]
        if probe:
            body.append("\n## Evidence\n\n")
            body.append(f"{probe[5]}\n\n```\n{probe[6]}\n```\n\n")
            body.append(f"Exit {probe[4]}, verdict `{probe[3]}`.\n")
            if probe[8]:
                body.append(
                    f"\nControl (must come out the other way):\n\n```\n{probe[8]}\n```\n\n"
                    f"Exit {probe[9]}.\n"
                )
        else:
            body.append(
                "\n## Evidence\n\nNo probe covers this construct, so availability here is "
                "**unknown rather than asserted**. Write a probe with a control before "
                "depending on it.\n"
            )
        if reachable == "no":
            body.append(
                "\n## Not reachable\n\nThis construct exists in the library but cannot be "
                "reached through the ripgrep CLI. See `../index/unreachable.tsv` for the reason "
                "and the alternative.\n"
            )
        (target / f"{name}.md").write_text("".join(body), encoding="utf-8")
    return {"constructs": len(regex)}


# --------------------------------------------------------------------------- catalogues


def render_catalogs(content: Path) -> dict[str, int]:
    """Write the direct-lookup tables that answer without an escalation ladder."""
    index = content / "index"
    target = content / "catalogs"
    target.mkdir(parents=True, exist_ok=True)
    written = 0

    flags = read_tsv(index / "flags.tsv")
    groups: dict[str, list[list[str]]] = {}
    for row in flags:
        groups.setdefault(row[4] or f"{row[1]} options", []).append(row)
    parts = [
        "# Flag families\n",
        "\nEvery flag of both tools, grouped as upstream groups them. ripgrep's categories come "
        "from `rg --help`; ast-grep's are per subcommand, because its help is organised that "
        "way.\n",
    ]
    for name in sorted(groups):
        rows = [
            [r[0], ("--" + r[2]) if r[2] else "", ("-" + r[3]) if r[3] else "", r[5], r[6], r[7]]
            for r in sorted(groups[name])
        ]
        parts.append(f"\n## {name}\n\n")
        parts.append(_table(["tool", "long", "short", "arg", "values", "what it does"], rows))
    (target / "flag-families.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    regex = read_tsv(index / "regex.tsv")
    parts = [
        "# Regex construct matrix\n",
        "\nWhat each engine supports, and how that was established. `observed` is the verdict "
        "of an executed probe with a control; `unknown` means no probe covers it and nothing "
        "should be inferred either way.\n",
        "\nPCRE2 10.48 is the asserted baseline, so `since pcre2` is history rather than a "
        "gate: a construct marked reachable is reachable. **Do not read `rg --pcre2-version` "
        "to decide otherwise** -- it prints a constant from ripgrep's own build and reads "
        "10.45 here while the linked library is 10.48.\n\n",
        _table(
            [
                "construct",
                "syntax",
                "default",
                "pcre2",
                "reachable",
                "min pcre2",
                "probe",
                "observed",
            ],
            [[r[0], f"`{r[1]}`", r[2], r[3], r[4], r[5] or "-", r[6] or "-", r[7]] for r in regex],
        ),
    ]
    (target / "regex-matrix.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    unreachable = read_tsv(index / "unreachable.tsv")
    parts = [
        "# Not reachable from the CLI\n",
        "\nCapability that exists in the underlying libraries and cannot be reached through "
        "these command-line tools. This page exists because an invisible limitation is "
        "indistinguishable from an absent capability: without it, an agent that fails to find "
        "a way to do one of these things cannot tell whether it looked in the wrong place.\n\n",
        _table(["capability", "exists in", "reachable", "why not", "instead"], unreachable),
    ]
    (target / "not-reachable.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    exits = read_tsv(index / "exit-codes.tsv")
    parts = [
        "# Exit codes\n",
        "\nThe two tools do not share an exit vocabulary, and in one case the status carries no "
        "information at all. Both facts break scripts quietly.\n\n",
        _table(["tool", "command", "code", "means", "trap"], exits),
    ]
    (target / "exit-codes.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    languages = read_tsv(index / "languages.tsv")
    accepted = [r for r in languages if r[1] == "accepted"]
    rejected = [r for r in languages if r[1] == "rejected"]
    parts = [
        "# Languages and node kinds\n",
        "\nast-grep does not enumerate its languages -- `--help` links to a web page. So this "
        "table was built by **probing the installed binary** with candidate names.\n",
        f"\n{len(accepted)} accepted, {len(rejected)} rejected by this binary.\n\n",
        _table(
            ["language", "status", "kinds", "fields", "rule schema"],
            sorted(accepted, key=lambda r: r[0]),
        ),
        "\n## Rejected by this binary\n\n",
        "These candidate names were probed and refused. That is evidence about this build, not "
        "proof that no such parser exists anywhere.\n\n",
        "`" + "`, `".join(sorted(r[0] for r in rejected)) + "`\n",
        "\n## The two kind catalogues disagree\n\n",
        "ast-grep publishes node kinds twice, in `schemas/languages.json` and in each "
        "`schemas/<lang>_rule.json`, and the two are not consistent. Disputed kinds were "
        "adjudicated by running them: `ast-grep run -k <kind>` exits 8 for a kind it cannot "
        "parse. Three published kinds turned out to be rejected -- see the `rejected` rows of "
        "`../index/kinds.tsv`. A rule using one of them fails rather than matching nothing.\n",
    ]
    (target / "languages.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    types = read_tsv(index / "file-types.tsv")
    parts = [
        "# File types\n",
        f"\nThe {len(types)} type definitions this ripgrep knows, usable as `-t<name>` to "
        "include and `-T<name>` to exclude. Extend with `--type-add`.\n\n",
        _table(["type", "globs"], types),
    ]
    (target / "file-types.md").write_text("".join(parts), encoding="utf-8")
    written += 1

    return {"catalogs": written}

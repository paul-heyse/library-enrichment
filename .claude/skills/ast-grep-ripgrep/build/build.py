"""Build the capability repository from the acquired inputs and the installed binaries.

Offline by construction: this module never imports `acquire`, so a rebuild reads only
`acquired/` and the two pinned binaries. That is what lets `verify.py` rebuild and compare
byte-for-byte -- if acquisition could run from here, a rebuild could pick up different bytes
from the network and still look deterministic.

    python3 build/build.py              full build
    python3 build/build.py --stage index    indexes only, skip corpus and pages

The binaries are not an optional convenience. Three of the indexes are observations rather than
readings -- the probe results, the adjudicated node kinds, and the accepted language roster --
and none can be produced from documentation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import sys
from pathlib import Path

import api
import oracles
import pages
import probes
import regexmap
import surfaces

BUILD_DIR = Path(__file__).resolve().parent
SKILL_DIR = BUILD_DIR.parent
ACQUIRED = BUILD_DIR / "acquired"
CONTENT = SKILL_DIR / "content"


class BuildError(RuntimeError):
    """The build cannot proceed without an input it refuses to guess at."""


def _assert_pins(manifest: dict, versions: dict) -> None:
    """Refuse to build unless the installed tools and the linked PCRE2 are the pinned ones.

    A repository built from 0.45.2 while claiming 0.45.3 would be wrong in exactly the way this
    skill exists to prevent, and the probes would be describing a different program from the
    schemas.

    PCRE2 is asserted the same way, which is what makes 10.48 a baseline rather than a guess.
    It cannot be checked against `rg --pcre2-version`: that string is a constant from ripgrep's
    own build and reads three releases stale here. PCRE2's version conditional is evaluated
    inside libpcre2, so it answers for the library that will actually run.
    """
    expected_ag = manifest["tools"]["ast_grep_version"]
    expected_rg = manifest["tools"]["ripgrep_version"]
    baseline = manifest["tools"]["pcre2_baseline"]
    if expected_ag not in str(versions["ast_grep"]):
        raise BuildError(
            f"manifest pins ast-grep {expected_ag} but {versions['ast_grep']} is installed"
        )
    if expected_rg not in str(versions["ripgrep"]):
        raise BuildError(
            f"manifest pins ripgrep {expected_rg} but {versions['ripgrep']} is installed"
        )
    if not oracles.pcre2_asserts(baseline):
        raise BuildError(
            f"the PCRE2 linked by ripgrep does not assert as {baseline}. This is checked with "
            f"PCRE2's own version conditional, which is evaluated inside libpcre2 itself, so it "
            f"reports the library actually in use -- not `rg --pcre2-version`, which on this "
            f"machine reads {versions['pcre2_build_string']} and describes ripgrep's build "
            f"instead. Resolution says: {versions['pcre2_resolution']} "
            f"{versions['pcre2_linked_library'] or '(unresolved)'}"
        )


def build_indexes(manifest: dict) -> tuple[dict[str, int], dict]:
    """Write every `content/index/*.tsv` and return the row counts plus the observations."""
    index = CONTENT / "index"
    schemas = ACQUIRED / "schemas"
    counts: dict[str, int] = {}

    versions = oracles.tool_versions()
    _assert_pins(manifest, versions)

    probe_results = probes.run_all(BUILD_DIR)

    counts["flags"] = surfaces.write_tsv(index / "flags.tsv", surfaces.flag_rows())
    counts["file-types"] = surfaces.write_tsv(index / "file-types.tsv", surfaces.file_type_rows())
    counts["exit-codes"] = surfaces.write_tsv(index / "exit-codes.tsv", surfaces.exit_code_rows())
    counts["rule-fields"] = surfaces.write_tsv(
        index / "rule-fields.tsv", surfaces.rule_field_rows(schemas)
    )
    counts["fields"] = surfaces.write_tsv(index / "fields.tsv", surfaces.field_rows(schemas))

    kinds, kind_stats = surfaces.kind_rows(schemas)
    counts["kinds"] = surfaces.write_tsv(index / "kinds.tsv", kinds)

    probed_languages = oracles.ast_grep_languages()
    counts["languages"] = surfaces.write_tsv(
        index / "languages.tsv", surfaces.language_rows(schemas, probed_languages)
    )

    counts["regex"] = surfaces.write_tsv(index / "regex.tsv", regexmap.regex_rows(probe_results))
    counts["unreachable"] = surfaces.write_tsv(
        index / "unreachable.tsv", regexmap.unreachable_rows()
    )
    counts["behaviors"] = surfaces.write_tsv(
        index / "behaviors.tsv", regexmap.behavior_rows(probe_results)
    )

    observations = {
        "versions": versions,
        "probe_summary": probes.summarise(probe_results),
        "kind_adjudication": kind_stats,
        "languages_accepted": sorted(n for n, ok in probed_languages.items() if ok),
        "languages_rejected": sorted(n for n, ok in probed_languages.items() if not ok),
    }
    return counts, observations


def build_api() -> dict[str, int]:
    """Index the library crates and the bindings into `api/`, `model/` and the symbol indexes."""
    index = CONTENT / "index"
    versions = json.loads((ACQUIRED / "CRATE_VERSIONS.json").read_text(encoding="utf-8"))
    models = api.build_models(BUILD_DIR / ".cache", versions)
    emitted = api.emit_api(models, CONTENT)

    counts = {"crates": len(models), "modules": emitted["modules"]}
    for name in ("symbols", "methods", "aliases", "impls", "unresolved"):
        counts[name] = surfaces.write_tsv(index / f"{name}.tsv", emitted[name])
    counts["bindings"] = surfaces.write_tsv(index / "bindings.tsv", api.binding_rows(ACQUIRED))
    return counts


def build_corpus() -> dict[str, int]:
    """Copy the vendored upstream material into `content/corpus/`, verbatim."""
    source = ACQUIRED / "corpus"
    target = CONTENT / "corpus"
    if target.exists():
        shutil.rmtree(target)
    if not source.exists():
        raise BuildError("acquired/corpus is missing. Run build/acquire.py first.")
    shutil.copytree(source, target)

    # The two source files an agent reads most often when a flag's behaviour is in question.
    reference = target / "ripgrep"
    reference.mkdir(parents=True, exist_ok=True)
    for name in ("GUIDE.md", "FAQ.md", "CHANGELOG.md", "crates/core/flags/defs.rs"):
        origin = ACQUIRED / "ripgrep" / name
        if origin.exists():
            destination = reference / Path(name).name
            destination.write_bytes(origin.read_bytes())

    for name in ("CHANGELOG.md",):
        origin = ACQUIRED / "ast-grep" / name
        if origin.exists():
            (target / "ast-grep" / name).write_bytes(origin.read_bytes())

    # The generated manual, captured from the binary rather than from a shipped file.
    (target / "ripgrep" / "rg.1.roff").write_text(oracles.ripgrep_man(), encoding="utf-8")

    counts: dict[str, int] = {}
    for child in sorted(target.iterdir()):
        if child.is_dir():
            counts[child.name] = sum(1 for p in child.rglob("*") if p.is_file())
    return counts


def write_provenance(manifest: dict, counts: dict, observations: dict, corpus: dict) -> None:
    """Record the pins, the counts, and every file's digest.

    The PCRE2 entry is the one to read carefully. `build_string` is what the binary prints;
    `probed_floor` is what execution demonstrates. Where they disagree the disagreement is
    published rather than reconciled, because an agent that trusts the string will conclude a
    working construct is unavailable.
    """
    acquisition = json.loads((ACQUIRED / "ACQUISITION.json").read_text(encoding="utf-8"))
    versions = observations["versions"]

    digests: dict[str, str] = {}
    for path in sorted(CONTENT.rglob("*")):
        if path.is_file() and path.name != "PROVENANCE.json":
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            digests[str(path.relative_to(CONTENT))] = digest

    document = {
        "repository": manifest["repository"],
        "generated_at": manifest.get("generated_at", ""),
        "tools": {
            "ast_grep": versions["ast_grep"],
            "ripgrep": versions["ripgrep"],
            "ripgrep_features": versions["ripgrep_features"],
            "python": sys.version.split()[0],
        },
        "pcre2": {
            "baseline": manifest["tools"]["pcre2_baseline"],
            "baseline_asserted": True,
            "linked_library": versions["pcre2_linked_library"],
            "linked_version": versions["pcre2_linked_version"],
            "unicode_version": versions["pcre2_unicode_version"],
            "resolution": versions["pcre2_resolution"],
            "build_string": versions["pcre2_build_string"],
            "build_string_raw": versions["pcre2_build_string_raw"],
            "docs_tag": manifest["tools"]["pcre2_docs_tag"],
            "jit": versions["pcre2_jit"],
            "note": (
                "Four readings, deliberately not reconciled. `baseline` is asserted with PCRE2's "
                "own version conditional, evaluated inside libpcre2, so it answers for the "
                "library that runs. `linked_version` is read from that library's own bytes. "
                "`build_string` is what ripgrep reports and is three releases stale here, "
                "because it is a constant from ripgrep's build rather than a measurement. An "
                "agent that reads the build string will conclude a working construct is "
                "unavailable; that is why the wrong number is kept beside the right ones."
            ),
        },
        "pins": {
            "repos": {r["repo"]: r["ref"] for r in acquisition["repos"]},
            "crates": acquisition["crates"],
        },
        "counts": {"index": counts, "corpus": corpus},
        "observations": {
            "probe_summary": observations["probe_summary"],
            "kind_adjudication": observations["kind_adjudication"],
            "languages_accepted": observations["languages_accepted"],
            "languages_rejected": observations["languages_rejected"],
        },
        "files": digests,
    }
    (CONTENT / "PROVENANCE.json").write_text(
        json.dumps(document, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stage", choices=("all", "index"), default="all")
    parser.add_argument("--manifest", default="manifests/ast-grep-ripgrep.json")
    args = parser.parse_args(argv)

    manifest_path = BUILD_DIR / args.manifest
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))

    if not ACQUIRED.exists():
        raise BuildError("build/acquired is missing. Run `python3 build/acquire.py` first.")

    CONTENT.mkdir(parents=True, exist_ok=True)
    counts, observations = build_indexes(manifest)

    page_counts: dict[str, int] = {}
    corpus: dict[str, int] = {}
    if args.stage == "all":
        page_counts.update(build_api())
        page_counts.update(pages.render_topics(BUILD_DIR, CONTENT))
        page_counts.update(pages.render_constructs(CONTENT))
        page_counts.update(pages.render_catalogs(CONTENT))
        corpus = build_corpus()

    write_provenance(manifest, counts | page_counts, observations, corpus)

    summary = {
        "index": counts,
        "pages": page_counts,
        "corpus": corpus,
        "probes": observations["probe_summary"],
        "pcre2": {
            "baseline": manifest["tools"]["pcre2_baseline"],
            "build_string": observations["versions"]["pcre2_build_string"],
            "linked_version": observations["versions"]["pcre2_linked_version"],
            "unicode_version": observations["versions"]["pcre2_unicode_version"],
        },
        "kinds": observations["kind_adjudication"],
    }
    sys.stdout.write(json.dumps(summary, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

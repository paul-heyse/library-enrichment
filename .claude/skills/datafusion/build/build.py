"""Build the DataFusion capability repository from pinned upstream sources.

Usage:
    python3 build.py [--manifest manifests/datafusion.json] [--stage model|content|all]

Standard library only, plus the `ast-grep` binary for the structural stages. Every input is
pinned by the manifest and cached, so a rebuild is reproducible and performs no network I/O.

Progress goes to stderr so that stdout stays free for a machine-readable summary.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

import catalogs
import emit
import fetch
import link
import model
import queries
import topics
import traits

HERE = Path(__file__).resolve().parent
SKILL_ROOT = HERE.parent
CONTENT = SKILL_ROOT / "content"
CACHE = HERE / ".cache"


def say(message: str) -> None:
    sys.stderr.write(message + "\n")
    sys.stderr.flush()


def load_manifest(path: Path) -> dict:
    return json.loads(path.read_text())


def crate_versions(manifest: dict) -> list[tuple[str, str]]:
    pairs: list[tuple[str, str]] = []
    for crate_set in manifest["crate_sets"]:
        version = crate_set["version"]
        pairs.extend((name, version) for name in crate_set["crates"])
    return pairs


def ast_grep_version() -> str:
    try:
        done = subprocess.run(
            ["ast-grep", "--version"], capture_output=True, text=True, check=False
        )
    except FileNotFoundError:
        return "absent"
    return done.stdout.strip() or "unknown"


def build_models(manifest: dict, cache: fetch.Cache) -> dict[str, model.CrateModel]:
    supported = manifest["tools"]["rustdoc_format_versions"]
    models: dict[str, model.CrateModel] = {}
    for name, version in crate_versions(manifest):
        payload = fetch.rustdoc_json(cache, name, version)
        record = model.build_crate(name, version, payload, supported)
        models[name] = record
        say(
            f"  {name:38} v{version:8} items={len(record.items):>5} "
            f"aliases={len(record.aliases):>4} globs={len(record.globs):>3}"
        )
    return models


def collect_crate_facts(manifest: dict, cache: fetch.Cache) -> dict[str, dict]:
    """Read each crate's published manifest for features and docs.rs build configuration."""
    facts: dict[str, dict] = {}
    for name, version in crate_versions(manifest):
        entry: dict = {"version": version}
        try:
            entry.update(fetch.parse_manifest_facts(fetch.crate_manifest(cache, name, version)))
        except fetch.FetchError as error:
            entry["error"] = str(error)
            say(f"  ! {name}: {error}")
        facts[name] = entry
    return facts


def write_corpora(manifest: dict, cache: fetch.Cache, content: Path) -> dict[str, int]:
    """Copy the upstream example and guide corpora in verbatim, for structural querying."""
    written: dict[str, int] = {}
    for corpus in manifest.get("corpora", []):
        destination = content / corpus["dest"]
        destination.mkdir(parents=True, exist_ok=True)
        files = fetch.repo_files(
            cache,
            corpus["repo"],
            corpus["ref"],
            corpus["source_prefix"],
            tuple(corpus["suffixes"]),
        )
        for relative, payload in files.items():
            target = destination / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(payload)
        written[corpus["name"]] = len(files)
        say(f"  corpus {corpus['name']:18} {len(files):>4} files -> {corpus['dest']}")
    return written


def module_page(item: model.Item) -> str:
    return emit.api_file(item.module)


def _clean(text: str) -> str:
    return text.replace("\t", " ").replace("\n", " ")


def write_indexes(
    items: dict[str, model.Item], facts: dict[str, dict], out: Path
) -> dict[str, int]:
    """Write the line-oriented projection used for fast existence checks."""
    out.mkdir(parents=True, exist_ok=True)
    counts: dict[str, int] = {}

    symbol_rows = [
        "\t".join(
            (
                item.path,
                item.kind,
                item.crate,
                module_page(item),
                str(len(item.aliases)),
                str(len(item.methods)),
                _clean(item.summary),
            )
        )
        for item in items.values()
    ]
    (out / "symbols.tsv").write_text("\n".join(sorted(symbol_rows)) + "\n")
    counts["symbols"] = len(symbol_rows)

    method_rows = {
        "\t".join(
            (
                item.path,
                method.name,
                method.via_trait or "-",
                _clean(method.signature),
                _clean(method.summary),
            )
        )
        for item in items.values()
        for method in item.methods
        if not (method.via_trait and emit.is_ubiquitous(method.via_trait))
    }
    (out / "methods.tsv").write_text("\n".join(sorted(method_rows)) + "\n")
    counts["methods"] = len(method_rows)

    impl_rows = {
        "\t".join((item.path, implementor, items[implementor].crate))
        for item in items.values()
        for implementor in item.implementors
        if implementor in items
    }
    (out / "impls.tsv").write_text("\n".join(sorted(impl_rows)) + "\n")
    counts["impls"] = len(impl_rows)

    alias_rows = {
        "\t".join((alias, item.path, item.kind))
        for item in items.values()
        for alias in item.aliases
    }
    (out / "aliases.tsv").write_text("\n".join(sorted(alias_rows)) + "\n")
    counts["aliases"] = len(alias_rows)

    # Feature gating is absent from rustdoc JSON entirely, so this table is the only evidence an
    # agent has about what a feature flag turns on. Keeping it separate makes that provenance
    # obvious rather than implying a signature could have carried it.
    feature_rows = {
        "\t".join(
            (
                crate,
                feature,
                ",".join(enables) or "-",
                "default" if feature in (entry.get("default_features") or []) else "-",
            )
        )
        for crate, entry in facts.items()
        for feature, enables in (entry.get("features") or {}).items()
    }
    (out / "features.tsv").write_text("\n".join(sorted(feature_rows)) + "\n")
    counts["features"] = len(feature_rows)

    return counts


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Build the DataFusion capability repository.")
    parser.add_argument("--manifest", type=Path, default=HERE / "manifests" / "datafusion.json")
    parser.add_argument("--stage", choices=("model", "content", "all"), default="all")
    parser.add_argument("--content", type=Path, default=CONTENT)
    args = parser.parse_args(argv)

    manifest = load_manifest(args.manifest)
    cache = fetch.Cache(CACHE)
    content = args.content

    pairs = crate_versions(manifest)
    say(f"Building {manifest['repository']['name']} from {len(pairs)} crates")
    models = build_models(manifest, cache)

    items, alias_index, unresolved = model.stitch(models)
    say(f"\nstitched: {len(items)} canonical items, {len(alias_index)} carrying aliases")
    if unresolved:
        say(f"unresolved access paths: {len(unresolved)} (targets outside the pinned set)")

    say("\nreading crate manifests for features and docs.rs configuration")
    facts = collect_crate_facts(manifest, cache)

    counts = write_indexes(items, facts, content / "index")
    say("index rows: " + ", ".join(f"{k}={v}" for k, v in sorted(counts.items())))

    if args.stage == "model":
        sys.stdout.write(json.dumps({"canonical_items": len(items), "index": counts}) + "\n")
        return 0

    grouped = emit.group_by_module(items)
    say(f"\nwriting {len(grouped)} module pages")
    emit.write_model(grouped, content)
    emit.write_api(grouped, content)

    say("fetching corpora")
    corpora = write_corpora(manifest, cache, content)

    say("deriving symbol-to-example edges with ast-grep")
    examples_root = content.joinpath("corpus", "examples")
    try:
        demonstrated = link.trait_implementations(examples_root, content)
        registered = link.registrations(examples_root)
        say(
            f"  {len(demonstrated)} traits implemented in the corpus, "
            f"{len(registered)} distinct registration calls"
        )
    except link.AstGrepMissing as error:
        # Structural linking is a real capability, not a nice-to-have. Say so rather than
        # emitting pages that silently claim no example demonstrates anything.
        say(f"  ! skipped: {error}")
        demonstrated, registered = {}, {}

    say("writing extension-point pages")
    trait_pages = traits.write_traits(items, content, demonstrated)
    say(f"  {trait_pages} pages")

    say("restructuring the generated catalogs")
    settings = catalogs.write_config_options(items, content)
    functions = catalogs.write_sql_functions(content)
    crate_rows = catalogs.write_crate_map(items, content)
    say(f"  {settings} settings, {functions} SQL functions, {crate_rows} crates")

    say("writing capability topic pages")
    topic_pages = topics.write_topics(items, content, HERE.joinpath("topics.json"))
    say(f"  {topic_pages} topics")

    say("generating rules from the model")
    crate_roots = sorted({name.replace("-", "_") for name, _ in pairs})
    generated = queries.generate(content / emit.MODEL_DIR, SKILL_ROOT / "queries", crate_roots)
    for rule_id, size in generated.items():
        say(f"  {rule_id}: {size} entries")

    tool_versions = {
        "ast_grep": ast_grep_version(),
        "python": sys.version.split()[0],
        "rustdoc_format_versions": manifest["tools"]["rustdoc_format_versions"],
    }
    totals = {
        "crates": len(models),
        "canonical_items": len(items),
        "modules": len(grouped),
        "unresolved_access_paths": len(unresolved),
        **counts,
        **{f"corpus_{k}": v for k, v in corpora.items()},
        **{f"generated_{k}": v for k, v in generated.items()},
        "trait_pages": trait_pages,
        "topic_pages": topic_pages,
        "config_settings": settings,
        "sql_functions": functions,
        "traits_demonstrated_by_examples": len(demonstrated),
        "registration_calls_demonstrated": len(registered),
    }
    emit.write_provenance(content, manifest, facts, totals, tool_versions)

    say("\nwrote PROVENANCE.json")
    sys.stdout.write(json.dumps(totals, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

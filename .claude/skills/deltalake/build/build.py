"""Build the delta-rs capability repository from pinned upstream sources.

Usage:
    python3 build.py [--manifest <manifest>] [--stage model|content|all]

Standard library only, plus the `ast-grep` binary for the structural stages. Every input is
pinned by the manifest and cached, so a rebuild is reproducible and performs no network I/O.
A crate set sourced from git is read from `acquired/`, which `acquire.py` produced; this stage
never runs cargo, so that `verify.py` can re-run it to prove determinism.

Progress goes to stderr so that stdout stays free for a machine-readable summary.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path

import capabilities
import catalogs
import contracts
import emit
import fetch
import inputs
import link
import model
import queries
import routes
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


def crate_specs(manifest: dict) -> list[dict]:
    """Normalise every crate entry to one record.

    A `crates` entry may be a bare string, as a docs.rs crate set writes it, or an object
    carrying a path and the feature envelope it was documented under. Both shapes come out the
    same here so that nothing downstream has to know which source a crate came from.
    """
    specs: list[dict] = []
    for crate_set in manifest["crate_sets"]:
        for entry in crate_set["crates"]:
            spec = {"package": entry} if isinstance(entry, str) else dict(entry)
            spec["version"] = crate_set["version"]
            spec["source"] = crate_set.get("source", "docs_rs")
            spec["crate_set"] = crate_set
            spec["lib"] = spec.get("lib") or spec["package"].replace("-", "_")
            specs.append(spec)
    return specs


def crate_versions(manifest: dict) -> list[tuple[str, str]]:
    return [(spec["package"], spec["version"]) for spec in crate_specs(manifest)]


def acquired_root(crate_set: dict) -> Path:
    """Locate the directory `acquire.py` wrote for this crate set, by recomputing its key."""
    import acquire

    return acquire.acquired_dir(crate_set)


def ast_grep_version() -> str:
    try:
        done = subprocess.run(
            ["ast-grep", "--version"], capture_output=True, text=True, check=False
        )
    except FileNotFoundError:
        return "absent"
    return done.stdout.strip() or "unknown"


def build_models(manifest: dict, cache: fetch.Cache) -> dict[str, model.CrateModel]:
    """Turn every crate's rustdoc JSON into a canonical item table.

    Two sources, one seam. `model.build_crate` takes bytes and asks nothing about where they
    came from, so a git-pinned crate documented by `acquire.py` and a published crate fetched
    from docs.rs are indistinguishable from here on.
    """
    supported = manifest["tools"]["rustdoc_format_versions"]
    models: dict[str, model.CrateModel] = {}
    records: dict[int, dict] = {}
    for spec in crate_specs(manifest):
        name, version = spec["package"], spec["version"]
        supplement = None
        if spec["source"] == "git":
            directory = acquired_root(spec["crate_set"])
            key = id(spec["crate_set"])
            if key not in records:
                records[key] = fetch.acquisition(directory)
            payload = fetch.rustdoc_json_local(directory, records[key], spec["lib"])
            supplement = fetch.rustdoc_supplement_local(directory, records[key], spec["lib"])
        else:
            payload = fetch.rustdoc_json(cache, name, version)
        record = model.build_crate(name, version, payload, supported, supplement)
        models[name] = record
        say(
            f"  {name:26} {version:16} items={len(record.items):>6} "
            f"aliases={len(record.aliases):>5} globs={len(record.globs):>3}"
        )
    return models


def collect_crate_facts(manifest: dict, cache: fetch.Cache) -> dict[str, dict]:
    """Read each crate's manifest for its feature table and build configuration."""
    facts: dict[str, dict] = {}
    workspaces: dict[int, str] = {}
    for spec in crate_specs(manifest):
        name, version = spec["package"], spec["version"]
        entry: dict = {"version": version, "source": spec["source"]}
        try:
            if spec["source"] == "git":
                directory = acquired_root(spec["crate_set"])
                key = id(spec["crate_set"])
                if key not in workspaces:
                    root = directory / "manifests" / "WORKSPACE.Cargo.toml"
                    workspaces[key] = root.read_text() if root.exists() else ""
                text = fetch.crate_manifest_local(directory, name)
                entry.update(fetch.parse_manifest_facts(text, workspaces[key]))
                entry["features_on"] = sorted(spec.get("features_on") or [])
                entry["features_off"] = dict(spec.get("features_off") or {})
                if spec.get("note"):
                    entry["note"] = spec["note"]
            else:
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
            corpus.get("ref_kind", "tag"),
        )
        for relative, payload in files.items():
            # A `source_prefix` naming a single file strips to nothing, which would write over
            # the destination directory itself. Fall back to the file's own name.
            name = relative or corpus["source_prefix"].rsplit("/", 1)[-1]
            target = destination / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(payload)
        written[corpus["name"]] = len(files)
        say(f"  corpus {corpus['name']:18} {len(files):>4} files -> {corpus['dest']}")
    return written


def apply_crate_aliases(items: dict[str, model.Item], manifest: dict) -> int:
    """Add the access paths created by a Cargo dependency rename.

    A crate's `[lib] name` is what rustdoc records; a `package = "..."` rename is what callers
    type. delta-rs takes `buoyant_kernel` as `delta_kernel`, so every kernel type is written
    `delta_kernel::schema::StructType` in its own source, in its doc examples, and in anything
    an agent is likely to be shown -- while the index knows only `buoyant_kernel::...`. Without
    this pass the name that actually appears in code resolves to nothing.
    """
    aliases = manifest.get("crate_aliases") or {}
    if not aliases:
        return 0
    added = 0
    for item in items.values():
        head, _, rest = item.path.partition("::")
        for spelling in aliases.get(head, []):
            if not rest:
                continue
            alias = f"{spelling}::{rest}"
            if alias not in item.aliases:
                item.aliases.append(alias)
                added += 1
        item.aliases = sorted(set(item.aliases))
    return added


def module_page(item: model.Item) -> str:
    return emit.api_file(item.module)


def _clean(text: str) -> str:
    return text.replace("\t", " ").replace("\n", " ")


def write_indexes(
    items: dict[str, model.Item],
    facts: dict[str, dict],
    public_modules: set[str],
    out: Path,
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

    counts["coverage"] = write_coverage(facts, out)
    counts["unnameable"] = write_unnameable(items, public_modules, out)
    return counts


def write_unnameable(items: dict[str, model.Item], public: set[str], out: Path) -> int:
    """Items that exist and appear in public signatures but cannot be named by a caller.

    A type defined in a private module with no re-export is real: a public method returns it,
    and you can call methods on it and await it. You just cannot `use` it, name it in a
    signature, or write a function that returns it. rustdoc compounds this by emitting the type
    without its impls, so it also looks like it has no methods.

    Neither half of that is visible from `symbols.tsv`, where such a type sits looking ordinary
    and empty. This table is the difference between "this builder takes no options" and "this
    builder's options were not emitted".
    """
    rows = [
        "\t".join((item.path, item.kind, item.crate, str(len(item.methods))))
        for item in items.values()
        if not item.aliases and item.module not in public
    ]
    (out / "unnameable.tsv").write_text("\n".join(sorted(rows)) + "\n")
    return len(rows)


def write_foreign_impl_index(
    items: dict[str, model.Item],
    hidden_impls: list[tuple[str, str]],
    out: Path,
) -> int:
    """Every implementation of a trait defined outside the indexed crates, complete.

    `impls.tsv` carries only edges whose *trait* is in this index, so the integration surface --
    the traits belonging to DataFusion, Arrow, object_store and the kernel -- has no greppable
    record there at all. The catalog summarises it but truncates each row to four names, and for
    private implementors the catalog is otherwise the only record, which would leave a type like
    `MergeValidationExec` provably present and impossible to find.

    The `nameable` column is the distinction that matters when reading a row: `no` means the
    implementation is real and runs, but the type is behind a private module, so it is machinery
    you reach through a public return value rather than an extension point you can substitute.
    """
    local = {item.crate.replace("-", "_") for item in items.values()}

    def foreign(trait_path: str) -> bool:
        root = trait_path.split("::", 1)[0]
        return root not in local and trait_path not in items

    rows = {
        (trait_path, item.path, "yes", item.crate)
        for item in items.values()
        for trait_path in item.implements
        if foreign(trait_path)
    }
    rows.update(
        (trait_path, implementor, "no", implementor.split("::", 1)[0].replace("_", "-"))
        for trait_path, implementor in hidden_impls
        if foreign(trait_path)
    )
    ordered = sorted("\t".join(row) for row in rows)
    (out / "foreign-impls.tsv").write_text("\n".join(ordered) + "\n")
    return len(ordered)


def write_coverage(facts: dict[str, dict], out: Path) -> int:
    """One row per feature of every documented crate, saying whether it was on and why not.

    This table exists because `--all-features` is impossible for this library: `rustls` and
    `native-tls` are mutually exclusive. Once a build documents a *subset*, absence from
    `symbols.tsv` stops meaning "does not exist" and starts meaning "not in this envelope" --
    and nothing in the index itself distinguishes the two. So the distinction is written down.

    `on` is what cargo actually resolved, not what the manifest asked for: resolver 3 unifies
    upward, and recording the request instead of the result would understate the surface.
    """
    rows: list[str] = []
    for crate, entry in sorted(facts.items()):
        declared = dict(entry.get("features") or {})
        defaults = set(entry.get("default_features") or [])
        on = set(entry.get("features_on") or [])
        reasons = entry.get("features_off") or {}
        optional = set(entry.get("optional_dependencies") or [])
        names = set(declared) | optional | on
        for feature in sorted(names):
            if feature == "default":
                continue
            if on:
                state = "on" if feature in on else "off"
            else:
                # A docs.rs crate set records no resolved set; defaults are all we can assert.
                state = "default" if feature in defaults else "unknown"
            reason = reasons.get(feature, "-")
            if state == "off" and reason == "-":
                reason = "not enabled by this envelope"
            enables = ",".join(declared.get(feature, [])) or "-"
            rows.append("\t".join((crate, feature, state, enables, _clean(reason))))
    (out / "coverage.tsv").write_text("\n".join(sorted(rows)) + "\n")
    return len(rows)


def write_unresolved(unresolved: list[str], out: Path) -> int:
    """Access paths whose target lies outside the indexed set.

    These were previously counted and thrown away, which makes a dangling re-export invisible:
    the type is named in dozens of signatures, absent from every index, and nothing says why.
    Writing them down turns that into a greppable row -- and into something a check can assert.
    """
    rows = sorted({entry.replace(" -> ", "\t", 1) for entry in unresolved})
    (out / "unresolved.tsv").write_text("\n".join(rows) + "\n")
    return len(rows)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Build the delta-rs capability repository.")
    parser.add_argument("--manifest", type=Path, default=HERE / "manifests" / "deltalake.json")
    parser.add_argument("--stage", choices=("model", "content", "all"), default="all")
    parser.add_argument("--content", type=Path, default=CONTENT)
    args = parser.parse_args(argv)

    manifest = load_manifest(args.manifest)
    inputs.verify(SKILL_ROOT)
    cache = fetch.Cache(CACHE)
    content = args.content

    pairs = crate_versions(manifest)
    say(f"Building {manifest['repository']['name']} from {len(pairs)} crates")
    models = build_models(manifest, cache)

    items, alias_index, unresolved, hidden_impls = model.stitch(models)
    say(f"\nstitched: {len(items)} canonical items, {len(alias_index)} carrying aliases")

    renamed = apply_crate_aliases(items, manifest)
    if renamed:
        say(f"crate renames added {renamed} access paths")

    say("\nreading crate manifests for features")
    facts = collect_crate_facts(manifest, cache)

    public_modules = {path for record in models.values() for path in record.public_modules}
    counts = write_indexes(items, facts, public_modules, content / "index")
    counts["unresolved"] = write_unresolved(unresolved, content / "index")
    counts["foreign_impls"] = write_foreign_impl_index(items, hidden_impls, content / "index")
    say("index rows: " + ", ".join(f"{k}={v}" for k, v in sorted(counts.items())))

    if args.stage == "model":
        sys.stdout.write(json.dumps({"canonical_items": len(items), "index": counts}) + "\n")
        return 0

    grouped = emit.group_by_module(items)
    say(f"\nwriting {len(grouped)} module pages")
    emit.write_model(grouped, content)
    emit.write_api(grouped, content)

    say("preserving full git-capture member contracts")
    full_models = contracts.acquire_contracts(models, items, crate_specs(manifest), acquired_root)
    counts.update(
        contracts.write(full_models, items, content, SKILL_ROOT / "authoring/contract-notes.json")
    )

    say("fetching corpora")
    corpora = write_corpora(manifest, cache, content)

    say("deriving symbol-to-example edges with ast-grep")
    # This library ships eight example files. Its integration suite is an order of magnitude
    # larger and is where the real usage evidence is, so both are searched -- and the corpus
    # keeps them in separate directories so a page can say which kind of evidence it found.
    usage_roots = [content.joinpath("corpus", "examples"), content.joinpath("corpus", "tests")]
    usage_roots = [root for root in usage_roots if root.exists()]
    try:
        demonstrated: dict = {}
        registered: dict = {}
        for root in usage_roots:
            for trait, files in link.trait_implementations(root, content).items():
                demonstrated.setdefault(trait, []).extend(files)
            registered.update(link.registrations(root))
        say(
            f"  {len(demonstrated)} traits implemented across {len(usage_roots)} corpus roots, "
            f"{len(registered)} distinct registration calls"
        )
        if len(demonstrated) < 5:
            # Saying "nothing demonstrates this" because the search was misaimed is the same
            # failure as saying a capability does not exist because the index was incomplete.
            say("  ! suspiciously few edges -- check the corpus paths before trusting the pages")
    except link.AstGrepMissing as error:
        # Structural linking is a real capability, not a nice-to-have. Say so rather than
        # emitting pages that silently claim no example demonstrates anything.
        say(f"  ! skipped: {error}")
        demonstrated, registered = {}, {}

    say("writing extension-point pages")
    trait_pages = traits.write_traits(items, content, demonstrated)
    say(f"  {trait_pages} pages")

    say("writing the generated catalogs")
    catalog_counts = catalogs.write_all(items, facts, hidden_impls, content)
    say("  " + ", ".join(f"{k}={v}" for k, v in sorted(catalog_counts.items())))

    say("writing capability topic pages")
    topic_pages = topics.write_topics(items, content, HERE.joinpath("topics.json"))
    say(f"  {topic_pages} topics")

    counts.update(routes.write(SKILL_ROOT, content))
    catalog_counts["awaitable_builder_subset"] = catalog_counts.pop("operations")
    catalog_counts["feature_variants_before_name_union"] = catalog_counts.pop("table_features")
    capture = fetch.acquisition(acquired_root(manifest["crate_sets"][0]))
    (content / "profile.json").write_text(
        json.dumps(
            {
                "documentation_capture": capture,
                "runtime_profile": "skill_improvement/evidence/implementation/runtime-profile.json",
                "runtime_receipt": "skill_improvement/evidence/implementation/probe-results.json",
                "distinction": (
                    "Documentation capture is broader than the local runtime prof"
                    "ile; package build features and table protocol features are "
                    "separate."
                ),
                "version_label": (
                    "The 1.0.0+58f07cd6 model label identifies the git capture, n"
                    "ot each package's published version. Consult captured manife"
                    "sts and runtime profile."
                ),
                "corpus_lock": "build/inputs.lock.json",
            },
            indent=2,
        )
        + "\n"
    )
    for folder in ["integration", "licenses"]:
        shutil.copytree(SKILL_ROOT / "authoring" / folder, content / folder, dirs_exist_ok=True)
    counts.update(capabilities.write(SKILL_ROOT, content))

    say("generating rules from the model")
    crate_roots = sorted(
        {name.replace("-", "_") for name, _ in pairs}
        | {
            spelling
            for spellings in (manifest.get("crate_aliases") or {}).values()
            for spelling in spellings
        }
    )
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
        **{f"catalog_{k}": v for k, v in catalog_counts.items()},
        "traits_demonstrated_by_examples": len(demonstrated),
        "registration_calls_demonstrated": len(registered),
    }
    emit.write_provenance(content, manifest, facts, totals, tool_versions)

    say("\nwrote PROVENANCE.json")
    sys.stdout.write(json.dumps(totals, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

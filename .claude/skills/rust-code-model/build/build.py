"""Generate `content/` from the acquired inputs. Offline: no network, no cargo build.

Stages, in order:

    assert pins     refuse to run unless the toolchain and the crate set are the pinned ones
    models          rustdoc JSON -> canonical items, stitched across crates
    indexes         line-oriented projections for ripgrep
    pages           model/ and api/ per module
    vocabulary      MIR, SyntaxKind and dataflow catalogues from pinned source
    probes          execute rustc / cargo / rust-analyzer and record what happened
    router          questions.tsv and layers.tsv -- which layer answers which question
    corpus          the pinned source and prose, verbatim
    provenance      pins, counts, readings, per-file sha256

`acquire.py` is never imported here. That separation is what makes the determinism check
meaningful: if acquisition could run inside the build, a rebuild could pick up different bytes
from the network and still look reproducible.
"""

from __future__ import annotations

import json
import shutil
import sys
from datetime import UTC, datetime
from pathlib import Path

import emit
import fetch
import model
import oracles
import probes as probe_module
import router as router_module
import vocab as vocab_module

HERE = Path(__file__).resolve().parent
SKILL = HERE.parent
MANIFEST = HERE / "manifests" / "rust-code-model.json"
ACQUIRED = HERE / "acquired"
CONTENT = SKILL / "content"


class BuildError(RuntimeError):
    """The build cannot proceed from what it found."""


def say(message: str) -> None:
    sys.stdout.write(f"{message}\n")
    sys.stdout.flush()


def _clean(text: str) -> str:
    return text.replace("\t", " ").replace("\n", " ")


# --------------------------------------------------------------------------- pins


def assert_pins(manifest: dict, acquisition: dict) -> dict[str, object]:
    """Refuse to build against anything but the pinned world, and record what was found.

    Two different jobs share this function. The assertions stop a build that would silently
    describe a different compiler or a different crate release. The readings are kept even when
    they disagree with each other, because the disagreements are among the most useful facts
    here -- see the `versions` block in PROVENANCE.
    """
    tools = manifest["tools"]
    toolchain = tools["toolchain"]

    reported = oracles.rustc_version(toolchain)
    if not oracles.rustc_asserts(toolchain, tools["rustc_release"], tools["rustc_commit"]):
        raise BuildError(
            f"{toolchain} reports {reported!r}, which is not the pinned "
            f"{tools['rustc_release']} ({tools['rustc_commit']}). Every MIR, THIR and dataflow "
            f"observation in this repository was made with that compiler, and the rustc source "
            f"indexed alongside them is fetched at that exact commit. A different compiler would "
            f"produce an index whose observed half and documented half describe different "
            f"programs."
        )

    emitted = oracles.rustdoc_format_emitted(toolchain)
    if emitted != tools["rustdoc_format_version"]:
        raise BuildError(
            f"{toolchain} emits rustdoc format {emitted}, not the pinned "
            f"{tools['rustdoc_format_version']}."
        )

    ast_grep = oracles.ast_grep_version()
    if tools["ast_grep_version"] not in ast_grep:
        raise BuildError(
            f"ast-grep reports {ast_grep!r}, not the pinned {tools['ast_grep_version']}. The "
            f"rule corpus and its snapshots are version-sensitive."
        )

    served = acquisition["format_versions_served"]
    return {
        "toolchain": toolchain,
        "rustc": reported,
        "rustdoc_format_emitted": emitted,
        "rustdoc_format_served_by_docs_rs": served,
        "ast_grep": ast_grep,
        "python": sys.version.split()[0],
        # Read from the compiler rather than written down, so the view catalogue cannot drift
        # away from the compiler that produced the observations in it.
        "unpretty_views": oracles.rustc_unpretty_values(toolchain),
        # Recorded, never asserted: no published mapping relates this to a ra_ap_* version.
        "rust_analyzer": oracles.rust_analyzer_report(),
        "cargo": oracles.cargo_metadata_schema(),
        "toolchains_installed": oracles.toolchain_roster(),
    }


# --------------------------------------------------------------------------- models


def crate_specs(manifest: dict) -> list[dict]:
    specs: list[dict] = []
    for crate_set in manifest["crate_sets"]:
        for entry in crate_set["crates"]:
            spec = dict(entry)
            spec["version"] = crate_set["version"]
            spec["crate_set"] = crate_set["name"]
            spec["lib"] = spec.get("lib") or spec["package"].replace("-", "_")
            specs.append(spec)
    return specs


def build_models(manifest: dict) -> dict[str, model.CrateModel]:
    """Parse every acquired rustdoc document into a canonical item table."""
    supported = manifest["tools"]["rustdoc_format_supported"]
    models: dict[str, model.CrateModel] = {}
    for spec in crate_specs(manifest):
        package, version = spec["package"], spec["version"]
        archive = ACQUIRED / "rustdoc" / f"{package}@{version}.json.zst"
        if not archive.exists():
            raise BuildError(
                f"{archive.name} is missing. Run `python3 build/acquire.py` first; it is the "
                f"only stage that uses the network."
            )
        payload = fetch.decompress_zstd(archive.read_bytes())
        record = model.build_crate(package, version, payload, supported)
        models[package] = record
        say(
            f"  {package:24} {version:10} format={record.format_version} "
            f"items={len(record.items):>5} aliases={len(record.aliases):>4}"
        )
    return models


def collect_crate_facts(manifest: dict, acquisition: dict) -> dict[str, dict]:
    """Read each crate's published manifest for the feature table rustdoc JSON never carries."""
    facts: dict[str, dict] = {}
    for spec in crate_specs(manifest):
        package = spec["package"]
        path = ACQUIRED / "manifests" / f"{package}.Cargo.toml"
        entry = fetch.parse_manifest_facts(path.read_text(encoding="utf-8"))
        entry["version"] = spec["version"]
        entry["layer"] = spec.get("layer", "unassigned")
        entry["role"] = spec.get("role", "primary")
        entry["crate_set"] = spec["crate_set"]
        entry["format_version_served"] = acquisition["crates"][package]["format_version"]
        entry["index_items_served"] = acquisition["crates"][package]["index_items"]
        facts[package] = entry
    return facts


# --------------------------------------------------------------------------- indexes


def write_indexes(
    items: dict[str, model.Item],
    facts: dict[str, dict],
    public_modules: set[str],
    layer_of: dict[str, str],
    out: Path,
) -> dict[str, int]:
    """Write the line-oriented projections. Tab separated, sorted, no header row."""
    out.mkdir(parents=True, exist_ok=True)
    counts: dict[str, int] = {}

    # `layer` is this repository's own column: it is what lets an agent narrow a symbol search to
    # the layer that answers its question instead of reading eleven crates' worth of names.
    symbol_rows = [
        "\t".join(
            (
                item.path,
                item.kind,
                item.crate,
                layer_of.get(item.crate, "unassigned"),
                emit.api_file(item.module),
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

    # Feature gating is absent from rustdoc JSON entirely -- formats 57 to 61 record only an
    # opaque marker, never `cfg(feature = ...)` -- so the manifest is the only evidence.
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

    unnameable_rows = [
        "\t".join((item.path, item.kind, item.crate, str(len(item.methods))))
        for item in items.values()
        if not item.aliases and item.module not in public_modules
    ]
    (out / "unnameable.tsv").write_text("\n".join(sorted(unnameable_rows)) + "\n")
    counts["unnameable"] = len(unnameable_rows)

    return counts


def write_unresolved(unresolved: list[str], out: Path) -> int:
    """Access paths whose target lies outside the indexed set -- a boundary, not a gap."""
    rows = sorted({entry.replace(" -> ", "\t", 1) for entry in unresolved})
    (out / "unresolved.tsv").write_text("\n".join(rows) + "\n")
    return len(rows)


def write_foreign_impls(
    items: dict[str, model.Item],
    hidden_impls: list[tuple[str, str]],
    out: Path,
) -> int:
    """Implementations of traits defined outside the indexed crates.

    `impls.tsv` carries only edges whose trait is in this index, so every implementation of a
    std or third-party trait -- `Iterator`, `AstNode`, `Visitor` -- would otherwise have no
    greppable record. The `nameable` column separates an extension point you can substitute from
    machinery you can only reach through a public return value.
    """
    local = {item.crate.replace("-", "_") for item in items.values()}

    def foreign(trait_path: str) -> bool:
        return trait_path.split("::", 1)[0] not in local and trait_path not in items

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


# --------------------------------------------------------------------------- corpus


def write_corpus(manifest: dict, content: Path) -> dict[str, int]:
    """Copy the pinned source and prose verbatim.

    This is the rung an agent reaches when the index says a thing exists and the question is
    what it actually does. For the rustc layers it is not a convenience: there is no rustdoc
    JSON for any compiler-internal crate anywhere, so these files are the API description.
    """
    destination = content / "corpus"
    if destination.exists():
        shutil.rmtree(destination)
    counts: dict[str, int] = {}
    for source in manifest["sources"]:
        origin = ACQUIRED / "source" / source["dest"]
        target = destination / source["dest"]
        target.mkdir(parents=True, exist_ok=True)
        for path in sorted(origin.iterdir()):
            shutil.copyfile(path, target / path.name)
        counts[f"corpus_{source['dest']}"] = len(list(origin.iterdir()))
    return counts


# --------------------------------------------------------------------------- entry point


def main() -> int:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    acquisition_path = ACQUIRED / "ACQUISITION.json"
    if not acquisition_path.exists():
        raise BuildError(
            "build/acquired/ACQUISITION.json is missing. Run `python3 build/acquire.py` first."
        )
    acquisition = json.loads(acquisition_path.read_text(encoding="utf-8"))

    say(f"asserting pins for {manifest['repository']}")
    readings = assert_pins(manifest, acquisition)
    say(f"  {readings['rustc']}")
    say(
        f"  rustdoc emits format {readings['rustdoc_format_emitted']}; docs.rs served "
        f"{readings['rustdoc_format_served_by_docs_rs']}"
    )

    if CONTENT.exists():
        shutil.rmtree(CONTENT)
    CONTENT.mkdir(parents=True)
    index_dir = CONTENT / "index"
    index_dir.mkdir(parents=True)

    say("\nbuilding crate models")
    models = build_models(manifest)
    items, alias_index, unresolved, hidden_impls = model.stitch(models)
    say(f"stitched: {len(items)} canonical items, {len(alias_index)} carrying aliases")

    facts = collect_crate_facts(manifest, acquisition)
    layer_of = {package: entry["layer"] for package, entry in facts.items()}
    public_modules = {path for record in models.values() for path in record.public_modules}

    counts = write_indexes(items, facts, public_modules, layer_of, index_dir)
    counts["unresolved"] = write_unresolved(unresolved, index_dir)
    counts["foreign_impls"] = write_foreign_impls(items, hidden_impls, index_dir)

    say("\nwriting module pages")
    grouped = emit.group_by_module(items)
    emit.write_model(grouped, CONTENT)
    emit.write_api(grouped, CONTENT)
    counts["modules"] = len(grouped)

    say("\nreading pinned source for the vocabulary catalogues")
    counts.update(vocab_module.write_all(ACQUIRED / "source", index_dir))

    say("\nexecuting behaviour probes")
    probe_results = probe_module.run_all(HERE)
    counts["behaviours"] = probe_module.write_index(probe_results, index_dir)
    summary = probe_module.summarise(probe_results)
    say(f"  {summary}")

    say("\nwriting the router")
    counts.update(router_module.write_all(HERE, index_dir, readings, facts, probe_results))

    say("\ncopying corpus")
    counts.update(write_corpus(manifest, CONTENT))

    say("\nrendering pages")
    import pages as pages_module

    counts.update(pages_module.write_all(HERE, CONTENT, items, probe_results, readings, facts))

    write_provenance(manifest, acquisition, readings, facts, counts)
    say("\nindex rows: " + ", ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    say(f"content: {len(list(CONTENT.rglob('*')))} paths")
    return 0


def write_provenance(
    manifest: dict,
    acquisition: dict,
    readings: dict[str, object],
    facts: dict[str, dict],
    counts: dict[str, int],
) -> None:
    """Record the pins, the counts, and the readings -- including the ones that disagree."""
    tools = manifest["tools"]
    provenance = {
        "repository": manifest["repository"],
        "generated_at": datetime.now(UTC).strftime("%Y-%m-%d"),
        "tools": readings,
        # Four readings of "which version", side by side and deliberately unreconciled. Each
        # answers a different question, and collapsing them into one number would destroy the
        # only evidence a reader has that the question is ambiguous at all.
        "versions": {
            "rustdoc_format_emitted_locally": readings["rustdoc_format_emitted"],
            "rustdoc_format_served_by_docs_rs": readings["rustdoc_format_served_by_docs_rs"],
            "rustdoc_types_crate": facts["rustdoc-types"]["version"],
            "rustdoc_types_served_at_format": facts["rustdoc-types"]["format_version_served"],
            "rust_analyzer_binary": readings["rust_analyzer"],
            "ra_ap_crates": facts["ra_ap_hir"]["version"],
            "ra_ap_source_tag": next(
                s["source_tag"] for s in manifest["crate_sets"] if s["name"] == "rust-analyzer"
            ),
            "ra_ap_tag_resolution": next(
                s["tag_resolution"] for s in manifest["crate_sets"] if s["name"] == "rust-analyzer"
            ),
            "cargo_metadata_schema": readings["cargo"],
            "note": (
                "rustdoc-types 0.61.0 defines FORMAT_VERSION = 61 and docs.rs serves its own "
                "documentation at format 60, because docs.rs built it before its fleet moved. "
                "The crate that defines the version number does not carry it. Nothing here "
                "reconciles that; branch on each payload's own format_version field."
            ),
        },
        "pins": {
            "toolchain": tools["toolchain"],
            "rustc_commit": tools["rustc_commit"],
            "crate_sets": {s["name"]: s["version"] for s in manifest["crate_sets"]},
            "sources": {
                name: {"repo": entry["repo"], "ref": entry["ref"], "ref_kind": entry["ref_kind"]}
                for name, entry in acquisition["sources"].items()
            },
        },
        "counts": counts,
        "crates": facts,
        "files": emit.digest_tree(CONTENT),
    }
    (CONTENT / "PROVENANCE.json").write_text(
        json.dumps(provenance, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BuildError as error:
        sys.stderr.write(f"build failed: {error}\n")
        raise SystemExit(1) from error

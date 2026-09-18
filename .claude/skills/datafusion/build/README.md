# The builder

Reader/research packaging, evidence scope and update workflow are in [maintenance.md](../maintenance.md).
The builder now preserves full member/module/field/variant contracts and renders reviewed task routes.
`content/operations` keeps raw type trees separate from display signatures; diagnostics retain unresolved
links rather than inventing paths. `authoring/` owns curated decisions; generated pages are never edited.

Additional checks: `python3 test_contracts.py` (all-record raw preservation plus reader behavior),
`python3 test_identity.py` (artifact-ID renumbering), and `../scripts/run_probes.py` (locked runtime
assertions and registry capture). These are distinct from navigation regexes and deterministic output.


Regenerates `content/` and the generated rules from pinned upstream sources.

```bash
python3 build.py                      # uses exact cached inputs; missing inputs are downloaded into .cache/
python3 build.py --stage model        # indexes only, skip pages and corpora
python3 verify.py                     # rebuild, then five checks
python3 verify.py --skip-rebuild      # check recorded digests only
```

## Dependencies

Python 3.11+ standard library, plus the `ast-grep` binary for the structural stages. Nothing else
— no pip install, no PyYAML (rule files are emitted by templating and validated by `ast-grep
test`), and no import from any host repository. `verify.py` enforces that last property.

Zstd comes from `compression.zstd` on Python 3.14+, falling back to the `zstd` CLI.

## Stages

| Module | Does |
|---|---|
| `fetch.py` | docs.rs rustdoc JSON, crates.io `.crate` manifests, GitHub tarballs. Everything cached under `.cache/` keyed by the exact pin, so a rebuild is offline. |
| `model.py` | Walks each crate into a canonical item table, then stitches re-exports across all 60. |
| `render.py` | rustdoc `Type` trees back into Rust signature text. |
| `link.py` | Runs ast-grep over the example corpus to derive symbol-to-example edges structurally. |
| `contracts.py` | Preserves and renders full contracts, source spans, raw types and artifact-scoped links. |
| `capabilities.py` | Validates authored decisions/evidence, generates task/crate/representation routes and invalidation edges. |
| `licenses.py` | Retains notices from every exact crate archive. |
| `emit.py` | Writes `model/`, `api/`, and `PROVENANCE.json`. |
| `traits.py` | Writes one page per extension point. |
| `topics.py` | Writes one page per capability axis, from the seeds in `topics.json`. |
| `catalogs.py` | Restructures the upstream config and function catalogs, and writes the crate map. |
| `queries.py` | Generates rules from the model, plus their tests. |
| `build.py` | Orchestrates, and writes `index/*.tsv`. |
| `verify.py` | Determinism, integrity, rule tests, navigation probes, transferability. |

## Three things that are easy to get wrong here

**Enumerate items from the index, not from the module walk.** A crate that defines a type in a
private module and re-exports it — the dominant DataFusion idiom — never surfaces that type as a
child of a public module. Walking the public tree lost 2,312 of 7,134 items. `model.py` enumerates
from `index` filtered to `crate_id == 0`, and uses the walk only to learn access paths.

**Associated items look exactly like free functions.** `inner` carries `function` for both, and
`paths` lists a method under its owner. The discriminator is membership in the owner's `items`
list, not the shape of the path — a path-shape test only works when rustdoc happened to emit the
parent module, and dropping items on that basis cost 2,312 more.

**Glob re-exports chain.** `datafusion::prelude` globs `datafusion_functions::expr_fn`, whose own
contents are eight further globs. A single expansion pass sees an empty module and drops the
entire prelude, so `stitch` iterates to a fixed point.

**A topic seed that does not resolve must fail the build.** `topics.py` raises rather than
skipping, because a page that quietly drops a type it can no longer find asserts by omission that
the capability does not exist — the exact failure this repository exists to prevent. Leaf names
are ambiguous across 60 crates (`Statistics` alone is four types), so a seed is either unique or
qualified, and ambiguity is an error too.

## Reading the result

`PROVENANCE.json` is the cache-invalidation key and the honesty record: crate versions, rustdoc
`format_version`, ast-grep version, per-crate docs.rs configuration (including the four crates
documented *without* `all-features`), counts, and a sha256 per generated file.

## Pointing it at something else

`manifests/datafusion.json` names the crate sets, versions, and corpora. A different library needs
a different manifest, not a different builder. The corpora entries are optional; without them the
structural linking stage reports that it was skipped rather than emitting pages that silently
claim nothing is demonstrated.

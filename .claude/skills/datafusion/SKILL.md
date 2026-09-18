---
name: datafusion
description: Choose and implement built-in DataFusion, Arrow, Parquet and object_store capabilities using task and crate routes, reviewed input/output contracts, exact-release API documentation and executable evidence. Use for query planning, Arrow kernels, providers, functions, schemas, storage and interoperability, including checking whether a custom implementation has a built-in alternative.
---

# DataFusion and Arrow reference

This offline reference supports choosing built-ins before designing an implementation. Start with
the task or data representation; retrieve the exact operation contract when its semantics matter.
The reference pins **DataFusion 55.1.0, Arrow/Parquet 59.3.0, object_store 0.13.2 and sqlparser
0.62.0**. Check the consumer's Cargo.lock and features before transferring a contract. Context7
and current-branch examples are discovery leads; an older upgrade guide is not proof for this pin.

## Find a suitable built-in

Commands below run from the skill directory. Scripts locate their resources relative to their own
file, so an absolute script path also works from another working directory. Python 3.11+ standard
library is sufficient for reading; no service, installation, source cache or network is required.

```bash
python3 scripts/reference.py find --task 'gather duplicate ordered indices'
python3 scripts/reference.py find --task 'blocking sort memory budget'
python3 scripts/reference.py find --crate arrow-cast --property errors
python3 scripts/reference.py show arrow.select
python3 scripts/reference.py compare arrow.select arrow.filter-reuse
```

Prefer direct files when they answer the question more cheaply:

- [Task routes](content/routes/tasks.md): selection, conversion, keys, execution, sources, pushdown,
  expressions, Parquet, functions, relations, schemas, aggregate/window state, reuse and interchange.
- [Crate roles](content/routes/crates.md): every pinned package, its layer and useful task vocabulary.
- [Representation routes](content/routes/representations.md): arrays/batches, expressions/plans,
  streams, files, state and interchange boundaries.
- [Runtime function registry](skill_improvement/evidence/implementation/runtime-registry.json):
  registered names, aliases and signatures in the explicitly recorded probe feature profile.
- [SQL catalog](content/catalogs/sql-functions.md), [configuration](content/catalogs/config-options.md)
  and [topic map](content/topics/00-map.md): broader discovery beyond the reviewed briefs.

A decision brief names alternatives, decisive conditions, semantic inputs/outputs, implementation
steps, evidence and unknowns. Its recommendations are conditional judgments. Sixteen briefs are
reviewed in depth; the broader API inventory is discovery coverage, not equivalent characterization
of every operation. Search silence does not establish absence.

## Retrieve the implementation contract

```bash
python3 scripts/reference.py show df.pushdown --view contract
python3 scripts/reference.py show datafusion::prelude::DataFrame --member execute_stream --view contract
python3 scripts/reference.py show arrow_cast::cast::CastOptions --member safe
rg -i 'pushdown' content/index/operations.tsv
rg -i 'expr_fn.*coalesce' content/index/symbols.tsv
```

`show` resolves facade aliases to defining paths and retains distinct implementation contexts.
`--view contract` includes complete upstream docs, raw rustdoc type trees, source spans, artifact
identity and resolved documentation links. Member/field/variant pages live under
`content/operations/`; module documentation lives under `content/modules/`. Older `content/api/`
pages and indexes remain useful compact summaries. A display signature is a projection, not the
authority when it contains a placeholder or expanded async-trait types.

Output defaults to 12,000 bytes. Larger results return an explicit text fragment and `next_offset`;
repeat the same command/view with `--offset` to continue, or read the named file. Do not treat a
partial result as the complete contract. Facets are lexical filters, not inferred type compatibility.

Before committing to a composition, read the relevant details: null and duplicate behavior,
ordering, projection/filter/limit sequence, schema/metadata propagation, errors versus panics,
scalar/array shape, ownership, state and feature/registration requirements. The brief identifies
which of these change the choice; the full contract supplies exact signatures and preconditions.

## Evidence and deeper discovery

[Probe receipts](skill_improvement/evidence/implementation/probe-results.json) and the adjacent
sources, lockfile, feature profile and logs distinguish executed assertions from source-backed
claims. A successful small fixture does not establish whole-query RSS, cloud I/O, performance or
cross-version compatibility. Hosted rustdoc coverage also depends on its build configuration;
see `content/PROVENANCE.json` and `content/index/features.tsv`. Config setter candidates are
name-based leads, not verified setter-to-setting mappings.

For unresolved behavior, inspect selected exact source with
[scripts/source_evidence.py](scripts/source_evidence.py), or search the upstream examples:

```bash
ast-grep run -l rust -p 'impl $TRAIT for $TYPE { $$$ }' content/corpus/examples
ast-grep scan -c queries/sgconfig.yml --filter '^project-' /path/to/consumer
```

Structural rules find candidates. They do not resolve receiver types or prove defects: another
async `collect` also matches. Use semantic resolution only when identity/dispatch is consequential.
Read [reference.md](reference.md) for schemas and queries, and [maintenance.md](maintenance.md) for
regeneration, targeted review, reproducible probes and portable reader/research bundles.

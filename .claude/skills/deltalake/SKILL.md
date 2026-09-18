---
name: deltalake
description: Choose built-in Delta Lake operations, kernel interfaces and storage/catalog adapters from task routes, reviewed input/output/effect contracts, exact git-capture APIs and executed evidence. Use for delta-rs reads, writes, merge, CDF, schema, transactions, protocol features, retention and DataFusion integration; check built-ins before designing custom implementations.
---

# Choose a Delta capability before designing the implementation

This reference targets delta-rs **58f07cd62bfbce3649a7e1c87c696288068ae184** and its buoyant
kernel fork **8ba063f8f84fec222000f66d40d70911d7c79675**, with DataFusion **55.1.0**,
Arrow/Parquet **59.3.0** and object_store **0.13.2**. This is an unpublished git profile,
not a claim about a published deltalake crate or current upstream head. Check the consumer
lockfile before transferring an API claim. See [capture/runtime profiles](content/profile.json).

## Start from the task

From this skill directory; absolute script paths work from any directory:

```sh
python3 scripts/reference.py find --task "replay batch after ambiguous commit error" --limit 2
python3 scripts/reference.py find --task "custom session UDF fallback" --limit 2
python3 scripts/reference.py find --crate deltalake-aws
python3 scripts/reference.py show delta.cdf
python3 scripts/reference.py compare delta.write delta.replace delta.merge
python3 scripts/reference.py show deltalake::DeltaTable --member scan_table --view contract
```

The offline reader uses only Python's standard library. Output has a byte budget; if it returns
`text_fragment` and `next_offset`, repeat with `--offset` to continue. `--view contract` expands
full docs/types; `--view evidence` expands support. No sibling skill or MCP server is required.
See [reader usage](scripts/README.md).

| Need | Route |
|---|---|
| Select without knowing a symbol | [Task decisions](content/routes/tasks.md) |
| Find ownership, features or adapters | [All 13 crate roles](content/routes/crates.md), [feature coverage](content/index/coverage.tsv) |
| Cross a representation boundary | [Input/output routes](content/routes/representations.md), [DataFusion/Arrow seam](content/integration/README.md) |
| Understand mutation and recovery | [Effects](content/routes/effects.md), [commit](content/capabilities/delta.commit.md), [replay](content/capabilities/delta.replay.md) |
| Find constructors and awaited/build/flush results | [Operation map](content/catalogs/operations.md) |
| Decide feature support | [Protocol matrix](content/catalogs/table-features.md) |
| Locate integration traits | [Integration routes](content/catalogs/integrations.md) |
| Search beyond reviewed families | [Full operations](content/index/operations.tsv), [aliases](content/index/aliases.tsv) |

## Read a decision as a contract

Each brief names conditional alternatives, inputs/outputs, effects, failure boundaries,
implementation considerations, evidence and unknowns. Eighteen reviewed families are selective
depth; the captured API is wider. Source observations, runtime assertions and recommendations
are different forms of evidence.

Before coding establish table/snapshot state, actual Arrow representation, Cargo/table-feature
prerequisites, session/planner/store context, result/error shape and publication boundary. Use
linked member contracts for consuming `self`, lifetimes, trait imports, associated future outputs
and defaults.

Canonical paths are identities, not automatically legal imports. **Returned-inferred** builders
such as LoadBuilder are callable through public return values; their private module paths cannot
be imported. **Internal trait** methods are implementation evidence, not caller APIs. Prefer public
operations over internal plans. Integration routes are entry leads, not verified call graphs.

## Pin-specific findings that change design choices

- Providers retain snapshots. Refresh and rebuild for current results; an already loaded snapshot
  takes precedence over the provider builder's version option in the probe.
- Delta scans apply log state, mapping and deletion vectors. Raw Parquet can return a different
  relation. A partition-first schema exposed a `scan_table().with_columns` projection-order defect;
  provider/DataFrame name projection worked.
- Delta operation sessions need Delta's planner. A trait wrapper's default fallback can discard
  caller functions; choose and verify its fallback policy.
- Repeating a transaction marker did not suppress sequential append. A hook error occurred after
  a commit was visible. Recovery needs state inspection before retrying.
- `SaveMode::Ignore` appended on the tested existing-table write path. CDF clamped an ending version
  beyond head; do not checkpoint past the observed range.
- Feature recognition, checker admission and operation support differ. Mapping scans worked while
  CDF rejected mapping. Vacuum defaults to **dry_run=false** and **Lite**.

Follow the briefs to assertions and limitations; these are bounded findings at this exact profile.

## Evidence and maintenance

Use [structural queries](queries/README.md) as leads; syntax does not resolve receiver types.
Context7/upstream guides aid discovery, while git captures, source, locks and assertions decide
version-sensitive claims. An empty search is not proof of absence. Cloud/native adapters need
separate environments; local tests do not certify them.

See [maintenance](MAINTENANCE.md) for replay, refresh, invalidation and portable bundles, and the
[implementation report](skill_improvement/IMPLEMENTATION_REPORT.md) for dated results and limits.

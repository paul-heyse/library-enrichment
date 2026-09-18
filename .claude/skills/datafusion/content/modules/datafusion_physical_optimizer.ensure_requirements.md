# `datafusion_physical_optimizer::ensure_requirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.json).

<a id="op-e9a13ff8f1943130f1ccc0ae"></a>
## ensure_requirements

`module` · `datafusion_physical_optimizer::ensure_requirements` · datafusion-physical-optimizer 55.1.0

```rust
mod ensure_requirements
```

Source: `src/ensure_requirements/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

[`EnsureRequirements`](../operations/datafusion_physical_optimizer.ensure_requirements.EnsureRequirements.md#op-70914b7c2b51ad60b1d7a82d) optimizer rule that enforces distribution and
sorting requirements together so that the two never invalidate each other.

This rule replaces the separate `EnforceDistribution` + `EnforceSorting`
rules with a unified approach inspired by Apache Spark's `EnsureRequirements`
and Presto/Trino's `AddExchanges`.

# Motivation

The previous two-rule design (`EnforceDistribution` then `EnforceSorting`)
suffers from non-idempotent composition: `EnforceSorting`'s `pushdown_sorts`
can break distribution invariants established by `EnforceDistribution`,
because `SortExec.preserve_partitioning` couples sorting and distribution
decisions. See <https://github.com/apache/datafusion/issues/21973> for details.

# Architecture

`optimize` runs several tree traversals. The defining property of this
rule is **Phase 2**: a single combined bottom-up pass that resolves
distribution *and* sorting for each node together. The surrounding phases
are independent traversals (top-down join-key reorder, then several
follow-up sort/order rewrites). Some of those could be consolidated
further in a follow-up.

```text
EnsureRequirements::optimize(plan)
│
├─ Phase 1: top-down join-key reorder        (adjust_input_keys_ordering)
│
├─ Phase 2: combined distribution + sorting  (single bottom-up pass)
│   └─ For each node (bottom-up), for each child:
│       Step 1: ensure distribution requirement
│         └─ insert RepartitionExec / CoalescePartitionsExec /
│            SortPreservingMergeExec as needed
│       Step 2: ensure ordering requirement (distribution-aware)
│         └─ insert SortExec with the correct `preserve_partitioning`,
│            with SortPreservingMergeExec on top if needed
│
└─ Phase 3: small follow-up passes (bottom-up unless noted)
    ├─ parallelize_sorts
    ├─ replace_with_order_preserving_variants
    ├─ pushdown_sorts                         (recursive walk)
    └─ replace_with_partial_sort
```

# Key Properties

- **Idempotent across the whole rule**: Running `EnsureRequirements`
  twice produces the same plan. This is the property that fixes
  <https://github.com/apache/datafusion/issues/21973>, where the old
  two-rule pipeline could regress a parallel sort plan into a serial one
  on pass 2.
- **Distribution before sorting**: For each child, distribution is
  resolved before ordering, so sorting decisions always have full
  distribution context.
- **Sort pushdown is implicit**: Phase 2 only adds `SortExec` where the
  child doesn't already satisfy the ordering requirement, so sorts land
  at the deepest valid position without a separate destructive pass.

# Behavior: parallelism via repartitioning

Phase 2 Step 1 inserts `RepartitionExec` to satisfy distribution
requirements. When configuration allows, it also increases parallelism by
repartitioning over otherwise-serial inputs. For example, given two
1-partition inputs feeding an operator that can run with more
parallelism:

```text
┌─────────────────────────────────┐
│          ExecutionPlan          │
└─────────────────────────────────┘
        ▲                 ▲
        │                 │
  ┌───────────┐     ┌───────────┐
  │  batch A  │     │  batch B  │      Input: 2 partitions
  └───────────┘     └───────────┘
```

`EnsureRequirements` inserts a `RepartitionExec` so the operator runs
with three partitions:

```text
┌─────────────────────────────────┐
│          ExecutionPlan          │      Input now has 3 partitions
└─────────────────────────────────┘
        ▲      ▲       ▲
        └──────┼───────┘
               │
┌─────────────────────────────────┐
│       RepartitionExec(3)        │      batches are repartitioned
│           RoundRobin            │
└─────────────────────────────────┘
        ▲                 ▲
  ┌───────────┐     ┌───────────┐
  │  batch A  │     │  batch B  │
  └───────────┘     └───────────┘
```

# Behavior: joint distribution + sorting

Resolving distribution and sorting together lets Phase 2 produce a
parallel sort plan in cases where the two-rule pipeline historically
risked a serial one. Given `Sort(DESC) ← Coalesce ← MultiPartitionSource`,
`EnsureRequirements` rewrites it into:

```text
SortPreservingMergeExec: [a DESC]            (cheap k-way merge of sorted streams)
  SortExec: [a DESC], preserve_partitioning=true   (N sorts run in parallel)
    MultiPartitionSource
```

Each input partition is sorted in parallel, then a `SortPreservingMergeExec`
at the top performs a cheap merge of pre-sorted streams. For TopK queries
(`fetch=K`), each parallel sort only keeps K rows per partition, so total
memory is `N × K` rather than coalescing the entire stream first.

# Behavior: strictest distribution match for joins

Distribution requirements are met in the strictest way. For example, a
hash join with keys `(a, b, c)` requires `Distribution(a, b, c)`. This
can in principle be satisfied by partitioning on any superset of any
subset of `(a, b, c)`, but this rule always partitions on the exact key
tuple `(a, b, c)`. This is sometimes more aggressive than strictly
necessary, but the strictest match helps avoid data skew in joins.

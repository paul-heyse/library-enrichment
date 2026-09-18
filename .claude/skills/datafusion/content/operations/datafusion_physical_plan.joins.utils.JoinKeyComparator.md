# `datafusion_physical_plan::joins::utils::JoinKeyComparator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.JoinKeyComparator.json).

<a id="op-cc7fb6cd04f88af47be9ca0d"></a>
## JoinKeyComparator

`struct` · `datafusion_physical_plan::joins::utils::JoinKeyComparator` · datafusion-physical-plan 55.1.0

```rust
struct JoinKeyComparator
```

Source: `src/joins/utils.rs:2359`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pre-built comparator for join key columns that eliminates per-row type
dispatch. Wraps `arrow_ord::ord::DynComparator` closures built once per
batch pair, used for all row comparisons within those batches.

The first key column is stored separately so that single-column joins
(the common case) avoid Vec iteration entirely, and multi-column joins
short-circuit without entering the loop when the first column is
selective.

Null handling is baked into the closures at construction time:
- `NullEqualsNull`: `make_comparator` returns `Equal` for both-null, which
  is the desired behavior. Closures are used as-is.
- `NullEqualsNothing`: columns where both sides contain nulls get a wrapper
  that returns `Less` for both-null. Columns where one side has no nulls
  skip the wrapper since both-null is impossible.

Because `NullEqualsNothing` wraps comparators to return `Less` for
both-null, `is_equal` will return `false` for both-null rows when that
mode is active. Callers needing both-null == equal semantics (e.g.,
buffered head/tail equality in SMJ) should construct with
`NullEqualsNull`.

<a id="op-2891d05a81958183786cd39b"></a>
## compare

`function` · `datafusion_physical_plan::joins::utils::JoinKeyComparator::compare` · datafusion-physical-plan 55.1.0

```rust
fn compare(&self, left: usize, right: usize) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::JoinKeyComparator", "path": "JoinKeyComparator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2364, 1], "end": [2449, 2], "filename": "src/joins/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/utils.rs:2419`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compare row `left` (in the left arrays) with row `right` (in the right
arrays). Returns the lexicographic ordering across all key columns.

<a id="op-cb98de308ee02c5fe7fccb5f"></a>
## is_equal

`function` · `datafusion_physical_plan::joins::utils::JoinKeyComparator::is_equal` · datafusion-physical-plan 55.1.0

```rust
fn is_equal(&self, left: usize, right: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::JoinKeyComparator", "path": "JoinKeyComparator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2364, 1], "end": [2449, 2], "filename": "src/joins/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/utils.rs:2438`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Check equality of row `left` (in the left arrays) with row `right`
(in the right arrays). Both-null is treated as equal when constructed
with `NullEqualsNull`. With `NullEqualsNothing`, both-null returns
`false` because the override is baked into the comparators.

<a id="op-f224aa21a543616fbbf4e453"></a>
## new

`function` · `datafusion_physical_plan::joins::utils::JoinKeyComparator::new` · datafusion-physical-plan 55.1.0

```rust
fn new(left_arrays: &[ArrayRef], right_arrays: &[ArrayRef], sort_options: &[SortOptions], null_equality: NullEquality) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::JoinKeyComparator", "path": "JoinKeyComparator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2364, 1], "end": [2449, 2], "filename": "src/joins/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/utils.rs:2366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build comparators for each join key column pair.

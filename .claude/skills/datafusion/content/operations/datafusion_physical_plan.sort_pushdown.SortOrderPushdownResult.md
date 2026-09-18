# `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sort_pushdown.SortOrderPushdownResult.json).

<a id="op-ad467981ba84958ccd43d767"></a>
## SortOrderPushdownResult

`enum` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult` · datafusion-physical-plan 55.1.0

```rust
enum SortOrderPushdownResult<T>
```

Source: `src/sort_pushdown.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Result of attempting to push down sort ordering to a node.

Used by [`ExecutionPlan::try_pushdown_sort`] to communicate
whether and how sort ordering was successfully pushed down.

[`ExecutionPlan::try_pushdown_sort`]: crate::ExecutionPlan::try_pushdown_sort

<a id="op-ed428f58d9deea77196e0270"></a>
## Exact

`variant` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::Exact` · datafusion-physical-plan 55.1.0

```rust
Exact
```

Source: `src/sort_pushdown.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The source can guarantee exact ordering (data is perfectly sorted).

When this is returned, the optimizer can safely remove the Sort operator
entirely since the data source guarantees the requested ordering.

<a id="op-d76a90b56fe6654ae567f842"></a>
## Inexact

`variant` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::Inexact` · datafusion-physical-plan 55.1.0

```rust
Inexact
```

Source: `src/sort_pushdown.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The source has optimized for the ordering but cannot guarantee perfect sorting.

This indicates the data source has been optimized (e.g., reordered files/row groups
based on statistics, enabled reverse scanning) but the data may not be perfectly
sorted. The optimizer should keep the Sort operator but benefits from the
optimization (e.g., faster TopK queries due to early termination).

<a id="op-2741d2a02b139c9a94076044"></a>
## Unsupported

`variant` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::Unsupported` · datafusion-physical-plan 55.1.0

```rust
Unsupported
```

Source: `src/sort_pushdown.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The source cannot optimize for this ordering.

The data source does not support the requested sort ordering and no
optimization was applied.

<a id="op-910bd0cdb27896b3e9444141"></a>
## clone

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SortOrderPushdownResult<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "src/sort_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_pushdown.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-486799393c1ab5742c462af4"></a>
## fmt

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/sort_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_pushdown.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8038f36cd1c3b24769282584"></a>
## into_inexact

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::into_inexact` · datafusion-physical-plan 55.1.0

```rust
fn into_inexact(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [120, 2], "filename": "src/sort_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_pushdown.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Convert this result to `Inexact`, downgrading `Exact` if present.

This is useful when an operation (like merging multiple partitions)
cannot guarantee exact ordering even if the input provides it.

# Examples

```
# use datafusion_physical_plan::SortOrderPushdownResult;
let exact = SortOrderPushdownResult::Exact { inner: 42 };
let inexact = exact.into_inexact();
assert!(matches!(inexact, SortOrderPushdownResult::Inexact { inner: 42 }));

let already_inexact = SortOrderPushdownResult::Inexact { inner: 42 };
let still_inexact = already_inexact.into_inexact();
assert!(matches!(still_inexact, SortOrderPushdownResult::Inexact { inner: 42 }));

let unsupported = SortOrderPushdownResult::<i32>::Unsupported;
let still_unsupported = unsupported.into_inexact();
assert!(matches!(still_unsupported, SortOrderPushdownResult::Unsupported));
```

<a id="op-4bcac7c5cdb082b70c9db296"></a>
## into_inner

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::into_inner` · datafusion-physical-plan 55.1.0

```rust
fn into_inner(self) -> Option<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [120, 2], "filename": "src/sort_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_pushdown.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Extract the inner value if present

<a id="op-325057dc99c3d2499263efb7"></a>
## map

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::map` · datafusion-physical-plan 55.1.0

```rust
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> SortOrderPushdownResult<U>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [120, 2], "filename": "src/sort_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_pushdown.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Map the inner value to a different type while preserving the variant.

<a id="op-49db43df1dd2bfc511ea874a"></a>
## try_map

`function` · `datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult::try_map` · datafusion-physical-plan 55.1.0

```rust
fn try_map<U, E, F: FnOnce(T) -> Result<U, E>>(self, f: F) -> Result<SortOrderPushdownResult<U>, E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::sort_pushdown::SortOrderPushdownResult", "path": "SortOrderPushdownResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [120, 2], "filename": "src/sort_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_pushdown.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Try to map the inner value, returning an error if the function fails.

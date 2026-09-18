# `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.FilterPushdownPhase.json).

<a id="op-a860d98a971302895a804c6d"></a>
## FilterPushdownPhase

`enum` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase` · datafusion-physical-plan 55.1.0

```rust
enum FilterPushdownPhase
```

Source: `src/filter_pushdown.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34adb4a7f105a7572c8879af"></a>
## Post

`variant` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::Post` · datafusion-physical-plan 55.1.0

```rust
Post
```

Source: `src/filter_pushdown.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pushdown that happens after most other optimizations.
This stage of filter pushdown allows filters that reference an [`ExecutionPlan`] to be pushed down.
Since subsequent optimizations should not change the structure of the plan tree except for calling [`ExecutionPlan::with_new_children`]
(which generally preserves internal references) it is safe for references between [`ExecutionPlan`]s to be established at this stage.

This phase is used to link a [`SortExec`] (with a TopK operator) or a [`HashJoinExec`] to a `DataSourceExec`.

[`ExecutionPlan`]: crate::ExecutionPlan
[`ExecutionPlan::with_new_children`]: crate::ExecutionPlan::with_new_children
[`SortExec`]: crate::sorts::sort::SortExec
[`HashJoinExec`]: crate::joins::HashJoinExec
[`ExecutionPlan::handle_child_pushdown_result`]: crate::ExecutionPlan::handle_child_pushdown_result

<a id="op-27b8e20c6fb884d4ec337bf5"></a>
## Pre

`variant` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::Pre` · datafusion-physical-plan 55.1.0

```rust
Pre
```

Source: `src/filter_pushdown.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pushdown that happens before most other optimizations.
This pushdown allows static filters that do not reference any [`ExecutionPlan`]s to be pushed down.
Filters that reference an [`ExecutionPlan`] cannot be pushed down at this stage since the whole plan tree may be rewritten
by other optimizations.
Implementers are however allowed to modify the execution plan themselves during this phase, for example by returning a completely
different [`ExecutionPlan`] from [`ExecutionPlan::handle_child_pushdown_result`].

Pushdown of [`FilterExec`] into `DataSourceExec` is an example of a pre-pushdown.
Unlike filter pushdown in the logical phase, which operates on the logical plan to push filters into the logical table scan,
the `Pre` phase in the physical plan targets the actual physical scan, pushing filters down to specific data source implementations.
For example, Parquet supports filter pushdown to reduce data read during scanning, while CSV typically does not.

[`ExecutionPlan`]: crate::ExecutionPlan
[`FilterExec`]: crate::filter::FilterExec
[`ExecutionPlan::handle_child_pushdown_result`]: crate::ExecutionPlan::handle_child_pushdown_result

<a id="op-17405e3fcbe28e132647545c"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> FilterPushdownPhase
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPhase", "path": "FilterPushdownPhase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 17], "end": [48, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7cfb878ec5a2c6aaa88f8e8"></a>
## eq

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &FilterPushdownPhase) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPhase", "path": "FilterPushdownPhase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 30], "end": [48, 39], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/filter_pushdown.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4da11fc13588dad8d284784f"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPhase", "path": "FilterPushdownPhase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [88, 2], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/filter_pushdown.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84389b504850df8c0f3fa32b"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPhase::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPhase", "path": "FilterPushdownPhase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
